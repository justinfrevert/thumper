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

fn main() {
    // Fake contributors journal, for now, to test verification
    // let contributors_count: u8 = env::read();

    // // let mut new_participant_set: Vec<Vec<u8>> = vec![];

    // for i in 0..contributors_count {
    //     let contributors_journal: Vec<u8> = env::read();
    //     env::verify(BALANCE_OF_ID, &contributors_journal).unwrap();

    //     // TODO: Get Identity Commitment out of journal
    // }
    let contributors_journal: Vec<u8> = env::read();
    env::verify(BALANCE_OF_ID, &contributors_journal).unwrap();

    // Commit to set
    // env::commit(&new_participant_set);
}