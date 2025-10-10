#!/usr/bin/env python3
"""
BPI Infrastructure API Usage Examples
Comprehensive examples for integrating with BPI-BPCI infrastructure
"""

import asyncio
import json
import os
from typing import Dict, List, Optional, Any
import aiohttp
import logging
from datetime import datetime

# Configure logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

class BpiApiClient:
    """
    BPI Infrastructure API Client
    Provides easy-to-use methods for interacting with all BPI services
    """
    
    def __init__(self, 
                 vm_server_url: str = "http://localhost:8080",
                 bpci_bridge_url: str = "http://localhost:8545", 
                 database_url: str = "http://localhost:27017",
                 orchestrator_url: str = "http://localhost:9090"):
        """
        Initialize BPI API Client
        
        Args:
            vm_server_url: VM Server endpoint
            bpci_bridge_url: BPCI Bridge endpoint  
            database_url: 4D Database endpoint
            orchestrator_url: Service Orchestrator endpoint
        """
        self.vm_server_url = vm_server_url
        self.bpci_bridge_url = bpci_bridge_url
        self.database_url = database_url
        self.orchestrator_url = orchestrator_url
        
        logger.info("🔌 BPI API Client initialized")
        logger.info(f"VM Server: {vm_server_url}")
        logger.info(f"BPCI Bridge: {bpci_bridge_url}")
        logger.info(f"4D Database: {database_url}")
        logger.info(f"Orchestrator: {orchestrator_url}")

    # ========================================
    # HEALTH & STATUS EXAMPLES
    # ========================================

    async def check_all_services_health(self) -> Dict[str, bool]:
        """
        Example: Check health of all BPI services
        
        Returns:
            Dict mapping service names to health status
        """
        logger.info("🏥 Checking health of all BPI services...")
        
        services = {
            "vm_server": f"{self.vm_server_url}/health",
            "bpci_bridge": f"{self.bpci_bridge_url}/health", 
            "database": f"{self.database_url}/health",
            "orchestrator": f"{self.orchestrator_url}/health"
        }
        
        health_status = {}
        
        async with aiohttp.ClientSession() as session:
            for service_name, health_url in services.items():
                try:
                    async with session.get(health_url, timeout=5) as response:
                        health_status[service_name] = response.status == 200
                        status_icon = "✅" if response.status == 200 else "❌"
                        logger.info(f"{status_icon} {service_name}: {'Healthy' if response.status == 200 else 'Unhealthy'}")
                except Exception as e:
                    health_status[service_name] = False
                    logger.error(f"❌ {service_name}: Connection failed - {e}")
        
        return health_status

    async def get_system_status(self) -> Dict[str, Any]:
        """
        Example: Get comprehensive system status
        
        Returns:
            System status information
        """
        logger.info("📊 Getting comprehensive system status...")
        
        try:
            async with aiohttp.ClientSession() as session:
                async with session.get(f"{self.orchestrator_url}/api/status") as response:
                    if response.status == 200:
                        status_data = await response.json()
                        logger.info("✅ System status retrieved successfully")
                        return status_data
                    else:
                        logger.error(f"❌ Failed to get system status: {response.status}")
                        return {}
        except Exception as e:
            logger.error(f"❌ System status error: {e}")
            return {}

    # ========================================
    # VM SERVER EXAMPLES
    # ========================================

    async def create_vm_instance(self, vm_config: Dict[str, Any]) -> Optional[str]:
        """
        Example: Create a new VM instance
        
        Args:
            vm_config: VM configuration parameters
            
        Returns:
            VM instance ID if successful, None otherwise
        """
        logger.info("🖥️ Creating new VM instance...")
        
        try:
            async with aiohttp.ClientSession() as session:
                async with session.post(
                    f"{self.vm_server_url}/api/vms/create",
                    json=vm_config,
                    timeout=30
                ) as response:
                    if response.status == 200:
                        result = await response.json()
                        vm_id = result.get("vm_id")
                        logger.info(f"✅ VM instance created: {vm_id}")
                        return vm_id
                    else:
                        logger.error(f"❌ VM creation failed: {response.status}")
                        return None
        except Exception as e:
            logger.error(f"❌ VM creation error: {e}")
            return None

    async def deploy_contract_to_vm(self, vm_id: str, contract_code: str, contract_type: str = "smart_contract") -> Optional[str]:
        """
        Example: Deploy a smart contract to VM
        
        Args:
            vm_id: Target VM instance ID
            contract_code: Contract source code
            contract_type: Type of contract
            
        Returns:
            Contract address if successful, None otherwise
        """
        logger.info(f"📜 Deploying contract to VM {vm_id}...")
        
        try:
            async with aiohttp.ClientSession() as session:
                payload = {
                    "vm_id": vm_id,
                    "contract_code": contract_code,
                    "contract_type": contract_type,
                    "quantum_safe": True
                }
                
                async with session.post(
                    f"{self.vm_server_url}/api/contracts/deploy",
                    json=payload,
                    timeout=60
                ) as response:
                    if response.status == 200:
                        result = await response.json()
                        contract_address = result.get("contract_address")
                        logger.info(f"✅ Contract deployed: {contract_address}")
                        return contract_address
                    else:
                        logger.error(f"❌ Contract deployment failed: {response.status}")
                        return None
        except Exception as e:
            logger.error(f"❌ Contract deployment error: {e}")
            return None

    async def execute_vm_code(self, vm_id: str, code: str, language: str = "python") -> Optional[Dict]:
        """
        Example: Execute code in VM instance
        
        Args:
            vm_id: Target VM instance ID
            code: Code to execute
            language: Programming language
            
        Returns:
            Execution result if successful, None otherwise
        """
        logger.info(f"⚡ Executing code in VM {vm_id}...")
        
        try:
            async with aiohttp.ClientSession() as session:
                payload = {
                    "vm_id": vm_id,
                    "code": code,
                    "language": language,
                    "timeout": 30
                }
                
                async with session.post(
                    f"{self.vm_server_url}/api/vms/{vm_id}/execute",
                    json=payload,
                    timeout=35
                ) as response:
                    if response.status == 200:
                        result = await response.json()
                        logger.info("✅ Code executed successfully")
                        return result
                    else:
                        logger.error(f"❌ Code execution failed: {response.status}")
                        return None
        except Exception as e:
            logger.error(f"❌ Code execution error: {e}")
            return None

    # ========================================
    # BPCI BRIDGE EXAMPLES  
    # ========================================

    async def submit_transaction(self, transaction_data: Dict[str, Any]) -> Optional[str]:
        """
        Example: Submit a blockchain transaction
        
        Args:
            transaction_data: Transaction parameters
            
        Returns:
            Transaction hash if successful, None otherwise
        """
        logger.info("💳 Submitting blockchain transaction...")
        
        try:
            async with aiohttp.ClientSession() as session:
                async with session.post(
                    f"{self.bpci_bridge_url}/api/transactions/submit",
                    json=transaction_data,
                    timeout=30
                ) as response:
                    if response.status == 200:
                        result = await response.json()
                        tx_hash = result.get("tx_hash")
                        logger.info(f"✅ Transaction submitted: {tx_hash}")
                        return tx_hash
                    else:
                        logger.error(f"❌ Transaction submission failed: {response.status}")
                        return None
        except Exception as e:
            logger.error(f"❌ Transaction submission error: {e}")
            return None

    async def get_transaction_status(self, tx_hash: str) -> Optional[Dict]:
        """
        Example: Get transaction status
        
        Args:
            tx_hash: Transaction hash to check
            
        Returns:
            Transaction status if found, None otherwise
        """
        logger.info(f"🔍 Checking transaction status: {tx_hash}")
        
        try:
            async with aiohttp.ClientSession() as session:
                async with session.get(
                    f"{self.bpci_bridge_url}/api/transactions/{tx_hash}",
                    timeout=10
                ) as response:
                    if response.status == 200:
                        result = await response.json()
                        logger.info(f"✅ Transaction status: {result.get('status', 'unknown')}")
                        return result
                    else:
                        logger.error(f"❌ Transaction not found: {response.status}")
                        return None
        except Exception as e:
            logger.error(f"❌ Transaction status error: {e}")
            return None

    async def get_blockchain_stats(self) -> Optional[Dict]:
        """
        Example: Get blockchain statistics
        
        Returns:
            Blockchain stats if successful, None otherwise
        """
        logger.info("📈 Getting blockchain statistics...")
        
        try:
            async with aiohttp.ClientSession() as session:
                async with session.get(
                    f"{self.bpci_bridge_url}/api/stats",
                    timeout=10
                ) as response:
                    if response.status == 200:
                        stats = await response.json()
                        logger.info("✅ Blockchain stats retrieved")
                        return stats
                    else:
                        logger.error(f"❌ Failed to get blockchain stats: {response.status}")
                        return None
        except Exception as e:
            logger.error(f"❌ Blockchain stats error: {e}")
            return None

    # ========================================
    # 4D DATABASE EXAMPLES
    # ========================================

    async def insert_document(self, collection: str, document: Dict[str, Any]) -> Optional[str]:
        """
        Example: Insert document into 4D Hash-Graph Database
        
        Args:
            collection: Collection name
            document: Document to insert
            
        Returns:
            Document ID if successful, None otherwise
        """
        logger.info(f"💾 Inserting document into collection '{collection}'...")
        
        try:
            async with aiohttp.ClientSession() as session:
                payload = {
                    "collection": collection,
                    "document": document
                }
                
                async with session.post(
                    f"{self.database_url}/api/4d/insert",
                    json=payload,
                    timeout=15
                ) as response:
                    if response.status == 200:
                        result = await response.json()
                        doc_id = result.get("document_id")
                        logger.info(f"✅ Document inserted: {doc_id}")
                        return doc_id
                    else:
                        logger.error(f"❌ Document insertion failed: {response.status}")
                        return None
        except Exception as e:
            logger.error(f"❌ Document insertion error: {e}")
            return None

    async def find_documents(self, collection: str, query: Dict[str, Any], limit: int = 10) -> Optional[List[Dict]]:
        """
        Example: Find documents in 4D Hash-Graph Database
        
        Args:
            collection: Collection name
            query: Query parameters
            limit: Maximum number of results
            
        Returns:
            List of documents if successful, None otherwise
        """
        logger.info(f"🔍 Finding documents in collection '{collection}'...")
        
        try:
            async with aiohttp.ClientSession() as session:
                payload = {
                    "collection": collection,
                    "query": query,
                    "limit": limit
                }
                
                async with session.post(
                    f"{self.database_url}/api/4d/find",
                    json=payload,
                    timeout=15
                ) as response:
                    if response.status == 200:
                        result = await response.json()
                        documents = result.get("documents", [])
                        logger.info(f"✅ Found {len(documents)} documents")
                        return documents
                    else:
                        logger.error(f"❌ Document search failed: {response.status}")
                        return None
        except Exception as e:
            logger.error(f"❌ Document search error: {e}")
            return None

    async def get_database_stats(self) -> Optional[Dict]:
        """
        Example: Get 4D database statistics
        
        Returns:
            Database stats if successful, None otherwise
        """
        logger.info("📊 Getting 4D database statistics...")
        
        try:
            async with aiohttp.ClientSession() as session:
                async with session.get(
                    f"{self.database_url}/api/4d/stats",
                    timeout=10
                ) as response:
                    if response.status == 200:
                        stats = await response.json()
                        logger.info("✅ Database stats retrieved")
                        return stats
                    else:
                        logger.error(f"❌ Failed to get database stats: {response.status}")
                        return None
        except Exception as e:
            logger.error(f"❌ Database stats error: {e}")
            return None

    # ========================================
    # SERVICE ORCHESTRATOR EXAMPLES
    # ========================================

    async def deploy_service(self, service_config: Dict[str, Any]) -> Optional[str]:
        """
        Example: Deploy a service using the orchestrator
        
        Args:
            service_config: Service configuration
            
        Returns:
            Service ID if successful, None otherwise
        """
        logger.info("🚀 Deploying service via orchestrator...")
        
        try:
            async with aiohttp.ClientSession() as session:
                async with session.post(
                    f"{self.orchestrator_url}/api/services/deploy",
                    json=service_config,
                    timeout=60
                ) as response:
                    if response.status == 200:
                        result = await response.json()
                        service_id = result.get("service_id")
                        logger.info(f"✅ Service deployed: {service_id}")
                        return service_id
                    else:
                        logger.error(f"❌ Service deployment failed: {response.status}")
                        return None
        except Exception as e:
            logger.error(f"❌ Service deployment error: {e}")
            return None

    async def scale_service(self, service_id: str, replicas: int) -> bool:
        """
        Example: Scale a service
        
        Args:
            service_id: Service to scale
            replicas: Number of replicas
            
        Returns:
            True if successful, False otherwise
        """
        logger.info(f"📈 Scaling service {service_id} to {replicas} replicas...")
        
        try:
            async with aiohttp.ClientSession() as session:
                payload = {
                    "service_id": service_id,
                    "replicas": replicas
                }
                
                async with session.post(
                    f"{self.orchestrator_url}/api/services/{service_id}/scale",
                    json=payload,
                    timeout=30
                ) as response:
                    if response.status == 200:
                        logger.info("✅ Service scaled successfully")
                        return True
                    else:
                        logger.error(f"❌ Service scaling failed: {response.status}")
                        return False
        except Exception as e:
            logger.error(f"❌ Service scaling error: {e}")
            return False

    async def get_service_logs(self, service_id: str, lines: int = 100) -> Optional[List[str]]:
        """
        Example: Get service logs
        
        Args:
            service_id: Service ID
            lines: Number of log lines to retrieve
            
        Returns:
            List of log lines if successful, None otherwise
        """
        logger.info(f"📋 Getting logs for service {service_id}...")
        
        try:
            async with aiohttp.ClientSession() as session:
                params = {"lines": lines}
                
                async with session.get(
                    f"{self.orchestrator_url}/api/services/{service_id}/logs",
                    params=params,
                    timeout=15
                ) as response:
                    if response.status == 200:
                        result = await response.json()
                        logs = result.get("logs", [])
                        logger.info(f"✅ Retrieved {len(logs)} log lines")
                        return logs
                    else:
                        logger.error(f"❌ Failed to get service logs: {response.status}")
                        return None
        except Exception as e:
            logger.error(f"❌ Service logs error: {e}")
            return None


