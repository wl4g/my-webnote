use std::rc::Rc;

use anyhow::{ Error, Ok };
use crate::context::state::AppState;
use crate::types::users::{ DeleteUserRequest, QueryUserRequest, SaveUserRequest, User };
use crate::types::{ PageRequest, PageResponse };

pub struct UserHandler<'a> {
  state: &'a AppState,
}

impl<'a> UserHandler<'a> {
  pub fn new(state: &'a AppState) -> Self {
    Self { state }
  }

  pub async fn get(
    &self,
    oidc_claims_sub: Option<String>,
    github_claims_sub: Option<String>,
    google_claims_sub: Option<String>
  ) -> Result<Option<Rc<User>>, Error> {
    let param = QueryUserRequest {
      page: PageRequest::default(),
      name: None,
      email: None,
      phone: None,
      oidc_claims_sub: oidc_claims_sub,
      github_claims_sub: github_claims_sub,
      google_claims_sub: google_claims_sub,
    };
    let res = self.find(param).await.unwrap().1;
    if res.len() > 0 {
      let user = Rc::new(res.get(0).unwrap().clone());
      return Ok(Some(user));
    } else {
      Ok(None)
    }
  }

  pub async fn find(&self, param: QueryUserRequest) -> Result<(PageResponse, Vec<User>), Error> {
    let repo = self.state.user_repo.lock().await;
    repo.repo(&self.state.config).select(param.to_user(), param.page).await
  }

  pub async fn save(&self, param: SaveUserRequest) -> Result<i64, Error> {
    let repo = self.state.user_repo.lock().await;
    if param.id.is_some() {
      repo.repo(&self.state.config).update(param.to_user()).await
    } else {
      repo.repo(&self.state.config).insert(param.to_user()).await
    }
  }

  pub async fn delete(&self, param: DeleteUserRequest) -> Result<u64, Error> {
    let repo = self.state.user_repo.lock().await;
    repo.repo(&self.state.config).delete_by_id(param.id).await
  }
}
