use crate::error::Error;

pub trait Connection {
    fn connect(&mut self) -> impl std::future::Future<Output = Result<(), Error>> + Send;
    fn read(&mut self) -> impl std::future::Future<Output = Result<Vec<u8>, Error>> + Send;
    fn write(
        &mut self,
        data: Vec<u8>,
    ) -> impl std::future::Future<Output = Result<(), Error>> + Send;
}
