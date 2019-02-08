static COUNTER_KEY: [u8; 32] = [
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

#[owasm_abi_derive::contract]
trait CounterContract {
    fn constructor(&mut self) {
        owasm_ethereum::write(&COUNTER_KEY.into(), &U256::zero().into());
    }

    #[constant]
    fn getCount(&mut self) -> U256 {
        println!("Getting count");
        U256::from(&owasm_ethereum::read(&COUNTER_KEY.into()))
    }

    fn increment(&mut self) {
        println!("Incrementing count");
        owasm_ethereum::write(&COUNTER_KEY.into(), &(self.getCount() + 1).into());
    }
}
