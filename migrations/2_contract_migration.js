const CounterContract = artifacts.require("CounterContract");

module.exports = function(deployer) {
  deployer.deploy(CounterContract);
}
