const { Stack, aws_dynamodb, aws_lambda } = require("aws-cdk-lib");

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
  }
}

module.exports = { InfraStack };
