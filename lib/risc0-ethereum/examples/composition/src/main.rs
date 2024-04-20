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

use composition_example_methods::{EXPONENTIATE_ELF, EXPONENTIATE_ID};
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};
use std::fs;

fn main() {
    println!("Beginning Participant Set Preparation");

    let identity_proof_bytes = fs::read("contributor.receipt").unwrap();
    let identity_receipt: Receipt = bincode::deserialize(&identity_proof_bytes).unwrap();

    println!("Received receipt");

    let env = ExecutorEnv::builder()
        .add_assumption(identity_receipt.clone())
        // Manually add...
        // .write(&1)
        .write(&identity_receipt.journal)
        .unwrap()
        .build()
        .unwrap();

    let receipt = default_prover()
        .prove(env, EXPONENTIATE_ELF)
        .unwrap();
        // .receipt;

    receipt.verify(EXPONENTIATE_ID).unwrap();

    println!("Done checking proofs for all members... Participant set is initialized.");
}
