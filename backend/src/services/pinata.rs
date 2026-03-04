use anyhow::{Context, Result};
use reqwest::Client;
use serde::Deserialize;

#[derive(Clone)]
pub struct PinataService {
    client: Client,
    jwt: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct PinResponse {
    #[serde(rename = "IpfsHash")]
    ipfs_hash: String,
    #[serde(rename = "PinSize")]
    pin_size: u64,
    #[serde(rename = "Timestamp")]
    timestamp: Option<String>,
}

impl PinataService {
    pub fn new(jwt: String) -> Self {
        Self {
            client: Client::new(),
            jwt,
        }
    }

    /// Upload bytes to Pinata IPFS, returns (cid, size_bytes)
    pub async fn pin_file(&self, filename: &str, data: Vec<u8>) -> Result<(String, u64)> {
        let size = data.len() as u64;
        let form = reqwest::multipart::Form::new().part(
            "file",
            reqwest::multipart::Part::bytes(data)
                .file_name(filename.to_string())
                .mime_str("application/octet-stream")?,
        );

        let resp = self
            .client
            .post("https://api.pinata.cloud/pinning/pinFileToIPFS")
            .bearer_auth(&self.jwt)
            .multipart(form)
            .send()
            .await
            .context("Failed to upload to Pinata")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("Pinata API error {}: {}", status, body);
        }

        let pin: PinResponse = resp.json().await.context("Failed to parse Pinata response")?;
        Ok((pin.ipfs_hash, size))
    }

    pub fn gateway_url(cid: &str) -> String {
        format!("https://gateway.pinata.cloud/ipfs/{}", cid)
    }
}
