#[tokio::main]
async fn main() -> Result<(), lambda_http::Error> {
    lambda_http::tracing::init_default_subscriber();
    lambda_http::run(lambda_http::service_fn(
        mcp_on_lambda::function_handler::function_handler,
    ))
    .await
}
