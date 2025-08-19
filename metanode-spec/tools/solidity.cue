package tools
import "github.com/metanode/metanode-spec/agreements"

// Generate Solidity smart contract for Metanode Agreement Settlement
out: """
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/**
 * @title \(agreements.agreement.onchain.contract_name)
 * @dev Metanode Agreement Settlement Contract
 * @notice Handles PoE validation, slashing, and economic settlement
 * Generated from CUE spec: \(agreements.agreement.id)
 */
contract \(agreements.agreement.onchain.contract_name) {
    // Events
    event AgreementCreated(bytes32 indexed agreementId, address indexed creator);
    event ReceiptAccepted(bytes32 indexed agreementId, bytes32 bundleCid, address indexed signer, uint256 timestamp);
    event ProofOfExecutionValidated(bytes32 indexed agreementId, bytes32 proofHash, uint256 latencyMs);
    event Slashed(address indexed party, uint256 amount, string reason, bytes32 agreementId);
    event RewardDistributed(address indexed recipient, uint256 amount, string token, bytes32 agreementId);
    event DisputeRaised(bytes32 indexed agreementId, address indexed challenger, string reason);
    
    // Constants from CUE spec
    uint256 public constant SLA_MS = \(agreements.agreement.terms.sla_ms);
    uint256 public constant SLASH_BPS = \(int(agreements.agreement.terms.slash_ratio * 10000));
    uint256 public constant STAKE_REQUIRED = \(int(agreements.agreement.terms.stake_required * 1e18));
    uint256 public constant REWARD_MULTIPLIER_BPS = \(int(agreements.agreement.terms.reward_multiplier * 10000));
    uint256 public constant FINALITY_BLOCKS = \(agreements.agreement.onchain.finality_blocks);
    
    // Token types (4-token system)
    enum TokenType { GOLD, SILVER, COPPER, IRON }
    TokenType public constant PAYMENT_TOKEN = TokenType.\(agreements.agreement.terms.payment_token);
    
    // Agreement state
    struct Agreement {
        bytes32 id;
        address[] parties;
        uint256 totalStake;
        uint256 createdAt;
        uint256 expiresAt;
        bool active;
        bool disputed;
    }
    
    struct ProofOfExecution {
        bytes32 bundleCid;
        bytes32 proofHash;
        uint256 timestamp;
        uint256 latencyMs;
        address executor;
        bool validated;
    }
    
    // Storage
    mapping(bytes32 => Agreement) public agreements;
    mapping(bytes32 => ProofOfExecution[]) public proofs;
    mapping(address => uint256) public stakes;
    mapping(address => uint256) public reputationScores;
    
    // Modifiers
    modifier onlyParty(bytes32 agreementId) {
        require(isParty(agreementId, msg.sender), "Not a party to agreement");
        _;
    }
    
    modifier agreementActive(bytes32 agreementId) {
        require(agreements[agreementId].active, "Agreement not active");
        require(block.timestamp < agreements[agreementId].expiresAt, "Agreement expired");
        _;
    }
    
    /**
     * @dev Create new agreement from CUE spec
     */
    function createAgreement(
        bytes32 agreementId,
        address[] memory parties,
        uint256 expiresAt
    ) external payable {
        require(agreements[agreementId].id == 0, "Agreement already exists");
        require(parties.length >= 2, "Need at least 2 parties");
        require(msg.value >= STAKE_REQUIRED, "Insufficient stake");
        
        agreements[agreementId] = Agreement({
            id: agreementId,
            parties: parties,
            totalStake: msg.value,
            createdAt: block.timestamp,
            expiresAt: expiresAt,
            active: true,
            disputed: false
        });
        
        stakes[msg.sender] += msg.value;
        emit AgreementCreated(agreementId, msg.sender);
    }
    
    /**
     * @dev Submit Proof of Execution with receipt validation
     */
    function submitProofOfExecution(
        bytes32 agreementId,
        bytes32 bundleCid,
        bytes32 proofHash,
        uint256 latencyMs,
        bytes memory signature
    ) external onlyParty(agreementId) agreementActive(agreementId) {
        // Validate signature (simplified for demo)
        require(signature.length == 65, "Invalid signature");
        
        // Check SLA compliance
        bool slaCompliant = latencyMs <= SLA_MS;
        
        ProofOfExecution memory poe = ProofOfExecution({
            bundleCid: bundleCid,
            proofHash: proofHash,
            timestamp: block.timestamp,
            latencyMs: latencyMs,
            executor: msg.sender,
            validated: slaCompliant
        });
        
        proofs[agreementId].push(poe);
        
        if (!slaCompliant) {
            // Apply slashing for SLA breach
            uint256 slashAmount = (stakes[msg.sender] * SLASH_BPS) / 10000;
            stakes[msg.sender] -= slashAmount;
            emit Slashed(msg.sender, slashAmount, "SLA breach", agreementId);
        } else {
            // Distribute reward for successful execution
            uint256 rewardAmount = calculateReward(agreementId);
            // Note: Actual token transfer would happen here
            emit RewardDistributed(msg.sender, rewardAmount, "\(agreements.agreement.terms.payment_token)", agreementId);
        }
        
        emit ReceiptAccepted(agreementId, bundleCid, msg.sender, block.timestamp);
        emit ProofOfExecutionValidated(agreementId, proofHash, latencyMs);
    }
    
    /**
     * @dev Raise dispute (Court Node integration)
     */
    function raiseDispute(bytes32 agreementId, string memory reason) 
        external onlyParty(agreementId) agreementActive(agreementId) {
        agreements[agreementId].disputed = true;
        emit DisputeRaised(agreementId, msg.sender, reason);
    }
    
    /**
     * @dev Calculate reward based on performance and multiplier
     */
    function calculateReward(bytes32 agreementId) internal view returns (uint256) {
        uint256 baseReward = agreements[agreementId].totalStake / agreements[agreementId].parties.length;
        return (baseReward * REWARD_MULTIPLIER_BPS) / 10000;
    }
    
    /**
     * @dev Check if address is party to agreement
     */
    function isParty(bytes32 agreementId, address addr) internal view returns (bool) {
        address[] memory parties = agreements[agreementId].parties;
        for (uint i = 0; i < parties.length; i++) {
            if (parties[i] == addr) return true;
        }
        return false;
    }
    
    /**
     * @dev Get agreement details
     */
    function getAgreement(bytes32 agreementId) external view returns (Agreement memory) {
        return agreements[agreementId];
    }
    
    /**
     * @dev Get proof count for agreement
     */
    function getProofCount(bytes32 agreementId) external view returns (uint256) {
        return proofs[agreementId].length;
    }
    
    /**
     * @dev Emergency pause (military-grade security)
     */
    function emergencyPause(bytes32 agreementId) external onlyParty(agreementId) {
        agreements[agreementId].active = false;
    }
}
"""
