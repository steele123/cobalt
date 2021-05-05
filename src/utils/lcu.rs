use eyre::Result;
use ureq::Response;
use winrt_notification::{Duration as WinDur, Toast};

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
            Method::GET => Ok(ureq::get(&format!("{}{}", self.base, endpoint.as_endpoint()))
                .set("Authorization", &self.token)
                .call()?),
            Method::POST => Ok(ureq::post(&format!("{}{}", self.base, endpoint.as_endpoint()))
                .set("Authorization", &self.token)
                .send_string(payload)?),
        }
    }

    pub fn crash_lobby(&self) -> Result<()> {
        let _cancel_lobby_response = self.send(&Endpoints::CancelLobby, &Method::POST, "{}")?;

        let _quick_search_response = self.send(&Endpoints::QuickSeach, &Method::POST, r#"{"queueId": 1110}"#)?;

        Toast::new(Toast::POWERSHELL_APP_ID)
            .title("kataRUST")
            .text1("Lobby has been dodged, you max exit the TFT game after about 45 seconds.")
            .duration(WinDur::Short)
            .show()?;

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
