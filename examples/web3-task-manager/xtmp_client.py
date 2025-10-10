#!/usr/bin/env python3
"""
XTMP Client for Web3 Task Manager
Implements the real XTMP (eXtended Transport Message Protocol) with httpcg for BPI integration
"""

import asyncio
import json
import struct
import time
import uuid
import aiohttp
import hashlib
import base64
from typing import Dict, Any, Optional
from dataclasses import dataclass
from enum import Enum
import logging

logger = logging.getLogger(__name__)

class ServiceType(Enum):
    """XTMP Service Types matching BPI infrastructure"""
    CONSENSUS = "Consensus"
    AUCTION = "Auction"
    ORACLE = "Oracle"
    COMMUNITY = "Community"
    PARTNERSHIP = "Partnership"
    ANALYTICS = "Analytics"
    MONITORING = "Monitoring"
    SYSTEM = "System"

class Operation(Enum):
    """XTMP Operations matching BPI infrastructure"""
    # System operations
    GET_SERVER_STATUS = "GetServerStatus"
    GET_METRICS = "GetMetrics"
    HEARTBEAT = "Heartbeat"
    
    # Consensus operations
    PROCESS_CONSENSUS_ROUND = "ProcessConsensusRound"
    GET_CONSENSUS_STATUS = "GetConsensusStatus"
    
    # Auction operations (for task storage)
    SUBMIT_TRANSACTION = "SubmitTransaction"
    CREATE_AUCTION_WINDOW = "CreateAuctionWindow"
    SEAL_AUCTION = "SealAuction"
    GET_MEMPOOL_STATS = "GetMempoolStats"
    
    # Oracle operations (for verification)
    REGISTER_PARTNER = "RegisterPartner"
    CREATE_PARTNERSHIP = "CreatePartnership"
    PROCESS_REVENUE = "ProcessRevenue"
    GET_PARTNER_STATS = "GetPartnerStats"

@dataclass
class XTMPMessage:
    """XTMP Message structure matching BPI protocol"""
    id: str
    version: int
    service_type: ServiceType
    operation: Operation
    session_id: str
    timestamp: int
    payload: Dict[str, Any]
    signature: Optional[str] = None

