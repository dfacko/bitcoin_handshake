use crate::builder::Builder;
use bitcoin_hashes::{sha256, Hash};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::io::AsyncWriteExt;

use crate::constants::*;

pub struct BTCMessageBuilder {}

impl Builder for BTCMessageBuilder {
    async fn build(&self, command: &str, payload: &[u8]) -> Vec<u8> {
        let mut message = Vec::new();

        // Bitcoin protocol magic bytes (regtest network)
        message.extend(&CHAIN_MAGIC_BYTES);

        // Command string, padded to 12 bytes
        let mut cmd = command.as_bytes().to_vec();
        cmd.resize(12, 0x00);
        message.extend(&cmd);

        // Payload length
        message.extend(&(payload.len() as u32).to_le_bytes());

        let checksum2 = sha256::Hash::hash(&sha256::Hash::hash(payload)[..]);

        message.extend(&checksum2[..4]);

        //message.extend(&[0x5d,0xf6,0xe0,0xe2]); // if payload is empty

        // Payload
        message.extend(payload);

        message
    }

    async fn version(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut payload = Vec::new();

        // Protocol version
        payload.write_i32_le(PROTOCOL_VERSION).await?;

        // Services
        payload.write_u64_le(SERVICES).await?;

        // Timestamp (current time)
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;

        //let timestamp = Utc::now().timestamp();

        payload.write_i64_le(timestamp).await?;

        // Sender address (empty, as we're not advertising the peer)
        payload.write_all(&[0; 26]).await?; // includes addr recv

        // Sender address (empty, as we're not advertising the peer)
        payload.write_all(&[0; 26]).await?; // includes addr recv

        // Nonce (random unique number for this connection)
        payload.write_all(&[0; 8]).await?;

        // User agent
        let user_agent_bytes = USER_AGENT.as_bytes();
        payload.write_u8(user_agent_bytes.len() as u8).await?;
        payload.write_all(user_agent_bytes).await?;

        payload.write_all(&[0; 1]).await?;

        // Start height (current block height)
        payload.write_i32_le(START_HEIGHT).await?;

        // Relay option (true, indicating the remote peer should relay transactions to us)
        payload.write_u8(1).await.unwrap();

        Ok(payload)
    }

    async fn verack(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        Ok(vec![])
    }
}
