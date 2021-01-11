use crate::managers::permission::PermissionManager;

pub struct AuthManager {
    pub permissions: PermissionManager,
    pub remote: String,
    pub is_admin: bool,
    pub is_root: bool,
    pub username: String,
    pub certificate: String,
    pub validated: bool,
}