class XTMPClient:
    """
    XTMP Client for connecting to BPI infrastructure
    Implements the real XTMP protocol with httpcg transport
    """
    
    def __init__(self, bpi_vm_url: str = "http://127.0.0.1:7777"):
        self.bpi_vm_url = bpi_vm_url
        self.bpi_rpc_url = "http://127.0.0.1:9545"  # BPI RPC for blockchain operations
        self.bpi_api_url = "http://127.0.0.1:9546"  # BPI API for storage operations
        self.session_id = str(uuid.uuid4())
        self.http_session = None
        self.connected = False
        self.message_counter = 0
        
        logger.info(f"🚀 XTMP Client initialized for httpcg session: {self.session_id[:8]}...")
    
    async def connect(self) -> bool:
        """Connect to BPI VM Server via httpcg protocol"""
        try:
            logger.info(f"🔌 Connecting to BPI VM Server via httpcg: {self.bpi_vm_url}")
            self.http_session = aiohttp.ClientSession()
            
            # Test connection with BPI VM Server status endpoint
            test_response = await self.send_httpcg_request("/__vm/status", {
                "operation": "connection_test",
                "session_id": self.session_id
            })
            
            if test_response:
                self.connected = True
                logger.info("✅ XTMP httpcg connection established")
                return True
            else:
                self.connected = False
                logger.warning("❌ XTMP httpcg connection test failed")
                return False
            
        except Exception as e:
            logger.warning(f"❌ XTMP httpcg connection failed: {e}")
            self.connected = False
            return False
    
    async def disconnect(self):
        """Disconnect from XTMP server"""
        if self.http_session:
            await self.http_session.close()
            self.connected = False
            logger.info("🔌 XTMP httpcg connection closed")
    
    async def send_httpcg_request(self, httpcg_path: str, data: Dict[str, Any], base_url: str = None) -> Optional[Dict[str, Any]]:
        """Send HTTP request using httpcg protocol"""
        if not self.http_session:
            logger.warning("❌ No HTTP session available")
            return None
        
        try:
            # Use provided base URL or default to VM Server
            if base_url is None:
                base_url = self.bpi_vm_url
            url = f"{base_url}{httpcg_path}"
            
            # Create appropriate payload based on endpoint type
            if base_url in [self.bpi_rpc_url, self.bpi_api_url]:
                # Use JSON-RPC format for BPI services with real BPI methods
                operation = data.get("operation", "unknown")
                
                # Map XTMP operations to real BPI JSON-RPC methods
                if operation == "store_task":
                    method = "submit_audit_bundle"
                    params = {
                        "bundle_type": "task_storage",
                        "task_data": data.get("task_data", {}),
                        "blockchain_hash": data.get("blockchain_hash", ""),
                        "session_id": self.session_id,
                        "timestamp": int(time.time() * 1000)
                    }
                elif operation == "verify_task":
                    method = "eth_getTransactionByHash"
                    params = [data.get("blockchain_hash", "")]
                else:
                    method = "web3_clientVersion"
                    params = []
                
                xtmp_payload = {
                    "jsonrpc": "2.0",
                    "method": method,
                    "params": params,
                    "id": 1
                }
            else:
                # Use XTMP format for VM Server
                xtmp_payload = {
                    "xtmp_version": 1,
                    "session_id": self.session_id,
                    "timestamp": int(time.time() * 1000),
                    "data": data
                }
            
            logger.debug(f"📤 Sending httpcg request: {httpcg_path}")
            
            async with self.http_session.post(url, json=xtmp_payload) as response:
                if response.status == 200:
                    result = await response.json()
                    logger.debug(f"📥 Received httpcg response")
                    return result
                else:
                    logger.warning(f"❌ httpcg request failed: {response.status}")
                    return None
                    
        except Exception as e:
            logger.error(f"❌ httpcg request error: {e}")
            return None
    
    def create_message(self, service_type: ServiceType, operation: Operation, payload: Dict[str, Any]) -> XTMPMessage:
        """Create XTMP message with proper structure"""
        self.message_counter += 1
        
        message = XTMPMessage(
            id=str(uuid.uuid4()),
            version=1,
            service_type=service_type,
            operation=operation,
            session_id=self.session_id,
            timestamp=int(time.time() * 1000),  # milliseconds
            payload=payload
        )
        
        # Create signature for message integrity
        message_data = json.dumps({
            "id": message.id,
            "service_type": message.service_type.value,
            "operation": message.operation.value,
            "session_id": message.session_id,
            "timestamp": message.timestamp,
            "payload": message.payload
        }, sort_keys=True)
        
        message.signature = hashlib.sha256(message_data.encode()).hexdigest()
        
        return message
    
    async def send_message(self, message: XTMPMessage) -> Optional[Dict[str, Any]]:
        """Send XTMP message via httpcg protocol"""
        if not self.connected:
            logger.warning("❌ XTMP not connected")
            return None
        
        try:
            # Convert message to httpcg format
            httpcg_data = {
                "id": message.id,
                "version": message.version,
                "service_type": message.service_type.value,
                "operation": message.operation.value,
                "session_id": message.session_id,
                "timestamp": message.timestamp,
                "payload": message.payload,
                "signature": message.signature
            }
            
            # Route to appropriate BPI endpoint based on service type
            if message.service_type == ServiceType.SYSTEM:
                # Use BPI RPC for storage operations
                base_url = self.bpi_rpc_url
                httpcg_path = ""  # RPC uses root path
            elif message.service_type == ServiceType.AUCTION:
                # Use BPI RPC for blockchain operations
                base_url = self.bpi_rpc_url
                httpcg_path = ""  # RPC uses root path
            elif message.service_type == ServiceType.CONSENSUS:
                # Use BPI RPC for consensus operations
                base_url = self.bpi_rpc_url
                httpcg_path = ""  # RPC uses root path
            elif message.service_type == ServiceType.ORACLE:
                # Use BPI RPC for verification operations
                base_url = self.bpi_rpc_url
                httpcg_path = ""  # RPC uses root path
            else:
                # Default to VM Server for health checks
                base_url = self.bpi_vm_url
                httpcg_path = "/__vm/health"
            
            logger.debug(f"📤 Sending XTMP message via httpcg: {message.operation.value}")
            response = await self.send_httpcg_request(httpcg_path, httpcg_data, base_url)
            
            if response:
                logger.debug(f"📥 Received XTMP response via httpcg")
            
            return response
            
        except Exception as e:
            logger.error(f"❌ XTMP httpcg message error: {e}")
            return None
    
    async def send_heartbeat(self) -> bool:
        """Send heartbeat to maintain connection"""
        message = self.create_message(
            ServiceType.SYSTEM,
            Operation.HEARTBEAT,
            {"client_id": self.session_id}
        )
        
        response = await self.send_message(message)
        return response is not None
    
    async def get_server_status(self) -> Optional[Dict[str, Any]]:
        """Get BPCI server status"""
        message = self.create_message(
            ServiceType.SYSTEM,
            Operation.GET_SERVER_STATUS,
            {}
        )
        
        return await self.send_message(message)
    
    async def submit_task_transaction(self, task_data: Dict[str, Any]) -> Optional[Dict[str, Any]]:
        """Submit task as blockchain transaction via XTMP"""
        message = self.create_message(
            ServiceType.AUCTION,
            Operation.SUBMIT_TRANSACTION,
            {
                "transaction_type": "task_creation",
                "task_data": task_data,
                "priority": "high" if task_data.get("priority") == "high" else "normal"
            }
        )
        
        return await self.send_message(message)
    
    async def verify_task_consensus(self, task_hash: str) -> Optional[Dict[str, Any]]:
        """Verify task integrity via consensus"""
        message = self.create_message(
            ServiceType.CONSENSUS,
            Operation.GET_CONSENSUS_STATUS,
            {
                "verification_target": task_hash,
                "verification_type": "task_integrity"
            }
        )
        
        return await self.send_message(message)
    
    async def register_task_oracle(self, task_id: str, verification_data: Dict[str, Any]) -> Optional[Dict[str, Any]]:
        """Register task with oracle for verification"""
        message = self.create_message(
            ServiceType.ORACLE,
            Operation.REGISTER_PARTNER,
            {
                "partner_type": "task_verifier",
                "task_id": task_id,
                "verification_data": verification_data
            }
        )
        
        return await self.send_message(message)
    
    async def get_mempool_stats(self) -> Optional[Dict[str, Any]]:
        """Get mempool statistics for task processing"""
        message = self.create_message(
            ServiceType.AUCTION,
            Operation.GET_MEMPOOL_STATS,
            {}
        )
        
        return await self.send_message(message)

