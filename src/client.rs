use std::cell::RefCell;
use std::rc::Rc;

use isahc::http::StatusCode;
use isahc::prelude::*;
use isahc::{HttpClient, Request};
use serde::de::DeserializeOwned;
use url::Url;

use crate::error::ClientError;
use crate::rpc::{
    RequestArgs, RpcRequest, RpcResponse, RpcResponseArguments, SessionSetArgs, TorrentActionArgs,
    TorrentAddArgs, TorrentGetArgs, TorrentRemoveArgs, TorrentSetArgs, TorrentSetLocationArgs,
};
use crate::{
    utils, Authentication, PortTest, Session, SessionMutator, SessionStats, Torrent, TorrentAdded,
    TorrentFiles, TorrentFilesList, TorrentList, TorrentMutator, TorrentPeers, TorrentPeersList,
    TorrentTrackers, TorrentTrackersList,
};

#[derive(Debug, Clone)]
pub struct Client {
    address: Url,
    authentication: Rc<RefCell<Option<Authentication>>>,
    http_client: HttpClient,
    session_id: Rc<RefCell<String>>,
}

impl Client {
    pub fn new(address: Url) -> Self {
        Client {
            address,
            ..Default::default()
        }
    }

    pub fn set_authentication(&self, auth: Option<Authentication>) {
        *self.authentication.borrow_mut() = auth;
    }

    pub async fn torrents(&self, ids: Option<Vec<i32>>) -> Result<Vec<Torrent>, ClientError> {
        let args = TorrentGetArgs {
            fields: utils::torrent_fields(),
            ids,
        };
        let request_args = Some(RequestArgs::TorrentGet(args));

        let response: RpcResponse<TorrentList> =
            self.send_request("torrent-get", request_args).await?;
        Ok(response.arguments.unwrap().torrents)
    }

    pub async fn torrents_files(
        &self,
        ids: Option<Vec<i32>>,
    ) -> Result<Vec<TorrentFiles>, ClientError> {
        let args = TorrentGetArgs {
            fields: utils::torrent_files_fields(),
            ids,
        };
        let request_args = Some(RequestArgs::TorrentGet(args));

        let response: RpcResponse<TorrentFilesList> =
            self.send_request("torrent-get", request_args).await?;
        Ok(response.arguments.unwrap().torrents)
    }

    pub async fn torrents_peers(
        &self,
        ids: Option<Vec<i32>>,
    ) -> Result<Vec<TorrentPeers>, ClientError> {
        let args = TorrentGetArgs {
            fields: utils::torrent_peers_fields(),
            ids,
        };
        let request_args = Some(RequestArgs::TorrentGet(args));

        let response: RpcResponse<TorrentPeersList> =
            self.send_request("torrent-get", request_args).await?;
        Ok(response.arguments.unwrap().torrents)
    }

    pub async fn torrents_trackers(
        &self,
        ids: Option<Vec<i32>>,
    ) -> Result<Vec<TorrentTrackers>, ClientError> {
        let args = TorrentGetArgs {
            fields: utils::torrent_trackers_fields(),
            ids,
        };
        let request_args = Some(RequestArgs::TorrentGet(args));

        let response: RpcResponse<TorrentTrackersList> =
            self.send_request("torrent-get", request_args).await?;
        Ok(response.arguments.unwrap().torrents)
    }

    pub async fn torrent_set(
        &self,
        ids: Option<Vec<String>>,
        mutator: TorrentMutator,
    ) -> Result<(), ClientError> {
        let args = TorrentSetArgs { ids, mutator };
        let request_args = Some(RequestArgs::TorrentSet(args));

        let _: RpcResponse<String> = self.send_request("torrent-set", request_args).await?;
        Ok(())
    }

