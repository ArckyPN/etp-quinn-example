use std::{
    fs::File,
    io::{BufReader, Cursor, Read},
    sync::Arc,
};

use rustls::{Certificate, PrivateKey, ServerConfig};

pub struct Tls {
    certs: Vec<Certificate>,
    key: PrivateKey,
}

impl Tls {
    pub fn new(key: &str, crt: &str) -> Self {
        let certs = File::open(crt).unwrap();
        let mut certs = BufReader::new(certs);
        let certs = rustls_pemfile::certs(&mut certs).unwrap();
        let certs: Vec<Certificate> = certs.into_iter().map(Certificate).collect();

        // read PEM private key
        let mut keys = File::open(key).unwrap();

        // Read the keys into a Vec so we can try parsing it twice
        let mut buf = Vec::new();
        keys.read_to_end(&mut buf).unwrap();

        // Try to parse a PKCS#8 key
        // -----BEGIN PRIVATE KEY-----
        let mut keys = rustls_pemfile::pkcs8_private_keys(&mut Cursor::new(&buf)).unwrap();

        // Try again but with EC keys this time
        // -----BEGIN EC PRIVATE KEY-----
        if keys.is_empty() {
            keys = rustls_pemfile::ec_private_keys(&mut Cursor::new(&buf)).unwrap();
        };

        if keys.is_empty() {
            panic!("TLS::new: could not find private key");
        }
        if keys.len() > 1 {
            panic!("TLS::new: expected a single key");
        }

        let key = PrivateKey(keys.remove(0));

        Self { certs, key }
    }
}

impl Tls {
    pub fn certs(&self) -> Vec<Certificate> {
        self.certs.clone()
    }
    pub fn key(&self) -> PrivateKey {
        self.key.clone()
    }
}

pub fn server_config(key: &str, crt: &str) -> quinn::ServerConfig {
    let tls = Tls::new(key, crt);

    let server_config = ServerConfig::builder()
        .with_safe_default_cipher_suites()
        .with_safe_default_kx_groups()
        .with_protocol_versions(&[&rustls::version::TLS13])
        .unwrap();
    let mut server_config = server_config
        .with_no_client_auth()
        .with_single_cert(tls.certs(), tls.key())
        .unwrap();
    server_config.max_early_data_size = u32::MAX;
    server_config.alpn_protocols = vec![webtransport_quinn::ALPN.to_vec()];

    let mut server_config = quinn::ServerConfig::with_crypto(Arc::new(server_config));

    let mut transport_config = quinn::TransportConfig::default();
    transport_config
        .congestion_controller_factory(Arc::new(quinn::congestion::BbrConfig::default()));
    transport_config.mtu_discovery_config(None); // disable MTU discovery
    let transport_config = Arc::new(transport_config);

    server_config.transport_config(transport_config);

    server_config
}
