use cosmwasm_std::{DepsMut, Env, MessageInfo};

use crate::{msg::ExecuteMsg, state::{rbac::QueuedMessage, storage::{MESSAGE_QUEUE, TIMELOCK_DELAY}}, error::ContractError};


/// Used to iterate over the message queue and process any messages that have passed the time lock delay.
/// 
/// If count is a non-zero value, we process no more than `count` proposals. This can be used to limit the number
/// of proposals processed in a single transaction to avoid running into OOG (out of gas) errors.
/// 
/// Because we iterate over the queue by popping items from the front, multiple transactions can be issued
/// in sequence to iterate over the queue
/// 
/// TODO: test
pub fn process_proposal_queue(deps: &mut DepsMut, env: Env, count: usize) -> Result<(), ContractError> {
    let queue_len = MESSAGE_QUEUE.len(deps.storage)? as usize;
    
    for idx in 0..queue_len {
        if idx + 1 > count {
            break;
        }
        if let Some(proposal) = MESSAGE_QUEUE.pop_front(deps.storage)? {
            // compute the minimum time at which the proposal is unlocked
            let min_unlock = proposal
            .submitted_at
            .plus_seconds(proposal.timelock_delay * 60 * 60);
            
            // check to see if the timelock delay has passed, which we need to first convert from hours int oseconds
            if env.block.time.ge(&min_unlock)
            {
                crate::contract::match_execute(deps, &env, proposal.message)?;
            } else {
                MESSAGE_QUEUE.push_back(deps.storage, &proposal)?;
            }
        }
    }
    Ok(())
}

/// Given a message to execute, insert into the proposal queued with execution delayed by the timelock that is applied to the sender of the message
/// 
/// Returns the id of the queued proposal
pub fn queue_message(
    deps: &mut DepsMut,
    env: Env,
    msg: ExecuteMsg,
    info: MessageInfo
) -> Result<String, ContractError> {
    
    let timelock_delay = TIMELOCK_DELAY.load(deps.storage, info.sender.to_string())?;
    let message_id = format!("{}_{}", env.block.height, env.transaction.unwrap().index);
    MESSAGE_QUEUE.push_back(
        deps.storage,
        &QueuedMessage {
            message_id: message_id.clone(),
            message: msg,
            timelock_delay,
            submitted_at: env.block.time,
        },
    )?;
    Ok(message_id)
}

/// Check to see if the message sender has a non-zero timelock delay configured
pub fn must_queue_message(
    deps: &mut DepsMut,
    info: &MessageInfo
) -> bool {
    // if a non zero value is set, then it means a timelock delay is required
    TIMELOCK_DELAY.load(deps.storage, info.sender.to_string()).unwrap_or(0) > 0
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{from_binary, testing::{mock_dependencies, mock_env}, Addr, MemoryStorage, Timestamp, TransactionInfo};

    use crate::{msg::QuotaMsg, query::get_queued_message, rbac::set_timelock_delay};

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

        assert!(must_queue_message(&mut deps, &foobar_info));
        assert!(!must_queue_message(&mut deps, &foobarbaz_info));
    }

    #[test]
    fn test_queue_message() {
        let mut env = mock_env();
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
        let foobar_test_msg = ExecuteMsg::AddPath {
            channel_id: "channel".to_string(),
            denom: "denom".to_string(),
            quotas: vec![
                QuotaMsg {
                    name: "quota".to_string(),
                    duration: 5,
                    send_recv: (10, 10)
                }
            ]
        };
        let foobarbaz_test_msg = ExecuteMsg::SetTimelockDelay { 
            signer: "gov".to_string(), 
            hours: 5 
        };
        set_timelock_delay(&mut deps, "foobar".to_string(), 10).unwrap();
        set_timelock_delay(&mut deps, "foobarbaz".to_string(), 1).unwrap();
        let foobar_message_id = {
            let mut env = env.clone();
            env.transaction = Some(TransactionInfo {
                index: 1
            });
            queue_message(
                &mut deps,
                env.clone(),
                foobar_test_msg.clone(),
                foobar_info.clone()
            ).unwrap()
        };
        let foobarbaz_message_id = {
            let mut env = env.clone();
            env.transaction = Some(TransactionInfo {
                index: 2
            });
            queue_message(
                &mut deps,
                env.clone(),
                foobarbaz_test_msg.clone(),
                foobarbaz_info.clone()
            ).unwrap()
        };
        // get foobar's queued message, and validate the type is as expected + timelock delays
        let msg = get_queued_message(
            deps.storage,
            foobar_message_id.clone()
        ).unwrap();
        let msg: QueuedMessage = from_binary(&msg).unwrap();
        assert_eq!(msg.timelock_delay, 10);
        assert_eq!(msg.message, foobar_test_msg);

        // get foobarbaz's queued message, and validate the type is as expected + timelock delays
        let msg = get_queued_message(
            deps.storage,
            foobarbaz_message_id.clone()
        ).unwrap();
        let msg: QueuedMessage = from_binary(&msg).unwrap();
        assert_eq!(msg.timelock_delay, 1);
        assert_eq!(msg.message, foobarbaz_test_msg);
    }

    #[test]
    fn test_process_message_queue_basic() {
        // basic test which simply iterates over the proposal queues
        // does include tests with some unlocked items vs some locked items

        let mut deps = mock_dependencies();
        let mut deps = deps.as_mut();
        let mut env = mock_env();
        create_n_messages(&mut deps, 10);
        assert_eq!(MESSAGE_QUEUE.len(deps.storage).unwrap(), 10);

        process_proposal_queue(
            &mut deps,
            env.clone(),
            1
        ).unwrap();
        assert_eq!(MESSAGE_QUEUE.len(deps.storage).unwrap(), 9);

        process_proposal_queue(
            &mut deps,
            env.clone(),
            0,
        ).unwrap();
        assert_eq!(MESSAGE_QUEUE.len(deps.storage).unwrap(), 9);
    }

    #[test]
    fn test_process_proposal_queue_complete() {
        // complete proposal queues testing, including some locked vs unlocked
        // as well as validating execution
    }

    // helper function which inserts N messages into the message queue
    // message types inserted are of ExecuteMsg::SetTimelockDelay
    fn create_n_messages(deps: &mut DepsMut, n: usize) {
        for i in 0..n {
            MESSAGE_QUEUE.push_back(
                deps.storage,
                &QueuedMessage {
                    message: ExecuteMsg::SetTimelockDelay {
                        signer: "signer".to_string(),
                        hours: i as u64 + 1
                    },
                    submitted_at: Timestamp::default(),
                    timelock_delay: 24,
                    message_id: "prop-1".to_string()
                }
            ).unwrap();
        }
    } 
}