// BPI Core Escrow Smart Contract written in Cue
// A decentralized escrow system for the BPI blockchain

package metanode

import (
	"time"
	"strings"
	"github.com/metanode/metanode-spec/schema"
)

// BPI Contract metadata
#BpiContract: {
	name:        "BpiEscrowContract"
	version:     "1.0.0"
	description: "Decentralized escrow for BPI blockchain"
	author:      "BPI Core Team"
	license:     "MIT"
	deployed_at: time.Time
	network:     "bpi-mainnet" | "bpi-testnet"
}

// BPI Escrow State schema
#BpiEscrowState: {
	// Escrow identifier (BPI format)
	escrow_id: string & =~"^BPI-ESC-[0-9A-F]{16}$"
	
	// BPI addresses
	buyer:  #BpiAddress
	seller: #BpiAddress
	arbiter?: #BpiAddress // Optional arbiter
	
	// Escrow amount in BPI native tokens
	amount: #BpiAmount
	
	// Contract terms
	terms: #BpiEscrowTerms
	
	// Current status
	status: #BpiEscrowStatus
	
	// BPI blockchain timestamps
	created_at:  time.Time
	updated_at:  time.Time
	expires_at?: time.Time
	
	// Release conditions
	conditions: #BpiReleaseConditions
	
	// BPI transaction history
	history: [...#BpiTransaction]
	
	// BPI block information
	block_info: #BpiBlockInfo
}

// BPI address validation (native BPI format)
#BpiAddress: string & =~"^bpi:[a-zA-Z0-9]{32,64}$"

// BPI native amount
#BpiAmount: {
	value:    number & >=0
	decimals: int & >=0 & <=18 | *18
	currency: "BPI" | "BPIG" | "BPIS" // BPI, BPI Gold, BPI Silver
}

// BPI-specific escrow terms
#BpiEscrowTerms: {
	// Service/goods description
	description: string & len(description) > 10 & len(description) <= 1000
	
	// BPI delivery requirements
	delivery: {
		method: "digital" | "physical" | "service" | "data"
		location?: string
		bpi_tracking_id?: string
		expected_block?: int & >=0 // Expected delivery block
	}
	
	// Quality and compliance
	quality: {
		bpi_standards: [...string] // BPI quality standards
		inspection_blocks: int & >=0 & <=1000 | *100 // Blocks for inspection
		return_policy: bool | *true
	}
	
	// BPI payment terms
	payment: {
		installments: int & >=1 & <=5 | *1
		late_fee_rate: number & >=0 & <=0.1 | *0.02
		early_discount: number & >=0 & <=0.15 | *0
		gas_fee_payer: "buyer" | "seller" | "split" | *"buyer"
	}
}

// BPI escrow status
#BpiEscrowStatus: "created" | "funded" | "active" | "disputed" | "completed" | "cancelled" | "expired"

// BPI-specific release conditions
#BpiReleaseConditions: {
	// Automatic release via BPI blockchain
	auto_release: {
		// Block-based time lock
		block_lock?: {
			release_block: int & >=0
			enabled: bool | *false
		}
		
		// BPI validator confirmation
		validator_confirmation?: {
			required: bool | *true
			min_validators: int & >=1 & <=21 | *3
			validator_addresses: [...#BpiAddress]
		}
		
		// BPI oracle integration
		bpi_oracles?: [...#BpiOracleCondition]
		
		// BPI multi-signature
		bpi_multisig?: {
			required_sigs: int & >=1 & <=7
			authorized_signers: [...#BpiAddress] & len(authorized_signers) >= required_sigs
		}
	}
	
	// Manual release conditions
	manual_release: {
		buyer_approval: bool | *true
		seller_confirmation: bool | *false
		arbiter_decision: bool | *false
	}
	
	// BPI dispute resolution
	dispute: {
		enabled: bool | *true
		bpi_court_address?: #BpiAddress
		voting_blocks: int & >=100 & <=10000 | *1000
		evidence_blocks: int & >=50 & <=5000 | *500
	}
}

// BPI Oracle condition
#BpiOracleCondition: {
	oracle_address: #BpiAddress
	condition_type: "price" | "delivery" | "identity" | "weather" | "custom"
	operator: "==" | "!=" | ">" | "<" | ">=" | "<="
	target_value: string | number | bool
	data_feed: string
	update_frequency_blocks: int & >=1 & <=1000 | *10
}

// BPI transaction record
#BpiTransaction: {
	tx_hash: string & =~"^0x[a-fA-F0-9]{64}$"
	tx_type: "deposit" | "release" | "refund" | "fee" | "dispute"
	amount: #BpiAmount
	from: #BpiAddress
	to: #BpiAddress
	timestamp: time.Time
	block_number: int & >=0
	gas_used: int & >=21000
	gas_price: number & >=0
	status: "pending" | "confirmed" | "failed"
	confirmations: int & >=0
}

