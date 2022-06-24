use rocket::http::Status; 
use serde::Serialize;
use super::serializable_status;

#[derive(Serialize)]
pub struct Error<T> {
    #[serde(with = "serializable_status")]
    status: Status,
    content: Option<T>,
}

impl<T> Error<T> {
    pub fn new(status: Status) -> Self {
        Error {
            status,
            content: None
        }
    }

    pub fn new_with_content(status: Status, content: T) -> Self {
        Error {
            status,
            content: Some(content)
        }
    }
}