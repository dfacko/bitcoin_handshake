use crate::{builder::Builder, connection::Connection, error::Error};

/** struct Handshaker
 *
 * This struct is the main module and is responsible to execute the handshake.
 * To do this it uses any other modules that fulfill the Connection and Builder contracts.

*/
pub struct Handshaker<C: Connection, B: Builder> {
    connection: C,
    data_processor: B,
}

impl<C: Connection, B: Builder> Handshaker<C, B> {
    /**
     * handshake - The main entrypoint of Handshaker
     *
     * This is where the actual handshake functionality is implemented
     * using connection's and data_processor's trait functions.
     */
    pub async fn handshake(&mut self) -> Result<(), Error> {
        self.connection.connect().await?;

        let version_message_payload = self.data_processor.version().await?;

        let built_version_message = self
            .data_processor
            .build("version", &version_message_payload)
            .await?;

        self.connection.write(built_version_message.clone()).await?;

        let recieved_version_payload = self.connection.read().await?;

        println!(
            "recieved ver payload len {}",
            recieved_version_payload.len()
        );

        println!(
            "{:#?}",
            self.data_processor
                .parse_message_payload(&recieved_version_payload)
                .await?
        );

        let verack_message = self.data_processor.verack().await?;

        let built_verack_message = self.data_processor.build("verack", &verack_message).await?;

        self.connection.write(built_verack_message).await?;

        let _recieved_verack_payload = self.connection.read().await?;

        Ok(())
    }
    /**
     * init - Initialite a Handshaker instance.
     * @connection: Any structure that statisfies the Connection Trait.
     * @data_processor: Any structure that statisfire the Connection Trait.
     *
     */
    pub fn init(connection: C, data_processor: B) -> Self {
        Self {
            connection,
            data_processor,
        }
    }
}