// BPI block information
#BpiBlockInfo: {
	deployment_block: int & >=0
	current_block: int & >=deployment_block
	block_hash: string & =~"^0x[a-fA-F0-9]{64}$"
	block_timestamp: time.Time
	validator: #BpiAddress
}

// BPI Contract functions with validation
#BpiContractFunctions: {
	// Initialize BPI escrow
	bpi_initialize: {
		input: {
			buyer: #BpiAddress
			seller: #BpiAddress
			amount: #BpiAmount
			terms: #BpiEscrowTerms
			conditions: #BpiReleaseConditions
		}
		
		// BPI-specific validation
		input: {
			// Ensure different parties
			if buyer == seller {
				_error: "Buyer and seller must be different BPI addresses"
			}
			
			// Validate BPI amount
			if amount.value <= 0 {
				_error: "BPI escrow amount must be positive"
			}
			
			// Validate BPI currency
			amount: currency: "BPI" | "BPIG" | "BPIS"
		}
		
		output: {
			escrow_id: string
			status: "created"
			bpi_gas_estimate: int & >=50000
			deployment_block: int & >=0
		}
	}
	
	// Fund BPI escrow
	bpi_fund: {
		input: {
			escrow_id: string
			amount: #BpiAmount
			from: #BpiAddress
			gas_limit: int & >=21000 & <=1000000 | *100000
		}
		
		// BPI pre-conditions
		preconditions: {
			escrow_exists: true
			status_is_created: true
			amount_matches: true
			sender_is_buyer: true
			sufficient_bpi_balance: true
		}
		
		output: {
			status: "funded"
			funded_amount: #BpiAmount
			bpi_tx_hash: string
			block_number: int & >=0
		}
	}
	
	// Release BPI funds
	bpi_release: {
		input: {
			escrow_id: string
			release_type: "auto" | "manual" | "dispute"
			bpi_signatures?: [...string]
			evidence_hash?: string
			gas_limit: int & >=21000 & <=1000000 | *150000
		}
		
		// BPI validation based on release type
		if input.release_type == "auto" {
			// Check BPI auto-release conditions
			bpi_conditions_met: true
			block_requirements_satisfied: true
		}
		
		if input.release_type == "manual" {
			// Require BPI authorization
			bpi_authorized_releaser: true
			valid_bpi_signature: true
		}
		
		if input.release_type == "dispute" {
			// Require BPI dispute resolution
			bpi_dispute_resolved: true
			evidence_hash: string & len(evidence_hash) > 0
		}
		
		output: {
			status: "completed"
			released_amount: #BpiAmount
			recipient: #BpiAddress
			bpi_tx_hash: string
			block_number: int & >=0
		}
	}
	
	// Dispute BPI escrow
	bpi_dispute: {
		input: {
			escrow_id: string
			disputer: #BpiAddress
			reason: string & len(reason) >= 20
			evidence_hash: string
			stake_amount: #BpiAmount // Dispute stake
		}
		
		// BPI validation
		input: {
			// Only parties can dispute
			if !(disputer == buyer || disputer == seller) {
				_error: "Only buyer or seller can dispute on BPI"
			}
			
			// Must be active escrow
			status: "funded" | "active"
			
			// Require dispute stake
			stake_amount: value: >=10 // Minimum 10 BPI stake
		}
		
		output: {
			status: "disputed"
			dispute_id: string
			bpi_court_assigned: #BpiAddress
			voting_start_block: int & >=0
			voting_end_block: int & >=voting_start_block
		}
	}
}

