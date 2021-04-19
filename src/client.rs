use isahc::{prelude::*, Error, HttpClient, Request};
use url::Url;

use std::cell::RefCell;

use crate::rpc::{
    DefaultResponseArgs, RequestArgs, RpcRequest, RpcResponse, TorrentActionArgs, TorrentGetArgs,
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
    pub async fn torrents(&self, ids: Option<Vec<i64>>) -> Result<Vec<Torrent>, Error> {
        let mut args = TorrentGetArgs::default();
        args.fields = utils::torrent_fields();
        args.ids = ids;
        let request_args = Some(RequestArgs::TorrentGetArgs(args));

        let result = self.send_request("torrent-get", request_args).await?;
        let response: RpcResponse<Torrents> = serde_json::from_str(&result).unwrap();
        Ok(response.arguments.torrents)
    }

    pub async fn start_torrent(
        &self,
        ids: Option<Vec<i64>>,
        bypass_queue: bool,
    ) -> Result<(), Error> {
        let mut args = TorrentActionArgs::default();
        args.ids = ids;
        let request_args = Some(RequestArgs::TorrentActionArgs(args));

        let method_name = if bypass_queue {
            "torrent-start-now"
        } else {
            "torrent-start"
        };

        let result = self.send_request(method_name, request_args).await?;
        let response: RpcResponse<DefaultResponseArgs> = serde_json::from_str(&result).unwrap();
        Ok(())
    }

    pub async fn session(&self) -> Result<Session, Error> {
        let result = self.send_request("session-get", None).await?;
        let response: RpcResponse<Session> = serde_json::from_str(&result).unwrap();
        Ok(response.arguments)
    }

    pub async fn session_stats(&self) -> Result<SessionStats, Error> {
        let result = self.send_request("session-stats", None).await?;
        let response: RpcResponse<SessionStats> = serde_json::from_str(&result).unwrap();
        Ok(response.arguments)
    }

    async fn send_request(
        &self,
        method: &str,
        arguments: Option<RequestArgs>,
    ) -> Result<String, Error> {
        let request = RpcRequest {
            method: method.into(),
            arguments,
        };

        let body = serde_json::to_string(&request).unwrap();
        println!("{}", &body);
        self.send_post(body).await
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
