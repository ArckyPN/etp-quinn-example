mod utils;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use utils::server_config;

#[tokio::main]
async fn main() {
    let key = "../cert/localhost-key.pem";
    let crt = "../cert/localhost.pem";
    let config = server_config(key, crt);
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 7001);

    let endpoint = quinn::Endpoint::server(config, addr).unwrap();

    println!(
        "listening on: https://localhost:{}/",
        endpoint.local_addr().unwrap().port()
    );

    loop {
        let conn: quinn::Connecting = endpoint.accept().await.unwrap();
        let conn: quinn::Connection = conn.await.unwrap();
        let req = webtransport_quinn::accept(conn).await.unwrap();
        let session = req.ok().await.unwrap();

        tokio::spawn(async move { handle(session) });
    }
}

async fn handle(session: webtransport_quinn::Session) {
    loop {}
}
