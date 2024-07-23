use crate::{
    builder::{Builder, VersionMessage},
    error::Error,
};
use bitcoin_hashes::{sha256, Hash};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::constants::*;

/**
 * struct BtcMessageBuilder - Message factory
 * @magic_bytes: The magic bytes used in message headers for the appropriate bitcoin network
 *
 * BtcMessageBuilder is a submodule used by Handhsaker with the purpose of
 * building the messages that are sent to the target node, its only responsibility is
 * building message payloads and parsing recieved message payloads.
*/
pub struct BTCMessageBuilder {
    magic_bytes: [u8; 4],
}

impl Default for BTCMessageBuilder {
    fn default() -> Self {
        Self {
            magic_bytes: REGTEST_CHAIN_MAGIC_BYTES,
        }
    }
}

impl Builder for BTCMessageBuilder {
    /**
     * build
     * @command: The command or message type (version,verack)
     * @payload: The payload of the message
     *
     *  The purpose of this function is to encapsulate the given payload and build the final message
     *  that will be sent.
     */
    async fn build(&self, command: &str, payload: &[u8]) -> Result<Vec<u8>, Error> {
        if command.len() > 12 {
            return Err(Error::Custom("command too long".to_owned()));
        }

        let mut message = Vec::new();

        message.extend(self.magic_bytes);

        let mut cmd = command.as_bytes().to_vec();
        cmd.resize(12, 0x00);
        message.extend(&cmd);

        message.extend(&(payload.len() as u32).to_le_bytes());

        let checksum = sha256::Hash::hash(&sha256::Hash::hash(payload)[..]);

        message.extend(&checksum[..4]);

        message.extend(payload);

        Ok(message)
    }

    /**
     * version
     *
     * the purpose of this function is to build the payload of the "version" message.
     */
    async fn version(&self) -> Result<Vec<u8>, Error> {
        let mut payload = Vec::new();

        payload.write_i32_le(PROTOCOL_VERSION).await?;

        payload.write_u64_le(SERVICES).await?;

        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;

        payload.write_i64_le(timestamp).await?;

        payload.write_all(&[0; 26]).await?; // includes services,ip adress, port

        payload.write_all(&[0; 26]).await?; // includes services,ip adress, port

        payload.write_all(&[0; 8]).await?;

        let user_agent_bytes = USER_AGENT.as_bytes();
        payload.write_u8(user_agent_bytes.len() as u8).await?;
        payload.write_all(user_agent_bytes).await?;

        payload.write_all(&[0; 1]).await?;

        payload.write_i32_le(START_HEIGHT).await?;

        payload.write_u8(1).await?;

        Ok(payload)
    }

    /**
     * verack
     *
     * same as the "version" function, but verack message payloads are empty.
     */
    async fn verack(&self) -> Result<Vec<u8>, Error> {
        Ok(vec![])
    }

    /**
     * parse_version_message_payload
     * @payload: the payload/bytes of the message to parse
     *
     *
     * this function is responsible of parsing the "version message" payload
     * Ideally this functionality would also be split up to parse any message but for the
     * purpose of this small demo app this is enough.
     */
    async fn parse_message_payload(&self, payload: &[u8]) -> Result<VersionMessage, Error> {
        // Ensure payload has at least the minimum size for a version message
        if payload.len() < 85 {
            return Err(Error::Custom(
                "Payload too short for a version message".to_owned(),
            ));
        }

        let mut cursor = std::io::Cursor::new(payload);

        let version = cursor.read_i32_le().await?;

        let services = cursor.read_u64_le().await?;

        let timestamp = cursor.read_i64_le().await?;

        let addr_recv_services = cursor.read_u64_le().await?;

        let position = cursor.position() as usize;

        let addr_recv_ip_address =
            String::from_utf8_lossy(&payload[position..position + 16]).to_string();

        cursor.set_position(position as u64 + 16);

        let addr_recv_port = cursor.read_u16_le().await?;

        let addr_trans_services = cursor.read_u64_le().await?;

        let position = cursor.position() as usize;

        let addr_trans_ip_address =
            String::from_utf8_lossy(&payload[position..position + 16]).to_string();

        cursor.set_position(position as u64 + 16);

        let addr_trans_port = cursor.read_u16_le().await?;

        let nonce = cursor.read_u64_le().await?;

        let user_agent_bytes = payload[cursor.position() as usize];

        let user_agent =
            String::from_utf8_lossy(&payload[81_usize..81_usize + user_agent_bytes as usize])
                .to_string();

        cursor.set_position(cursor.position() + user_agent_bytes as u64);

        let start_height = cursor.read_i32_le().await?;

        let relay = payload[cursor.position() as usize] != 0;

        let version_message = VersionMessage {
            version,
            services,
            timestamp,
            addr_recv_services,
            addr_recv_ip_address,
            addr_recv_port,
            addr_trans_services,
            addr_trans_ip_address,
            addr_trans_port,
            nonce,
            user_agent_bytes,
            user_agent,
            start_height,
            relay,
        };

        Ok(version_message)
    }
}

