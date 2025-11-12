# Instagres

A Rust SDK for instant PostgreSQL instance provisioning via [Neon](https://neon.com).

> [!IMPORTANT]
> This crate is under heavy development

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
instagres = "0.1.0"
```

## Usage

### Basic Example

```rust
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

    let request = RequestBody {
        message: "Hello from SDK".to_string(),
        data: Some(serde_json::json!({"key": "value"})),
    };

    // The referrer parameter is required
    let response: ResponseBody = client.post("my-app-name", &request).await?;

    println!("Connection String: {}", response.connection_string);
    println!("Claim URL: {}", response.claim_url);

    Ok(())
}
```

### API Methods

#### `post<T>(referrer: impl AsRef<str>, body: impl Serialize) -> Result<T>`

Make a POST request and return a typed JSON response.

**Parameters:**

- `referrer` (required): Your application identifier. Cannot be empty.
- `body`: The request body to send as JSON.

**Returns:** Deserialized response of type `T`.

**Throws:** Error if referrer is empty or whitespace.

#### `post_with_headers<T>(referrer: impl AsRef<str>, body: impl Serialize, headers: HashMap<String, String>) -> Result<T>`

Make a POST request with custom headers and return a typed JSON response.

**Parameters:**

- `referrer` (required): Your application identifier. Cannot be empty.
- `body`: The request body to send as JSON.
- `headers`: Custom HTTP headers to include.

**Returns:** Deserialized response of type `T`.

#### `post_raw(referrer: impl AsRef<str>, body: impl Serialize) -> Result<String>`

Make a POST request and return the raw JSON string response.

**Parameters:**

- `referrer` (required): Your application identifier. Cannot be empty.
- `body`: The request body to send as JSON.

**Returns:** Raw JSON string.

### Response Structure

The Neon API returns the following structure:

```rust
{
    "id": "019a7939-50b0-7b41-9e55-cb93f4d2c480",
    "status": "UNCLAIMED",
    "connection_string": "postgresql://neondb_owner:npg_xxx@ep-xxx.aws.neon.tech/neondb?sslmode=require",
    "claim_url": "https://neon.new/claim/019a7939-50b0-7b41-9e55-cb93f4d2c480",
    "created_at": "2025-11-12T17:59:51Z",
    "updated_at": "2025-11-12T17:59:51Z",
    "expires_at": "2025-11-15T17:59:51Z"
}
```

### Error Handling

The client will return an error in the following cases:

- **Empty referrer**: If the referrer parameter is empty or contains only whitespace, an error will be returned with the message: `"referrer is required and cannot be empty"`
- **Network errors**: Any network-related issues during the request
- **Deserialization errors**: If the response cannot be parsed into the expected type

### Custom HTTP Client

You can provide your own configured `reqwest::Client`:

```rust
use reqwest::Client;
use instagres::InstagresClient;

let custom_client = Client::builder()
    .timeout(std::time::Duration::from_secs(30))
    .build()?;

let client = InstagresClient::with_client(custom_client);
```
