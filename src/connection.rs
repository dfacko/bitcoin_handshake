use crate::error::Error;

pub trait Connection {
    /// Establishes a connection on ip:port and updates self
    /// to contain the connection stream
    fn connect(&mut self) -> impl std::future::Future<Output = Result<(), Error>> + Send;

    /// Reads data from self contained stream, considering we are only performing handshakes
    /// message headers are ignored and the returned data only contains the message payload.
    fn read(&mut self) -> impl std::future::Future<Output = Result<Vec<u8>, Error>> + Send;

    /// Write data to the self containd stream
    fn write(
        &mut self,
        data: Vec<u8>,
    ) -> impl std::future::Future<Output = Result<(), Error>> + Send;
}