    pub async fn torrent_add_filename(
        &self,
        filename: &str,
    ) -> Result<Option<Torrent>, ClientError> {
        let args = TorrentAddArgs {
            filename: Some(filename.into()),
            ..Default::default()
        };
        let request_args = Some(RequestArgs::TorrentAdd(args));
        self.torrent_add(request_args).await
    }

    pub async fn torrent_add_metainfo(
        &self,
        metainfo: &str,
    ) -> Result<Option<Torrent>, ClientError> {
        let args = TorrentAddArgs {
            metainfo: Some(metainfo.into()),
            ..Default::default()
        };
        let request_args = Some(RequestArgs::TorrentAdd(args));
        self.torrent_add(request_args).await
    }

    async fn torrent_add(
        &self,
        request_args: Option<RequestArgs>,
    ) -> Result<Option<Torrent>, ClientError> {
        let response: RpcResponse<TorrentAdded> =
            self.send_request("torrent-add", request_args).await?;

        let result_args = response.arguments.unwrap();
        if result_args.torrent_added.is_some() {
            Ok(result_args.torrent_added)
        } else {
            Ok(result_args.torrent_duplicate)
        }
    }

    pub async fn torrent_remove(
        &self,
        ids: Option<Vec<String>>,
        delete_local_data: bool,
    ) -> Result<(), ClientError> {
        let args = TorrentRemoveArgs {
            ids,
            delete_local_data,
        };
        let request_args = Some(RequestArgs::TorrentRemove(args));

        let _: RpcResponse<String> = self.send_request("torrent-remove", request_args).await?;
        Ok(())
    }

    pub async fn torrent_start(
        &self,
        ids: Option<Vec<String>>,
        bypass_queue: bool,
    ) -> Result<(), ClientError> {
        let args = TorrentActionArgs { ids };
        let request_args = Some(RequestArgs::TorrentAction(args));

        let method_name = if bypass_queue {
            "torrent-start-now"
        } else {
            "torrent-start"
        };

        let _: RpcResponse<String> = self.send_request(method_name, request_args).await?;
        Ok(())
    }

    pub async fn torrent_stop(&self, ids: Option<Vec<String>>) -> Result<(), ClientError> {
        self.send_torrent_action("torrent-stop", ids).await?;
        Ok(())
    }

    pub async fn torrent_verify(&self, ids: Option<Vec<String>>) -> Result<(), ClientError> {
        self.send_torrent_action("torrent-verify", ids).await?;
        Ok(())
    }

    pub async fn torrent_reannounce(&self, ids: Option<Vec<String>>) -> Result<(), ClientError> {
        self.send_torrent_action("torrent-reannounce", ids).await?;
        Ok(())
    }

    pub async fn torrent_set_location(
        &self,
        ids: Option<Vec<String>>,
        location: String,
        move_data: bool,
    ) -> Result<(), ClientError> {
        let args = TorrentSetLocationArgs {
            ids,
            location,
            move_data,
        };
        let request_args = Some(RequestArgs::TorrentSetLocation(args));

        let _: RpcResponse<String> = self
            .send_request("torrent-set-location", request_args)
            .await?;
        Ok(())
    }

    pub async fn queue_move_top(&self, ids: Option<Vec<String>>) -> Result<(), ClientError> {
        self.send_torrent_action("queue-move-top", ids).await?;
        Ok(())
    }

    pub async fn queue_move_up(&self, ids: Option<Vec<String>>) -> Result<(), ClientError> {
        self.send_torrent_action("queue-move-up", ids).await?;
        Ok(())
    }

    pub async fn queue_move_down(&self, ids: Option<Vec<String>>) -> Result<(), ClientError> {
        self.send_torrent_action("queue-move-down", ids).await?;
        Ok(())
    }

    pub async fn queue_move_bottom(&self, ids: Option<Vec<String>>) -> Result<(), ClientError> {
        self.send_torrent_action("queue-move-bottom", ids).await?;
        Ok(())
    }

