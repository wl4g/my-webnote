// use axum::{
//   routing::get,
//   Router,
//   extract::{ State, Query, Json },
//   response::IntoResponse,
//   http::StatusCode,
// };
// use crate::context::state::AppState;
// use crate::models::documents::Document;
// use crate::models::documents::QueryDocumentRequest;
// use crate::handlers::documents::DocumentHandler;

// pub fn init() -> Router<AppState> {
//   Router::new().route("/sys/document/query", get(get_documents))
// }

// pub async fn get_documents(
//   State(state): State<AppState>,
//   Query(param): Query<QueryDocumentRequest>
// ) -> impl IntoResponse {
//   let handler = DocumentHandler::new(&state);
//   match handler.get_documents().await {
//     Ok(documents) => (StatusCode::OK, Json(documents)).into_response(),
//     Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
//   }
// }

// async fn create_document(
//   State(state): State<AppState>,
//   Json(document): Json<Document>
// ) -> impl IntoResponse {
//   let handler = DocumentHandler::new(&state);
//   match handler.create_document(document).await {
//     Ok(created_document) => (StatusCode::CREATED, Json(created_document)).into_response(),
//     Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
//   }
// }
