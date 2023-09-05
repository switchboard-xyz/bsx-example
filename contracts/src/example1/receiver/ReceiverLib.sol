//SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

// This prob wont work
import "@pythnetwork/pyth-sdk-solidity/IPyth.sol";
import "@pythnetwork/pyth-sdk-solidity/PythStructs.sol";
import "@chainlink/contracts/src/v0.8/interfaces/AggregatorV3Interface.sol";

library ReceiverLib {
    bytes32 constant DIAMOND_STORAGE_POSITION = keccak256("receiverlib.v1.storage");
    IPyth constant pyth = IPyth(address(0xff1a0f4744e8582DF1aE09D5611b887B6a12925C));


    struct DiamondStorage {
        PythStructs.Price pythPrice;
        int chainlinkPrice;
        uint256 switchboardPrice;
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

    // Switchboard Function will call this function with the feed ids and values
    function callback(
        uint256[] calldata switchboardPrices,
        address[] calldata chainlinkPriceIds,
        bytes32[] calldata pythPriceIds,
        bytes[] calldata pythVaas
    ) internal {
        DiamondStorage storage ds = diamondStorage();
        // https://docs.pyth.network/documentation/pythnet-price-feeds/evm
        // uint fee = pyth.getUpdateFee(pythVaas);
        // pyth.updatePriceFeeds{ value: fee }(pythVaas);
        // PythStructs.Price memory pythPrice = pyth.getPrice(pythPriceIds[i]);
        AggregatorV3Interface clPriceFeed = AggregatorV3Interface(chainlinkPriceIds[0]);
        (, int chainlinkPrice, , , ) = clPriceFeed.latestRoundData();
        ds.switchboardPrice = switchboardPrices[0];
        // ds.pythPrice = pythPrice;
        ds.chainlinkPrice = chainlinkPrice;
    }

    function viewData() internal view returns (uint256 switchboardPrice, int chainlinkPrice, PythStructs.Price memory pythPrice) {
        DiamondStorage storage ds = diamondStorage();
        switchboardPrice = ds.switchboardPrice;
        chainlinkPrice = ds.chainlinkPrice;
        pythPrice = ds.pythPrice;
    }
}
