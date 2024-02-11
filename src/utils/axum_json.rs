use axum::async_trait;
use axum::extract::rejection::JsonRejection;
use axum::extract::{FromRequest, Request};
use axum::response::{IntoResponse, Response};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct Json<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for Json<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = JsonRejection;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let bytes = bytes::Bytes::from_request(req, state).await?;
        Self::from_bytes(&bytes)
    }
}

impl<T> Json<T>
where
    T: DeserializeOwned,
{
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, JsonRejection> {
        axum::extract::Json::from_bytes(bytes).map(|it| Json(it.0))
    }
}

impl<T> IntoResponse for Json<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        axum::extract::Json::into_response(axum::extract::Json(self.0))
    }
}

impl<T> From<T> for Json<T> {
    fn from(inner: T) -> Self {
        Self(inner)
    }
}
