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
    // fetch env variable
    let github_token = std::env::var("GITHUB_TOKEN");

    let mut headers = HeaderMap::new();

    if let Ok(github_token) = github_token {
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", github_token)).unwrap(),
        );
    }

    let client = Client::builder().default_headers(headers).build()?;

    Ok(client)
}
