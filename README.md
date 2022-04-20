# Building a DynamoDB and Serverless Powered Pokemon API with CDK

A [Rust Adventure](https://www.rustadventure.dev/) Workshop

## Building the lambda binary

```shell
cargo zigbuild --target x86_64-unknown-linux-gnu.2.26 --release
mkdir -p lambdas/pokemon-api
cp target/x86_64-unknown-linux-gnu/release/pokemon-api lambdas/pokemon-api/bootstrap
```
