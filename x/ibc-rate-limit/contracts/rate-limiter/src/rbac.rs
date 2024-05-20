use std::collections::HashSet;

use cosmwasm_std::{DepsMut, MessageInfo};

use crate::{msg::ExecuteMsg, state::{rbac::Roles, storage::{RBAC_PERMISSIONS, TIMELOCK_DELAY}}, ContractError};

/// Check to see if the sender of the message can invoke the message by holding the required rbac role
/// 
/// # Errors
/// 
/// ContractError::Unauthorized if the sender does not have the required permission
/// 
/// StdErr::NotFound if the RBAC_PERMISSIONS storage variable does not have an entry for the sender
pub fn can_invoke_message(
    deps: &DepsMut,
    info: &MessageInfo,
    msg: &ExecuteMsg,
) -> Result<(), ContractError> {
    // get the required permission to execute the message
    let Some(required_permission) = msg.required_permission() else {
        // no permission required so return ok
        return Ok(());
    };
    let permissions = RBAC_PERMISSIONS.load(deps.storage, info.sender.to_string())?;
    if permissions.contains(&required_permission) {
        return Ok(())
    }
    Err(ContractError::Unauthorized {  })
}

/// Sets a timelock delay for `signer` of `hours`
pub fn set_timelock_delay(
    deps: &mut DepsMut,
    signer: String,
    hours: u64
) -> Result<(), ContractError> {
    Ok(TIMELOCK_DELAY.save(deps.storage, signer, &hours)?)
}

pub fn grant_role(
    deps: &mut DepsMut,
    signer: String,
    roles: Vec<Roles>
) -> Result<(), ContractError> {
    // get the current roles, if no current roles will be an empty vec
    let mut current_roles = RBAC_PERMISSIONS.load(deps.storage, signer.clone()).unwrap_or_default();
    for role in roles {
        current_roles.insert(role);
    }

    // persist new roles
    Ok(RBAC_PERMISSIONS.save(deps.storage, signer, &current_roles)?)
}

pub fn revoke_role(
    deps: &mut DepsMut,
    signer: String,
    roles: Vec<Roles>
) -> Result<(), ContractError> {
    let mut current_roles = RBAC_PERMISSIONS.load(deps.storage, signer.clone())?;
    for role in roles {
        current_roles.remove(&role);
    }
    Ok(RBAC_PERMISSIONS.save(deps.storage, signer, &current_roles)?)
}

#[cfg(test)]
mod test {
    use cosmwasm_std::{testing::mock_dependencies, Addr};

    use crate::{msg::QuotaMsg, state::rbac::Roles};

    use super::*;
    #[test]
    fn test_set_timelock_delay() {
        let mut deps = mock_dependencies();
        assert!(TIMELOCK_DELAY.load(&deps.storage, "foobar".to_string()).is_err());
        set_timelock_delay(&mut deps.as_mut(), "foobar".to_string(), 6).unwrap();
        assert_eq!(TIMELOCK_DELAY.load(&deps.storage, "foobar".to_string()).unwrap(), 6);
    }
    #[test]
    fn test_can_invoke_add_path() {
        let mut deps = mock_dependencies();


        let info_foobar = MessageInfo {
            sender: Addr::unchecked("foobar".to_string()),
            funds: vec![]
        };
        let info_foobarbaz = MessageInfo {
            sender: Addr::unchecked("foobarbaz".to_string()),
            funds: vec![]
        };
        let msg = ExecuteMsg::AddPath { 
            channel_id: "channelid".into(), 
            denom: "denom".into(), 
            quotas: vec![]
        };
        RBAC_PERMISSIONS.save(&mut deps.storage, "foobar".to_string(), &vec![Roles::AddRateLimit].into_iter().collect()).unwrap();

        assert!(can_invoke_message(
            &deps.as_mut(),
            &info_foobar,
            &msg            
        ).is_ok());
        assert!(can_invoke_message(
            &deps.as_mut(),
            &info_foobarbaz,
            &msg            
        ).is_err());

    }

    #[test]
    fn test_can_invoke_remove_path() {
        let mut deps = mock_dependencies();


        let info_foobar = MessageInfo {
            sender: Addr::unchecked("foobar".to_string()),
            funds: vec![]
        };
        let info_foobarbaz = MessageInfo {
            sender: Addr::unchecked("foobarbaz".to_string()),
            funds: vec![]
        };
        let msg = ExecuteMsg::RemovePath { 
            channel_id: "channelid".into(), 
            denom: "denom".into(), 
        };
        RBAC_PERMISSIONS.save(&mut deps.storage, "foobar".to_string(), &vec![Roles::RemoveRateLimit].into_iter().collect()).unwrap();

        assert!(can_invoke_message(
            &deps.as_mut(),
            &info_foobar,
            &msg            
        ).is_ok());
        assert!(can_invoke_message(
            &deps.as_mut(),
            &info_foobarbaz,
            &msg            
        ).is_err());
    }

    #[test]
    fn test_can_invoke_reset_path_quota() {
        let mut deps = mock_dependencies();


        let info_foobar = MessageInfo {
            sender: Addr::unchecked("foobar".to_string()),
            funds: vec![]
        };
        let info_foobarbaz = MessageInfo {
            sender: Addr::unchecked("foobarbaz".to_string()),
            funds: vec![]
        };

        let msg = ExecuteMsg::ResetPathQuota { 
            channel_id: "channelid".into(), 
            denom: "denom".into(),
            quota_id: "quota".into()
        };
        RBAC_PERMISSIONS.save(&mut deps.storage, "foobar".to_string(), &vec![Roles::ResetPathQuota].into_iter().collect()).unwrap();

        assert!(can_invoke_message(
            &deps.as_mut(),
            &info_foobar,
            &msg            
        ).is_ok());
        assert!(can_invoke_message(
            &deps.as_mut(),
            &info_foobarbaz,
            &msg            
        ).is_err());
    }

