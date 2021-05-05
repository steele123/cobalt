use attohttpc::Response;
use eyre::Result;

pub struct LCUClient {
    base: String,
    token: String,
}

impl LCUClient {
    pub fn new(token: &str, port: i32) -> Result<Self> {
        Ok(Self {
            base: format!("https://127.0.0.1:{}", port),
            token: format!("Basic {}", base64::encode(format!("riot:{}", token))),
        })
    }

    pub fn send(&self, endpoint: &Endpoints, method: &Method, payload: &str) -> Result<Response> {
        match method {
            Method::GET => Ok(attohttpc::get(&format!("{}{}", self.base, endpoint.as_endpoint()))
                .header("Authorization", &self.token)
                .danger_accept_invalid_certs(true)
                .send()?),
            Method::POST => Ok(attohttpc::post(&format!("{}{}", self.base, endpoint.as_endpoint()))
                .header("Authorization", &self.token)
                .header_append("Content-Type", "application/json")
                .danger_accept_invalid_certs(true)
                .text(payload)
                .send()?),
        }
    }

    pub fn crash_lobby(&self) -> Result<()> {
        let _cancel_lobby_response = self.send(&Endpoints::CancelLobby, &Method::POST, "{}").unwrap();

        let _quick_search_response = self
            .send(&Endpoints::QuickSeach, &Method::POST, r#"{"queueId": 1110}"#)
            .unwrap();

        Ok(())
    }
}

pub enum Endpoints {
    CancelLobby,
    QuickSeach,
}

pub enum Method {
    POST,
    GET,
}

impl Endpoints {
    pub const fn as_endpoint(&self) -> &'static str {
        match self {
            Endpoints::CancelLobby => "/lol-lobby/v1/lobby/custom/cancel-champ-select",
            Endpoints::QuickSeach => "/lol-lobby/v2/matchmaking/quick-search",
        }
    }
}
