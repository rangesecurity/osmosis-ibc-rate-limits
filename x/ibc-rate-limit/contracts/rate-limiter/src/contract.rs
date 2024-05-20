#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, SudoMsg};
use crate::state::rbac::Roles;
use crate::state::storage::RBAC_PERMISSIONS;
use crate::state::{flow::FlowType, storage::{GOVMODULE, IBCMODULE}};
use crate::{execute, query, sudo};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:rate-limiter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    mut deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    IBCMODULE.save(deps.storage, &msg.ibc_module)?;
    GOVMODULE.save(deps.storage, &msg.gov_module)?;
    // grant the gov address full permissions
    RBAC_PERMISSIONS.save(deps.storage, msg.gov_module.to_string(), &Roles::all_roles())?;

    execute::add_new_paths(&mut deps, msg.paths, env.block.time)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("ibc_module", msg.ibc_module.to_string())
        .add_attribute("gov_module", msg.gov_module.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    // check to see if special permissions are required to invoke the message, and that the sender has the required permissions
    crate::rbac::can_invoke_message(&deps, &info, &msg)?;
    // check to see if messages sent by MessageInfo::sender require a timelock
    //
    // if a timelock is required the message must be queued for execution
    if crate::message_queue::must_queue_message(
        &mut deps,
        &info
    ) {
        let proposal_id = crate::message_queue::queue_proposal(deps, env, msg, info)?;
        Ok(Response::new().add_attribute("proposal.id", proposal_id))
    } else {
        match_execute(&mut deps, &env, msg)
    }
    
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut, env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
    match msg {
        SudoMsg::SendPacket {
            packet,
            #[cfg(test)]
            channel_value_mock,
        } => sudo::process_packet(
            deps,
            packet,
            FlowType::Out,
            env.block.time,
            #[cfg(test)]
            channel_value_mock,
        ),
        SudoMsg::RecvPacket {
            packet,
            #[cfg(test)]
            channel_value_mock,
        } => sudo::process_packet(
            deps,
            packet,
            FlowType::In,
            env.block.time,
            #[cfg(test)]
            channel_value_mock,
        ),
        SudoMsg::UndoSend { packet } => sudo::undo_send(deps, packet),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetQuotas { channel_id, denom } => query::get_quotas(deps.storage, channel_id, denom),
        QueryMsg::GetRoleOwners => query::get_role_owners(deps.storage),
        QueryMsg::GetRoles { owner } => query::get_roles(deps.storage, owner),
        QueryMsg::GetMessageIds => query::get_message_ids(deps.storage),
        QueryMsg::GetMessage { id } => query::get_queued_message(deps.storage, id)
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    let gov_module = GOVMODULE.load(deps.storage)?;
    
    // grant the gov address full permissions
    RBAC_PERMISSIONS.save(deps.storage, gov_module.to_string(), &Roles::all_roles())?;

    Ok(Response::new().add_attribute("method", "migrate"))
}


pub(crate) fn match_execute(
    deps: &mut DepsMut,
    env: &Env,
    msg: ExecuteMsg
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AddPath {
            channel_id,
            denom,
            quotas,
        } => execute::try_add_path(deps, channel_id, denom, quotas, env.block.time),
        ExecuteMsg::RemovePath { channel_id, denom } => {
            execute::try_remove_path(deps, channel_id, denom)
        }
        ExecuteMsg::ResetPathQuota {
            channel_id,
            denom,
            quota_id,
        } => execute::try_reset_path_quota(
            deps,
            channel_id,
            denom,
            quota_id,
            env.block.time,
        ),
        ExecuteMsg::GrantRole { signer, roles } => todo!(),
        ExecuteMsg::RevokeRole { signer, roles } => todo!(),
        ExecuteMsg::EditPathQuota {
            channel_id,
            denom,
            quota,
        } => todo!(),
        ExecuteMsg::RemoveProposal { proposal_id } => todo!(),
        ExecuteMsg::SetTimelockDelay { signer, hours } => {
            crate::rbac::set_timelock_delay(deps, signer.clone(), hours)?;
            Ok(Response::new()
                .add_attribute("method", "set_timelock_delay")
                .add_attribute("signer", signer)
                .add_attribute("hours", hours.to_string()))
        }
        ExecuteMsg::ProcessProposals { count } => todo!(),
    }
}