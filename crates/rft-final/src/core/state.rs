use std::sync::Arc;

use crate::domain::user::UserService;

#[derive(Clone)]
pub struct AppState {
    pub user_service: Arc<UserService>,
}
