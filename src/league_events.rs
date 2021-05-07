use tungstenite::{client::AutoStream, connect, http::Request, WebSocket};

pub struct EventHandler {
    socket: WebSocket<AutoStream>,
}

impl EventHandler {
    pub fn new(&self, port: i32, token: String) -> Self {
        let socket = self.create_socket(port, token);

        Self { socket }
    }

    fn create_socket(&self, port: i32, token: String) -> WebSocket<AutoStream> {
        let request = Request::builder()
            .uri(format!("wss://127.0.0.1:{}/", port))
            .header(
                "Authorization",
                format!("Basic {}", base64::encode(format!("riot:{}", token))),
            )
            .body(())
            .unwrap();

        let (socket, _response) = connect(request).expect("Can't connect");

        socket
    }

    pub fn set_settings(&self, port: i32, token: String) {
        let (mut socket, response) = connect(format!("wss://127.0.0.1:{}/", port)).unwrap();
    }

    pub fn read_messages(&self) {}
}
