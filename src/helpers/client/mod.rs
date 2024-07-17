use std::env::var;

use anyhow::Error;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use reqwest::header::AUTHORIZATION;
use reqwest::Client;

/// Creates a new `reqwest::Client` with default headers.
///
/// This function fetches the `GITHUB_TOKEN` environment variable and uses it to
/// set the `Authorization` header for the client.
///
/// # Returns
///
/// This function returns a `Result` that contains a `reqwest::Client` if the
/// client was successfully created, or an `Error` if the client could not be
/// created.
///
/// # Example
///
/// ```rust
/// let client = create_reqwest_client(); 
/// ```
///
/// # Errors
///
/// This function will return an error if the `reqwest::Client` could not be
/// built.
pub fn create_reqwest_client() -> Result<Client, Error> {
    let token = format!("Bearer {}", var("GITHUB_TOKEN").unwrap_or_default());
    let mut headers = HeaderMap::new();
    if let Some(token) = token.strip_prefix("Bearer ") {
        headers.insert(AUTHORIZATION, HeaderValue::from_str(token)?);
    }

    Ok(Client::builder().default_headers(headers).build()?)
}
