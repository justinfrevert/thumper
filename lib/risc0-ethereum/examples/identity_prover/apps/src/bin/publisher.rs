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

// This application demonstrates how to send an off-chain proof request
// to the Bonsai proving service and publish the received proofs directly
// to your deployed app contract.

use alloy_primitives::{address, Address};
use alloy_sol_types::{sol, SolCall, SolInterface};
use anyhow::Result;
// use apps::{BonsaiProver, TxSender};
use clap::Parser;
use erc20_counter_methods::{BALANCE_OF_ID, BALANCE_OF_ELF};
use risc0_ethereum_view_call::{
    config::ETH_SEPOLIA_CHAIN_SPEC, ethereum::EthViewCallEnv, EvmHeader, ViewCall,
};
use risc0_zkvm::serde::to_vec;
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};
use tracing_subscriber::EnvFilter;
use serde::Serialize;
use std::fs;

/// Address of the deployed contract to call the function on. Here: USDT contract on Sepolia
/// Must match the guest code.
const CONTRACT: Address = address!("F66a26e6D7A310bdb8E34fF028568B1D5e59cA43");

sol! {
    /// ERC-20 balance function signature.
    interface IERC20 {
        function balanceOf(address account) external view returns (uint);
        // function commitments() external view returns (uint256[]);
        function commitments() external view returns (bytes32[]);
    }
}

// `ICounter` interface automatically generated via the alloy `sol!` macro.
sol! {
    interface ICounter {
        function increment(bytes calldata journal, bytes32 post_state_digest, bytes calldata seal);
    }
}

/// Arguments of the publisher CLI.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Ethereum chain ID
    #[clap(long)]
    chain_id: u64,

    /// Ethereum Node endpoint.
    #[clap(long, env)]
    eth_wallet_private_key: String,

    /// Ethereum Node endpoint.
    #[clap(long, env)]
    rpc_url: String,

    /// Counter's contract address on Ethereum
    #[clap(long)]
    contract: String,

    /// Account address to read the balance_of on Ethereum
    #[clap(long)]
    account: Address,

    #[clap(long, env)]
    private_secret_key: String
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

    // Get args
    let private_secret_key = args.private_secret_key;
    let account = args.account;

    // let call = IERC20::fakeBalanceOfCall { account };
    // let call = crate::fakeBalanceOfCall { account };
    let call = IERC20::balanceOfCall { account };
    // let call = IERC20::commitmentsCall {};

    // Preflight the view call to construct the input that is required to execute the function in
    // the guest. It also returns the result of the call.
    let (view_call_input, returns) = ViewCall::new(call, CONTRACT).preflight(env)?;
    println!(
        "For block {} `{}` returns: {:?}",
        number,
        IERC20::balanceOfCall::SIGNATURE,
        // IERC20::fakeBalanceOfCall::SIGNATURE,
        // IERC20::commitmentsCall::SIGNATURE,
        returns._0
    );

    println!("With hidden private key as input: {:?}", private_secret_key);

    let env = ExecutorEnv::builder()
        .write(&view_call_input)
        .unwrap()
        .write(&account)
        .unwrap()
        .write(&private_secret_key)
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();
    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove(env, BALANCE_OF_ELF).unwrap();
    receipt.verify(BALANCE_OF_ID);

    println!("Outputting receipt to local.receipt");
    let serialized = bincode::serialize(&receipt).unwrap();
    fs::write("local.receipt", serialized)?;

    // let committed_values: Vec<u8> = receipt.journal.decode().expect(
    //     "Journal output should deserialize into the same types (& order) that it was written",
    // );

    // println!("Journal is: {:?}", committed_values);

    Ok(())
}

pub struct InputBuilder {
    input: Vec<u32>,
}

impl InputBuilder {
    pub fn new() -> Self {
        InputBuilder { input: Vec::new() }
    }

    pub fn write(mut self, input: impl serde::Serialize) -> Result<Self> {
        self.input.extend(to_vec(&input)?);
        Ok(self)
    }

    pub fn bytes(self) -> Vec<u8> {
        bytemuck::cast_slice(&self.input).to_vec()
    }
}
