use std::sync::{Arc, RwLock};

use attohttpc::Response;
use eyre::Result;

#[derive(Clone)]
pub struct LCUClient {
    inner: Arc<RwLock<LCUClientInner>>,
}

struct LCUClientInner {
    base: String,
    token: String,
    can_send: bool,
    stealth_mode: bool,
}

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 6.2; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) \
                          LeagueOfLegendsClient/11.10.374.9538 (CEF 74) Safari/537.36";

impl LCUClient {
    pub fn new(token: &str, port: i32) -> Result<Self> {
        Ok(Self {
            inner: Arc::new(RwLock::new(LCUClientInner {
                base: format!("https://127.0.0.1:{}", port),
                token: format!("Basic {}", token),
                can_send: true,
                stealth_mode: false,
            })),
        })
    }

    pub fn send(&self, endpoint: &Endpoints, method: &Method, payload: &str) -> Result<Response> {
        let lcu_inner = self.inner.read().unwrap();

        match method {
            Method::GET => Ok(attohttpc::get(&format!("{}{}", lcu_inner.base, endpoint.as_endpoint()))
                .header("Authorization", &lcu_inner.token)
                .header_append("User-Agent", USER_AGENT)
                .danger_accept_invalid_certs(true)
                .send()?),
            Method::POST => Ok(attohttpc::post(&format!("{}{}", lcu_inner.base, endpoint.as_endpoint()))
                .header("Authorization", &lcu_inner.token)
                .header_append("Content-Type", "application/json")
                .header_append("User-Agent", USER_AGENT)
                .danger_accept_invalid_certs(true)
                .text(payload)
                .send()?),
        }
    }

    pub fn crash_lobby(&self) -> Result<()> {
        let is_lobby = self.send(&Endpoints::ChampSelect, &Method::GET, "")?.is_success();

        if !is_lobby {
            return Ok(());
        }

        let _cancel_lobby_response = self.send(&Endpoints::CancelLobby, &Method::POST, "{}").unwrap();

        let _quick_search_response = self
            .send(&Endpoints::QuickSearch, &Method::POST, r#"{"queueId": 1110}"#)
            .unwrap();

        Ok(())
    }

    pub fn enable_stealth(&self) {}

    pub fn reconnect(&mut self, token: &str, port: i32) {
        let mut lcu_inner = self.inner.write().unwrap();
        lcu_inner.base = format!("https://127.0.0.1:{}", port);
        lcu_inner.token = format!("Basic {}", token);
        lcu_inner.can_send = true;
    }

    pub fn disconnect(&mut self) {
        let mut lcu_inner = self.inner.write().unwrap();

        lcu_inner.can_send = false;
    }
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
