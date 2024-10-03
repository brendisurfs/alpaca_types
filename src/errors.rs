use http_serde::http::StatusCode;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
/// # ErrorMessage
/// defines the general error message received when an error has occured calling the API.
/// * `code`: the error code belonging to this error.
/// * `message`: describes what went wrong.
pub struct ErrorMessage {
    #[serde(with = "http_serde::status_code")]
    pub code: StatusCode,
    pub message: String,
}
