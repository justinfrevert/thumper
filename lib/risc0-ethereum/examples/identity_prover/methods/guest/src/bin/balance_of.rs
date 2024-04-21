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
use alloy_sol_types::{sol, SolValue};
use risc0_ethereum_view_call::{
    config::ETH_SEPOLIA_CHAIN_SPEC, ethereum::EthViewCallInput, ViewCall,
};
use risc0_zkvm::{guest::env, sha::Digest};
use sha2::{Digest as _, Sha256};

// use k256::{
//     ecdsa::{signature::Verifier, Signature, VerifyingKey},
//     EncodedPoint,
// };

risc0_zkvm::guest::entry!(main);

/// Specify the function to call using the [`sol!`] macro.
/// This parses the Solidity syntax to generate a struct that implements the [SolCall] trait.
/// The struct instantiated with the arguments can then be passed to the [ViewCall] to execute the
/// call. For example:
/// `IERC20::balanceOfCall { account: address!("9737100D2F42a196DE56ED0d1f6fF598a250E7E4") }`
sol! {
    /// ERC-20 balance function signature.
    interface IERC20 {
        function balanceOf(address account) external view returns (uint);
        // function fakeBalanceOf(address account) external view returns (uint);
        function fakeBalanceOf(address account) external view returns (uint);
        // function commitments() external view returns (uint256[]);
        function commitments() external view returns (bytes32[]);
    }
    // function commitments() external view returns (uint256[]);
    // function getCommitments() external view returns (u256[]);

}

/// Address of the deployed contract to call the function on. Here: USDT contract on Sepolia
const CONTRACT: Address = address!("41EE7701040a4206Af38786827E9863838F8D47f");

const PROTOCOL_ID: u8 = 1;

fn main() {
    // Read the input from the guest environment.
    let input: EthViewCallInput = env::read();
    let account: Address = env::read();    
    let private_key: Vec<u8> = env::read();
    // let encoded_verifying_key: EncodedPoint = env::read();
    // let message: Vec<u8> = env::read();
    // let signature: String = env::read();

    // Converts the input into a `ViewCallEnv` for execution. The `with_chain_spec` method is used
    // to specify the chain configuration. It checks that the state matches the state root in the
    // header provided in the input.
    let view_call_env = input.into_env().with_chain_spec(&ETH_SEPOLIA_CHAIN_SPEC);
    // Commit the block hash and number used when deriving `view_call_env` to the journal. (leave for now in order to check the other committed values - the identity commitment)
    // As far as I can tell, this should be fine
    // env::commit_slice(&view_call_env.block_commitment().abi_encode());

    // Execute the view call; it returns the result in the type generated by the `sol!` macro.
    let call = IERC20::balanceOfCall { account };
    let returns = ViewCall::new(call, CONTRACT).execute(view_call_env);
    assert!(returns._0 >= U256::from(1));

    let data = format!("{:?}{}", private_key, PROTOCOL_ID);
    let digest = Sha256::digest(&data.as_bytes());
    let digest = Digest::try_from(digest.as_slice()).unwrap();
    // TODO: We need to prove that we own this private key. 
    // // Ensure privately we own the account we are talking about
    // // Decode the verifying key, message, and signature from the inputs.
    // let (encoded_verifying_key, message, signature): (EncodedPoint, Vec<u8>, Signature) =
    //     env::read();
    // let verifying_key = VerifyingKey::from_encoded_point(&encoded_verifying_key).unwrap();

    // // Verify the signature, panicking if verification fails.
    // verifying_key
    //     .verify(&message, &signature)
    //     .expect("ECDSA signature verification failed");
    
    // let member_digest = returns._0.iter().find(|identity_commitment| {
    //     // TODO: Check that we are a member in this collection
    //     identity_commitment.as_slice() == digest.as_slice()
    //     // env::log(&format!("retrieved identity commitment is: {:?}, reconstructed identity commitment is {:?}", identity_commitment, digest));
    //     // true
    // });

    env::commit(&digest);
}
