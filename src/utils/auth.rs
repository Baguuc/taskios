/// grant project permissions to a user in bulk. <br>
/// this function _do not check_ if provided permissions exist so use cautiously (make sure the taskios server inited the permissions beforehand).
pub async fn bulk_grant_project_permissions(
    user_token: String,
    project_id: i32,
    permission_names: Vec<String>,
    authios_client: std::sync::Arc<authios_sdk::AuthiosClient>,
    root_password: String
) -> Result<(), crate::errors::utils::auth::BulkProjectPermissionGrantError> {
    use crate::utils::panic::UtilPanics;
    use crate::errors::utils::auth::BulkProjectPermissionGrantError as Error;
    use authios_sdk::requests::{
        SpecificUserGrantResourcePermissionRequest as PermissionGrantRequest,
        LoggedUserGetInfoRequest as GetInfoRequest
    };
    use authios_sdk::responses::LoggedUserGetInfoResponse as GetInfoResponse;

    let auth_response = authios_client.query()
        .user()
        .logged(user_token.clone())
        .get_info(GetInfoRequest {
            get_id: true,
            get_login: false,
            get_password_hash: false
        })
        .await;

    let user = match auth_response {
        GetInfoResponse::Ok { user } => user,
        // won't happen anyways
        GetInfoResponse::InvalidToken => return Err(Error::InvalidToken),
        GetInfoResponse::ServerNotAuthios => { UtilPanics::server_not_authios(); std::process::exit(1); },
        GetInfoResponse::ServerUnavailable => { UtilPanics::authios_unavailable(); std::process::exit(1); }
    };

    let user_id = user.id.unwrap();

    for permission_name in permission_names {
        let _ = authios_client.query()
            .user()
            .specific(root_password.clone(), user_id)
            .permissions()
            .resource()
            .grant(PermissionGrantRequest {
                service_id: String::from("taskios"),
                resource_type: String::from("project"),
                resource_id: project_id.to_string(),
                permission_name
            })
            .await;
    }

    Ok(())
}

/// grant project permissions to a user in bulk. <br>
/// this function _do not check_ if provided permissions exist so use cautiously (make sure the taskios server inited the permissions beforehand)..
pub async fn bulk_revoke_project_permissions(
    user_token: String,
    project_id: i32,
    permission_names: Vec<String>,
    authios_client: std::sync::Arc<authios_sdk::AuthiosClient>,
    root_password: String
) -> Result<(), crate::errors::utils::auth::BulkProjectPermissionRevokeError> {
    use crate::utils::panic::UtilPanics;
    use crate::errors::utils::auth::BulkProjectPermissionRevokeError as Error;
    use authios_sdk::requests::{
        SpecificUserRevokeResourcePermissionRequest as PermissionRevokeRequest,
        LoggedUserGetInfoRequest as GetInfoRequest
    };
    use authios_sdk::responses::LoggedUserGetInfoResponse as GetInfoResponse;

    let auth_response = authios_client.query()
        .user()
        .logged(user_token.clone())
        .get_info(GetInfoRequest {
            get_id: true,
            get_login: false,
            get_password_hash: false
        })
        .await;

    let user = match auth_response {
        GetInfoResponse::Ok { user } => user,
        // won't happen anyways
        GetInfoResponse::InvalidToken => return Err(Error::InvalidToken),
        GetInfoResponse::ServerNotAuthios => { UtilPanics::server_not_authios(); std::process::exit(1); },
        GetInfoResponse::ServerUnavailable => { UtilPanics::authios_unavailable(); std::process::exit(1); }
    };

    let user_id = user.id.unwrap();

    for permission_name in permission_names {
        let _ = authios_client.query()
            .user()
            .specific(root_password.clone(), user_id)
            .permissions()
            .resource()
            .revoke(PermissionRevokeRequest {
                service_id: String::from("taskios"),
                resource_type: String::from("project"),
                resource_id: project_id.to_string(),
                permission_name
            })
            .await;
    }

    Ok(())
}