// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use alloy_primitives::{address, Address};
use alloy_sol_types::{sol, SolCall, SolValue};
use anyhow::{Context, Result};
use clap::Parser;
use methods::METHOD_ELF;
use risc0_ethereum_view_call::{
    config::ETH_SEPOLIA_CHAIN_SPEC, ethereum::EthViewCallEnv, EvmHeader, ViewCall,
};
use risc0_zkvm::{default_executor, ExecutorEnv};
use tracing_subscriber::EnvFilter;

/// Address of the custom contract on the given EVM network
const CONTRACT: Address = address!("e7f1725e7734ce288f8367e1bb143e90bb3f0512");
/// Function to call
// const CALL: ICommitmentStorage::getCommitments = ICommitmentStorage::getCommitments { index: 0 };
const CALL: ICommitmentStorage::getCommitments = ICommitmentStorage::getCommitments { index: 0 };
/// Caller address
const CALLER: Address = address!("f39Fd6e51aad88F6F4ce6aB8827279cffFb92266");

sol! {
    interface ICommitmentStorage {
        // function getCommitment(uint index) external view returns (bytes32[]);
        // TODO: Can be switched out with something like a merkle root later
        // function getCommitment(uint index) external view returns (bytes32[]);
        // function getCommitments() external view returns (uint256);
        function getCommitments() external view returns (bytes32[]);
    }
}

/// Simple program to show the use of Ethereum contract data inside the guest.
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    /// URL of the RPC endpoint
    #[arg(short, long, env = "RPC_URL")]
    rpc_url: String,
}

fn main() -> Result<()> {
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    // parse the command line arguments
    let args = Args::parse();

    // Create a view call environment from an RPC endpoint and a block number. If no block number is
    // provided, the latest block is used. The `with_chain_spec` method is used to specify the
    // chain configuration.
    let env =
        EthViewCallEnv::from_rpc(&args.rpc_url, None)?.with_chain_spec(&ETH_SEPOLIA_CHAIN_SPEC);
    let number = env.header().number();
    let commitment = env.block_commitment();

    // Preflight the view call to construct the input that is required to execute the function in
    // the guest. It also returns the result of the call.
    let (input, returns) = ViewCall::new(CALL, CONTRACT)
        // .with_caller(CALLER)
        .preflight(env)?;
    println!(
        "For block {} `{}` returns: {}",
        number,
        IERC20::balanceOfCall::SIGNATURE,
        returns._0
    );

    println!("Running the guest with the constructed input:");
    let session_info = {
        let env = ExecutorEnv::builder()
            .write(&input)
            .unwrap()
            .build()
            .context("Failed to build exec env")?;
        let exec = default_executor();
        exec.execute(env, METHOD_ELF)
            .context("failed to run executor")?
    };

    // extract the proof from the session info and validate it
    let bytes = session_info.journal.as_ref();
    assert_eq!(&bytes[..64], &commitment.abi_encode());

    Ok(())
}
