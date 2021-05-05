use eyre::Result;
use reqwest::{Body, Method, Response};

pub struct LCUClient {
    base: String,
    client: reqwest::Client,
}

impl LCUClient {
    pub fn new(token: &str, port: i32) -> Result<Self> {
        let mut headmap = reqwest::header::HeaderMap::new();
        headmap.insert(
            "Authorization",
            format!("Basic {}", base64::encode(format!("riot:{}", token))).parse()?,
        );

        Ok(Self {
            base: format!("https://127.0.0.1:{}", port),
            client: reqwest::ClientBuilder::new()
                .default_headers(headmap)
                .danger_accept_invalid_certs(true)
                .build()?,
        })
    }

    pub async fn send(&self, endpoint: Endpoints, method: Method, payload: String) -> Result<Response> {
        match method {
            Method::GET => Ok(self
                .client
                .get(&format!("{}{}", self.base, endpoint.as_endpoint()))
                .body(payload)
                .send()
                .await?),
            Method::POST => Ok(self
                .client
                .post(&format!("{}{}", self.base, endpoint.as_endpoint()))
                .body(payload)
                .send()
                .await?),
            _ => Err(eyre::eyre!("what in the hell are u tryna do")),
        }
    }

    pub async fn crash_lobby(&self) -> Result<()> {
        let cancel_lobby_response: Response = lcu
            .send(Endpoints::CancelLobby, reqwest::Method::POST, "{}".into())
            .await
            .unwrap();

        let quick_search_response = lcu
            .send(Endpoints::QuickSeach, reqwest::Method::POST, r#"{"queueId": 1110}"#.into())
            .await
            .unwrap();

        Ok(())
    }
}

pub enum Endpoints {
    CancelLobby,
    QuickSeach,
    Example,
}

impl Endpoints {
    pub const fn as_endpoint(&self) -> &'static str {
        match self {
            Endpoints::CancelLobby => "/lol-lobby/v1/lobby/custom/cancel-champ-select",
            Endpoints::QuickSeach => "/lol-lobby/v2/matchmaking/quick-search",
            Endpoints::Example => "/riotclient/app-name",
        }
    }
}
