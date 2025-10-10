#!/usr/bin/env python3
"""
Sample DeFi Application for BPI-BPCI Infrastructure
Demonstrates how to build a pilot application using the BPI ecosystem
"""

import asyncio
import json
import os
import sys
from datetime import datetime
from typing import Dict, List, Optional
import aiohttp
import logging

# Configure logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

class BpiDefiApp:
    """Sample DeFi application showcasing BPI infrastructure capabilities"""
    
    def __init__(self):
        # BPI Infrastructure endpoints from environment variables
        self.vm_server_url = os.getenv('BPI_VM_SERVER_URL', 'http://localhost:8080')
        self.bpci_bridge_url = os.getenv('BPI_BPCI_BRIDGE_URL', 'http://localhost:8545')
        self.database_url = os.getenv('BPI_DATABASE_URL', 'http://localhost:27017')
        self.orchestrator_url = os.getenv('BPI_ORCHESTRATOR_URL', 'http://localhost:9090')
        
        # Application state
        self.user_portfolios: Dict[str, Dict] = {}
        self.liquidity_pools: Dict[str, Dict] = {}
        self.staking_positions: Dict[str, Dict] = {}
        
        logger.info(f"🚀 BPI DeFi App initialized")
        logger.info(f"VM Server: {self.vm_server_url}")
        logger.info(f"BPCI Bridge: {self.bpci_bridge_url}")
        logger.info(f"4D Database: {self.database_url}")
        logger.info(f"Orchestrator: {self.orchestrator_url}")

    async def start(self):
        """Start the DeFi application"""
        logger.info("🏦 Starting BPI DeFi Application...")
        
        # Check infrastructure health
        if not await self.check_infrastructure_health():
            logger.error("❌ Infrastructure health check failed")
            return False
        
        # Initialize application components
        await self.initialize_liquidity_pools()
        await self.setup_staking_contracts()
        await self.load_user_data()
        
        # Start main application loop
        await self.run_application()
        
        return True

    async def check_infrastructure_health(self) -> bool:
        """Check if all BPI infrastructure components are healthy"""
        logger.info("🏥 Checking infrastructure health...")
        
        endpoints = {
            "VM Server": f"{self.vm_server_url}/health",
            "BPCI Bridge": f"{self.bpci_bridge_url}/health",
            "4D Database": f"{self.database_url}/health",
            "Orchestrator": f"{self.orchestrator_url}/health"
        }
        
        healthy_count = 0
        async with aiohttp.ClientSession() as session:
            for name, url in endpoints.items():
                try:
                    async with session.get(url, timeout=5) as response:
                        if response.status == 200:
                            logger.info(f"✅ {name}: Healthy")
                            healthy_count += 1
                        else:
                            logger.warning(f"⚠️ {name}: Unhealthy (status: {response.status})")
                except Exception as e:
                    logger.warning(f"❌ {name}: Connection failed - {e}")
        
        health_percentage = (healthy_count / len(endpoints)) * 100
        logger.info(f"Infrastructure Health: {health_percentage:.1f}% ({healthy_count}/{len(endpoints)} services)")
        
        return healthy_count >= 2  # At least 50% of services must be healthy

    async def initialize_liquidity_pools(self):
        """Initialize DeFi liquidity pools"""
        logger.info("💧 Initializing liquidity pools...")
        
        # Sample liquidity pools
        pools = {
            "BPI-ETH": {
                "token_a": "BPI",
                "token_b": "ETH",
                "liquidity": 1000000,
                "apy": 12.5,
                "total_value_locked": 2500000,
                "fee_tier": 0.3
            },
            "BPI-USDC": {
                "token_a": "BPI",
                "token_b": "USDC",
                "liquidity": 2000000,
                "apy": 8.7,
                "total_value_locked": 5000000,
                "fee_tier": 0.05
            },
            "ETH-USDC": {
                "token_a": "ETH",
                "token_b": "USDC",
                "liquidity": 5000000,
                "apy": 6.2,
                "total_value_locked": 12000000,
                "fee_tier": 0.05
            }
        }
        
        for pool_id, pool_data in pools.items():
            self.liquidity_pools[pool_id] = pool_data
            
            # Store in 4D Database
            await self.store_in_4d_database("liquidity_pools", {
                "pool_id": pool_id,
                "data": pool_data,
                "timestamp": datetime.now().isoformat(),
                "type": "liquidity_pool"
            })
            
            logger.info(f"✅ Pool {pool_id}: TVL ${pool_data['total_value_locked']:,} | APY {pool_data['apy']}%")

    async def setup_staking_contracts(self):
        """Setup staking contracts using VM Server"""
        logger.info("🔒 Setting up staking contracts...")
        
        staking_contracts = {
            "BPI_STAKING": {
                "token": "BPI",
                "min_stake": 100,
                "lock_period_days": 30,
                "apy": 15.0,
                "total_staked": 10000000,
                "rewards_pool": 1500000
            },
            "LP_STAKING": {
                "token": "BPI-ETH-LP",
                "min_stake": 10,
                "lock_period_days": 60,
                "apy": 25.0,
                "total_staked": 5000000,
                "rewards_pool": 1250000
            }
        }
        
        for contract_id, contract_data in staking_contracts.items():
            # Deploy contract to VM Server
            contract_deployed = await self.deploy_contract_to_vm(contract_id, contract_data)
            
            if contract_deployed:
                logger.info(f"✅ Staking contract {contract_id}: APY {contract_data['apy']}% | Total Staked: ${contract_data['total_staked']:,}")
            else:
                logger.warning(f"⚠️ Failed to deploy staking contract {contract_id}")

    async def deploy_contract_to_vm(self, contract_id: str, contract_data: Dict) -> bool:
        """Deploy a smart contract to the VM Server"""
        try:
            async with aiohttp.ClientSession() as session:
                payload = {
                    "contract_id": contract_id,
                    "contract_type": "staking",
                    "data": contract_data,
                    "quantum_safe": True
                }
                
                async with session.post(
                    f"{self.vm_server_url}/api/contracts/deploy",
                    json=payload,
                    timeout=10
                ) as response:
                    if response.status == 200:
                        result = await response.json()
                        logger.info(f"Contract {contract_id} deployed: {result.get('contract_address', 'N/A')}")
                        return True
                    else:
                        logger.error(f"Contract deployment failed: {response.status}")
                        return False
        except Exception as e:
            logger.error(f"Contract deployment error: {e}")
            return False

    async def store_in_4d_database(self, collection: str, document: Dict):
        """Store data in the 4D Hash-Graph Database"""
        try:
            async with aiohttp.ClientSession() as session:
                async with session.post(
                    f"{self.database_url}/api/4d/insert",
                    json={
                        "collection": collection,
                        "document": document
                    },
                    timeout=5
                ) as response:
                    if response.status == 200:
                        logger.debug(f"Stored document in {collection}")
                    else:
                        logger.warning(f"Failed to store in 4D database: {response.status}")
        except Exception as e:
            logger.debug(f"4D database storage error: {e}")

    async def load_user_data(self):
        """Load user portfolio data"""
        logger.info("👤 Loading user data...")
        
        # Sample user portfolios
        sample_users = {
            "pilot_user_1": {
                "balance": {
                    "BPI": 10000,
                    "ETH": 5.5,
                    "USDC": 25000
                },
                "staking_positions": {
                    "BPI_STAKING": {
                        "amount": 5000,
                        "start_date": "2024-01-01",
                        "lock_period": 30,
                        "rewards_earned": 625
                    }
                },
                "liquidity_positions": {
                    "BPI-ETH": {
                        "lp_tokens": 100,
                        "value_usd": 12500,
                        "fees_earned": 156.25
                    }
                }
            },
            "pilot_user_2": {
                "balance": {
                    "BPI": 25000,
                    "ETH": 12.0,
                    "USDC": 50000
                },
                "staking_positions": {
                    "LP_STAKING": {
                        "amount": 50,
                        "start_date": "2024-01-15",
                        "lock_period": 60,
                        "rewards_earned": 1250
                    }
                },
                "liquidity_positions": {
                    "BPI-USDC": {
                        "lp_tokens": 200,
                        "value_usd": 25000,
                        "fees_earned": 312.50
                    }
                }
            }
        }
        
        for user_id, portfolio in sample_users.items():
            self.user_portfolios[user_id] = portfolio
            
            # Store user data in 4D database
            await self.store_in_4d_database("user_portfolios", {
                "user_id": user_id,
                "portfolio": portfolio,
                "timestamp": datetime.now().isoformat(),
                "type": "user_portfolio"
            })
            
            total_value = self.calculate_portfolio_value(portfolio)
            logger.info(f"✅ User {user_id}: Portfolio Value ${total_value:,.2f}")

    def calculate_portfolio_value(self, portfolio: Dict) -> float:
        """Calculate total portfolio value in USD"""
        # Sample price data (in production, this would come from price oracles)
        prices = {
            "BPI": 2.50,
            "ETH": 2500.00,
            "USDC": 1.00
        }
        
        total_value = 0.0
        
        # Calculate token balances
        for token, amount in portfolio.get("balance", {}).items():
            if token in prices:
                total_value += amount * prices[token]
        
        # Add staking positions
        for position in portfolio.get("staking_positions", {}).values():
            total_value += position.get("rewards_earned", 0)
        
        # Add liquidity positions
        for position in portfolio.get("liquidity_positions", {}).values():
            total_value += position.get("value_usd", 0)
            total_value += position.get("fees_earned", 0)
        
        return total_value

    async def run_application(self):
        """Main application loop"""
        logger.info("🎮 Starting DeFi application interface...")
        
        while True:
            print("\n" + "="*60)
            print("🏦 BPI DeFi Application - Pilot Demo")
            print("="*60)
            print("1. View Portfolio")
            print("2. Stake Tokens")
            print("3. Provide Liquidity")
            print("4. View Liquidity Pools")
            print("5. Check Rewards")
            print("6. System Health")
            print("7. Exit")
            print("="*60)
            
            try:
                choice = input("Select an option (1-7): ").strip()
                
                if choice == "1":
                    await self.show_portfolio()
                elif choice == "2":
                    await self.stake_tokens()
                elif choice == "3":
                    await self.provide_liquidity()
                elif choice == "4":
                    await self.show_liquidity_pools()
                elif choice == "5":
                    await self.check_rewards()
                elif choice == "6":
                    await self.show_system_health()
                elif choice == "7":
                    logger.info("👋 Exiting BPI DeFi Application")
                    break
                else:
                    print("❌ Invalid option. Please try again.")
                    
            except KeyboardInterrupt:
                logger.info("\n👋 Application interrupted by user")
                break
            except Exception as e:
                logger.error(f"Application error: {e}")

    async def show_portfolio(self):
        """Display user portfolio"""
        print("\n💼 User Portfolios")
        print("-" * 40)
        
        for user_id, portfolio in self.user_portfolios.items():
            total_value = self.calculate_portfolio_value(portfolio)
            print(f"\n👤 {user_id}")
            print(f"   Total Value: ${total_value:,.2f}")
            
            print("   Balances:")
            for token, amount in portfolio.get("balance", {}).items():
                print(f"     {token}: {amount:,.2f}")
            
            print("   Staking:")
            for stake_id, position in portfolio.get("staking_positions", {}).items():
                print(f"     {stake_id}: {position['amount']:,.0f} tokens (${position['rewards_earned']:,.2f} rewards)")
            
            print("   Liquidity:")
            for pool_id, position in portfolio.get("liquidity_positions", {}).items():
                print(f"     {pool_id}: ${position['value_usd']:,.2f} (${position['fees_earned']:,.2f} fees)")

    async def stake_tokens(self):
        """Simulate token staking"""
        print("\n🔒 Token Staking")
        print("-" * 30)
        
        print("Available Staking Contracts:")
        staking_options = {
            "1": ("BPI_STAKING", "BPI Token Staking - 15% APY"),
            "2": ("LP_STAKING", "LP Token Staking - 25% APY")
        }
        
        for key, (contract_id, description) in staking_options.items():
            print(f"{key}. {description}")
        
        choice = input("Select staking option (1-2): ").strip()
        if choice in staking_options:
            contract_id, description = staking_options[choice]
            amount = input("Enter amount to stake: ").strip()
            
            try:
                stake_amount = float(amount)
                success = await self.process_staking(contract_id, stake_amount)
                
                if success:
                    print(f"✅ Successfully staked {stake_amount} tokens in {contract_id}")
                else:
                    print(f"❌ Staking failed for {contract_id}")
            except ValueError:
                print("❌ Invalid amount entered")

    async def process_staking(self, contract_id: str, amount: float) -> bool:
        """Process staking transaction via BPCI Bridge"""
        try:
            async with aiohttp.ClientSession() as session:
                transaction_data = {
                    "type": "staking",
                    "contract_id": contract_id,
                    "amount": amount,
                    "user_id": "pilot_user_1",
                    "timestamp": datetime.now().isoformat()
                }
                
                async with session.post(
                    f"{self.bpci_bridge_url}/api/transactions/submit",
                    json=transaction_data,
                    timeout=10
                ) as response:
                    if response.status == 200:
                        result = await response.json()
                        logger.info(f"Staking transaction submitted: {result.get('tx_hash', 'N/A')}")
                        
                        # Store transaction in 4D database
                        await self.store_in_4d_database("transactions", {
                            "transaction": transaction_data,
                            "result": result,
                            "type": "staking_transaction"
                        })
                        
                        return True
                    else:
                        logger.error(f"Staking transaction failed: {response.status}")
                        return False
        except Exception as e:
            logger.error(f"Staking processing error: {e}")
            return False

    async def provide_liquidity(self):
        """Simulate liquidity provision"""
        print("\n💧 Provide Liquidity")
        print("-" * 25)
        
        print("Available Liquidity Pools:")
        for i, (pool_id, pool_data) in enumerate(self.liquidity_pools.items(), 1):
            print(f"{i}. {pool_id} - APY: {pool_data['apy']}% | TVL: ${pool_data['total_value_locked']:,}")
        
        choice = input("Select pool (1-3): ").strip()
        pool_ids = list(self.liquidity_pools.keys())
        
        try:
            pool_index = int(choice) - 1
            if 0 <= pool_index < len(pool_ids):
                pool_id = pool_ids[pool_index]
                amount = float(input("Enter liquidity amount (USD): "))
                
                success = await self.process_liquidity_provision(pool_id, amount)
                if success:
                    print(f"✅ Successfully provided ${amount:,.2f} liquidity to {pool_id}")
                else:
                    print(f"❌ Liquidity provision failed for {pool_id}")
            else:
                print("❌ Invalid pool selection")
        except (ValueError, IndexError):
            print("❌ Invalid input")

    async def process_liquidity_provision(self, pool_id: str, amount: float) -> bool:
        """Process liquidity provision transaction"""
        try:
            # Simulate liquidity provision via BPCI Bridge
            transaction_data = {
                "type": "liquidity_provision",
                "pool_id": pool_id,
                "amount_usd": amount,
                "user_id": "pilot_user_1",
                "timestamp": datetime.now().isoformat()
            }
            
            # Store in 4D database
            await self.store_in_4d_database("liquidity_transactions", transaction_data)
            
            # Update pool data
            if pool_id in self.liquidity_pools:
                self.liquidity_pools[pool_id]["total_value_locked"] += amount
                self.liquidity_pools[pool_id]["liquidity"] += amount * 0.8  # Simplified calculation
            
            logger.info(f"Liquidity provision processed: {pool_id} +${amount}")
            return True
            
        except Exception as e:
            logger.error(f"Liquidity provision error: {e}")
            return False

    async def show_liquidity_pools(self):
        """Display liquidity pools information"""
        print("\n💧 Liquidity Pools")
        print("-" * 40)
        
        for pool_id, pool_data in self.liquidity_pools.items():
            print(f"\n🏊 {pool_id}")
            print(f"   Pair: {pool_data['token_a']}/{pool_data['token_b']}")
            print(f"   APY: {pool_data['apy']}%")
            print(f"   TVL: ${pool_data['total_value_locked']:,}")
            print(f"   Liquidity: ${pool_data['liquidity']:,}")
            print(f"   Fee Tier: {pool_data['fee_tier']}%")

    async def check_rewards(self):
        """Check and display rewards"""
        print("\n🎁 Rewards Summary")
        print("-" * 30)
        
        total_rewards = 0.0
        
        for user_id, portfolio in self.user_portfolios.items():
            user_rewards = 0.0
            print(f"\n👤 {user_id}")
            
            # Staking rewards
            for stake_id, position in portfolio.get("staking_positions", {}).items():
                rewards = position.get("rewards_earned", 0)
                user_rewards += rewards
                print(f"   {stake_id}: ${rewards:,.2f}")
            
            # Liquidity fees
            for pool_id, position in portfolio.get("liquidity_positions", {}).items():
                fees = position.get("fees_earned", 0)
                user_rewards += fees
                print(f"   {pool_id} Fees: ${fees:,.2f}")
            
            print(f"   Total Rewards: ${user_rewards:,.2f}")
            total_rewards += user_rewards
        
        print(f"\n💰 Total Platform Rewards: ${total_rewards:,.2f}")

    async def show_system_health(self):
        """Display system health information"""
        print("\n🏥 System Health")
        print("-" * 25)
        
        health_ok = await self.check_infrastructure_health()
        
        print(f"\nOverall Status: {'✅ Healthy' if health_ok else '❌ Issues Detected'}")
        print(f"Liquidity Pools: {len(self.liquidity_pools)} active")
        print(f"User Portfolios: {len(self.user_portfolios)} loaded")
        print(f"Total TVL: ${sum(pool['total_value_locked'] for pool in self.liquidity_pools.values()):,}")

async def main():
    """Main entry point"""
    print("🚀 BPI DeFi Application - Pilot Demo")
    print("Demonstrating BPI-BPCI Infrastructure Capabilities")
    print("=" * 60)
    
    app = BpiDefiApp()
    
    try:
        success = await app.start()
        if not success:
            print("❌ Failed to start application")
            sys.exit(1)
    except KeyboardInterrupt:
        print("\n👋 Application interrupted")
    except Exception as e:
        logger.error(f"Application startup error: {e}")
        sys.exit(1)

if __name__ == "__main__":
    asyncio.run(main())
