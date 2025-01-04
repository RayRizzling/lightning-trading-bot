use hmac::{Hmac, Mac};
use sha2::Sha256;
use base64::{engine::general_purpose, Engine};

/// Generates an HMAC-SHA256 signature for authentication.
///
/// This function creates a cryptographic signature to be used in API authentication, ensuring that the
/// request's integrity and authenticity are verified. The signature is calculated based on the provided
/// secret key, timestamp, HTTP method, request path, and body data. The resulting signature is encoded
/// in Base64 format.
///
/// # Parameters
/// - `secret`: The secret key used to generate the HMAC signature. It is shared between the client and
///   the server and should be kept confidential.
/// - `timestamp`: The current timestamp (in milliseconds) to ensure that the request is time-bound.
/// - `method`: The HTTP method of the request (e.g., "GET", "POST"). It must be uppercase.
/// - `path`: The endpoint path for the API request (e.g., "/v2/user").
/// - `data`: The request body data, which is usually an empty string for GET requests or JSON data for POST requests.
///
/// # Returns
/// - A Base64-encoded string representing the generated signature.
///
/// # Example
/// ```rust
/// use trading_backend::utils::get_signature::generate_signature;
/// let secret = "your_api_secret";
/// let timestamp = 1627632000000;
/// let method = "GET";
/// let path = "/v2/user";
/// let data = "";
/// let signature = generate_signature(secret, timestamp, method, path, Some(data));
/// println!("Generated signature: {}", signature);
/// assert!(!signature.is_empty());
/// ```
///
/// # Errors
/// This function will panic if the HMAC initialization fails (e.g., if the secret key is invalid).
///
/// # Security Note
/// - Ensure that the secret key is kept secure and never exposed in client-side code.
/// - Timestamps should be used to prevent replay attacks by ensuring that requests are time-bound.
pub fn generate_signature(secret: &str, timestamp: i64, method: &str, path: &str, data: Option<&str>) -> String {
    let data_str = match data {
        Some(d) => d.replace("\n", "").replace(" ", ""),
        None => "".to_string(),
    };
    // Construct the prehash string by concatenating the timestamp, HTTP method, path, and data.
    let prehash = format!("{}{}{}{}", timestamp, method.to_uppercase(), path, data_str);

    // Create a new HMAC instance using the secret key and the SHA-256 hashing algorithm.
    let mac_result = Hmac::<Sha256>::new_from_slice(secret.as_bytes())
        .map(|mut mac| {
            // Update the HMAC with the prehash string bytes.
            mac.update(prehash.as_bytes());
            // Finalize the HMAC computation and return the result.
            mac.finalize().into_bytes()
        })
        .map_err(|e| format!("Failed to initialize HMAC: {}", e))
        .expect("HMAC could not be initialized");

    // Return the result as a Base64-encoded string.
    general_purpose::STANDARD.encode(mac_result)
}
