/// Returned when bulk project permission grant fails
#[derive(Debug, thiserror::Error)]
pub enum BulkProjectPermissionGrantError {
    /// Returned when the token user provided is invalid
    #[error("invalid_token")]
    InvalidToken,
}

/// Returned when bulk project permission revoke fails
#[derive(Debug, thiserror::Error)]
pub enum BulkProjectPermissionRevokeError {
    /// Returned when the token user provided is invalid
    #[error("invalid_token")]
    InvalidToken,
}

/// Returned when checking if the user has the global service permission fails
#[derive(Debug, thiserror::Error)]
pub enum ServicePermissionCheckError {
    /// Returned when the token user provided is invalid
    #[error("invalid_token")]
    InvalidToken,
}

/// Returned when checking if the user has a project permission fails
#[derive(Debug, thiserror::Error)]
pub enum ProjectPermissionCheckError {
    /// Returned when the token user provided is invalid
    #[error("invalid_token")]
    InvalidToken,
}
