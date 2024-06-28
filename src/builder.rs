pub trait Builder {
    fn build(&self, command: &str, payload: &[u8]) -> impl std::future::Future<Output = Vec<u8>> + Send;
    fn version(&self) -> impl std::future::Future<Output = Result<Vec<u8>, Box<dyn std::error::Error>>> + Send;
    fn verack(&self) -> impl std::future::Future<Output = Result < Vec<u8>, Box<dyn std::error::Error>>> + Send;
}