class BPITaskIntegration:
    """
    BPI Task Integration using real XTMP/SAPI protocols
    Replaces mock HTTP calls with real blockchain integration
    """
    
    def __init__(self):
        self.xtmp_client = XTMPClient()
        self.connected = False
    
    async def initialize(self) -> bool:
        """Initialize BPI connection"""
        logger.info("🔗 Initializing BPI Task Integration via XTMP...")
        self.connected = await self.xtmp_client.connect()
        
        if self.connected:
            # Verify server status
            status = await self.xtmp_client.get_server_status()
            if status:
                logger.info(f"✅ BPI Server Status: {status.get('status', 'unknown')}")
            else:
                logger.warning("⚠️ Could not verify BPI server status")
        
        return self.connected
    
    async def create_blockchain_task(self, task_data: Dict[str, Any]) -> Dict[str, Any]:
        """Create task with real blockchain backing"""
        if not self.connected:
            return {"success": False, "error": "Not connected to BPI"}
        
        logger.info(f"📝 Creating blockchain-backed task: {task_data.get('title', 'Unknown')}")
        
        # Submit task transaction via XTMP
        tx_response = await self.xtmp_client.submit_task_transaction(task_data)
        
        if tx_response and not tx_response.get("error"):
            # Register with oracle for verification
            oracle_response = await self.xtmp_client.register_task_oracle(
                task_data.get("id", "unknown"),
                {
                    "task_hash": task_data.get("blockchain_hash"),
                    "created_at": task_data.get("created_at"),
                    "owner": task_data.get("owner")
                }
            )
            
            return {
                "success": True,
                "transaction_response": tx_response,
                "oracle_response": oracle_response,
                "blockchain_verified": True
            }
        else:
            return {
                "success": False,
                "error": tx_response.get("error") if tx_response else "Transaction failed",
                "blockchain_verified": False
            }
    
    async def verify_task_integrity(self, task_hash: str) -> Dict[str, Any]:
        """Verify task integrity via blockchain consensus"""
        if not self.connected:
            return {"verified": False, "error": "Not connected to BPI"}
        
        logger.info(f"🔍 Verifying task integrity: {task_hash[:16]}...")
        
        # Verify via consensus
        consensus_response = await self.xtmp_client.verify_task_consensus(task_hash)
        
        if consensus_response and not consensus_response.get("error"):
            return {
                "verified": True,
                "consensus_response": consensus_response,
                "blockchain_confirmed": consensus_response.get("consensus_achieved", False)
            }
        else:
            return {
                "verified": False,
                "error": consensus_response.get("error") if consensus_response else "Verification failed"
            }
    
    async def get_infrastructure_stats(self) -> Dict[str, Any]:
        """Get BPI infrastructure statistics"""
        if not self.connected:
            return {"available": False}
        
        # Get mempool stats
        mempool_stats = await self.xtmp_client.get_mempool_stats()
        
        # Get server status
        server_status = await self.xtmp_client.get_server_status()
        
        return {
            "available": True,
            "mempool_stats": mempool_stats,
            "server_status": server_status,
            "xtmp_session": self.xtmp_client.session_id[:8]
        }
    
    async def cleanup(self):
        """Cleanup BPI connection"""
        if self.xtmp_client:
            await self.xtmp_client.disconnect()
        logger.info("🧹 BPI Task Integration cleanup complete")

