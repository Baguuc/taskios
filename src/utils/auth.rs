/// grant project permissions to a user in bulk. <br>
/// this function _do not check_ if provided permissions exist so use cautiously (make sure the taskios server inited the permissions beforehand).
pub async fn bulk_grant_project_permissions(
    user_token: String,
    project_id: i32,
    permission_names: Vec<String>,
    authios_client: std::sync::Arc<authios_sdk::AuthiosClient>,
    root_password: String,
) -> Result<(), crate::errors::utils::auth::BulkProjectPermissionGrantError> {
    use crate::errors::utils::auth::BulkProjectPermissionGrantError as Error;
    use crate::utils::panic::UtilPanics;
    use authios_sdk::requests::{
        LoggedUserGetInfoRequest as GetInfoRequest,
        SpecificUserGrantResourcePermissionRequest as PermissionGrantRequest,
    };
    use authios_sdk::responses::LoggedUserGetInfoResponse as GetInfoResponse;

    let auth_response = authios_client
        .query()
        .user()
        .logged(user_token.clone())
        .get_info(GetInfoRequest {
            get_id: true,
            get_login: false,
            get_password_hash: false,
        })
        .await;

    let user = match auth_response {
        GetInfoResponse::Ok { user } => user,
        // won't happen anyways
        GetInfoResponse::InvalidToken => return Err(Error::InvalidToken),
        GetInfoResponse::ServerNotAuthios => {
            UtilPanics::server_not_authios();
            std::process::exit(1);
        }
        GetInfoResponse::ServerUnavailable => {
            UtilPanics::authios_unavailable();
            std::process::exit(1);
        }
    };

    let user_id = user.id.unwrap();

    for permission_name in permission_names {
        let _ = authios_client
            .query()
            .user()
            .specific(root_password.clone(), user_id)
            .permissions()
            .resource()
            .grant(PermissionGrantRequest {
                service_id: crate::config::SERVICE_NAME.to_string(),
                resource_type: String::from("project"),
                resource_id: project_id.to_string(),
                permission_name,
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
    root_password: String,
) -> Result<(), crate::errors::utils::auth::BulkProjectPermissionRevokeError> {
    use crate::errors::utils::auth::BulkProjectPermissionRevokeError as Error;
    use crate::utils::panic::UtilPanics;
    use authios_sdk::requests::{
        LoggedUserGetInfoRequest as GetInfoRequest,
        SpecificUserRevokeResourcePermissionRequest as PermissionRevokeRequest,
    };
    use authios_sdk::responses::LoggedUserGetInfoResponse as GetInfoResponse;

    let auth_response = authios_client
        .query()
        .user()
        .logged(user_token.clone())
        .get_info(GetInfoRequest {
            get_id: true,
            get_login: false,
            get_password_hash: false,
        })
        .await;

    let user = match auth_response {
        GetInfoResponse::Ok { user } => user,
        // won't happen anyways
        GetInfoResponse::InvalidToken => return Err(Error::InvalidToken),
        GetInfoResponse::ServerNotAuthios => {
            UtilPanics::server_not_authios();
            std::process::exit(1);
        }
        GetInfoResponse::ServerUnavailable => {
            UtilPanics::authios_unavailable();
            std::process::exit(1);
        }
    };

    let user_id = user.id.unwrap();

    for permission_name in permission_names {
        let _ = authios_client
            .query()
            .user()
            .specific(root_password.clone(), user_id)
            .permissions()
            .resource()
            .revoke(PermissionRevokeRequest {
                service_id: crate::config::SERVICE_NAME.to_string(),
                resource_type: String::from("project"),
                resource_id: project_id.to_string(),
                permission_name,
            })
            .await;
    }

    Ok(())
}

/// check if user is permitted to access a project with provided permission.
pub async fn check_user_project_permission(
    user_token: String,
    project_id: i32,
    permission_name: String,
    authios_client: std::sync::Arc<authios_sdk::AuthiosClient>,
) -> Result<bool, crate::errors::utils::auth::ProjectPermissionCheckError> {
    use crate::errors::utils::auth::ProjectPermissionCheckError as Error;
    use crate::utils::panic::UtilPanics;
    use authios_sdk::requests::LoggedUserCheckResourcePermissionRequest as Request;
    use authios_sdk::responses::LoggedUserCheckResourcePermissionResponse as Response;

    let auth_response = authios_client
        .query()
        .user()
        .logged(user_token)
        .permissions()
        .resource()
        .check(Request {
            service_id: crate::config::SERVICE_NAME.to_string(),
            resource_type: String::from("project"),
            resource_id: project_id.to_string(),
            permission_name,
        })
        .await;

    match auth_response {
        Response::Ok { has_permission } => Ok(has_permission),
        Response::InvalidToken => Err(Error::InvalidToken),
        Response::ServerNotAuthios => {
            UtilPanics::server_not_authios();
            std::process::exit(1);
        }
        Response::ServerUnavailable => {
            UtilPanics::authios_unavailable();
            std::process::exit(1);
        }
        Response::PermissionNotFound => {
            UtilPanics::authios_not_inited();
            std::process::exit(1);
        }
    }
}

/// check if user has the global service permission to access this service.
pub async fn check_user_service_permission(
    user_token: String,
    authios_client: std::sync::Arc<authios_sdk::AuthiosClient>,
) -> Result<bool, crate::errors::utils::auth::ServicePermissionCheckError> {
    use crate::errors::utils::auth::ServicePermissionCheckError as Error;
    use crate::utils::panic::UtilPanics;
    use authios_sdk::requests::LoggedUserCheckServicePermissionRequest as Request;
    use authios_sdk::responses::LoggedUserCheckServicePermissionResponse as Response;

    let auth_response = authios_client
        .query()
        .user()
        .logged(user_token)
        .permissions()
        .service()
        .check(Request {
            service_id: crate::config::SERVICE_NAME.to_string(),
        })
        .await;

    match auth_response {
        Response::Ok { has_permission } => Ok(has_permission),
        Response::InvalidToken => Err(Error::InvalidToken),
        Response::ServerNotAuthios => {
            UtilPanics::server_not_authios();
            std::process::exit(1);
        }
        Response::ServerUnavailable => {
            UtilPanics::authios_unavailable();
            std::process::exit(1);
        }
        Response::PermissionNotFound => {
            UtilPanics::authios_not_inited();
            std::process::exit(1);
        }
    }
}
