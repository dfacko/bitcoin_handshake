pub trait Connection {
    async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    async fn read(&mut self) -> Result<Vec<u8>,Box<dyn std::error::Error>>;
    async fn write(&mut self, data: Vec<u8>) -> Result <(), Box<dyn std::error::Error> >;
}