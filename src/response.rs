use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use utoipa::ToSchema;

/// The structure of the body of a response from the api.
/// 
/// # Examples
///
/// ```
/// use crate response::ResponseBody;
///
/// let response = ResponseBody::ok("Data fetched", vec![]);
/// 
/// assert_eq!(response.code, StatusCode::OK);
/// ```
#[derive(Debug, Serialize, ToSchema)]
pub struct ResponseBody<T> {
    pub status: &'static str,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(ignore)]
    pub data: Option<T>,
    #[serde(skip)]
    pub code: StatusCode,
}

/// Type alias for [`ResponseBody`] whose type generic is set to an empty value, because an error
/// response contains no data.
pub type ErrorResponse = ResponseBody<()>;

impl<T> ResponseBody<T> {
    /// Creates a new response with a 200 status code
    pub fn ok(message: &str, data: T) -> Self {
        ResponseBody {
            status: "success",
            message: message.to_string(),
            data: Some(data),
            code: StatusCode::OK,
        }
    }

    /// Creates a new response with a 201 status code
    pub fn created(message: &str, data: T) -> Self {
        ResponseBody {
            status: "success",
            message: message.to_string(),
            data: Some(data),
            code: StatusCode::CREATED,
        }
    }

    /// Creates a new response with a 500 status code
    pub fn internal_error(err: &str) -> Self {
        ResponseBody {
            status: "fail",
            message: err.to_string(),
            data: None,
            code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    /// Creates a new response with a 404 status code
    pub fn not_found_error(err: &str) -> Self {
        ResponseBody {
            status: "fail",
            message: err.to_string(),
            data: None,
            code: StatusCode::NOT_FOUND,
        }
    }

    /// Creates a new response with a 401 status code
    pub fn unauthorized_error(err: &str) -> Self {
        ResponseBody {
            status: "fail",
            message: err.to_string(),
            data: None,
            code: StatusCode::UNAUTHORIZED,
        }
    }
}

impl<T> IntoResponse for ResponseBody<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        (self.code, Json(self)).into_response()
    }
}
