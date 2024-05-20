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
pub fn process_proposal_queue(deps: &mut DepsMut, env: Env, count: usize) -> Result<(), ContractError> {
    let queue_len = PROPOSAL_QUEUE.len(deps.storage)? as usize;
    
    for idx in 0..queue_len {
        if idx + 1 > count {
            break;
        }
        if let Some(proposal) = PROPOSAL_QUEUE.pop_front(deps.storage)? {
            // compute the minimum time at which the proposal is unlocked
            let min_unlock = proposal
            .submitted_at
            .plus_seconds(proposal.timelock_delay * 60 * 60);
            
            // check to see if the timelock delay has passed, which we need to first convert from hours int oseconds
            if env.block.time.ge(&min_unlock)
            {
                crate::contract::match_execute(deps, &env, proposal.message)?;
            } else {
                PROPOSAL_QUEUE.push_back(deps.storage, &proposal)?;
            }
        }
    }
    Ok(())
}

/// Given a message to execute, insert into the proposal queued with execution delayed by the timelock that is applied to the sender of the message
/// 
/// Returns the id of the queued proposal
/// 
/// TODO: test
pub fn queue_proposal(
    deps: DepsMut,
    env: Env,
    msg: ExecuteMsg,
    info: MessageInfo
) -> Result<String, ContractError> {
    
    let timelock_delay = TIMELOCK_DELAY.load(deps.storage, info.sender.to_string())?;
    let proposal_id = format!("{}_{}", env.block.height, env.transaction.unwrap().index);
    PROPOSAL_QUEUE.push_back(
        deps.storage,
        &QueuedProposal {
            proposal_id: proposal_id.clone(),
            message: msg,
            timelock_delay,
            submitted_at: env.block.time,
        },
    )?;
    Ok(proposal_id)
}

/// Check to see if the message sender has a non-zero timelock delay configured
/// TODO: test
pub fn must_queue_proposal(
    deps: &mut DepsMut,
    info: &MessageInfo
) -> bool {
    // if a non zero value is set, then it means a timelock delay is required
    TIMELOCK_DELAY.load(deps.storage, info.sender.to_string()).unwrap_or(0) > 0
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::{mock_dependencies, mock_env}, Addr, MemoryStorage, Timestamp};

    use super::*;

    #[test]
    fn test_must_queue_proposal() {
        let mut deps = mock_dependencies();
        let mut deps = deps.as_mut();
        let foobar_info = MessageInfo {
            sender: Addr::unchecked("foobar"),
            funds: vec![]
        };
        let foobarbaz_info = MessageInfo {
            sender: Addr::unchecked("foobarbaz"),
            funds: vec![]
        };

        TIMELOCK_DELAY.save(deps.storage, "foobar".to_string(), &1).unwrap();

        assert!(must_queue_proposal(&mut deps, &foobar_info));
        assert!(!must_queue_proposal(&mut deps, &foobarbaz_info));
    }

    #[test]
    fn test_process_proposal_queue_basic() {
        // basic test which simply iterates over the proposal queues
        // does include tests with some unlocked items vs some locked items

        let mut deps = mock_dependencies();
        let mut deps = deps.as_mut();
        let mut env = mock_env();
        create_n_proposals(&mut deps, 10);
        assert_eq!(PROPOSAL_QUEUE.len(deps.storage).unwrap(), 10);

        process_proposal_queue(
            &mut deps,
            env.clone(),
            1
        ).unwrap();
        assert_eq!(PROPOSAL_QUEUE.len(deps.storage).unwrap(), 9);

        process_proposal_queue(
            &mut deps,
            env.clone(),
            0,
        ).unwrap();
        assert_eq!(PROPOSAL_QUEUE.len(deps.storage).unwrap(), 9);
    }

    #[test]
    fn test_process_proposal_queue_complete() {
        // complete proposal queues testing, including some locked vs unlocked
        // as well as validating execution
    }


    fn create_n_proposals(deps: &mut DepsMut, n: usize) {
        for i in 0..n {
            PROPOSAL_QUEUE.push_back(
                deps.storage,
                &QueuedProposal {
                    message: ExecuteMsg::SetTimelockDelay {
                        signer: "signer".to_string(),
                        hours: i as u64 + 1
                    },
                    submitted_at: Timestamp::default(),
                    timelock_delay: 24,
                    proposal_id: "prop-1".to_string()
                }
            ).unwrap();
        }
    } 
}