use tungstenite::{client::AutoStream, connect, http::Request, WebSocket};

pub struct EventHandler {
    socket: WebSocket<AutoStream>,
}

impl EventHandler {
    pub fn new(&self, port: i32, token: String) -> Self {
        self.set_settings(port, token);

        Self
    }

    fn create_socket(&self, port: i32, token: String) {
        let req = Request::builder()
            .uri(format!("wss://127.0.0.1:{}/", port))
            .header("Credentials");

        let (mut socket, response) = connect(req);

        socket.set_config()
    }

    pub fn set_settings(&self, port: i32, token: String) {
        let (mut socket, response) = connect(format!("wss://127.0.0.1:{}/", port)).unwrap();
    }

    pub fn read_messages(&self) {}
}
