
### Configuration Requirement for `deploy`, `test`, and `call`

The `deploy`, `test`, and `call` subcommands require a `alephium.config.yaml` file to be present in the working directory. This file provides the necessary details for deploying and interacting with your contracts.

Here is an example of a `alephium.config.yaml` file with a detailed explanation of each field:

```yaml
# The 'contracts' field contains a map of contract names to their configurations.
contracts:
  # This is the name of your contract, for example 'MyContract'.
  MyContract:
    # 'initialFields' defines the initial state of your contract when it's deployed.
    # The structure of this field must match the fields defined in your contract's source code.
    initialFields:
      # You can have simple key-value pairs.
      someStringField: "someValue"
      someNumberField: 123
      someBooleanField: true
      # You can also have nested structures.
      nestedObject:
        subField1: "hello"
        subField2: 456
      # And arrays of values.
      arrayField:
        - "value1"
        - "value2"

    # 'initialAsset' defines the initial assets held by the contract upon deployment.
    initialAsset:
      # 'attoAlphAmount' is the amount of ALPH to send to the contract, in attos (1 ALPH = 10^18 attos).
      attoAlphAmount: "1000000000000000000" # 1 ALPH
      # 'tokens' is an optional list of tokens to send to the contract.
      tokens:
        - id: "someTokenId"
          amount: "100"
    # 'inputAssets' defines the assets that will be used to pay for the deployment or contract call.
    # These assets are taken from the address associated with the private key used for the transaction.
    inputAssets:
      - address: "someAddress"
        asset:
          attoAlphAmount: "500000000000000000" # 0.5 ALPH
          tokens: []
```

#### `initialFields` Types

The `initialFields` section supports a variety of types to match the fields in your Alephium contract. Here is a list of the supported types and how to format them in your `alephium.config.yaml`:

*   **`Bool`**: A boolean value.
    ```yaml
    myBooleanField: true
    ```

*   **`U256` and `I256`**: Unsigned and signed 256-bit integers. These should be represented as strings to avoid precision issues.
    ```yaml
    myU256Field: "10000000000000000000"
    myI256Field: "-500"
    ```

*   **`ByteVec`**: A byte vector. You can provide this as a hex string (starting with `0x`) or a regular string (which will be converted to its hex representation).
    ```yaml
    myHexByteVec: "0xdeadbeef"
    myStringByteVec: "hello world"
    ```

*   **`Address`**: An Alephium address.
    ```yaml
    myAddressField: "1HYMvM5e5p5f7iN1o5P6Z6t8qW3d2b"
    ```

*   **`Array`**: A fixed-size array of elements of the same type.
    ```yaml
    myArrayField:
      - "value1"
      - "value2"
    ```

*   **`Tuple`**: A tuple of elements, which can be of different types.
    ```yaml
    myTupleField:
      - "someValue"
      - 123
      - true
    ```

*   **`Structure`**: A nested object with its own fields.
    ```yaml
    myStructField:
      subField1: "hello"
      subField2: 456
    ```

### Argument Parsing for `call` and `test`

When using the `call` and `test` subcommands, you need to provide arguments for the contract methods. These arguments are passed using the `--args` flag, with each argument in the format `KEY=VALUE`.

The parser supports a variety of types and formats:

*   **Simple Types**: For simple types like `Bool`, `U256`, `I256`, `ByteVec`, and `Address`, you can provide the value directly.
    ```bash
    --args myBool=true myU256=123 myByteVec="hello"
    ```

*   **Hex Values for `ByteVec`**: You can provide `ByteVec` values as hex strings by prefixing them with `0x`.
    ```bash
    --args myByteVec=0xdeadbeef
    ```

*   **Nested Structures**: For nested structures, use dot notation to specify the fields.
    ```bash
    --args myStruct.field1="value1" myStruct.field2=456
    ```

*   **Arrays**: For arrays, provide the value as a JSON-formatted string.
    ```bash
    --args myArray="[1, 2, 3]"
    ```
