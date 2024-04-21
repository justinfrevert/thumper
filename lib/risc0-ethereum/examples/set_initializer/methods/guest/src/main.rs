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

use erc20_counter_methods::BALANCE_OF_ID;
use risc0_zkvm::{guest::env, serde};
use risc0_zkvm::sha::Digest;
use crate::serde::from_slice;

fn main() {
    // let contributors_count: u8 = env::read();
    // let mut participants = vec![];

    // // let mut single_com: Digest = Default::default();

    // for i in 0..contributors_count {
    //     let contributors_journal: Vec<u8> = env::read();
    //     env::verify(BALANCE_OF_ID, &contributors_journal).unwrap();
    //     // let recreated_journal = Journal::new(contributors_journal);
    //     // let identity_commitment: Digest = recreated_journal.decode();
    //     let identity_commitment: Digest = from_slice(&contributors_journal).unwrap();
    //     // env::commit(identity_commitment);
    //     participants.push(identity_commitment);
    //     // single_com = identity_commitment;
    // }

    let contributors_journal_value: Digest = env::read();
    env::verify(BALANCE_OF_ID, &serde::to_vec(&contributors_journal_value).unwrap()).unwrap();

    // Commit to set
    // env::commit(&single_com);
}