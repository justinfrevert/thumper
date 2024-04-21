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

#![allow(unused_doc_comments)]
#![no_main]

use alloy_primitives::{address, Address, U256};
use alloy_primitives::FixedBytes;
use alloy_sol_types::{sol, SolValue};
use risc0_ethereum_view_call::{
    config::ETH_SEPOLIA_CHAIN_SPEC, ethereum::EthViewCallInput, ViewCall,
};
use risc0_zkvm::guest::env;
use risc0_zkvm::sha::Digest;
use ark_bls12_381::{Fr, G1Projective};
use ark_ec::Group;

use risc0_zkvm_platform::syscall::nr::SYS_RANDOM;
use ark_serialize::{CanonicalSerialize, CanonicalDeserialize};
use sha2::{Digest as _, Sha256};

risc0_zkvm::guest::entry!(main);

sol! {
    interface IERC20 {
        function commitments() external view returns (bytes32[]);
        function getCommitments() external view returns (bytes32[]);
        
    }
}

const CONTRACT: Address = address!("Ad59e59419e78bD3AE1F6c1350EeF30567D9EA4A");
const PROTOCOL_ID: u8 = 1;

fn main() {
    // Read the input from the guest environment.
    let input: EthViewCallInput = env::read();
    let private_key: Address = env::read();

    // Converts the input into a `ViewCallEnv` for execution. The `with_chain_spec` method is used
    // to specify the chain configuration. It checks that the state matches the state root in the
    // header provided in the input.
    let view_call_env = input.into_env().with_chain_spec(&ETH_SEPOLIA_CHAIN_SPEC);
    // Commit the block hash and number used when deriving `view_call_env` to the journal.
    env::commit_slice(&view_call_env.block_commitment().abi_encode());

    let call = IERC20::getCommitmentsCall { };
    let returns = ViewCall::new(call, CONTRACT).execute(view_call_env);

    // Recreate an identity commitment from our key, for the given protocol/ceremony
    let data = format!("{:?}{}", private_key, PROTOCOL_ID);
    let digest = Sha256::digest(&data.as_bytes());
    let digest = Digest::try_from(digest.as_slice()).unwrap();

    // Prove we have the key for the chosen initator(just the first key)
    // assert!(&returns._0[0] == digest);

    let mut rand_bytes = [0_u32; 5];
    let randoms = env::syscall(SYS_RANDOM, &[], rand_bytes.as_mut_slice());

    let mut contribution = G1Projective::generator();

    for i in rand_bytes {
        let scalar = Fr::from(i);
        contribution = G1Projective::generator() * scalar;
    }

    let mut compressed_bytes = Vec::new();
    contribution.serialize_compressed(&mut compressed_bytes).unwrap();
    // We commit to the bytes of the current contribution, as well as a number indicating the index of the next participant.
    // The next participant will need to prove that they are at the given index of the participants in order to generate a proof
    // env::commit(&(compressed_bytes, 1));
    env::commit(&compressed_bytes);
}

