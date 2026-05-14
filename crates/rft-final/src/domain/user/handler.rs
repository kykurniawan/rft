use axum::{Json, extract::State};
use axum_extra::extract::Query;

use crate::{
    core::state::AppState,
    domain::{
        shared::{
            dto::{PaginatedResponse, PaginationRequest},
            error::AppError,
        },
        user::{UserResponse, UserServiceError},
    },
};

pub async fn index(
    State(state): State<AppState>,
    Query(request): Query<PaginationRequest>,
) -> Result<Json<PaginatedResponse<UserResponse>>, AppError> {
    let users = state.user_service.get_users(request.into_query()).await;

    let users = match users {
        Ok(users) => users,
        Err(error) => match error {
            UserServiceError::InvalidFilter(_) => {
                return Err(AppError::BadRequest(error.to_string()));
            }
            _ => return Err(AppError::Internal(error.to_string())),
        },
    };

    Ok(Json(PaginatedResponse {
        items: users.items.into_iter().map(UserResponse::from).collect(),
        total: users.total,
        page: users.page,
        page_size: users.page_size,
    }))
}
