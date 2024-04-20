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
use anyhow::{Context, Result};


fn main() {
    println!(" Participant Set Preparation");

    let identity_proof_bytes = fs::read("contributor.receipt").unwrap();
    // let receipt_dir = "./contributor_receipts";
    // let mut receipt_files = fs::read_dir(receipt_dir)
    //     .unwrap()
    //     .map(|res| res.map(|e| e.path()))
    //     .collect::<Result<Vec<_>, std::io::Error>>()
    //     .unwrap();

    let identity_receipt: Receipt = bincode::deserialize(&identity_proof_bytes).unwrap();

    // println!("Received receipt");

    // let env = ExecutorEnv::builder()
        // .add_assumption(identity_receipt.clone())
    //     .write(&identity_receipt.journal)
    //     .unwrap()
    //     .build()
    //     .unwrap();

    let input = InputBuilder::new()
        .write(identity_receipt)
        .unwrap()
        .write(identity_receipt.journal)
        .unwrap()
        .bytes();
    let (journal, post_state_digest, seal) = BonsaiProver::prove(EXPONENTIATE_ELF, &input)?;

    // let receipt = default_prover()
    //     .prove(env, EXPONENTIATE_ELF)
    //     .unwrap();

    // receipt.verify(EXPONENTIATE_ID).unwrap();

    println!("Done checking proofs for all members... Participant set is initialized.");
}


/// An implementation of a Prover that runs on Bonsai.
pub struct BonsaiProver {}
impl BonsaiProver {
    /// Generates a snark proof as a triplet (`Vec<u8>`, `FixedBytes<32>`,
    /// `Vec<u8>) for the given elf and input.
    pub fn prove(elf: &[u8], input: &[u8]) -> Result<(Vec<u8>, FixedBytes<32>, Vec<u8>)> {
        let client = bonsai_sdk::Client::from_env(risc0_zkvm::VERSION)?;

        // Compute the image_id, then upload the ELF with the image_id as its key.
        let image_id = compute_image_id(elf)?;
        let image_id_hex = image_id.to_string();
        client.upload_img(&image_id_hex, elf.to_vec())?;
        log::info!("Image ID: 0x{}", image_id_hex);

        // Prepare input data and upload it.
        let input_id = client.upload_input(input.to_vec())?;

        // Start a session running the prover.
        let session = client.create_session(image_id_hex, input_id, vec![])?;
        log::info!("Created session: {}", session.uuid);
        let _receipt = loop {
            let res = session.status(&client)?;
            if res.status == "RUNNING" {
                log::info!(
                    "Current status: {} - state: {} - continue polling...",
                    res.status,
                    res.state.unwrap_or_default()
                );
                std::thread::sleep(Duration::from_secs(15));
                continue;
            }
            if res.status == "SUCCEEDED" {
                // Download the receipt, containing the output.
                let receipt_url = res
                    .receipt_url
                    .context("API error, missing receipt on completed session")?;

                let receipt_buf = client.download(&receipt_url)?;
                let receipt: Receipt = bincode::deserialize(&receipt_buf)?;

                break receipt;
            }

            panic!(
                "Workflow exited: {} - | err: {}",
                res.status,
                res.error_msg.unwrap_or_default()
            );
        };

        // Fetch the snark.
        let snark_session = client.create_snark(session.uuid)?;
        log::info!("Created snark session: {}", snark_session.uuid);
        let snark_receipt = loop {
            let res = snark_session.status(&client)?;
            match res.status.as_str() {
                "RUNNING" => {
                    log::info!("Current status: {} - continue polling...", res.status,);
                    std::thread::sleep(Duration::from_secs(15));
                    continue;
                }
                "SUCCEEDED" => {
                    break res.output.context("No snark generated :(")?;
                }
                _ => {
                    panic!(
                        "Workflow exited: {} err: {}",
                        res.status,
                        res.error_msg.unwrap_or_default()
                    );
                }
            }
        };

        let snark = snark_receipt.snark;
        log::debug!("Snark proof!: {snark:?}");

        let seal = Seal::abi_encode(snark).context("Read seal")?;
        let post_state_digest: FixedBytes<32> = snark_receipt
            .post_state_digest
            .as_slice()
            .try_into()
            .context("Read post_state_digest")?;
        let journal = snark_receipt.journal;

        Ok((journal, post_state_digest, seal))
    }
}