# ========================================
# USAGE EXAMPLES AND DEMOS
# ========================================

async def demo_health_checks():
    """Demo: Health check examples"""
    print("\n🏥 === HEALTH CHECK EXAMPLES ===")
    
    client = BpiApiClient()
    
    # Check all services
    health_status = await client.check_all_services_health()
    print(f"Health Status: {health_status}")
    
    # Get system status
    system_status = await client.get_system_status()
    print(f"System Status: {json.dumps(system_status, indent=2)}")

async def demo_vm_operations():
    """Demo: VM Server operations"""
    print("\n🖥️ === VM SERVER EXAMPLES ===")
    
    client = BpiApiClient()
    
    # Create VM instance
    vm_config = {
        "name": "demo-vm",
        "memory_mb": 1024,
        "cpu_cores": 2,
        "disk_gb": 10,
        "quantum_safe": True
    }
    
    vm_id = await client.create_vm_instance(vm_config)
    if vm_id:
        print(f"Created VM: {vm_id}")
        
        # Execute code in VM
        python_code = """
print("Hello from BPI VM!")
result = 2 + 2
print(f"2 + 2 = {result}")
"""
        
        execution_result = await client.execute_vm_code(vm_id, python_code, "python")
        if execution_result:
            print(f"Execution Result: {execution_result}")
        
        # Deploy contract to VM
        contract_code = """
contract SimpleStorage {
    uint256 public value;
    
    function setValue(uint256 _value) public {
        value = _value;
    }
    
    function getValue() public view returns (uint256) {
        return value;
    }
}
"""
        
        contract_address = await client.deploy_contract_to_vm(vm_id, contract_code, "solidity")
        if contract_address:
            print(f"Contract deployed at: {contract_address}")

