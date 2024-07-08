use anyhow::Error;
use crate::context::state::AppState;
use crate::models::users::User;

pub struct UserHandler<'a> {
  state: &'a AppState,
}

impl<'a> UserHandler<'a> {
  pub fn new(state: &'a AppState) -> Self {
    Self { state }
  }

  pub async fn find_all(&self) -> Result<Vec<User>, Error> {
    let mut repo = self.state.user_repo.lock().await;
    repo.repo(&self.state.config).select_all().await
  }

  pub async fn save(&self, user: User) -> Result<i64, Error> {
    let mut repo = self.state.user_repo.lock().await;
    repo.repo(&self.state.config).insert(user).await
  }
}