    #[test]
    fn test_can_invoke_grant_role() {
        let mut deps = mock_dependencies();


        let info_foobar = MessageInfo {
            sender: Addr::unchecked("foobar".to_string()),
            funds: vec![]
        };
        let info_foobarbaz = MessageInfo {
            sender: Addr::unchecked("foobarbaz".to_string()),
            funds: vec![]
        };

        let msg = ExecuteMsg::GrantRole { 
            signer: "signer".into(),
            roles: vec![Roles::GrantRole]
        };
        RBAC_PERMISSIONS.save(&mut deps.storage, "foobar".to_string(), &vec![Roles::GrantRole].into_iter().collect()).unwrap();

        assert!(can_invoke_message(
            &deps.as_mut(),
            &info_foobar,
            &msg            
        ).is_ok());
        assert!(can_invoke_message(
            &deps.as_mut(),
            &info_foobarbaz,
            &msg            
        ).is_err());
    }

    #[test]
    fn test_can_invoke_revoke_role() {
        let mut deps = mock_dependencies();


        let info_foobar = MessageInfo {
            sender: Addr::unchecked("foobar".to_string()),
            funds: vec![]
        };
        let info_foobarbaz = MessageInfo {
            sender: Addr::unchecked("foobarbaz".to_string()),
            funds: vec![]
        };

        let msg = ExecuteMsg::RevokeRole { 
            signer: "signer".into(),
            roles: vec![Roles::GrantRole]
        };
        RBAC_PERMISSIONS.save(&mut deps.storage, "foobar".to_string(), &vec![Roles::RevokeRole].into_iter().collect()).unwrap();
        assert!(can_invoke_message(
            &deps.as_mut(),
            &info_foobar,
            &msg            
        ).is_ok());
        assert!(can_invoke_message(
            &deps.as_mut(),
            &info_foobarbaz,
            &msg            
        ).is_err());
    }

    #[test]
    fn test_can_invoke_edit_path_quota() {
        let mut deps = mock_dependencies();


        let info_foobar = MessageInfo {
            sender: Addr::unchecked("foobar".to_string()),
            funds: vec![]
        };
        let info_foobarbaz = MessageInfo {
            sender: Addr::unchecked("foobarbaz".to_string()),
            funds: vec![]
        };

        let msg = ExecuteMsg::EditPathQuota { 
            quota: QuotaMsg {
                name: "name".into(),
                duration: 0,
                send_recv: (1, 2),
            },
            channel_id: "channel_id".into(),
            denom: "denom".into()
        };
        RBAC_PERMISSIONS.save(&mut deps.storage, "foobar".to_string(), &vec![Roles::EditPathQuota].into_iter().collect()).unwrap();
        assert!(can_invoke_message(
            &deps.as_mut(),
            &info_foobar,
            &msg            
        ).is_ok());
        assert!(can_invoke_message(
            &deps.as_mut(),
            &info_foobarbaz,
            &msg            
        ).is_err());
    }

    #[test]
    fn test_can_invoke_remove_message() {
        let mut deps = mock_dependencies();


        let info_foobar = MessageInfo {
            sender: Addr::unchecked("foobar".to_string()),
            funds: vec![]
        };
        let info_foobarbaz = MessageInfo {
            sender: Addr::unchecked("foobarbaz".to_string()),
            funds: vec![]
        };

        let msg = ExecuteMsg::RemoveMessage { 
            message_id: "message".into()
        };
        RBAC_PERMISSIONS.save(&mut deps.storage, "foobar".to_string(), &vec![Roles::RemoveMessage].into_iter().collect()).unwrap();
        assert!(can_invoke_message(
            &deps.as_mut(),
            &info_foobar,
            &msg            
        ).is_ok());
        assert!(can_invoke_message(
            &deps.as_mut(),
            &info_foobarbaz,
            &msg            
        ).is_err());
    }


    #[test]
    fn test_can_invoke_set_timelock_delay() {
        let mut deps = mock_dependencies();


        let info_foobar = MessageInfo {
            sender: Addr::unchecked("foobar".to_string()),
            funds: vec![]
        };
        let info_foobarbaz = MessageInfo {
            sender: Addr::unchecked("foobarbaz".to_string()),
            funds: vec![]
        };

        let msg = ExecuteMsg::SetTimelockDelay { 
            signer: "signer".into(),
            hours: 5,
        };
        RBAC_PERMISSIONS.save(&mut deps.storage, "foobar".to_string(), &vec![Roles::SetTimelockDelay].into_iter().collect()).unwrap();
        assert!(can_invoke_message(
            &deps.as_mut(),
            &info_foobar,
            &msg            
        ).is_ok());
        assert!(can_invoke_message(
            &deps.as_mut(),
            &info_foobarbaz,
            &msg            
        ).is_err());

    }

    #[test]
    fn test_can_invoke_process_messages() {
        let mut deps = mock_dependencies();


        let info_foobar = MessageInfo {
            sender: Addr::unchecked("foobar".to_string()),
            funds: vec![]
        };
        let info_foobarbaz = MessageInfo {
            sender: Addr::unchecked("foobarbaz".to_string()),
            funds: vec![]
        };

        let msg = ExecuteMsg::ProcessMessages { count: 1 };

        // all addresses should be able to invoke this
        assert!(
            can_invoke_message(
                &deps.as_mut(),
                &info_foobar,
                &msg
            ).is_ok()
        );
        assert!(
            can_invoke_message(
                &deps.as_mut(),
                &info_foobarbaz,
                &msg
            ).is_ok()
        );

    }
}