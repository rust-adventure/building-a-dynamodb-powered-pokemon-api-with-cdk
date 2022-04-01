const { Stack, aws_dynamodb } = require("aws-cdk-lib");

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
  }
}

module.exports = { InfraStack };
