use cosmwasm_std::{DepsMut, Env, MessageInfo};

use crate::{msg::ExecuteMsg, state::{rbac::QueuedProposal, storage::{PROPOSAL_QUEUE, TIMELOCK_DELAY}}, error::ContractError};


/// Used to iterate over the proposal queue and process any proposals that have passed the time lock delay.
/// 
/// If count is a non-zero value, we process no more than `count` proposals. This can be used to limit the number
/// of proposals processed in a single transaction to avoid running into OOG (out of gas) errors.
/// 
/// Because we iterate over the queue by popping items from the front, multiple transactions can be issued
/// in sequence to iterate over the queue
/// 
/// TODO: test
pub fn process_proposal_queue(deps: DepsMut, env: Env, count: usize) -> Result<(), ContractError> {
    let queue_len = PROPOSAL_QUEUE.len(deps.storage)? as usize;

    for idx in 0..queue_len {
        // rust ranges are exclusive of the end, so +1 to the index to see if it matches the count
        if count != 0 && idx + 1 >= count {
            break;
        }
        if let Some(proposal) = PROPOSAL_QUEUE.pop_front(deps.storage)? {
            // check to see if the timelock delay has passed, which we need to first convert from hours int oseconds
            if env.block.time.ge(&proposal
                .submitted_at
                .plus_seconds(proposal.timelock_delay * 60 * 60))
            {
                // invoke message processing functions
            } else {
                PROPOSAL_QUEUE.push_back(deps.storage, &proposal)?;
            }
        }
    }
    Ok(())
}

/// Given a message to execute, insert into the proposal queued with execution delayed by the timelock that is applied to the sender of the message
/// TODO: test
pub fn queue_proposal(
    deps: DepsMut,
    env: Env,
    msg: ExecuteMsg,
    info: MessageInfo
) -> Result<(), ContractError> {
    
    //The timelocking functionality only applies to specific message types
    // so check the incoming message to see if time
    let timelock_delay = TIMELOCK_DELAY.load(deps.storage, info.sender.to_string())?;

    // if the current timelock delay is 0, proposals are executed immediately without any delay
    if timelock_delay == 0 {
        // invoke functions that process the individual `ExecuteMsg` variants
        return Ok(());
    }
    PROPOSAL_QUEUE.push_back(
        deps.storage,
        &QueuedProposal {
            proposal_id: format!("{}_{}", env.block.height, env.transaction.unwrap().index),
            message: msg,
            timelock_delay,
            submitted_at: env.block.time,
        },
    )?;
    Ok(())
}

/// Check to see if the message sender has a non-zero timelock delay configured
/// TODO: test
pub fn must_queue_proposal(
    deps: DepsMut,
    info: &MessageInfo
) -> bool {
    // if a non zero value is set, then it means a timelock delay is required
    TIMELOCK_DELAY.load(deps.storage, info.sender.to_string()).unwrap_or(0) > 0
}