# Thumper

<img src="./images/thumper3.webp" alt="Thumper Image" width="500" height="400"/>

Like the [KZG summoning ceremony]([url](https://ceremony.ethereum.org/)), this project also focuses on *summoning randomness*.

*In the vast expanse of the known universe, there exists a desert planet called Arrakis, more commonly known as Dune. Its landscape is a sprawling sea of sand and home to the Fremen, a tribe of hardy people who have adapted to the harsh environment. Within the sands live the colossal sandworms, creatures of the deep desert, and worshipped by the Fremen as the keepers of Arrakis.*

*The Fremen have mastered the art of summoning these behemoths through a device known as a 'thumper'. When activated, the Fremen remains at bay. The thumper pounds the desert floor with a rhythmic, and steady cadence, each beat a call through the sands. The sandworms, attuned to the vibrations, are drawn to the surface, interpreting the rhythm as a challenge to their domain. Each invocation of the thumper must be precise, its rhythm unerring, for the sandworms are not easily fooled, and the penalty for a misstep is dire.*


State Call
- In `lib/risc0-ethereum/examples/erc20`
- RPC_URL=https://eth-sepolia.g.alchemy.com/v2/{API_KEY} RUST_LOG=info cargo run --release


Overall Structure
- Contracts - The onchain coordinator/identity contracts
    - coordinator - verifies proofs of `identity-aggregator`, accepting private sets of ceremony participants to a given protocol, establishing the order of contributions
- offchain - offchain zkvm hosts and guests
    - identity-attestor - Reads an onchain identity and creates a commitment to it that is specific to a new protocol
    - identity-aggregator - verifies multiple identities in order to prepare them to register as a set of participants
    - ceremony-initializer - Creates the first ceremony contribution by privately verifying the participant is the correct member of the set
    - ceremony-contributor - Creates n+1th contributions to the ceremony by privately proving the participants i the correct member of the set, receiving previous contributions and adding to it.

Goals
- Non-Centralized Coordinator
    via your choice of
        - Onchain coordinator
        - ZKVM coordinator
- EVM-friendly Sybil Resistance through private-but-proven KYC'ed identities
- KZG and then later Groth16

# Running
## Deploy on a local network

Perform the following in `lib/risc0-ethereum/examples/identity_prover`

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
cast send --private-key $ETH_WALLET_PRIVATE_KEY --rpc-url http://localhost:8545 $COORDINATOR_ADDRESS 'updateIdentityCommitments(uint256,bytes32[])' 1 "[0x994368d0308f9dcdc1f30a18d8fe2c371a8eb199c64f21fdfe28b469cd8ee97d, 0x1eaa1e5f20ccc915f7190ff9e5126e09870ec7bab852cf9d801b1c01641139af]"

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

1. Query the state:

    ```bash
    cast call --rpc-url http://localhost:8545 ${ZKKYC_ADDRESS:?} 'get()(uint256)'
    ```

2. Publish a new state/get a view call + identity proof for the identity guest program
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
    cast call --rpc-url http://localhost:8545 ${ZKKYC_ADDRESS:?} 'get()(uint256)'
    ```

4. Publish a new state/get a view call + perform the initial ceremony contribution
In a *separate terminal*, ensure you have $COORDINATOR_ADDRESS set to the coordinator contract we deployed earlier(Step 4.), and cd to `/lib/risc0-ethereum/examples/ceremony_initiator`

    ```bash
    cargo run --bin publisher -- \
        --chain-id=1337 \
        --rpc-url=http://localhost:8545 \
        --contract=${COORDINATOR_ADDRESS:?} \
        --account=0x83bF19D749cb807c19B4a6dF8e30fed56E158DD7 \
        --secret-key=0x447db9e9f28d0be24e439f4c5437c3321884ba000dddc3949b0dbb4f12380a6b
    ```

    and for the second account we minted an "identity" for
    ```bash
    cargo run --bin publisher -- \
        --chain-id=1337 \
        --rpc-url=http://localhost:8545 \
        --contract=${COORDINATOR_ADDRESS:?} \
        --account=0xa527546eBF9faa960C7f287561a1ECE8298beB15 \
        --secret-key=0xf3d540f79c72415e0d5eebb2a23bf7f87613ee6b83ad48d3ec102e614e656aa2
    ```


# TODOS:
- Prove you own the account inside of the guest which queries the zkkyc contract