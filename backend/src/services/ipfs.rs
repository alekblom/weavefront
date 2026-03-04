use anyhow::{Context, Result};
use reqwest::Client;
use serde::Deserialize;

#[derive(Clone)]
pub struct IpfsService {
    client: Client,
    api_url: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct IpfsAddResponse {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Hash")]
    hash: String,
    #[serde(rename = "Size")]
    size: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct IpfsIdResponse {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "AgentVersion")]
    agent_version: Option<String>,
}

impl IpfsService {
    pub fn new(api_url: String) -> Self {
        Self {
            client: Client::new(),
            api_url,
        }
    }

    /// Check if the IPFS node is reachable
    pub async fn health_check(&self) -> Result<bool> {
        let resp = self
            .client
            .post(format!("{}/api/v0/id", self.api_url))
            .send()
            .await
            .context("Failed to reach IPFS node")?;
        Ok(resp.status().is_success())
    }

    /// Upload bytes to IPFS, returns the CID hash
    pub async fn upload(&self, filename: &str, data: Vec<u8>) -> Result<String> {
        let form = reqwest::multipart::Form::new().part(
            "file",
            reqwest::multipart::Part::bytes(data).file_name(filename.to_string()),
        );

        let resp: IpfsAddResponse = self
            .client
            .post(format!("{}/api/v0/add", self.api_url))
            .multipart(form)
            .send()
            .await
            .context("Failed to upload to IPFS")?
            .json()
            .await
            .context("Failed to parse IPFS response")?;

        Ok(resp.hash)
    }

    /// Pin a CID so it persists on the node
    pub async fn pin(&self, cid: &str) -> Result<()> {
        self.client
            .post(format!("{}/api/v0/pin/add?arg={}", self.api_url, cid))
            .send()
            .await
            .context("Failed to pin CID")?;
        Ok(())
    }

    pub fn gateway_url(&self, cid: &str) -> String {
        format!("https://ipfs.io/ipfs/{}", cid)
    }
}