// BPI Events emitted by the contract
#BpiEvents: {
	BpiEscrowCreated: {
		escrow_id: string
		buyer: #BpiAddress
		seller: #BpiAddress
		amount: #BpiAmount
		block_number: int & >=0
		timestamp: time.Time
	}
	
	BpiEscrowFunded: {
		escrow_id: string
		amount: #BpiAmount
		funder: #BpiAddress
		bpi_tx_hash: string
		block_number: int & >=0
		timestamp: time.Time
	}
	
	BpiFundsReleased: {
		escrow_id: string
		recipient: #BpiAddress
		amount: #BpiAmount
		release_type: string
		bpi_tx_hash: string
		block_number: int & >=0
		timestamp: time.Time
	}
	
	BpiDisputeInitiated: {
		escrow_id: string
		disputer: #BpiAddress
		reason: string
		stake_amount: #BpiAmount
		dispute_start_block: int & >=0
		timestamp: time.Time
	}
	
	BpiDisputeResolved: {
		escrow_id: string
		winner: #BpiAddress
		resolution: string
		final_block: int & >=0
		timestamp: time.Time
	}
}

// BPI Contract configuration
#BpiConfig: {
	// BPI fee structure
	bpi_fees: {
		platform_fee_rate: number & >=0 & <=0.03 | *0.015 // 1.5%
		validator_fee_rate: number & >=0 & <=0.02 | *0.005 // 0.5%
		gas_price_multiplier: number & >=1 & <=2 | *1.1
	}
	
	// BPI limits
	bpi_limits: {
		min_escrow_amount: #BpiAmount & {value: >=1}
		max_escrow_amount: #BpiAmount & {value: <=10000000}
		max_dispute_blocks: int & >=1000 & <=100000 | *10000
		max_validators: int & >=3 & <=21 | *7
	}
	
	// BPI security settings
	bpi_security: {
		require_bpi_identity: bool | *true
		blacklist_check: bool | *true
		rate_limiting: {
			max_escrows_per_block: int & >=1 & <=50 | *5
			max_amount_per_address_daily: #BpiAmount
		}
	}
}

// Example BPI escrow instance
bpi_example_escrow: #BpiEscrowState & {
	escrow_id: "BPI-ESC-1234567890ABCDEF"
	buyer: "bpi:buyer1234567890123456789012345678901234567890"
	seller: "bpi:seller123456789012345678901234567890123456789"
	amount: {
		value: 500.0
		decimals: 18
		currency: "BPI"
	}
	terms: {
		description: "Premium BPI blockchain development and smart contract audit services"
		delivery: {
			method: "digital"
			expected_block: 1000000
		}
		quality: {
			bpi_standards: ["BPI-SEC-001", "BPI-AUDIT-002"]
			inspection_blocks: 200
			return_policy: true
		}
		payment: {
			installments: 1
			late_fee_rate: 0.02
			early_discount: 0.01
			gas_fee_payer: "buyer"
		}
	}
	status: "created"
	created_at: "2024-08-20T08:00:00Z"
	updated_at: "2024-08-20T08:00:00Z"
	expires_at: "2024-12-31T23:59:59Z"
	conditions: {
		auto_release: {
			block_lock: {
				release_block: 1001000
				enabled: true
			}
			validator_confirmation: {
				required: true
				min_validators: 3
				validator_addresses: [
					"bpi:validator1234567890123456789012345678901234567890",
					"bpi:validator2345678901234567890123456789012345678901",
					"bpi:validator3456789012345678901234567890123456789012"
				]
			}
		}
		manual_release: {
			buyer_approval: true
			seller_confirmation: false
			arbiter_decision: false
		}
		dispute: {
			enabled: true
			voting_blocks: 1000
			evidence_blocks: 500
		}
	}
	history: []
	block_info: {
		deployment_block: 999500
		current_block: 999500
		block_hash: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef12"
		block_timestamp: "2024-08-20T08:00:00Z"
		validator: "bpi:validator1234567890123456789012345678901234567890"
	}
}

// Validation: Ensure example escrow is valid
bpi_example_escrow: #BpiEscrowState
