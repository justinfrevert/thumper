// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {IERC20} from "openzeppelin-contracts/token/ERC20/IERC20.sol";

contract ERC20 is IERC20 {
    uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;
    // mapping(address => uint256) public fakeBalanceOf;
    // uint256[] public commitmentList;
    bytes32[] public commitmentList;

    mapping(address => mapping(address => uint256)) public allowance;
    string public name;
    string public symbol;
    uint8 public decimals;

    constructor(string memory _name, string memory _symbol, uint8 _decimals) {
        name = _name;
        symbol = _symbol;
        decimals = _decimals;
    }

    // function identityCommitments(uint protocolId) external view returns (uint256[] memory) {
    function identityCommitments(uint protocolId) external view returns (bytes32[] memory) {
        return commitmentList;
    }

    function transfer(address recipient, uint256 amount) external returns (bool) {
        balanceOf[msg.sender] -= amount;
        balanceOf[recipient] += amount;
        emit Transfer(msg.sender, recipient, amount);
        return true;
    }

    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        emit Approval(msg.sender, spender, amount);
        return true;
    }

    function transferFrom(address sender, address recipient, uint256 amount) external returns (bool) {
        allowance[sender][msg.sender] -= amount;
        balanceOf[sender] -= amount;
        balanceOf[recipient] += amount;
        emit Transfer(sender, recipient, amount);
        return true;
    }

    // function _mint(address to, uint256 amount) internal {
    //     // balanceOf[to] += amount;
    //     // fakeBalanceOf[to] += amount;
    //     // commitments[0] = amount;
    //     commitmentList.push(amount);

    //     // totalSupply += amount;
    //     emit Transfer(address(0), to, amount);
    // }
    function mint(bytes32 commitment) external {
        commitmentList.push(commitment);
    }

    function _burn(address from, uint256 amount) internal {
        balanceOf[from] -= amount;
        totalSupply -= amount;
        emit Transfer(from, address(0), amount);
    }

    // function mint(address to, uint256 amount) external {
    //     _mint(to, amount);
    // }

    function burn(address from, uint256 amount) external {
        _burn(from, amount);
    }
}
