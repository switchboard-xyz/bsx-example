//SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import {ReceiverLib} from "./ReceiverLib.sol";
import {AdminLib} from "../admin/AdminLib.sol";
import {ErrorLib} from "../error/ErrorLib.sol";
import "@pythnetwork/pyth-sdk-solidity/PythStructs.sol";

// Get the Switchboard Library - this is the Core Mainnet Deployment, you can swap this for one of the networks below
import {Switchboard} from "@switchboard-xyz/evm.js/contracts/arbitrum/testnet/Switchboard.sol";

contract Receiver {

    function callback(
        uint256[] calldata switchboardPrices,
        address[] calldata chainlinkPriceIds,
        bytes32[] calldata pythPriceIds,
        bytes[] calldata pythVaas
    ) external {
        address functionId = Switchboard.getEncodedFunctionId();
        if (AdminLib.functionId() == address(0)) {
            AdminLib.setFunctionId(functionId);
        }

        // Assert that the sender is switchboard & the correct function id is encoded
        if (functionId != AdminLib.functionId()) {
            revert ErrorLib.InvalidSender(AdminLib.functionId(), functionId);
        }
        ReceiverLib.callback(switchboardPrices, chainlinkPriceIds, pythPriceIds, pythVaas);
    }

    function viewData() external view returns (uint256 switchboardPrice, int chainlinkPrice, PythStructs.Price memory pythPrice) {
        return ReceiverLib.viewData();
    }
}
