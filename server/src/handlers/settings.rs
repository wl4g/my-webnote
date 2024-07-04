use anyhow::Error;
use crate::context::state::AppState;
use crate::models::settings::Settings;

pub struct SettingsHandler<'a> {
  state: &'a AppState,
}

impl<'a> SettingsHandler<'a> {
  pub fn new(state: &'a AppState) -> Self {
    Self { state }
  }

  pub async fn get_settings(&self) -> Result<Vec<Settings>, Error> {
    let mut repo = self.state.settings_repo.lock().await;
    repo.repo(&self.state.config).select_all()
  }

  pub async fn create_settings(&self, settings: Settings) -> Result<Settings, Error> {
    let mut repo = self.state.settings_repo.lock().await;
    repo.repo(&self.state.config).insert(settings)
  }

  // More functions ...
}
