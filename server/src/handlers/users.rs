use anyhow::Error;
use crate::context::state::AppState;
use crate::types::users::{ DeleteUserRequest, QueryUserRequest, SaveUserRequest, User };

pub struct UserHandler<'a> {
  state: &'a AppState,
}

impl<'a> UserHandler<'a> {
  pub fn new(state: &'a AppState) -> Self {
    Self { state }
  }

  pub async fn find(&self, param: QueryUserRequest) -> Result<Vec<User>, Error> {
    let mut repo = self.state.user_repo.lock().await;
    repo.repo(&self.state.config).select_all().await
  }

  pub async fn save(&self, param: SaveUserRequest) -> Result<i64, Error> {
    let mut repo = self.state.user_repo.lock().await;
    if param.id.is_some() {
      repo.repo(&self.state.config).update(param.to_user()).await
    } else {
      repo.repo(&self.state.config).insert(param.to_user()).await
    }
  }

  pub async fn delete(&self, param: DeleteUserRequest) -> Result<u64, Error> {
    let mut repo = self.state.user_repo.lock().await;
    repo.repo(&self.state.config).delete_by_id(param.id).await
  }
}
