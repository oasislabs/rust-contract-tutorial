#![no_std]

extern crate owasm_std;

static COUNTER_KEY: H256 = H256([
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
]);

#[owasm_abi_derive::contract]
trait CounterContract {
    fn constructor(&mut self) {
        let count: u32 = 0;
        owasm_ethereum::set_bytes(&COUNTER_KEY, &count.to_le_bytes());
    }

    #[constant]
    fn getCount(&mut self) -> u32 {
        let mut count_bytes: [u8; 4] = Default::default();
        count_bytes.copy_from_slice(&owasm_ethereum::get_bytes(&COUNTER_KEY).unwrap());
        u32::from_le_bytes(count_bytes)
    }

    fn increment(&mut self) {
        let count = self.getCount() + 1;
        owasm_ethereum::set_bytes(&COUNTER_KEY, &count.to_le_bytes());
    }
}
