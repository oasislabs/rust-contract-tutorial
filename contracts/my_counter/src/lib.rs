#![allow(non_snake_case)]

use oasis_std::prelude::*;

static COUNTER_KEY: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
];

#[contract]
trait CounterContract {
    fn constructor(&mut self) {
        let count: u32 = 0;
        set_bytes(&COUNTER_KEY.into(), &count.to_le_bytes()).unwrap();
    }

    #[constant]
    fn getCount(&mut self) -> u32 {
        let mut count_bytes = [0u8, 0, 0, 0];
        count_bytes.copy_from_slice(&get_bytes(&COUNTER_KEY.into()).unwrap());
        u32::from_le_bytes(count_bytes)
    }

    fn increment(&mut self) {
        let count = self.getCount() + 1;
        set_bytes(&COUNTER_KEY.into(), &count.to_le_bytes()).unwrap();
    }
}
