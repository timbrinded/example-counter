#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
extern crate alloc;

use stylus_sdk::{alloy_primitives::U256, prelude::*, storage::StorageU256};

#[storage]
#[entrypoint]
pub struct Counter {
    number: StorageU256,
}

#[public]
impl Counter {
    pub fn number(&self) -> Result<U256, Vec<u8>> {
        Ok(self.number.get())
    }

    pub fn set_number(&mut self, new_number: U256) -> Result<(), Vec<u8>> {
        self.number.set(new_number);
        Ok(())
    }

    pub fn increment(&mut self) -> Result<(), Vec<u8>> {
        let number = self.number.get();
        self.set_number(number + U256::from(1))?;
        Ok(())
    }
}