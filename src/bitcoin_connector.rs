use crate::connection::Connection;
use bitcoin_hashes::{sha256, Hash};
use native_tls::{Certificate, TlsConnector as NativeTlsConnector};
use std::io::{Error, ErrorKind};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use tokio_native_tls::{TlsConnector, TlsStream};

use crate::constants::{CHAIN_MAGIC_BYTES,VERSION};
pub struct BitcoinConnection {
    pub ip_address: String,
    pub port: String,
    stream: Option<TcpStream>,
    tls_stream: Option<TlsStream<TcpStream>>,
    version: i32,
}

impl BitcoinConnection {
    pub fn init(ip_address: &str, port: &str) -> Self {
        Self {
            ip_address: ip_address.to_owned(),
            port: port.to_owned(),
            stream: None,
            tls_stream: None,
            version: VERSION,
        }
    }

    async fn create_tls_connector(&self) -> Result<TlsConnector, Box<dyn std::error::Error>> {
        let mut file = File::open("../cert.pem").await?;
        let mut cert = Vec::new();
        file.read_to_end(&mut cert).await?;
        let cert = Certificate::from_pem(&cert).unwrap();

        let connector = NativeTlsConnector::builder()
            .add_root_certificate(cert)
            .build()
            .unwrap();

        Ok(TlsConnector::from(connector))
    }
}

impl Connection for BitcoinConnection {
    async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        match self.version {
            1 => {
                let stream =
                    TcpStream::connect(format!("{}:{}", self.ip_address, self.port)).await?;
                self.stream = Some(stream);
            }
            _ => {
                let tls_connector = self.create_tls_connector().await?;

                let stream =
                    TcpStream::connect(format!("{}:{}", self.ip_address, self.port)).await?;
                let domain = "localhost"; // Replace with the actual domain name
                let tls_stream = tls_connector.connect(domain, stream).await?;

                self.tls_stream = Some(tls_stream);
            }
        }

        Ok(())
    }

    async fn read(&mut self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        if let Some(ref mut stream) = self.stream {
            let mut header = [0; 24];
            stream.read_exact(&mut header).await?;

            let magic = &header[0..4];
            if magic != CHAIN_MAGIC_BYTES {
                return Err(Box::new(Error::new(
                    ErrorKind::InvalidData,
                    "Invalid magic bytes",
                )));
            }

            let command = &header[4..16];
            let command = String::from_utf8_lossy(command)
                .trim_end_matches('\0')
                .to_string();

            let b0 = header[16] as u32;
            let b1 = header[17] as u32;
            let b2 = header[18] as u32;
            let b3 = header[19] as u32;

            let length = (b3 << 24) | (b2 << 16) | (b1 << 8) | b0;

            let checksum = &header[20..24];

            let mut payload = vec![0; length as usize];
            stream.read_exact(&mut payload).await?;

            let calculated_checksum =
                &sha256::Hash::hash(sha256::Hash::hash(&payload).as_byte_array())[..4];
            if calculated_checksum != checksum {
                return Err(Box::new(Error::new(
                    ErrorKind::InvalidData,
                    "Invalid checksum",
                )));
            }

            println!("Received message: command={}, length={}", command, length);

            return Ok(payload);
        }
        Err(Box::new(Error::new(
            ErrorKind::NotConnected,
            "No connection",
        )))
    }

    async fn write(&mut self, data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref mut stream) = self.stream {
            stream.write_all(&data).await?;
            return Ok(());
        }
        Err(Box::new(Error::new(
            ErrorKind::NotConnected,
            "No connection",
        )))
    }
}
