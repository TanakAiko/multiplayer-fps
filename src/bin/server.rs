use multiplayer_fps::server::{udp::Server, utils::exeption::ServerError};

fn main() -> Result<(), ServerError> {
    Server::start_server()
}
