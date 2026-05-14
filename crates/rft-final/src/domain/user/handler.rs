use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use axum_extra::extract::{Query, WithRejection};
use uuid::Uuid;

use crate::{
    core::state::AppState,
    domain::{
        shared::{
            dto::{PaginatedResponse, PaginationRequest},
            error::AppError,
        },
        user::{CreateUserRequest, UpdateUserRequest, UserResponse, UserServiceError},
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
                return Err(AppError::ValidationError(error.to_string()));
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

pub async fn show(
    State(state): State<AppState>,
    WithRejection(Path(id), _): WithRejection<Path<Uuid>, AppError>,
) -> Result<Json<UserResponse>, AppError> {
    let user = state.user_service.get_user_by_id(id).await;

    let user = match user {
        Ok(user) => {
            if user.is_none() {
                return Err(AppError::NotFound(format!("user with id {} not found", id)));
            }
            user.unwrap()
        }
        Err(error) => match error {
            UserServiceError::UserNotFound => return Err(AppError::NotFound(error.to_string())),
            _ => return Err(AppError::Internal(error.to_string())),
        },
    };

    Ok(Json(UserResponse::from(user)))
}

pub async fn store(
    State(state): State<AppState>,
    WithRejection(Json(request), _): WithRejection<Json<CreateUserRequest>, AppError>,
) -> Result<(StatusCode, Json<UserResponse>), AppError> {
    let user = state.user_service.create_user(&request.name).await;

    let user = match user {
        Ok(user) => user,
        Err(error) => match error {
            UserServiceError::NameAlreadyExists => {
                return Err(AppError::Conflict(error.to_string()));
            }
            _ => return Err(AppError::Internal(error.to_string())),
        },
    };

    Ok((StatusCode::CREATED, Json(UserResponse::from(user))))
}

pub async fn update(
    State(state): State<AppState>,
    WithRejection(Path(id), _): WithRejection<Path<Uuid>, AppError>,
    WithRejection(Json(request), _): WithRejection<Json<UpdateUserRequest>, AppError>,
) -> Result<(StatusCode, Json<UserResponse>), AppError> {
    let name = request.name;
    let is_active = request.is_active;
    let user = state.user_service.update_user(id, name, is_active).await;

    let user = match user {
        Ok(user) => user,
        Err(error) => match error {
            UserServiceError::UserNotFound => return Err(AppError::NotFound(error.to_string())),
            _ => return Err(AppError::Internal(error.to_string())),
        },
    };

    Ok((StatusCode::OK, Json(UserResponse::from(user))))
}