    pub async fn session(&self) -> Result<Session, ClientError> {
        let response: RpcResponse<Session> = self.send_request("session-get", None).await?;
        Ok(response.arguments.unwrap())
    }

    pub async fn session_set(&self, mutator: SessionMutator) -> Result<(), ClientError> {
        let args = SessionSetArgs { mutator };
        let request_args = Some(RequestArgs::SessionSet(args));

        let _: RpcResponse<String> = self.send_request("session-set", request_args).await?;
        Ok(())
    }

    pub async fn session_stats(&self) -> Result<SessionStats, ClientError> {
        let response: RpcResponse<SessionStats> = self.send_request("session-stats", None).await?;
        Ok(response.arguments.unwrap())
    }

    pub async fn session_close(&self) -> Result<(), ClientError> {
        let _: RpcResponse<String> = self.send_request("session-close", None).await?;
        Ok(())
    }

    pub async fn port_test(&self) -> Result<bool, ClientError> {
        let response: RpcResponse<PortTest> = self.send_request("port-test", None).await?;
        Ok(response.arguments.unwrap().port_is_open)
    }

    async fn send_torrent_action(
        &self,
        action: &str,
        ids: Option<Vec<String>>,
    ) -> Result<(), ClientError> {
        let args = TorrentActionArgs { ids };
        let request_args = Some(RequestArgs::TorrentAction(args));

        let _: RpcResponse<String> = self.send_request(action, request_args).await?;
        Ok(())
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
        let post_result = self.send_post(body).await?;

        let de = &mut serde_json::Deserializer::from_str(&post_result);
        let serde_result: Result<RpcResponse<T>, _> = serde_path_to_error::deserialize(de);

        match serde_result {
            Ok(response) => {
                if response.result != "success" {
                    return Err(ClientError::TransmissionError(response.result));
                }

                Ok(response)
            }
            Err(err) => {
                let path = err.path().to_string();
                error!("Unable to parse json: {} ({})", path, err.to_string());
                warn!("Path: {path}");
                warn!("JSON: {post_result}");

                Err(err.into_inner().into())
            }
        }
    }

    async fn send_post(&self, body: String) -> Result<String, ClientError> {
        let request = self.http_request(body.clone())?;
        let mut response = self.http_client.send_async(request).await?;

        // Update session id
        let headers = response.headers();
        if let Some(session_id) = headers.get("X-Transmission-Session-Id") {
            let session_id = session_id.to_str().unwrap().to_string();
            *self.session_id.borrow_mut() = session_id;
        }

        // Check html status code
        match response.status() {
            // Invalid session id header, resend the request
            StatusCode::CONFLICT => {
                debug!("Received status code 409, resend request.");
                let request = self.http_request(body.clone())?;
                response = self.http_client.send_async(request).await?;
            }
            // Authentication needed
            StatusCode::UNAUTHORIZED => {
                return Err(ClientError::TransmissionUnauthorized);
            }
            _ => (),
        }

        Ok(response.text().await.unwrap())
    }

    fn http_request(&self, body: String) -> Result<Request<String>, ClientError> {
        let session_id = self.session_id.borrow().clone();

        let request = if let Some(auth) = &*self.authentication.borrow() {
            Request::post(self.address.to_string())
                .header("X-Transmission-Session-Id", session_id)
                .header("Authorization", auth.base64_encoded())
                .body(body)?
        } else {
            Request::post(self.address.to_string())
                .header("X-Transmission-Session-Id", session_id)
                .body(body)?
        };

        Ok(request)
    }
}

impl Default for Client {
    fn default() -> Self {
        let address = Url::parse("http://127.0.0.1:9091/transmission/rpc/").unwrap();
        let http_client = HttpClient::builder()
            .authentication(isahc::auth::Authentication::all())
            .build()
            .unwrap();
        let session_id = Rc::new(RefCell::new("0".into()));

        Self {
            address,
            authentication: Rc::default(),
            http_client,
            session_id,
        }
    }
}
