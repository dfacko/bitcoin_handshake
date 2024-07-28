use crate::{
    connection::Connection,
    constants::{DEVNET_CHAIN_MAGIC_BYTES, REGTEST_CHAIN_MAGIC_BYTES},
    error::Error,
};
use bitcoin_hashes::{sha256, Hash};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

/// BitcoinConnection - Represents a connection to a Bitcoin network
///
/// module is used by Handshaker to connect to the target node and communicate with it.
/// This module's only responsibility is to establish a connection and send/recieve data.
/// What type of data and data processing should not matter in this module as those should be independent.
///
pub struct BitcoinConnection {
    pub ip_address: String,
    pub port: String,
    stream: Option<TcpStream>,
}

impl BitcoinConnection {
    /// Helper function to initialize the BitcoinConnection struct
    ///
    pub fn init(ip_address: &str, port: &str) -> Self {
        Self {
            ip_address: ip_address.to_owned(),
            port: port.to_owned(),
            stream: None,
        }
    }
}

impl Connection for BitcoinConnection {
    async fn connect(&mut self) -> Result<(), Error> {
        let stream = TcpStream::connect(format!("{}:{}", self.ip_address, self.port)).await?;

        self.stream = Some(stream);

        Ok(())
    }

    async fn read(&mut self) -> Result<Vec<u8>, Error> {
        if let Some(ref mut stream) = self.stream {
            let mut header = [0; 24];

            stream.read_exact(&mut header).await?;

            let magic = &header[0..4];
            if magic != REGTEST_CHAIN_MAGIC_BYTES && magic != DEVNET_CHAIN_MAGIC_BYTES {
                return Err(Error::Custom("invalid magic (bytes".to_owned()));
            }

            let command = &header[4..16];
            let command = String::from_utf8_lossy(command)
                .trim_end_matches('\0')
                .to_owned();

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
                return Err(Error::Custom("invalid checksum".to_owned()));
            }

            println!("Received message: command={}, length={}", command, length);

            return Ok(payload);
        }
        Err(Error::Custom("no connection".to_owned()))
    }

    async fn write(&mut self, data: Vec<u8>) -> Result<(), Error> {
        if let Some(ref mut stream) = self.stream {
            stream.write_all(&data).await?;
            return Ok(());
        }
        Err(Error::Custom("no connection".to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitcoin_connection_init() {
        let connection = BitcoinConnection::init("127.0.0.1", "18445");
        assert_eq!(connection.ip_address, "127.0.0.1");
        assert_eq!(connection.port, "18445");
        assert!(connection.stream.is_none());
    }

    #[tokio::test]
    async fn test_bitcoin_connection_connect_success() {
        let mut connection =
            BitcoinConnection::init("testnet-seed.bitcoin.jonasschnelli.ch", "18333");

        let result = connection.connect().await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_bitcoin_connection_connect_fail_wrong_address() {
        let mut connection = BitcoinConnection::init("testneelli.ch", "18333");

        let result = connection.connect().await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_bitcoin_connection_connect_fail_wrong_port() {
        let mut connection = BitcoinConnection::init("testneelli.ch", "33");

        let result = connection.connect().await;

        assert!(result.is_err());
    }
}
