// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Coordinator {
    // Mapping of protocol ID to array of identity commitments. Protocol id is unique to one protocol's ceremony, and the identity commitments are the private participants
    mapping(uint256 => bytes32[]) public identityCommitments;
    bytes32[] public commitments;

    // Event emitted when identity commitments are updated
    event IdentityCommitmentsUpdated(uint256 indexed protocolId, bytes32[] identityCommitments);

    // This is meant to be called by the preparer of the proof containing the participants set. Eventually, it should require a proof verification in order to be updated. The structure of this is heavily TBD
    function updateIdentityCommitments(uint256 protocolId, bytes32[] memory newCommitments) external {
        // Update the identity commitments for the given protocol ID
        identityCommitments[protocolId] = newCommitments;
        commitments = newCommitments;
        
        // Emit an event to log the update
        emit IdentityCommitmentsUpdated(protocolId, newCommitments);
    }

    // // For local testing: add one identity commitment
    // function addIdentityCommitment(bytes32 commitment) external {
    //     // Update the identity commitments for the given protocol ID
    //     identityCommitments[protocolId].push(commitment);

    //     // Emit an event to log the update
    //     emit IdentityCommitmentsUpdated(protocolId, identityCommitments[protocolId]);
    // }

    function getCommitments() public view returns (bytes32[] memory) {
        return commitments;
    }
}