#[cfg(test)]
mod tests {

    impl BTCMessageBuilder {
        pub fn set_magic_bytes(&mut self, magic_bytes: [u8; 4]) {
            self.magic_bytes = magic_bytes;
        }
    }

    use super::*;
    use tokio::io::AsyncReadExt;

    #[tokio::test]
    async fn test_build_empty_message() {
        let data_processor = BTCMessageBuilder::default();

        let result = data_processor.build("command", &vec![]).await;

        assert!(result.is_ok());

        assert_eq!(result.unwrap().len(), 24);
    }

    #[tokio::test]
    async fn test_empty_message_regtest_magic_bytes() {
        let data_processor = BTCMessageBuilder::default();

        let command = "command";

        let result = data_processor.build(command, &vec![]).await;

        let bytes = &result.unwrap()[0..4];

        assert_eq!(bytes, REGTEST_CHAIN_MAGIC_BYTES);
    }

    #[tokio::test]
    async fn test_empty_message_devnet_magic_bytes() {
        let mut data_processor = BTCMessageBuilder::default();

        data_processor.set_magic_bytes(DEVNET_CHAIN_MAGIC_BYTES);

        let command = "command";

        let result = data_processor.build(command, &vec![]).await;

        assert!(result.is_ok());

        let bytes = &result.unwrap()[0..4];

        assert_eq!(bytes, DEVNET_CHAIN_MAGIC_BYTES);
    }

    #[tokio::test]
    async fn test_empty_message_command() {
        let data_processor = BTCMessageBuilder::default();

        let command = "command";

        let result = data_processor.build(command, &vec![]).await;

        let mut read_command = String::from_utf8_lossy(&result.unwrap()[4..16]).to_string();

        read_command.replace_range(command.len().., "");

        assert_eq!(read_command, command);
    }

    #[tokio::test]
    async fn test_empty_message_command_len() {
        let data_processor = BTCMessageBuilder::default();

        let command = "commandthatiswaytoolong";

        let result = data_processor.build(command, &vec![]).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_empty_message_payload_len() {
        let data_processor = BTCMessageBuilder::default();

        let command = "command";

        let result = data_processor.build(command, &vec![]).await.unwrap();

        let mut cursor = std::io::Cursor::new(result);

        cursor.set_position(16);

        let payload_size = cursor.read_u32_le().await.unwrap();

        assert_eq!(payload_size, 0);
    }

    #[tokio::test]
    async fn test_empty_message_payload_hash() {
        let data_processor = BTCMessageBuilder::default();

        let command = "command";

        let result = data_processor.build(command, &vec![]).await.unwrap();

        // this is the expected value for an empty payload hash
        assert_eq!(&result[20..], &[0x5d, 0xf6, 0xe0, 0xe2]);
    }

    #[tokio::test]
    async fn test_version_message_build() {
        let data_processor = BTCMessageBuilder::default();

        let result = data_processor.version().await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_version_message_parse() {
        let data_processor = BTCMessageBuilder::default();

        let result = data_processor.version().await.unwrap();

        let version_message = data_processor.parse_message_payload(&result).await;

        assert!(version_message.is_ok());

        let version_message = version_message.unwrap();

        assert_eq!(version_message.user_agent, USER_AGENT);
        assert_eq!(version_message.version, PROTOCOL_VERSION);
        assert_eq!(version_message.services, SERVICES);
    }
}
