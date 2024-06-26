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
//
// SPDX-License-Identifier: Apache-2.0

pragma solidity ^0.8.20;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/console2.sol";

import {zkKYC} from "../contracts/zkkyc.sol";
import {Coordinator} from "../contracts/coordinator.sol";

contract zkKYCDeploy is Script {
    function run() external {
        uint256 deployerKey = uint256(vm.envBytes32("ETH_WALLET_PRIVATE_KEY"));

        vm.startBroadcast(deployerKey);

        zkKYC dummyZkKYC = new zkKYC("ZKKYC", "ZKYC");
        console2.log("Deployed zkKYC dummyzkKYC to", address(dummyZkKYC));
        console2.log("export ZKKYC_ADDRESS=", address(dummyZkKYC));

        Coordinator coordinator = new Coordinator();
        console2.log("Deployed Coordinator to", address(coordinator));
        console2.log("export COORDINATOR_ADDRESS=", address(coordinator));

        vm.stopBroadcast();
    }
}
