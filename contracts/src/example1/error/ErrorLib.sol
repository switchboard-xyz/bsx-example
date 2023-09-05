// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

library ErrorLib {
    error Generic(address account);
    error ACLAdminAlreadyInitialized();
    error ACLNotAdmin(address account);
    error InvalidSender(address functionId, address otherFunctionId);
}
