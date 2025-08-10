// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Pausable.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/utils/math/SafeMath.sol";

/**
 * @title ConsciousnessToken (CONS)
 * @dev ERC20 token for Consciousness Engine ecosystem
 * Features:
 * - Reward users for high-quality conversations
 * - Stake tokens for premium features
 * - Governance voting rights
 * - Deflationary mechanism through burning
 */
contract ConsciousnessToken is ERC20, ERC20Burnable, ERC20Pausable, Ownable, ReentrancyGuard {
    using SafeMath for uint256;

    // Token configuration
    uint256 public constant MAX_SUPPLY = 1_000_000_000 * 10**18; // 1 billion tokens
    uint256 public constant INITIAL_SUPPLY = 100_000_000 * 10**18; // 100 million tokens
    
    // Reward configuration
    uint256 public conversationRewardRate = 10 * 10**18; // 10 CONS per quality conversation
    uint256 public stakingRewardRate = 5; // 5% annual staking reward
    uint256 public minimumStakeAmount = 1000 * 10**18; // 1000 CONS minimum stake
    
    // Governance configuration
    uint256 public proposalThreshold = 10000 * 10**18; // 10,000 CONS to create proposal
    uint256 public votingPeriod = 7 days;
    
    // Staking data
    struct StakeInfo {
        uint256 amount;
        uint256 timestamp;
        uint256 rewardDebt;
    }
    
    mapping(address => StakeInfo) public stakes;
    mapping(address => uint256) public conversationRewards;
    mapping(address => bool) public authorizedRewarders;
    
    // Governance data
    struct Proposal {
        uint256 id;
        address proposer;
        string description;
        uint256 forVotes;
        uint256 againstVotes;
        uint256 startTime;
        uint256 endTime;
        bool executed;
        mapping(address => bool) hasVoted;
        mapping(address => bool) voteChoice; // true = for, false = against
    }
    
    mapping(uint256 => Proposal) public proposals;
    uint256 public proposalCount;
    
    // Events
    event ConversationRewarded(address indexed user, uint256 amount, string conversationId);
    event TokensStaked(address indexed user, uint256 amount);
    event TokensUnstaked(address indexed user, uint256 amount);
    event StakingRewardClaimed(address indexed user, uint256 amount);
    event ProposalCreated(uint256 indexed proposalId, address indexed proposer, string description);
    event VoteCast(uint256 indexed proposalId, address indexed voter, bool support, uint256 weight);
    event ProposalExecuted(uint256 indexed proposalId);
    
    constructor() ERC20("Consciousness Token", "CONS") {
        _mint(msg.sender, INITIAL_SUPPLY);
        authorizedRewarders[msg.sender] = true;
    }
    
    /**
     * @dev Reward user for quality conversation
     * @param user Address of the user to reward
     * @param conversationId ID of the conversation
     * @param qualityScore Quality score of the conversation (0-100)
     */
    function rewardConversation(
        address user,
        string memory conversationId,
        uint256 qualityScore
    ) external {
        require(authorizedRewarders[msg.sender], "Not authorized to reward");
        require(qualityScore >= 70, "Quality score too low for reward");
        require(user != address(0), "Invalid user address");
        
        // Calculate reward based on quality score
        uint256 baseReward = conversationRewardRate;
        uint256 qualityMultiplier = qualityScore.mul(100).div(100); // Scale quality score
        uint256 rewardAmount = baseReward.mul(qualityMultiplier).div(100);
        
        // Mint tokens if under max supply
        if (totalSupply().add(rewardAmount) <= MAX_SUPPLY) {
            _mint(user, rewardAmount);
            conversationRewards[user] = conversationRewards[user].add(rewardAmount);
            
            emit ConversationRewarded(user, rewardAmount, conversationId);
        }
    }
    
    /**
     * @dev Stake tokens for rewards and governance rights
     * @param amount Amount of tokens to stake
     */
    function stakeTokens(uint256 amount) external nonReentrant {
        require(amount >= minimumStakeAmount, "Amount below minimum stake");
        require(balanceOf(msg.sender) >= amount, "Insufficient balance");
        
        // Claim existing rewards before updating stake
        if (stakes[msg.sender].amount > 0) {
            claimStakingRewards();
        }
        
        // Transfer tokens to contract
        _transfer(msg.sender, address(this), amount);
        
        // Update stake info
        stakes[msg.sender].amount = stakes[msg.sender].amount.add(amount);
        stakes[msg.sender].timestamp = block.timestamp;
        stakes[msg.sender].rewardDebt = 0;
        
        emit TokensStaked(msg.sender, amount);
    }
    
    /**
     * @dev Unstake tokens
     * @param amount Amount of tokens to unstake
     */
    function unstakeTokens(uint256 amount) external nonReentrant {
        require(stakes[msg.sender].amount >= amount, "Insufficient staked amount");
        
        // Claim rewards before unstaking
        claimStakingRewards();
        
        // Update stake info
        stakes[msg.sender].amount = stakes[msg.sender].amount.sub(amount);
        
        // Transfer tokens back to user
        _transfer(address(this), msg.sender, amount);
        
        emit TokensUnstaked(msg.sender, amount);
    }
    
    /**
     * @dev Claim staking rewards
     */
    function claimStakingRewards() public nonReentrant {
        StakeInfo storage stake = stakes[msg.sender];
        require(stake.amount > 0, "No tokens staked");
        
        uint256 stakingDuration = block.timestamp.sub(stake.timestamp);
        uint256 rewardAmount = stake.amount
            .mul(stakingRewardRate)
            .mul(stakingDuration)
            .div(365 days)
            .div(100);
        
        if (rewardAmount > 0 && totalSupply().add(rewardAmount) <= MAX_SUPPLY) {
            _mint(msg.sender, rewardAmount);
            stake.rewardDebt = stake.rewardDebt.add(rewardAmount);
            stake.timestamp = block.timestamp;
            
            emit StakingRewardClaimed(msg.sender, rewardAmount);
        }
    }
    
    /**
     * @dev Create a governance proposal
     * @param description Description of the proposal
     */
    function createProposal(string memory description) external {
        require(balanceOf(msg.sender) >= proposalThreshold, "Insufficient tokens for proposal");
        require(bytes(description).length > 0, "Empty proposal description");
        
        proposalCount++;
        Proposal storage proposal = proposals[proposalCount];
        proposal.id = proposalCount;
        proposal.proposer = msg.sender;
        proposal.description = description;
        proposal.startTime = block.timestamp;
        proposal.endTime = block.timestamp.add(votingPeriod);
        proposal.executed = false;
        
        emit ProposalCreated(proposalCount, msg.sender, description);
    }
    
    /**
     * @dev Vote on a proposal
     * @param proposalId ID of the proposal
     * @param support True for support, false for against
     */
    function vote(uint256 proposalId, bool support) external {
        require(proposalId > 0 && proposalId <= proposalCount, "Invalid proposal ID");
        
        Proposal storage proposal = proposals[proposalId];
        require(block.timestamp >= proposal.startTime, "Voting not started");
        require(block.timestamp <= proposal.endTime, "Voting ended");
        require(!proposal.hasVoted[msg.sender], "Already voted");
        
        uint256 votingPower = balanceOf(msg.sender).add(stakes[msg.sender].amount);
        require(votingPower > 0, "No voting power");
        
        proposal.hasVoted[msg.sender] = true;
        proposal.voteChoice[msg.sender] = support;
        
        if (support) {
            proposal.forVotes = proposal.forVotes.add(votingPower);
        } else {
            proposal.againstVotes = proposal.againstVotes.add(votingPower);
        }
        
        emit VoteCast(proposalId, msg.sender, support, votingPower);
    }
    
    /**
     * @dev Execute a successful proposal
     * @param proposalId ID of the proposal to execute
     */
    function executeProposal(uint256 proposalId) external {
        require(proposalId > 0 && proposalId <= proposalCount, "Invalid proposal ID");
        
        Proposal storage proposal = proposals[proposalId];
        require(block.timestamp > proposal.endTime, "Voting still active");
        require(!proposal.executed, "Proposal already executed");
        require(proposal.forVotes > proposal.againstVotes, "Proposal rejected");
        
        proposal.executed = true;
        
        // Execute proposal logic here
        // This would typically call other contract functions or update parameters
        
        emit ProposalExecuted(proposalId);
    }
    
    /**
     * @dev Get voting power of an address
     * @param account Address to check
     * @return Voting power (balance + staked tokens)
     */
    function getVotingPower(address account) external view returns (uint256) {
        return balanceOf(account).add(stakes[account].amount);
    }
    
    /**
     * @dev Get pending staking rewards
     * @param account Address to check
     * @return Pending reward amount
     */
    function getPendingRewards(address account) external view returns (uint256) {
        StakeInfo storage stake = stakes[account];
        if (stake.amount == 0) return 0;
        
        uint256 stakingDuration = block.timestamp.sub(stake.timestamp);
        return stake.amount
            .mul(stakingRewardRate)
            .mul(stakingDuration)
            .div(365 days)
            .div(100);
    }
    
    /**
     * @dev Add authorized rewarder
     * @param rewarder Address to authorize
     */
    function addAuthorizedRewarder(address rewarder) external onlyOwner {
        authorizedRewarders[rewarder] = true;
    }
    
    /**
     * @dev Remove authorized rewarder
     * @param rewarder Address to remove authorization
     */
    function removeAuthorizedRewarder(address rewarder) external onlyOwner {
        authorizedRewarders[rewarder] = false;
    }
    
    /**
     * @dev Update conversation reward rate
     * @param newRate New reward rate
     */
    function updateConversationRewardRate(uint256 newRate) external onlyOwner {
        conversationRewardRate = newRate;
    }
    
    /**
     * @dev Update staking reward rate
     * @param newRate New staking reward rate (percentage)
     */
    function updateStakingRewardRate(uint256 newRate) external onlyOwner {
        require(newRate <= 20, "Reward rate too high"); // Max 20% annual
        stakingRewardRate = newRate;
    }
    
    /**
     * @dev Pause token transfers
     */
    function pause() external onlyOwner {
        _pause();
    }
    
    /**
     * @dev Unpause token transfers
     */
    function unpause() external onlyOwner {
        _unpause();
    }
    
    /**
     * @dev Emergency withdrawal of tokens (only owner)
     * @param amount Amount to withdraw
     */
    function emergencyWithdraw(uint256 amount) external onlyOwner {
        require(amount <= balanceOf(address(this)), "Insufficient contract balance");
        _transfer(address(this), owner(), amount);
    }
    
    // Override required by Solidity
    function _beforeTokenTransfer(
        address from,
        address to,
        uint256 amount
    ) internal override(ERC20, ERC20Pausable) {
        super._beforeTokenTransfer(from, to, amount);
    }
    
    /**
     * @dev Burn tokens to reduce supply (deflationary mechanism)
     * @param amount Amount of tokens to burn
     */
    function burnTokens(uint256 amount) external {
        require(balanceOf(msg.sender) >= amount, "Insufficient balance to burn");
        _burn(msg.sender, amount);
    }
    
    /**
     * @dev Get proposal details
     * @param proposalId ID of the proposal
     * @return Proposal details
     */
    function getProposal(uint256 proposalId) external view returns (
        uint256 id,
        address proposer,
        string memory description,
        uint256 forVotes,
        uint256 againstVotes,
        uint256 startTime,
        uint256 endTime,
        bool executed
    ) {
        require(proposalId > 0 && proposalId <= proposalCount, "Invalid proposal ID");
        
        Proposal storage proposal = proposals[proposalId];
        return (
            proposal.id,
            proposal.proposer,
            proposal.description,
            proposal.forVotes,
            proposal.againstVotes,
            proposal.startTime,
            proposal.endTime,
            proposal.executed
        );
    }
}
