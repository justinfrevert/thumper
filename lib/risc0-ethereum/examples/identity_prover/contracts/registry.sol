// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Coordinator {
    // Mapping of protocol ID to array of identity commitments. Protocol id is unique to one protocol's ceremony, and the identity commitments are the private participants
    mapping(uint256 => bytes32[]) public identityCommitments;

    // Event emitted when identity commitments are updated
    event IdentityCommitmentsUpdated(uint256 indexed protocolId, bytes32[] identityCommitments);

    // Function to update identity commitments for a given protocol ID
    function updateIdentityCommitments(uint256 protocolId, bytes32[] memory commitments) external {
        // Update the identity commitments for the given protocol ID
        identityCommitments[protocolId] = commitments;

        // Emit an event to log the update
        emit IdentityCommitmentsUpdated(protocolId, commitments);
    }
}
