/// returned when bulk project permission grant fails
#[derive(Debug, thiserror::Error)]
pub enum BulkProjectPermissionGrantError {
    /// returned when the token user provided is invalid
    #[error("invalid_token")]
    InvalidToken
}

/// returned when bulk project permission revoke fails
#[derive(Debug, thiserror::Error)]
pub enum BulkProjectPermissionRevokeError {
    /// returned when the token user provided is invalid
    #[error("invalid_token")]
    InvalidToken
}

/// returned when checking if the user has the global service permission fails
#[derive(Debug, thiserror::Error)]
pub enum ServicePermissionCheckError {
    /// returned when the token user provided is invalid
    #[error("invalid_token")]
    InvalidToken
}

/// returned when checking if the user has a project permission fails
#[derive(Debug, thiserror::Error)]
pub enum ProjectPermissionCheckError {
    /// returned when the token user provided is invalid
    #[error("invalid_token")]
    InvalidToken
}