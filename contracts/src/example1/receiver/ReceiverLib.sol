//SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

library ReceiverLib {
    bytes32 constant DIAMOND_STORAGE_POSITION = keccak256("receiverlib.v1.storage");


    struct DiamondStorage {
        uint256 twap;
    }

    function diamondStorage()
        internal
        pure
        returns (DiamondStorage storage ds)
    {
        bytes32 position = DIAMOND_STORAGE_POSITION;
        assembly {
            ds.slot := position
        }
    }

    function callback(uint256 twap) internal {
        DiamondStorage storage ds = diamondStorage();
        ds.twap = twap;
    }

    function viewData() internal view returns (uint256 twap) {
        DiamondStorage storage ds = diamondStorage();
        twap = ds.twap;
    }
}
