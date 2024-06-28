use bitcoin_connector::BitcoinConnection;
use connection::Connection;
use builder::Builder;
use message_builder::BTCMessageBuilder;

mod bitcoin_connector;
mod builder;
mod connection;
mod message_builder;
mod constants;

pub struct Handshaker < C: Connection, B:Builder > {
    connection : C,
    data_processor: B
}

impl<C: Connection, B:Builder>  Handshaker <C, B> {

    pub async fn handshake(&mut self) -> Result<(),Box<dyn std::error::Error>>{

        self.connection.connect().await?;

        let version_message = self.data_processor.version().await?;

        let built_version_message = self.data_processor.build("version", &version_message).await;

        self.connection.write(built_version_message).await?;

        let _recieved_version_payload = self.connection.read().await?;

        let verack_message = self.data_processor.verack().await?;

        let built_verack_message = self.data_processor.build("verack", &verack_message).await;

        self.connection.write(built_verack_message).await?;

        let _recieved_verack_payload = self.connection.read().await?;


        Ok(())

    }
}

#[tokio::main]
async fn main() {

    let connection = BitcoinConnection::init("127.0.0.1", "18445");

    let data_processor = BTCMessageBuilder {};

    
    let mut  handshaker = Handshaker {
        connection, 
        data_processor 
    };

    if let Err(error) = handshaker.handshake().await {
        println!("{}", error);
    }

    println!("Hands have been shaken!");

}
