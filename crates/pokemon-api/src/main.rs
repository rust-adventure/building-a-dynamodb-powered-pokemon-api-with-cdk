use aws_lambda_events::{
    encodings::Body,
    event::apigw::{
        ApiGatewayV2httpRequest, ApiGatewayV2httpResponse,
    },
};
use aws_sdk_dynamodb::{model::AttributeValue, Client};
use http::HeaderMap;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::json;
use std::env;
use tokio::sync::OnceCell;

static DDB_CLIENT: OnceCell<Client> = OnceCell::const_new();

async fn get_global_client() -> &'static Client {
    DDB_CLIENT
        .get_or_init(|| async {
            let shared_config =
                aws_config::load_from_env().await;
            let client = Client::new(&shared_config);
            client
        })
        .await
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    get_global_client().await;
    let handler_fn = service_fn(handler);
    lambda_runtime::run(handler_fn).await?;
    Ok(())
}

async fn handler(
    event: LambdaEvent<ApiGatewayV2httpRequest>,
) -> Result<ApiGatewayV2httpResponse, Error> {
    let (event, _context) = event.into_parts();

    match event.path_parameters.get("pokemon") {
        None => Ok(ApiGatewayV2httpResponse {
            status_code: 400,
            headers: HeaderMap::new(),
            multi_value_headers: HeaderMap::new(),
            body: Some(Body::Text(serde_json::to_string(
                &json!({
                    "err": "No pokemon requested",
                    "data": {}
                }),
            )?)),
            is_base64_encoded: Some(false),
            cookies: vec![],
        }),
        Some(pokemon_requested) => {
            let pokemon_table = env::var("POKEMON_TABLE")?;
            let client = get_global_client().await;

            let resp = client
                .get_item()
                .key(
                    "pk",
                    AttributeValue::S(
                        pokemon_requested.to_string(),
                    ),
                )
                .table_name(pokemon_table)
                .send()
                .await?;

            match resp.item {
                Some(item) => {
                    Ok(ApiGatewayV2httpResponse {
                        status_code: 200,
                        headers: HeaderMap::new(),
                        multi_value_headers: HeaderMap::new(
                        ),
                        body: Some(Body::Text(
                            serde_json::to_string(
                                &json!({
                                    "data": {
                                        "id": item.get("pk").unwrap().as_s().unwrap(),
                                        "name": item.get("name").unwrap().as_s().unwrap(),
                                        "healthPoints": item.get("health_points").unwrap().as_n().unwrap()
                                    },
                                }),
                            )?,
                        )),
                        is_base64_encoded: Some(false),
                        cookies: vec![],
                    })
                }
                None => Ok(ApiGatewayV2httpResponse {
                    status_code: 200,
                    headers: HeaderMap::new(),
                    multi_value_headers: HeaderMap::new(),
                    body: Some(Body::Text(
                        serde_json::to_string(&json!({
                            "data": {}
                        }))?,
                    )),
                    is_base64_encoded: Some(false),
                    cookies: vec![],
                }),
            }
        }
    }
}
