import { ethers } from "hardhat";
import moment from "moment-timezone";
import Big from "big.js";
const BigNumber = require("bignumber.js");

(async function main() {
  const exampleProgram = process.env.EXAMPLE_PROGRAM ?? "";

  const divisor = new BigNumber("100000000");

  if (!exampleProgram) {
    throw new Error(
      "Please set the diamond address with: export EXAMPLE_PROGRAM=..."
    );
  }

  const push = await ethers.getContractAt("Receiver", exampleProgram);
  const p = await push.deployed();

  const twap = await p.viewData();
  console.log("============");
  console.log(`TWAP: ${new Big(twap.toString()).div(divisor).toString()}`);
  console.log("============");
})();