async def demo_blockchain_operations():
    """Demo: BPCI Bridge operations"""
    print("\n💳 === BLOCKCHAIN EXAMPLES ===")
    
    client = BpiApiClient()
    
    # Submit transaction
    transaction_data = {
        "from": "0x1234567890123456789012345678901234567890",
        "to": "0x0987654321098765432109876543210987654321",
        "value": "1000000000000000000",  # 1 ETH in wei
        "gas": 21000,
        "gasPrice": "20000000000",  # 20 gwei
        "data": "0x"
    }
    
    tx_hash = await client.submit_transaction(transaction_data)
    if tx_hash:
        print(f"Transaction submitted: {tx_hash}")
        
        # Check transaction status
        tx_status = await client.get_transaction_status(tx_hash)
        if tx_status:
            print(f"Transaction Status: {tx_status}")
    
    # Get blockchain stats
    blockchain_stats = await client.get_blockchain_stats()
    if blockchain_stats:
        print(f"Blockchain Stats: {json.dumps(blockchain_stats, indent=2)}")

async def demo_database_operations():
    """Demo: 4D Database operations"""
    print("\n💾 === 4D DATABASE EXAMPLES ===")
    
    client = BpiApiClient()
    
    # Insert document
    document = {
        "user_id": "demo_user_123",
        "name": "John Doe",
        "email": "john@example.com",
        "balance": 1000.50,
        "created_at": datetime.now().isoformat(),
        "metadata": {
            "source": "api_demo",
            "version": "1.0"
        }
    }
    
    doc_id = await client.insert_document("users", document)
    if doc_id:
        print(f"Document inserted: {doc_id}")
        
        # Find documents
        query = {"user_id": "demo_user_123"}
        documents = await client.find_documents("users", query, limit=5)
        if documents:
            print(f"Found documents: {len(documents)}")
            for doc in documents:
                print(f"  - {doc.get('name', 'Unknown')} ({doc.get('email', 'No email')})")
    
    # Get database stats
    db_stats = await client.get_database_stats()
    if db_stats:
        print(f"Database Stats: {json.dumps(db_stats, indent=2)}")

