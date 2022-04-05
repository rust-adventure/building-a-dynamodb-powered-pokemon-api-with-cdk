const { Stack, CfnOutput, aws_dynamodb, aws_lambda } = require("aws-cdk-lib");
const integrations = require("@aws-cdk/aws-apigatewayv2-integrations-alpha");
const apigateway = require("@aws-cdk/aws-apigatewayv2-alpha");

class InfraStack extends Stack {
  /**
   *
   * @param {Construct} scope
   * @param {string} id
   * @param {StackProps=} props
   */
  constructor(scope, id, props) {
    super(scope, id, props);

    const pokemonTable = new aws_dynamodb.Table(this, "PokemonTable", {
      billingMode: aws_dynamodb.BillingMode.PAY_PER_REQUEST,
      partitionKey: {
        name: "pk",
        type: aws_dynamodb.AttributeType.STRING,
      },
    });

    // https://aws.amazon.com/amazon-linux-2/faqs/
    // AL2 holds libc 2.26
    const pokemonLambda = new aws_lambda.Function(this, "PokemonHandler", {
      runtime: aws_lambda.Runtime.PROVIDED_AL2,
      handler: "pokemon-handler",
      code: aws_lambda.Code.fromAsset("../lambdas/pokemon-api"),
      memorySize: 1024,
    });

    const pokemonIntegration = new integrations.HttpLambdaIntegration(
      "PokemonIntegration",
      pokemonLambda
    );

    const httpApi = new apigateway.HttpApi(this, "pokemon-api");
    httpApi.addRoutes({
      path: "/pokemon/{pokemon}",
      methods: [apigateway.HttpMethod.GET],
      integration: pokemonIntegration,
    });

    new CfnOutput(this, "pokemonTable", {
      value: pokemonTable.tableName,
      description: "The name of the DynamoDB Table",
    });

    new CfnOutput(this, "pokemonLambda", {
      value: pokemonLambda.functionName,
      description: "The name of the Pokemon Lambda",
    });
    new CfnOutput(this, "pokemonBaseUrl", {
      value: httpApi.apiEndpoint,
      description: "The root URL for the pokemon endpoint",
    });
  }
}

module.exports = { InfraStack };
