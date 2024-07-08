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
