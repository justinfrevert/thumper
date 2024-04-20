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

1. Start local node with contracts
    a. Follow instructions in lib/risc0-ethereum/examples/erc20-counter/deployment-guide.md to start ganache local node
    a. *New terminal*: cd to lib/risc0-ethereum/examples/erc20-counter
    b. cargo build
    c.  forge script --rpc-url http://localhost:8545 --broadcast script/DeployERC20.s.sol
    d. Follow remaining instructions

1. Start local node with contracts
    a. cd to `contracts`
    b. `anvil`
    c. Separate terminal: `ETH_WALLET_PRIVATE_KEY="$PRIVATE_KEY" forge script --rpc-url http://localhost:8545 --broadcast script/Deploy.s.sol`
2. Query state from the node verifiably
    a. Separate terminal
    b. cd to `lib/risc0-ethereum/examples/erc20$`
    c. `RPC_URL=http://127.0.0.1:8545 RUST_LOG=info cargo run --release`
