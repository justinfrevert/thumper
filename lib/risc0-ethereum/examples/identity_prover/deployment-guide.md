# RISC Zero View Call Proofs ERC20 Example Ethereum Deployment Guide

> **Note: This software is not production ready. Do not use in production.**

Welcome to the [RISC Zero] View Call Proofs ERC20 Example Ethereum Deployment guide!

You can either:

- [Deploy to a local network]
- [Deploy to a testnet]

## Deploy on a local network

You can deploy your contracts and run an end-to-end test or demo as follows:

1. Install `ganache`: 
    You must first install [Node.js] >= v16.0.0 and npm >= 7.10.0.
    To install Ganache globally, run:
    ```
    npm install ganache --global
    ```

2. Start a local testnet with `ganache` by running:

    ```bash
    ganache
    ```

    Once ganache is started, look at its logs and copy any of the Private Keys. You'll need one for the next step.
    Then, keep it running in the terminal, and switch to a new terminal.

2. Set your environment variables:
    > ***Note:*** *This requires having access to a Bonsai API Key. To request an API key [complete the form here](https://bonsai.xyz/apply).*

    ```bash
    # Ganache sets up a number of private keys, use the one you copied during the previous step.
    export ETH_WALLET_PRIVATE_KEY="YOUR_GANACHE_PRIVATE_KEY"
    export BONSAI_API_KEY="YOUR_API_KEY" # see form linked in the previous section
    export BONSAI_API_URL="BONSAI_API_URL" # provided with your api key
    ```

4. Deploy the dummy zkkyc contract, as well as the coordinator contract.
    ```
    forge script --rpc-url http://localhost:8545 --broadcast script/DeployProject.s.sol
    ```
    Save the `ERC721 ZKKYC` and `COORDINATOR_ADDRESS` contract addresses to an env variable:
    ```
    export ZKKYC_ADDRESS=# Copy zkkyc token address
    export COORDINATOR_ADDRESS=# Copy zkkyc token address
    ```

5. Mint some dummy zkkyc'ed identities:
    ```
    cast send --private-key $ETH_WALLET_PRIVATE_KEY --rpc-url http://localhost:8545 $ZKKYC_ADDRESS 'mint(address,bytes32)' 0x83bF19D749cb807c19B4a6dF8e30fed56E158DD7 0x2bcf9d7c56545585f04d5567d8c8a4fd01f8d405a0d2bc7a043ad6d4ab3e1430

    cast send --private-key $ETH_WALLET_PRIVATE_KEY --rpc-url http://localhost:8545 $ZKKYC_ADDRESS 'mint(address,bytes32)' 0xa527546eBF9faa960C7f287561a1ECE8298beB15 0174073c906caf76c5a587877e936875ad2960de2c0816eb844e02683dd52cd8
    ```

6. Populate Some ceremony participants(should be outputs from `identity_prover`)
```
cast send --private-key $ETH_WALLET_PRIVATE_KEY --rpc-url http://localhost:8545 $COORDINATOR_ADDRESS 'updateIdentityCommitments(uint256,bytes32[])' 1 ["0x994368d0308f9dcdc1f30a18d8fe2c371a8eb199c64f21fdfe28b469cd8ee97d", "0x1eaa1e5f20ccc915f7190ff9e5126e09870ec7bab852cf9d801b1c01641139af"]

```

3. Build the project:
    
    Before building the project, make sure the contract address on both the [methods/guest/src/bin/balance_of.rs] as well [apps/src/bin/publisher.rs] is set to the value of your deployed `ZKKYC_ADDRESS`:

    ```rust
    const CONTRACT: Address = address!("<PLACE YOUR TOYKEN ADDRESS HERE>");
    ```
    
    Then run:

    ```bash
    cargo build
    ```

### Interact with your local deployment


1. Publish a new state/get a view call + identity proof for the identity guest program
Note that the private secret key is a different argument from the ETH_WALLET_PRIVATE_KEY in that it just represents the zkkycced account you can attest to owning.
You can also pass in the private secret key 

    ```bash
    cargo run --bin publisher -- \
        --chain-id=1337 \
        --rpc-url=http://localhost:8545 \
        --contract=${ZKKYC_ADDRESS:?} \
        --account=0x83bF19D749cb807c19B4a6dF8e30fed56E158DD7 \
        --private-secret-key=0x447db9e9f28d0be24e439f4c5437c3321884ba000dddc3949b0dbb4f12380a6b
    ```

    and for the second account we minted an "identity" for
    ```bash
    cargo run --bin publisher -- \
        --chain-id=1337 \
        --rpc-url=http://localhost:8545 \
        --contract=${ZKKYC_ADDRESS:?} \
        --account=0xa527546eBF9faa960C7f287561a1ECE8298beB15 \
        --private-secret-key=0xf3d540f79c72415e0d5eebb2a23bf7f87613ee6b83ad48d3ec102e614e656aa2
    ```

3. Query the state again to see the change:

    ```bash
    cast call --rpc-url http://localhost:8545 ${COUNTER_ADDRESS:?} 'get()(uint256)'
    ```

## Deploy your project on a testnet

You can deploy the Counter contract on a testnet such as `Sepolia` and run an end-to-end test or demo as follows:
> ***Note***: we'll be using an existing ERC20 contract for this example, specifically the USDT ERC20 contract deployed on Sepolia at address [0xaA8E23Fb1079EA71e0a56F48a2aA51851D8433D0].

1. Get access to Bonsai and an Ethereum node running on a given testnet, e.g., Sepolia (in this example, we will be using [Alchemy](https://www.alchemy.com/) as our Ethereum node provider) and export the following environment variables:
    > ***Note:*** *This requires having access to a Bonsai API Key. To request an API key [complete the form here](https://bonsai.xyz/apply).*

    ```bash
    export BONSAI_API_KEY="YOUR_API_KEY" # see form linked in the previous section
    export BONSAI_API_URL="BONSAI_API_URL" # provided with your api key
    export ALCHEMY_API_KEY="YOUR_ALCHEMY_API_KEY" # the API_KEY provided with an alchemy account
    export ETH_WALLET_PRIVATE_KEY="YOUR_WALLET_PRIVATE_KEY" # the private hex-encoded key of your Sepolia testnet wallet
    ```

2. Build the project:
    
    Before building the project, make sure the contract address on both the [methods/guest/src/bin/balance_of.rs] as well [apps/src/bin/publisher.rs] is set to `aA8E23Fb1079EA71e0a56F48a2aA51851D8433D0`
    
    ```rust
    const CONTRACT: Address = address!("F66a26e6D7A310bdb8E34fF028568B1D5e59cA43");
    ```
    
    Then run:

    ```bash
    cargo build
    ```

3. Deploy the Counter contract by running:

    ```bash
    forge script script/DeployCounter.s.sol --rpc-url https://eth-sepolia.g.alchemy.com/v2/${ALCHEMY_API_KEY:?} --broadcast
    ```

     This command should output something similar to:

    ```bash
    ...
    == Logs ==
    Deployed RiscZeroGroth16Verifier to 0x5FbDB2315678afecb367f032d93F642f64180aa3
    Deployed Counter to 0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512
    ...
    ```

    Save the `Counter` contract address to an env variable:

    ```bash
    export COUNTER_ADDRESS=#COPY COUNTER ADDRESS FROM DEPLOY LOGS
    ```

    > You can also use the following command to set the contract address if you have [`jq`][jq] installed:
    >
    > ```bash
    > export COUNTER_ADDRESS=$(jq -re '.transactions[] | select(.contractName == "Counter") | .contractAddress' ./broadcast/DeployCounter.s.sol/11155111/run-latest.json)
    > ```

### Interact with your testnet deployment

1. Query the state:

    ```bash
    cast call --rpc-url https://eth-sepolia.g.alchemy.com/v2/${ALCHEMY_API_KEY:?} ${COUNTER_ADDRESS:?} 'get()(uint256)'
    ```

2. Publish a new state

    ```bash
    cargo run --bin publisher -- \
        --chain-id=11155111 \
        --rpc-url=https://eth-sepolia.g.alchemy.com/v2/${ALCHEMY_API_KEY:?} \
        --contract=${COUNTER_ADDRESS:?} \
        --account=0x83bF19D749cb807c19B4a6dF8e30fed56E158DD7
    ```

3. Query the state again to see the change:

    ```bash
    cast call --rpc-url https://eth-sepolia.g.alchemy.com/v2/${ALCHEMY_API_KEY:?} ${COUNTER_ADDRESS:?} 'get()(uint256)'
    ```

[Deploy to a testnet]: #deploy-your-project-on-a-testnet
[Deploy your project to a local network]: #deploy-your-project-on-a-local-network
[RISC Zero]: https://www.risczero.com/
[Node.js]: https://nodejs.org/
[jq]: https://jqlang.github.io/jq/
[methods]: ./methods/
[tested]: ./README.md#run-the-tests
[0xaA8E23Fb1079EA71e0a56F48a2aA51851D8433D0]: https://sepolia.etherscan.io/address/0xaA8E23Fb1079EA71e0a56F48a2aA51851D8433D0#code
[methods/guest/src/bin/balance_of.rs]: ./methods/guest/src/bin/balance_of.rs
[apps/src/bin/publisher.rs]: ./apps/src/bin/publisher.rs