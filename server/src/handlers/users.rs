use anyhow::Error;
use crate::context::state::AppState;
use crate::models::users::User;

// use crate::models::users::QueryUserRequest;
// use std::error::Error;

// // async fn query(param: QueryUserRequest) -> Result<Vec<User>, _> {
// //   let user = User {
// //     base: None,
// //     name: param.name.unwrap_or_default(),
// //     email: param.email.unwrap_or_default(),
// //     password: None,
// //   };
// //   users_sqlite_store::select_users(user).await;
// // }

// use crate::db::users_sqlite_store;
// use crate::models::users::QueryUserRequest;

// use crate::models::users::User;
// use crate::db::RepositoryContainer;
// use crate::config::config::ApiConfig;
// use std::error::Error;

// pub struct UserHandler {
//   repo_container: RepositoryContainer<User>,
//   config: ApiConfig,
// }

// impl UserHandler {
//   pub fn new(repo_container: RepositoryContainer<User>, config: ApiConfig) -> Self {
//     UserHandler { repo_container, config }
//   }

//   pub async fn get_users(&mut self) -> Result<Vec<User>, Error> {
//     self.repo_container.repo(&self.config).select()
//   }

//   pub async fn get_user_by_id(&mut self, id: i32) -> Result<User, Error> {
//     self.repo_container.repo(&self.config).select_by_id(id)
//   }

//   pub async fn create_user(&mut self, user: User) -> Result<User, Error> {
//     self.repo_container.repo(&self.config).insert(user)
//   }

//   pub async fn update_user(&mut self, user: User) -> Result<User, Error> {
//     self.repo_container.repo(&self.config).update(user)
//   }

//   pub async fn delete_user(&mut self, id: i32) -> Result<i32, Error> {
//     self.repo_container.repo(&self.config).delete_by_id(id)
//   }
// }

// use std::sync::Arc;
// use axum::{ extract::{ State, Json }, http::StatusCode, response::IntoResponse };
// use axum::extract::Path;
// use crate::context::state::AppState;
// use crate::models::users::User;

// pub async fn get_users(State(state): State<Arc<AppState>>) -> impl IntoResponse {
//   let mut repo = state.user_repo.lock().await;
//   match repo.repo(&state.config).select() {
//     Ok(users) => (StatusCode::OK, Json(users)).into_response(),
//     Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
//   }
// }

// pub async fn get_user_by_id(
//   State(state): State<Arc<AppState>>,
//   Path(id): Path<i32>
// ) -> impl IntoResponse {
//   let mut repo = state.user_repo.lock().await;
//   match repo.repo(&state.config).select_by_id(id) {
//     Ok(user) => (StatusCode::OK, Json(user)).into_response(),
//     Err(_) => StatusCode::NOT_FOUND.into_response(),
//   }
// }

// pub async fn create_user(
//   State(state): State<Arc<AppState>>,
//   Json(user): Json<User>
// ) -> impl IntoResponse {
//   let mut repo = state.user_repo.lock().await;
//   match repo.repo(&state.config).insert(user) {
//     Ok(created_user) => (StatusCode::CREATED, Json(created_user)).into_response(),
//     Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
//   }
// }

// pub async fn update_user(
//   State(state): State<Arc<AppState>>,
//   Json(user): Json<User>
// ) -> impl IntoResponse {
//   let mut repo = state.user_repo.lock().await;
//   match repo.repo(&state.config).update(user) {
//     Ok(updated_user) => (StatusCode::OK, Json(updated_user)).into_response(),
//     Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
//   }
// }

// pub async fn delete_user(
//   State(state): State<Arc<AppState>>,
//   Path(id): Path<i32>
// ) -> impl IntoResponse {
//   let mut repo = state.user_repo.lock().await;
//   match repo.repo(&state.config).delete_by_id(id) {
//     Ok(_) => StatusCode::NO_CONTENT.into_response(),
//     Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
//   }
// }

pub struct UserHandler<'a> {
  state: &'a AppState,
}

impl<'a> UserHandler<'a> {
  pub fn new(state: &'a AppState) -> Self {
    Self { state }
  }

  pub async fn get_users(&self) -> Result<Vec<User>, Error> {
    let mut repo = self.state.user_repo.lock().await;
    repo.repo(&self.state.config).select_all()
  }

  pub async fn create_user(&self, user: User) -> Result<User, Error> {
    let mut repo = self.state.user_repo.lock().await;
    repo.repo(&self.state.config).insert(user)
  }

  // More functions ...
}
