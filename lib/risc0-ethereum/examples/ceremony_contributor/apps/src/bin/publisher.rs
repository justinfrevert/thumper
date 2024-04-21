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
use alloy_primitives::U256;
use alloy_sol_types::{sol, SolCall, SolInterface};
use anyhow::Result;
use apps::{BonsaiProver, TxSender};
use clap::Parser;
use erc20_counter_methods::{BALANCE_OF_ELF, BALANCE_OF_ID};
use risc0_ethereum_view_call::{
    config::ETH_SEPOLIA_CHAIN_SPEC, ethereum::EthViewCallEnv, EvmHeader, ViewCall,
};
use risc0_zkvm::serde::to_vec;
use risc0_zkvm::{ExecutorEnv};
use risc0_zkvm::Receipt;
use risc0_zkvm::default_prover;
use tracing_subscriber::EnvFilter;
use std::fs;

/// Address of the deployed contract to call the function on. Here: USDT contract on Sepolia
/// Must match the guest code.
const CONTRACT: Address = address!("Ad59e59419e78bD3AE1F6c1350EeF30567D9EA4A");
const PROTOCOL_ID: u8 = 1;

sol! {
    interface IERC20 {
        function commitments() external view returns (bytes32[]);
        function getCommitments() external view returns (bytes32[]);
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
    secret_key: String,
}

fn main() -> Result<()> {
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    // parse the command line arguments
    let args = Args::parse();

    let contribution_receipt_bytes = fs::read("./contributor.receipt").unwrap();
    let contribution_receipt: Receipt = bincode::deserialize(&contribution_receipt_bytes).unwrap();

    // let (current_contribution, order): (Vec<u8>, u64) = receipt.journal.decode().unwrap();
    let compressed_contribution: Vec<u8> = contribution_receipt.journal.decode().unwrap();

    // let current_contribution = G1Projective::deserialize_compressed(&*compressed_contribution).unwrap();

    let env =
        EthViewCallEnv::from_rpc(&args.rpc_url, None)?.with_chain_spec(&ETH_SEPOLIA_CHAIN_SPEC);
    let number = env.header().number();

    let account = args.account;
    let secret_key = args.secret_key;
    let call = IERC20::getCommitmentsCall { };

    // Preflight the view call to construct the input that is required to execute the function in
    // the guest. It also returns the result of the call.
    let (view_call_input, returns) = ViewCall::new(call, CONTRACT).preflight(env)?;
    println!(
        "For block {} `{}` returns: {:?}",
        number,
        IERC20::commitmentsCall::SIGNATURE,
        returns._0
    );

    let env = ExecutorEnv::builder()
        .add_assumption(contribution_receipt.clone())
        .write(&view_call_input)
        .unwrap()
        .write(&account)
        .unwrap()
        .write(&secret_key.as_bytes())
        .unwrap()
        .write(&contribution_receipt.journal)
        .unwrap()
        .write(&compressed_contribution)
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();
    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove(env, BALANCE_OF_ELF).unwrap();
    receipt.verify(BALANCE_OF_ID);

    println!("Outputting receipt to contribution.receipt. This is for you to submit");
    let serialized = bincode::serialize(&receipt).unwrap();
    fs::write("local.receipt", serialized)?;

    Ok(())
}