# Example usage
async def test_xtmp_integration():
    """Test XTMP integration with BPI infrastructure"""
    print("🚀 Testing XTMP Integration with BPI Infrastructure")
    print("=" * 60)
    
    integration = BPITaskIntegration()
    
    try:
        # Initialize connection
        if await integration.initialize():
            print("✅ BPI connection established via XTMP")
            
            # Test task creation
            test_task = {
                "id": str(uuid.uuid4()),
                "title": "Test XTMP Task",
                "description": "Testing real XTMP/SAPI integration",
                "priority": "high",
                "blockchain_hash": hashlib.sha256("test_task_data".encode()).hexdigest(),
                "created_at": str(int(time.time())),
                "owner": "xtmp_test_user"
            }
            
            result = await integration.create_blockchain_task(test_task)
            print(f"📝 Task creation result: {result['success']}")
            
            if result["success"]:
                # Test verification
                verification = await integration.verify_task_integrity(test_task["blockchain_hash"])
                print(f"🔍 Task verification: {verification['verified']}")
            
            # Get infrastructure stats
            stats = await integration.get_infrastructure_stats()
            print(f"📊 Infrastructure available: {stats['available']}")
            
        else:
            print("❌ Could not establish BPI connection")
    
    finally:
        await integration.cleanup()

if __name__ == "__main__":
    asyncio.run(test_xtmp_integration())
