use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct ApiClient {
    pub base_url: String,
    client:       Client,
}

// ── Response types ────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct StatusResponse {
    pub ollama: bool,
}

#[derive(Serialize)]
pub struct RunRequest {
    pub goal:  String,
    pub crew:  String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    pub chain: bool,
    pub hitl:  bool,
}

#[derive(Deserialize)]
pub struct StartRunResponse {
    pub run_id: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct RunStatus {
    pub run_id: String,
    pub status: String,
    pub output: Vec<String>,
    pub files:  Vec<String>,
}

#[derive(Deserialize)]
pub struct OutputFile {
    pub path: String,
    pub size: u64,
}

#[derive(Deserialize)]
pub struct OutputsResponse {
    pub files: Vec<OutputFile>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct CancelResponse {
    pub run_id: String,
    pub status: String,
}

// ── Client impl ───────────────────────────────────────────────────────────────

impl ApiClient {
    pub fn new(base_url: String) -> Self {
        Self { base_url, client: Client::new() }
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    fn conn_err(&self, e: reqwest::Error) -> String {
        if e.is_connect() || e.is_timeout() {
            format!(
                "cannot reach Smith_Agentic at {}\n  Is it running? Try: cd Smith_Agentic && python ui/server.py",
                self.base_url
            )
        } else {
            format!("request failed: {}", e)
        }
    }

    async fn check(resp: reqwest::Response) -> Result<reqwest::Response, String> {
        let status = resp.status();
        if status.is_success() {
            return Ok(resp);
        }
        let body: serde_json::Value = resp.json().await.unwrap_or(serde_json::Value::Null);
        let msg = body.get("error").and_then(|v| v.as_str())
            .or_else(|| body.get("detail").and_then(|v| v.as_str()))
            .unwrap_or("unknown error")
            .to_string();
        Err(format!("HTTP {}: {}", status.as_u16(), msg))
    }

    pub async fn status(&self) -> Result<StatusResponse, String> {
        let resp = self.client.get(self.url("/api/status"))
            .send().await.map_err(|e| self.conn_err(e))?;
        resp.json().await.map_err(|e| format!("parse error: {}", e))
    }

    pub async fn crew_defaults(&self) -> Result<HashMap<String, String>, String> {
        let resp = self.client.get(self.url("/api/crew-defaults"))
            .send().await.map_err(|e| self.conn_err(e))?;
        resp.json().await.map_err(|e| format!("parse error: {}", e))
    }

    pub async fn start_run(&self, req: &RunRequest) -> Result<StartRunResponse, String> {
        let resp = self.client.post(self.url("/api/run"))
            .json(req).send().await.map_err(|e| self.conn_err(e))?;
        Self::check(resp).await?
            .json().await.map_err(|e| format!("parse error: {}", e))
    }

    pub async fn get_run(&self, run_id: &str) -> Result<RunStatus, String> {
        let resp = self.client.get(self.url(&format!("/api/run/{}", run_id)))
            .send().await.map_err(|e| self.conn_err(e))?;
        Self::check(resp).await?
            .json().await.map_err(|e| format!("parse error: {}", e))
    }

    pub async fn cancel_run(&self, run_id: &str) -> Result<CancelResponse, String> {
        let resp = self.client.post(self.url(&format!("/api/run/{}/cancel", run_id)))
            .send().await.map_err(|e| self.conn_err(e))?;
        Self::check(resp).await?
            .json().await.map_err(|e| format!("parse error: {}", e))
    }

    pub async fn list_outputs(&self) -> Result<OutputsResponse, String> {
        let resp = self.client.get(self.url("/api/outputs"))
            .send().await.map_err(|e| self.conn_err(e))?;
        resp.json().await.map_err(|e| format!("parse error: {}", e))
    }
}
