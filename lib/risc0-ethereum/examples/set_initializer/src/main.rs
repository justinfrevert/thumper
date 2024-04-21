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
use risc0_zkvm::sha::Digest;

fn main() {
    println!("Participant Set Preparation");
    // let identity_proof_bytes = fs::read("contributor.receipt").unwrap();

    let receipt_dir = "./contributor_receipts";
    let mut receipt_files = fs::read_dir(receipt_dir)
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()
        .unwrap();


    // let receipt_file = "./contributor_receipts/local.receipt/";

    // let receipt_file_bytes = fs::read("./contributor_receipts/local.receipt/");
    // let identity_receipt: Receipt = bincode::deserialize(&receipt_file_bytes).unwrap();

    // let identity_receipt: Receipt = bincode::deserialize(&identity_proof_bytes).unwrap();

    // println!("Received receipt");

    // let env = ExecutorEnv::builder()
    //     .add_assumption(identity_receipt.clone())
    //     .write(&identity_receipt.journal)
    //     .unwrap()
    //     .build()
    //     .unwrap();

    let mut env_builder = ExecutorEnv::builder();

    // env_builder.add_assumption(identity_receipt.clone()).unwrap();
    // env_builder.write(identity_receipt.journal).unwrap();

    env_builder.write(&receipt_files.len());

    for receipt_file in receipt_files {
        let identity_proof_bytes = fs::read(&receipt_file).unwrap();
        let identity_receipt: Receipt = bincode::deserialize(&identity_proof_bytes).unwrap();
        let journal_values: Digest = identity_receipt.journal.decode().expect(
            "Journal output should deserialize into the same types (& order) that it was written",
        );  

        env_builder
            .add_assumption(identity_receipt.clone())
            .write(&journal_values)
            .unwrap();
    }

    // Build the environment
    let env = env_builder.build().unwrap();

    let receipt = default_prover()
        .prove(env, EXPONENTIATE_ELF)
        .unwrap();

    let committed_values: Digest = receipt.journal.decode().expect(
        "Journal output should deserialize into the same types (& order) that it was written",
    );  

    // receipt.verify(EXPONENTIATE_ID).unwrap();

    println!("Done checking proofs for all members... Participant set is initialized.");
}

