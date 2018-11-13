const CounterContract = artifacts.require("CounterContract");
const WasmTokenContract = artifacts.require("WasmTokenContract");

module.exports = function(deployer) {
  deployer.deploy(CounterContract);
  deployer.deploy(WasmTokenContract, 100);
}
