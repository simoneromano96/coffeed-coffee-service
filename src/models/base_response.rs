/* TODO
use async_graphql::{OutputValueType, Type};
use chrono::{DateTime, Utc};
use crate::graphql::coffee::Coffee;

pub struct BaseResponse<T: Type> {
    error: bool,
    // No timezone, all datetimes are with UTC time
    timestamp: DateTime<Utc>,
    message: Option<String>,
    data: Option<T>,
}

impl<T> BaseResponse<T>
where
    T: Type,
{
    pub fn new_success(message: Option<String>, data: Option<T>) -> Self {
        Self {
            error: false,
            timestamp: Utc::now(),
            message,
            data,
        }
    }

    pub fn error(message: Option<String>, data: Option<T>) -> Self {
        Self {
            error: true,
            timestamp: Utc::now(),
            message,
            data,
        }
    }
}
*/
