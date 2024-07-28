use crate::error::Error;

#[derive(Debug, PartialEq, Eq)]
pub struct VersionMessage {
    pub version: i32,
    pub services: u64,
    pub timestamp: i64,
    pub addr_recv_services: u64,
    pub addr_recv_ip_address: String,
    pub addr_recv_port: u16,
    pub addr_trans_services: u64,
    pub addr_trans_ip_address: String,
    pub addr_trans_port: u16,
    pub nonce: u64,
    pub user_agent_bytes: u8,
    pub user_agent: String,
    pub start_height: i32,
    pub relay: bool,
}

pub trait Builder {
    ///  The purpose of this function is to encapsulate the given payload and build the final message
    ///  that will be sent.
    fn build(
        &self,
        command: &str,
        payload: &[u8],
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Error>> + Send;

    /// The purpose of this function is to build the payload of the "version" message.
    fn version(&self) -> impl std::future::Future<Output = Result<Vec<u8>, Error>> + Send;

    /// Same as the "version" function, but verack message payloads are empty.
    fn verack(&self) -> impl std::future::Future<Output = Result<Vec<u8>, Error>> + Send;

    /// This function is responsible of parsing the "version message" payload
    /// Ideally this functionality would also be split up to parse any message but for the
    /// purpose of this small demo app this is enough.
    fn parse_message_payload(
        &self,
        payload: &[u8],
    ) -> impl std::future::Future<Output = Result<VersionMessage, Error>> + Send;
}
