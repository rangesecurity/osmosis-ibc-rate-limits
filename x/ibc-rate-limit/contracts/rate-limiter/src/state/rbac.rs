use cosmwasm_std::{Addr, Timestamp, Uint256};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::cmp;

use crate::{
    msg::{ExecuteMsg, QuotaMsg},
    ContractError,
};

/// Roles defines the available permissions that can be assigned to addresses as part of the RBAC system
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub enum Roles {
    /// Has the ability to add a new rate limit
    AddRateLimit,
    /// Has the ability to complete remove a configured rate limit
    RemoveRateLimit,
    /// Has the ability to reset tracked quotas
    ResetPathQuota,
    /// Has the ability to edit existing quotas
    EditPathQuota,
    /// Has the ability to grant roles to an address
    GrantRole,
    /// Has the ability to revoke granted roles to an address
    RevokeRole,
    /// Has the ability to remove queued proposals
    RemoveProposal,
    /// Has the ability to alter timelock delay's
    SetTimelockDelay,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct QueuedProposal {
    /// the message that submitted to the contract after a successful proposal
    pub message: ExecuteMsg,
    /// the time which the message was processed by the contract
    pub submitted_at: Timestamp,
    /// the timelock delay that was in place when the proposal was queued for execution
    pub timelock_delay: u64,
    /// Constructed using format!("{}_{}", Env::BlockInfo::Height Env::Transaction::Index)
    ///
    /// Can be used to remove a proposal from the queue without processing it
    pub proposal_id: String,
}
