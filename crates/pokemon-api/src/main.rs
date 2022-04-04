use aws_lambda_events::{
    encodings::Body,
    event::apigw::{
        ApiGatewayV2httpRequest, ApiGatewayV2httpResponse,
    },
};
use http::HeaderMap;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler_fn = service_fn(handler);
    lambda_runtime::run(handler_fn).await?;
    Ok(())
}

async fn handler(
    event: LambdaEvent<ApiGatewayV2httpRequest>,
) -> Result<ApiGatewayV2httpResponse, Error> {
    let (_event, _context) = event.into_parts();

    Ok(ApiGatewayV2httpResponse {
        status_code: 200,
        headers: HeaderMap::new(),
        multi_value_headers: HeaderMap::new(),
        body: Some(Body::Text(serde_json::to_string(
            &json!({
                "data": {}
            }),
        )?)),
        is_base64_encoded: Some(false),
        cookies: vec![],
    })
}
