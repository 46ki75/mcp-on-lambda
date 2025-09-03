use aws_config::{BehaviorVersion, SdkConfig};
use futures::Stream;
use rmcp::transport::{
    WorkerTransport,
    streamable_http_server::{
        SessionManager,
        session::local::{LocalSessionManager, LocalSessionWorker},
    },
};

pub struct DynamoDbSessionManager {
    dynamodb_client: aws_sdk_dynamodb::Client,
    pk_name: String,
    value_name: String,
    table_name: String,
}

impl DynamoDbSessionManager {
    async fn new(pk: &str) -> Self {
        let sdk_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
        let dynamodb_client = aws_sdk_dynamodb::Client::new(&sdk_config);
        Self {
            dynamodb_client,
            pk_name: pk.to_string(),
            value_name: "value".to_string(),
            table_name: "sessions".to_string(),
        }
    }

    async fn get(&self, key: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let sdk_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
        let dynamodb_client = aws_sdk_dynamodb::Client::new(&sdk_config);

        let resp = dynamodb_client
            .get_item()
            .table_name(&self.table_name)
            .key(
                &self.pk_name,
                aws_sdk_dynamodb::types::AttributeValue::S(key.to_string()),
            )
            .send()
            .await?;

        if let Some(item) = resp.item {
            if let Some(attr) = item.get("data") {
                if let aws_sdk_dynamodb::types::AttributeValue::S(data) = attr {
                    return Ok(data.clone());
                }
            }
        }

        Err("Session not found or invalid data".into())
    }

    async fn set(
        &self,
        key: &str,
        value: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let sdk_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
        let dynamodb_client = aws_sdk_dynamodb::Client::new(&sdk_config);

        dynamodb_client
            .put_item()
            .table_name(&self.table_name)
            .item(
                &self.pk_name,
                aws_sdk_dynamodb::types::AttributeValue::S(key.to_string()),
            )
            .item(
                &self.value_name,
                aws_sdk_dynamodb::types::AttributeValue::S(value.to_string()),
            )
            .send()
            .await?;

        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DynamoDbSessionManagerError {}

impl SessionManager for DynamoDbSessionManager {
    type Error = DynamoDbSessionManagerError;

    type Transport = WorkerTransport<LocalSessionWorker>;

    fn create_session(
        &self,
    ) -> impl Future<
        Output = Result<
            (
                rmcp::transport::streamable_http_server::SessionId,
                Self::Transport,
            ),
            Self::Error,
        >,
    > + Send {
        async { todo!() }
    }

    fn initialize_session(
        &self,
        id: &rmcp::transport::streamable_http_server::SessionId,
        message: rmcp::model::ClientJsonRpcMessage,
    ) -> impl Future<Output = Result<rmcp::model::ServerJsonRpcMessage, Self::Error>> + Send {
        async { todo!() }
    }

    fn has_session(
        &self,
        id: &rmcp::transport::streamable_http_server::SessionId,
    ) -> impl Future<Output = Result<bool, Self::Error>> + Send {
        async { todo!() }
    }

    fn close_session(
        &self,
        id: &rmcp::transport::streamable_http_server::SessionId,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async { todo!() }
    }

    fn create_stream(
        &self,
        id: &rmcp::transport::streamable_http_server::SessionId,
        message: rmcp::model::ClientJsonRpcMessage,
    ) -> impl Future<
        Output = Result<
            impl Stream<Item = rmcp::transport::common::server_side_http::ServerSseMessage>
            + Send
            + Sync
            + 'static,
            Self::Error,
        >,
    > + Send {
        async { todo!() }
    }

    fn accept_message(
        &self,
        id: &rmcp::transport::streamable_http_server::SessionId,
        message: rmcp::model::ClientJsonRpcMessage,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async { todo!() }
    }

    fn create_standalone_stream(
        &self,
        id: &rmcp::transport::streamable_http_server::SessionId,
    ) -> impl Future<
        Output = Result<
            impl Stream<Item = rmcp::transport::common::server_side_http::ServerSseMessage>
            + Send
            + Sync
            + 'static,
            Self::Error,
        >,
    > + Send {
        async { todo!() }
    }

    fn resume(
        &self,
        id: &rmcp::transport::streamable_http_server::SessionId,
        last_event_id: String,
    ) -> impl Future<
        Output = Result<
            impl Stream<Item = rmcp::transport::common::server_side_http::ServerSseMessage>
            + Send
            + Sync
            + 'static,
            Self::Error,
        >,
    > + Send {
        async { todo!() }
    }
}
