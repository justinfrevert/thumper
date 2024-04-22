# Thumper

<img src="./images/thumper3.webp" alt="Thumper Image" width="500" height="400"/>

Like the [KZG Summoning Ceremony](https://ceremony.ethereum.org/) this project also focuses on *summoning randomness*.

*In the vast expanse of the known universe, there exists a desert planet called Arrakis, more commonly known as Dune. Its landscape is a sprawling sea of sand and home to the Fremen, a tribe of hardy people who have adapted to the harsh environment. Within the sands live the colossal sandworms, creatures of the deep desert, and worshipped by the Fremen as the keepers of Arrakis.*

*The Fremen have mastered the art of summoning these behemoths through a device known as a 'thumper'. When activated, The thumper pounds the desert floor with a rhythmic, and steady cadence, each beat a call through the sands. The sandworms, attuned to the vibrations, are drawn to the surface, interpreting the rhythm as a challenge to their domain. Each invocation of the thumper must be precise, its rhythm unerring, for the sandworms are not easily fooled, and the penalty for a misstep is dire.*

*Fremen have a concept describing the oneness of a community, and is associated with spiritual and communal ceremonies. That term is known as [Tau]([url](https://dune.fandom.com/wiki/Tau)). Violating Tau results in expulsion from a community.*

## Introduction
This project attempts to implement a ZKVM-based trusted setup ceremony, utilizing contributor identity privacy, all secured by ZKKYC.

## Overall Structure
Most of the project was adapted from risc-ethereum example boilerplate, so project code currently still sits in `./lib/risc0-ethereum`
```
thumper
│
└───lib/risc0-ethereum/examples # Project directory
│   │   ceremony_contributor # Locally-run ZKVM program that checks whether a preimage for an identity commitment is ready to participate, if so, it contributes randomness
│   │   ceremony_initiator # Locally-run ZKVM program that checks whether a preimage for an identity commitment is the initiator, if so, it initiates the contributions
│   │   identity_prover # Locally-run ZKVM program which checks whether the account owns a ZKKYC'ed identity, and if so, creates a ceremony-specific (private)identity commitment
│       └───contracts
│           │   zkkyc.sol. # A non-verifying version of the zkkyc which exists for us to make requests to in order to check that the view call can correctly retrieve zkkyc tokens and interpret in the guest
│           │   coordinator.sol # A contract meant to house ceremony coordination logic. Contains the set of participants(list ceremony-specific identity commitments)
│   │   set_initializer # Locally-run ZKVM program(meant to be Bonsai) which aggregates some amount of identity proofs and forms a set of contributors for a ceremony to put onchain
└───offchain # UNUSED
└───contracts # UNUSED
```

## Status and TODOS
Unfortunately, I did find as much time as I would have liked to spend on this hackathon. See the status of various components at the time of submission:
- Identity Prover:
   * Need to verify ownership of the account id used for ZKKYC retrieval, based on the priavate key we are passing
- Ceremony Contributor:
   * Need to verify the receipt of the previous contribution
* Set initializer:
    * Need to get the proof composition working. Need to use Bonsai to send the proof to the contract
- ZKKYC Contract
   * Need to get the original ZKKYC contract integrated in a way this project can work with it

## Goals
- Non-Centralized Coordinator
    via your choice of
        - Onchain coordinator
        - ZKVM coordinator
- EVM-friendly Sybil Resistance through private-but-proven KYC'ed identities
- KZG and then later Groth16
- Multiple contributions per participant-step, and random selection out of previous contributor's contributions.
- Commitment to set of contributors instead of list

# Running
## Deploy on a local network

Steps begin with someone proving privately that they have a zkkyc'ed identity in `identity_prover`. Perform the following in `lib/risc0-ethereum/examples/identity_prover`

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
    ```

4. Deploy the dummy zkkyc contract, as well as the coordinator contract.
    ```
    forge script --rpc-url http://localhost:8545 --broadcast script/DeployProject.s.sol
    ```
    Save the `ERC721 ZKKYC` and `COORDINATOR_ADDRESS` contract addresses to an env variable(see logs near top of output)
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

Using the first identity specifically, as it is the initator:
    ```bash
    cargo run --bin publisher -- \
        --chain-id=1337 \
        --rpc-url=http://localhost:8545 \
        --contract=${COORDINATOR_ADDRESS:?} \
        --account=0x83bF19D749cb807c19B4a6dF8e30fed56E158DD7 \
        --secret-key=0x447db9e9f28d0be24e439f4c5437c3321884ba000dddc3949b0dbb4f12380a6b
    ```

Note the contribution.receipt file, and place it into `lib/risc0-ethereum/examples/ceremony_contributor`


5. Publish a new state/get a view call + perform the ceremony contribution
In a *separate terminal*, ensure you have $COORDINATOR_ADDRESS set to the coordinator contract we deployed earlier(Step 4.), and cd to `lib/risc0-ethereum/examples/ceremony_contributor`

Using the second identity:
```bash
cargo run --bin publisher -- \
    --chain-id=1337 \
    --rpc-url=http://localhost:8545 \
    --contract=${COORDINATOR_ADDRESS:?} \
    --account=0xa527546eBF9faa960C7f287561a1ECE8298beB15 \
    --secret-key=0xf3d540f79c72415e0d5eebb2a23bf7f87613ee6b83ad48d3ec102e614e656aa2
```


