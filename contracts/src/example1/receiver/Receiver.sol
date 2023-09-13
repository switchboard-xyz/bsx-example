pragma solidity ^0.8.9;

import {ReceiverLib} from "./ReceiverLib.sol";
import {AdminLib} from "../admin/AdminLib.sol";
import {ErrorLib} from "../error/ErrorLib.sol";

import {Switchboard} from "@switchboard-xyz/evm.js/contracts/arbitrum/testnet/Switchboard.sol";

contract Receiver {

    function callback(uint256 twap) external {
        address functionId = Switchboard.getEncodedFunctionId();
        if (AdminLib.functionId() == address(0)) {
            AdminLib.setFunctionId(functionId);
        }

        // Assert that the sender is switchboard & the correct function id is encoded
        if (functionId != AdminLib.functionId()) {
            revert ErrorLib.InvalidSender(AdminLib.functionId(), functionId);
        }
        ReceiverLib.callback(twap);
    }

    function viewData() external view returns (uint256 twap) {
        return ReceiverLib.viewData();
    }
}
