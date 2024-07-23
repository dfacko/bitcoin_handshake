use bitcoin_connector::BitcoinConnection;
use handshaker::Handshaker;
use message_builder::BTCMessageBuilder;

mod bitcoin_connector;
mod builder;
mod connection;
mod constants;
mod error;
mod handshaker;
mod message_builder;

#[tokio::main]
async fn main() {
    let connection = BitcoinConnection::init("127.0.0.1", "18445");

    let data_processor = BTCMessageBuilder::default();

    let mut handshaker = Handshaker::init(connection, data_processor);

    if let Err(error) = handshaker.handshake().await {
        println!("{}", error);
        return;
    }

    println!("Hands have been shaken!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use constants::DEVNET_CHAIN_MAGIC_BYTES;

    #[tokio::test]
    async fn test_devnet_handhsake() {
        let connection = BitcoinConnection::init("testnet-seed.bitcoin.jonasschnelli.ch", "18333");

        let mut data_processor = BTCMessageBuilder::default();

        data_processor.set_magic_bytes(DEVNET_CHAIN_MAGIC_BYTES);

        let mut handshaker = Handshaker::init(connection, data_processor);

        let result = handshaker.handshake().await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_local_handshake() {
        let connection = BitcoinConnection::init("127.0.0.1", "18445");

        let data_processor = BTCMessageBuilder::default();

        let mut handshaker = Handshaker::init(connection, data_processor);

        let result = handshaker.handshake().await;

        assert!(result.is_ok());
    }
}
