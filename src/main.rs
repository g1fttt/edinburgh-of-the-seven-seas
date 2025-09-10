mod backend;

use backend::http_server::Server;

use std::io;

fn main() -> io::Result<()> {
  Server::bind("localhost:7777").and_then(|mut server| server.handle_conns())
}
