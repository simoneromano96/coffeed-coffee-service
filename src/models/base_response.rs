use async_graphql::{OutputValueType, Type};
use chrono::{DateTime, Utc};
use crate::graphql::coffee::Coffee;

#[async_graphql::SimpleObject]
pub struct BaseResponse {
    error: bool,
    // No timezone, all datetimes are with UTC time
    timestamp: DateTime<Utc>,
    message: Option<String>,
    data: Option<Coffee>,
}

impl BaseResponse
{
    pub fn with_success(data: Option<Coffee>) -> Self {
        Self {
            error: false,
            timestamp: Utc::now(),
            message: None,
            data,
        }
    }

    pub fn with_error(message: Option<String>) -> Self {
        Self {
            error: true,
            timestamp: Utc::now(),
            message,
            data: None,
        }
    }
}
