# Smart Contract

The smart contract must have the following functionality:

1. Whitelist Checks
    a. Are the tokens in the swap available for trading.
    b. Are the DEX’s in the swap available for trading.
    c. Is the transaction owner available to make this swap.
Error and Failure Contexts:
Provide usable feedback to feed back into the metrics explained above.
Schedule multiple transactions between various DEX’s
Transactions can be bundled into a list sequential transactions between DEX’s.
They are then executed sequentially with low latency transactions to minimize slippage and give maximum return.
Admin responsibilities:
Dynamically change Token and DEX 
Custody of funds
Hold funds on chain therefore allowing faster access to details.
Essentially a hot wallet.
