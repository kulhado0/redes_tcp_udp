use rocket::http::Status;
use serde::{Serialize, Serializer};

pub fn serialize<S>(val: &Status, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    val.to_string().serialize(serializer)
}
