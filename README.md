# calldatafmt

This tool should be used after independently verifying the calldata for a specific transaction. After the calldata is verified, it can be passed through this tool so that it can be formatted as it is on a hardware wallet, making it easier to confirm that the calldata displayed on the hardware wallet is the calldata that you intend to send for your transaction.

This tool currently supports:

- Ledger Flex
- Trezor Safe 7

Note: This tool has only been tested on EVM transactions.


## Installation

This CLI requires Rust to be installed.

Once Rust is installed, you can install `calldatafmt` from the GitHub repository.

```bash
cargo install --git https://github.com/lucasfhope/calldatafmt.git
```

Check that it is installed properly with `calldatafmt --help`.


## Usage

The `calldatafmt` CLI requires a wallet identifier and calldata to print the display. You must use the correct identifier of a supported wallet. The calldata must be hex-encoded and include a 4-byte function selector followed by optional 32-byte parameters.

Remember that it is important to verify the calldata that you expect to send for your transaction before using this tool. Also make sure that other transaction values are verified, including the contract you are sending the transaction to and the nonce (if applicable). These values are usually displayed after the calldata.

## Wallets

**Ledger Flex** 

If you are using the Ledger Flex, ensure that `Debug smart contracts` is enabled in the settings of the blockchain app. This will have the user go through the calldata being sent in the transaction.

For Ledger Flex formatting, pass the calldata to the CLI using the `flex` wallet parameter.

```bash
calldatafmt flex <CALLDATA>
```

This will format the calldata and break it into parameters as the Ledger Flex will display.

**Trezor Safe 7** 

If you are using the Trezor Safe 7, once you initiate the transaction, click `View all data`. This will break the calldata into pages.

For Trezor Safe 7 formatting, pass the calldata to the CLI using the `safe7` wallet parameter.

```bash
calldatafmt safe7 <CALLDATA>
```

This will format the calldata by breaking it into pages as the Trezor Safe 7 will display.