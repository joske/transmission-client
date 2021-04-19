use isahc::{prelude::*, HttpClient, Request};
use serde::de::DeserializeOwned;
use url::Url;

use std::cell::RefCell;

use crate::error::ClientError;
use crate::rpc::{
    RequestArgs, RpcRequest, RpcResponse, RpcResponseArguments, TorrentActionArgs, TorrentGetArgs,
};
use crate::utils;
use crate::{Session, SessionStats, Torrent, Torrents};

#[derive(Debug, Clone)]
pub struct Client {
    address: Url,
    http_client: HttpClient,
    session_id: RefCell<String>,
}

impl Client {
    pub async fn torrents(&self, ids: Option<Vec<i64>>) -> Result<Vec<Torrent>, ClientError> {
        let mut args = TorrentGetArgs::default();
        args.fields = utils::torrent_fields();
        args.ids = ids;
        let request_args = Some(RequestArgs::TorrentGetArgs(args));

        let response: RpcResponse<Torrents> =
            self.send_request("torrent-get", request_args).await?;
        Ok(response.arguments.unwrap().torrents)
    }

    pub async fn torrent_start(
        &self,
        ids: Option<Vec<i64>>,
        bypass_queue: bool,
    ) -> Result<(), ClientError> {
        let mut args = TorrentActionArgs::default();
        args.ids = ids;
        let request_args = Some(RequestArgs::TorrentActionArgs(args));

        let method_name = if bypass_queue {
            "torrent-start-now"
        } else {
            "torrent-start"
        };

        let _: RpcResponse<String> = self.send_request(method_name, request_args).await?;
        Ok(())
    }

    pub async fn torrent_stop(&self, ids: Option<Vec<i64>>) -> Result<(), ClientError> {
        let mut args = TorrentActionArgs::default();
        args.ids = ids;
        let request_args = Some(RequestArgs::TorrentActionArgs(args));

        let _: RpcResponse<String> = self.send_request("torrent-stop", request_args).await?;
        Ok(())
    }

    pub async fn torrent_verify(&self, ids: Option<Vec<i64>>) -> Result<(), ClientError> {
        let mut args = TorrentActionArgs::default();
        args.ids = ids;
        let request_args = Some(RequestArgs::TorrentActionArgs(args));

        let _: RpcResponse<String> = self.send_request("torrent-verify", request_args).await?;
        Ok(())
    }

    pub async fn torrent_reannounce(&self, ids: Option<Vec<i64>>) -> Result<(), ClientError> {
        let mut args = TorrentActionArgs::default();
        args.ids = ids;
        let request_args = Some(RequestArgs::TorrentActionArgs(args));

        let _: RpcResponse<String> = self
            .send_request("torrent-reannounce", request_args)
            .await?;
        Ok(())
    }

    pub async fn session(&self) -> Result<Session, ClientError> {
        let response: RpcResponse<Session> = self.send_request("session-get", None).await?;
        Ok(response.arguments.unwrap())
    }

    pub async fn session_stats(&self) -> Result<SessionStats, ClientError> {
        let response: RpcResponse<SessionStats> = self.send_request("session-stats", None).await?;
        Ok(response.arguments.unwrap())
    }

    async fn send_request<T: RpcResponseArguments + DeserializeOwned>(
        &self,
        method: &str,
        arguments: Option<RequestArgs>,
    ) -> Result<RpcResponse<T>, ClientError> {
        let request = RpcRequest {
            method: method.into(),
            arguments,
        };

        let body = serde_json::to_string(&request)?;
        let result = self.send_post(body).await?;
        let response: RpcResponse<T> = serde_json::from_str(&result)?;

        if response.result != "success" {
            return Err(ClientError::TransmissionError(response.result));
        }

        Ok(response)
    }

    async fn send_post(&self, body: String) -> Result<String, ClientError> {
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

    fn http_request(&self, body: String) -> Result<Request<String>, ClientError> {
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
