use serde::{ Deserialize, Serialize };
use super::BaseBean;

#[derive(Serialize, Deserialize, Clone)]
pub struct Document {
  #[serde(flatten)]
  pub base: Option<BaseBean>,
  pub name: String,
  pub email: String,
  pub password: Option<String>,
}

#[derive(Deserialize)]
pub struct QueryDocumentRequest {
  pub name: Option<String>,
  pub email: Option<String>,
}

#[derive(Serialize)]
pub struct QueryDocumentResponse {
  users: Vec<Document>,
}

#[derive(Deserialize)]
pub struct SaveDocumentRequest {
  name: String,
  email: String,
  password: String,
}

#[derive(Serialize)]
pub struct SaveDocumentResponse {
  user: Document,
}

#[derive(Deserialize)]
pub struct DeleteDocumentRequest {
  id: String,
}

#[derive(Serialize)]
pub struct DeleteDocumentResponse {
  id: String,
}
