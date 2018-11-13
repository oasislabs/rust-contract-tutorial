const CounterContract = artifacts.require("CounterContract");
const Web3 = require("web3");
const web3 = new Web3(CounterContract.web3.currentProvider);

contract("CounterContract", (accounts) => {

  const instance = new web3.eth.Contract(CounterContract.abi, CounterContract.address, {
    from: accounts[0]
  });

  it("should have a count of zero", async () => {
    const count = await instance.methods.getCount().call();

    assert.equal(count, 0);
  });

  it("should increment by one", async () => {
    await instance.methods.increment().send();
    const count = await instance.methods.getCount().call();

    assert.equal(count, 1);
  })
})
