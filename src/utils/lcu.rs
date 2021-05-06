use attohttpc::Response;
use eyre::Result;

use crate::utils::toast;

pub struct LCUClient {
    base: String,
    token: String,
    pub can_send: bool,
}

impl LCUClient {
    pub fn new(token: &str, port: i32) -> Result<Self> {
        Ok(Self {
            base: format!("https://127.0.0.1:{}", port),
            token: format!("Basic {}", base64::encode(format!("riot:{}", token))),
            can_send: true,
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
        let is_lobby = self.send(&Endpoints::ChampSelect, &Method::GET, "")?.is_success();

        if !is_lobby {
            return Err(eyre::eyre!("Not in a lobby to crash..."));
        }

        let _cancel_lobby_response = self.send(&Endpoints::CancelLobby, &Method::POST, "{}").unwrap();

        let _quick_search_response = self
            .send(&Endpoints::QuickSearch, &Method::POST, r#"{"queueId": 1110}"#)
            .unwrap();

        toast::send("Lobby has been dodged, you can leave the TFT game ~45 seconsd.")?;
        Ok(())
    }

    pub fn reconnect(&mut self, token: &str, port: i32) {
        self.base = format!("https://127.0.0.1:{}", port);
        self.token = format!("Basic {}", base64::encode(format!("riot:{}", token)));
        self.can_send = true;
    }

    pub fn disconnect(&mut self) { self.can_send = false; }
}

pub enum Endpoints {
    CancelLobby,
    QuickSearch,
    AramBoost,
    ChampSelect,
}

pub enum Method {
    POST,
    GET,
}

impl Endpoints {
    pub const fn as_endpoint(&self) -> &'static str {
        match self {
            Endpoints::CancelLobby => "/lol-lobby/v1/lobby/custom/cancel-champ-select",
            Endpoints::QuickSearch => "/lol-lobby/v2/matchmaking/quick-search",
            Endpoints::AramBoost => {
                r#"/lol-login/v1/session/invoke?destination=lcdsServiceProxy&method=call&args=["","teambuilder-draft","activateBattleBoostV1",""]"#
            },
            Endpoints::ChampSelect => "/lol-champ-select/v1/session",
        }
    }
}