async def demo_orchestrator_operations():
    """Demo: Service Orchestrator operations"""
    print("\n🚀 === ORCHESTRATOR EXAMPLES ===")
    
    client = BpiApiClient()
    
    # Deploy service
    service_config = {
        "name": "demo-api-service",
        "image": "nginx:latest",
        "replicas": 2,
        "ports": [{"container_port": 80, "host_port": 8080}],
        "environment": {
            "ENV": "demo",
            "DEBUG": "true"
        },
        "resources": {
            "memory_mb": 512,
            "cpu_cores": 1
        }
    }
    
    service_id = await client.deploy_service(service_config)
    if service_id:
        print(f"Service deployed: {service_id}")
        
        # Scale service
        scale_success = await client.scale_service(service_id, 3)
        if scale_success:
            print("Service scaled successfully")
        
        # Get service logs
        logs = await client.get_service_logs(service_id, lines=50)
        if logs:
            print(f"Service logs ({len(logs)} lines):")
            for log_line in logs[-5:]:  # Show last 5 lines
                print(f"  {log_line}")

async def main():
    """Main demo function"""
    print("🔌 BPI Infrastructure API Examples")
    print("=" * 50)
    
    try:
        # Run all demos
        await demo_health_checks()
        await demo_vm_operations()
        await demo_blockchain_operations()
        await demo_database_operations()
        await demo_orchestrator_operations()
        
        print("\n✅ All API examples completed successfully!")
        
    except Exception as e:
        logger.error(f"❌ Demo error: {e}")

if __name__ == "__main__":
    asyncio.run(main())
