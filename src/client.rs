use isahc::{prelude::*, Error, HttpClient, Request};
use url::Url;

use std::cell::RefCell;

use crate::rpc::{RpcRequest, RpcResponse};
use crate::{Session, SessionStats};

#[derive(Debug, Clone)]
pub struct Client {
    address: Url,
    http_client: HttpClient,
    session_id: RefCell<String>,
}

impl Client {
    pub async fn session(&self) -> Result<Session, Error> {
        debug!("Send");

        let result = self.send_request("session-get").await?;
        let response: RpcResponse<Session> = serde_json::from_str(&result).unwrap();
        Ok(response.arguments)
    }

    pub async fn session_stats(&self) -> Result<SessionStats, Error> {
        debug!("Send");

        let result = self.send_request("session-stats").await?;
        let response: RpcResponse<SessionStats> = serde_json::from_str(&result).unwrap();
        Ok(response.arguments)
    }

    async fn send_request(&self, method: &str) -> Result<String, Error> {
        let request = RpcRequest {
            method: method.into(),
            ..Default::default()
        };

        self.send_post(serde_json::to_string(&request).unwrap())
            .await
    }

    async fn send_post(&self, body: String) -> Result<String, Error> {
        let request = self.http_request(body.clone())?;
        let mut response = self.http_client.send_async(request).await?;

        // Check for session id
        let headers = response.headers();
        if let Some(session_id) = headers.get("X-Transmission-Session-Id") {
            let session_id = session_id.to_str().unwrap().to_string();
            *self.session_id.borrow_mut() = session_id;

            if response.status().as_u16() == 409 {
                debug!("Received status code 409, resend request.");
                let request = self.http_request(body.clone())?;
                response = self.http_client.send_async(request).await?;
            }
        }

        Ok(response.text().await.unwrap())
    }

    fn http_request(&self, body: String) -> Result<Request<String>, Error> {
        let session_id = self.session_id.borrow().clone();
        let request = Request::post(self.address.to_string())
            .header("X-Transmission-Session-Id", session_id)
            .body(body.clone())?;
        Ok(request)
    }
}

impl Default for Client {
    fn default() -> Self {
        let address = Url::parse("http://127.0.0.1:9091/transmission/rpc/").unwrap();
        let http_client = HttpClient::new().unwrap();
        let session_id = RefCell::new("0".into());

        Self {
            address,
            http_client,
            session_id,
        }
    }
}
