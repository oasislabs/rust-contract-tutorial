#![no_std]

extern crate owasm_std;

use owasm_std::logger::debug;

static COUNTER_KEY: H256 = H256([
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
]);

#[owasm_abi_derive::contract]
trait CounterContract {
    fn constructor(&mut self) {
        owasm_ethereum::write(&COUNTER_KEY, &U256::zero().into());
    }

    #[constant]
    fn getCount(&mut self) -> U256 {
        debug("Getting count");
        U256::from(&owasm_ethereum::read(&COUNTER_KEY))
    }

    fn increment(&mut self) {
        debug("Incrementing count");
        owasm_ethereum::write(&COUNTER_KEY, &(self.getCount() + 1).into());
    }
}
