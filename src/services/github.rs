use std::borrow::Cow;

use anyhow::anyhow;
use anyhow::Result;
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;

/// Represents an error response from the GitHub API.
///
/// This struct contains information about an error response from the GitHub
/// API, including the error message and the URL of the documentation related to
/// the error.
///
/// # Fields
///
/// * `message: String` - The error message from the GitHub API.
/// * `documentation_url: String` - The URL of the documentation related to the
///   error.
///
/// # Example
///
/// ```rust
/// let error_response = ErrorResponse {
///     message: "Not Found".to_string(),
///     documentation_url: "https://docs.github.com/rest".to_string(),
/// };
/// println!("The error message is {}", error_response.message);
/// println!(
///     "The documentation URL is {}",
///     error_response.documentation_url
/// );
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    pub message: String,
    pub documentation_url: String,
}

pub async fn api(client: &Client, url: Cow<'_, str>) -> Result<String> {
    let response = client
        .get(url.as_ref())
        .header(reqwest::header::USER_AGENT, "hyper-jump")
        .header(reqwest::header::ACCEPT, "application/vnd.github.v3+json")
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    Ok(response)
}

/// Deserializes a JSON response from the GitHub API.
///
/// # Parameters
///
/// * `response: String` - The JSON response from the GitHub API as a string.
///
/// # Returns
///
/// * `Result<T>` - The deserialized response as the specified type `T`, or an
///   error if the response could not be deserialized or contains an error
///   message.
///
/// # Errors
///
/// This function will return an error if the response contains a "message"
/// field (indicating an error from the GitHub API), or if the response could
/// not be deserialized into the specified type `T`.
///
/// # Example
///
/// ```rust
/// let response = "{\"data\": \"some data\"}";
/// let result: Result<MyType> = deserialize_response(response);
/// match result {
///     Ok(data) => println!("Received data: {:?}", data),
///     Err(e) => println!("An error occurred: {:?}", e),
/// }
/// ```
pub fn deserialize_response<T: DeserializeOwned>(response: String) -> Result<T> {
    let value: serde_json::Value = serde_json::from_str(&response)?;
    if value.get("message").is_some() {
        let result: ErrorResponse = serde_json::from_value(value)?;
        if result.documentation_url.contains("rate-limiting") {
            return Err(anyhow!("Rate limited by GitHub API"));
        }

        return Err(anyhow!(result.message));
    }

    Ok(serde_json::from_value(value)?)
}
