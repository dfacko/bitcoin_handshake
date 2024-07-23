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
    fn build(
        &self,
        command: &str,
        payload: &[u8],
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Error>> + Send;
    fn version(&self) -> impl std::future::Future<Output = Result<Vec<u8>, Error>> + Send;
    fn verack(&self) -> impl std::future::Future<Output = Result<Vec<u8>, Error>> + Send;
    fn parse_message_payload(
        &self,
        payload: &[u8],
    ) -> impl std::future::Future<Output = Result<VersionMessage, Error>> + Send;
}
