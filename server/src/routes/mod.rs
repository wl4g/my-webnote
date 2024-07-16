use axum::{ async_trait, extract::Query, Json };
use axum::extract::rejection::{ JsonRejection, QueryRejection };
use axum::response::{ IntoResponse, Response };
use axum::extract::{ FromRequest, Request };
use serde::de::DeserializeOwned;
use hyper::StatusCode;
use validator::Validate;

pub mod auths;
pub mod documents;
pub mod folders;
pub mod settings;
pub mod users;

pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S>
    for ValidatedJson<T>
    where
        T: DeserializeOwned + Validate,
        S: Send + Sync,
        Json<T>: FromRequest<S, Rejection = JsonRejection>
{
    type Rejection = Response;

    async fn from_request(
        req: Request<axum::body::Body>,
        state: &S
    ) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>
            ::from_request(req, state).await
            .map_err(|e|
                (StatusCode::BAD_REQUEST, format!("Json parsing error: {}", e)).into_response()
            )?;

        value
            .validate()
            .map_err(|e| {
                (StatusCode::BAD_REQUEST, format!("Validation error: {:?}", e)).into_response()
            })?;

        Ok(ValidatedJson(value))
    }
}

pub struct ValidatedQuery<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S>
    for ValidatedQuery<T>
    where
        T: DeserializeOwned + Validate,
        S: Send + Sync,
        Query<T>: FromRequest<S, Rejection = QueryRejection>
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Query(value) = Query::<T>
            ::from_request(req, state).await
            .map_err(|e|
                (StatusCode::BAD_REQUEST, format!("Query parsing error: {:?}", e)).into_response()
            )?;

        value
            .validate()
            .map_err(|e| {
                (StatusCode::BAD_REQUEST, format!("Validation error: {:?}", e)).into_response()
            })?;

        Ok(ValidatedQuery(value))
    }
}
