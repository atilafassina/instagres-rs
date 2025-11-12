use instagres::InstagresClient;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct RequestBody {
    message: String,
    data: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponseBody {
    id: String,
    status: String,
    connection_string: String,
    claim_url: String,
    created_at: String,
    updated_at: String,
    expires_at: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = InstagresClient::new();

    // Prepare the request body
    let request = RequestBody {
        message: "Hello from SDK".to_string(),
        data: Some(serde_json::json!({"key": "value"})),
    };

    // Make POST request and get typed response
    let response: ResponseBody = client.post("instagres-rust-sdk", &request).await?;

    println!("Response: {:?}", response);
    println!("\nConnection String: {}", response.connection_string);
    println!("Claim URL: {}", response.claim_url);
    println!("Status: {}", response.status);
    println!("Expires at: {}", response.expires_at);

    Ok(())
}
