use crate::managers::permission::PermissionManager;

/// A utility structure for managing authentication with the client
#[allow(dead_code)]
pub struct AuthManager {
    /// A structure representing the permissions that the user has across all of the system
    pub permissions: PermissionManager,

    /// The remote address of the client
    pub remote: String,

    /// If the user is considered an administrator of the system
    pub is_admin: bool,

    /// If the user is considered to be the "root" user
    pub is_root: bool,

    /// The user's name
    pub username: String,

    /// The certificate which the user used during authentication
    pub certificate: String,

    /// If the user has been successfully logged in, or has been instantiated outside of a log-on environment
    pub validated: bool,
}

