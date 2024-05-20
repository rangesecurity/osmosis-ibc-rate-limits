use crate::state::{path::Path, storage::{MESSAGE_QUEUE, RATE_LIMIT_TRACKERS, RBAC_PERMISSIONS}};
use cosmwasm_std::Order::Ascending;
use cosmwasm_std::{to_binary, Binary, Deps, StdResult};

pub fn get_quotas(
    deps: Deps,
    channel_id: impl Into<String>,
    denom: impl Into<String>,
) -> StdResult<Binary> {
    let path = Path::new(channel_id, denom);
    to_binary(&RATE_LIMIT_TRACKERS.load(deps.storage, path.into())?)
}

/// Returns all addresses which have been assigned one or more roles
pub fn get_role_owners(deps: Deps) -> StdResult<Binary> {
    to_binary(
        &RBAC_PERMISSIONS
            .keys(deps.storage, None, None, Ascending)
            .filter_map(|key| key.ok())
            .collect::<Vec<_>>(),
    )
}

/// Returns all the roles that have been granted to `owner` (if any)
pub fn get_roles(deps: Deps, owner: String) -> StdResult<Binary> {
    to_binary(&RBAC_PERMISSIONS.load(deps.storage, owner)?)
}

/// Returns the id's of all queued proposals
pub fn get_message_ids(deps: Deps) -> StdResult<Binary> {
    to_binary(
        &MESSAGE_QUEUE
            .iter(deps.storage)?
            .filter_map(|proposal| Some(proposal.ok()?.message_id))
            .collect::<Vec<_>>(),
    )
}

#[cfg(test)]
mod test {
    use cosmwasm_std::{from_binary, testing::mock_dependencies, Timestamp};

    use crate::{
        msg::ExecuteMsg,
        state::rbac::{QueuedMessage, Roles},
    };

    use super::*;
    #[test]
    fn test_get_role_owners() {
        let mut deps = mock_dependencies();

        // test getting role owners when no owners exist
        let response = get_role_owners(deps.as_ref()).unwrap();
        let decoded: Vec<String> = from_binary(&response).unwrap();
        assert!(decoded.is_empty());

        // insert 1 role owner, and test getting role owners
        RBAC_PERMISSIONS
            .save(
                &mut deps.storage,
                "foobar".to_string(),
                &vec![Roles::SetTimelockDelay],
            )
            .unwrap();
        let response = get_role_owners(deps.as_ref()).unwrap();
        let decoded: Vec<String> = from_binary(&response).unwrap();
        assert_eq!(decoded.len(), 1);
        assert_eq!(decoded[0], "foobar");

        // insert another role owner and test getting role owners
        RBAC_PERMISSIONS
            .save(
                &mut deps.storage,
                "foobarbaz".to_string(),
                &vec![Roles::SetTimelockDelay],
            )
            .unwrap();
        let response = get_role_owners(deps.as_ref()).unwrap();
        let decoded: Vec<String> = from_binary(&response).unwrap();
        assert_eq!(decoded.len(), 2);
        assert_eq!(decoded[0], "foobar");
        assert_eq!(decoded[1], "foobarbaz");
    }

    #[test]
    fn test_get_roles() {
        let mut deps = mock_dependencies();

        // test retrieving roles for a missing role owner
        assert!(get_roles(deps.as_ref(), "foobar".to_string()).is_err());

        // assign roles and test retrieving roles owned by address
        RBAC_PERMISSIONS
            .save(
                &mut deps.storage,
                "foobar".to_string(),
                &vec![Roles::SetTimelockDelay],
            )
            .unwrap();
        let response = get_roles(deps.as_ref(), "foobar".to_string()).unwrap();
        let decoded: Vec<Roles> = from_binary(&response).unwrap();
        assert_eq!(decoded.len(), 1);
        assert_eq!(decoded[0], Roles::SetTimelockDelay);

        // add additional roles foobar, and test retrierval
        RBAC_PERMISSIONS
            .save(
                &mut deps.storage,
                "foobar".to_string(),
                &vec![Roles::SetTimelockDelay, Roles::EditPathQuota],
            )
            .unwrap();
        let response = get_roles(deps.as_ref(), "foobar".to_string()).unwrap();
        let decoded: Vec<Roles> = from_binary(&response).unwrap();
        assert_eq!(decoded.len(), 2);
        assert_eq!(decoded[0], Roles::SetTimelockDelay);
        assert_eq!(decoded[1], Roles::EditPathQuota);
    }

    #[test]
    fn test_get_proposal_ids() {
        let mut deps = mock_dependencies();
        let response = get_message_ids(deps.as_ref()).unwrap();
        let decoded: Vec<String> = from_binary(&response).unwrap();
        assert_eq!(decoded.len(), 0);
        
        MESSAGE_QUEUE
            .push_back(
                &mut deps.storage,
                &QueuedMessage {
                    message_id: "prop-1".to_string(),
                    message: ExecuteMsg::ProcessProposals { count: 1 },
                    submitted_at: Timestamp::default(),
                    timelock_delay: 0,
                },
            )
            .unwrap();
        MESSAGE_QUEUE
            .push_back(
                &mut deps.storage,
                &QueuedMessage {
                    message_id: "prop-2".to_string(),
                    message: ExecuteMsg::ProcessProposals { count: 1 },
                    submitted_at: Timestamp::default(),
                    timelock_delay: 0,
                },
            )
            .unwrap();
        let response = get_message_ids(deps.as_ref()).unwrap();
        let decoded: Vec<String> = from_binary(&response).unwrap();
        assert_eq!(decoded.len(), 2);
        assert_eq!(decoded[0], "prop-1");
        assert_eq!(decoded[1], "prop-2");
    }
}
