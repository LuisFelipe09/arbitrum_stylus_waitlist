//! Implements a hello-world example for Arbitrum Stylus, providing a Solidity ABI-equivalent
//! Rust implementation of the Counter contract example provided by Foundry.
//! Warning: this code is a template only and has not been audited.
//! ```
//! contract Counter {
//!     map(address=>uint256) public include;
//!     function setInclude(address from) public {
//!         include[from] = 1;
//!     }
//!     function delete(address from) public {
//!         include[from] = 0;
//!     }
//! }
//! ```

// Only run this as a WASM if the export-abi feature is not set.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

/// Initializes a custom, global allocator for Rust programs compiled to WASM.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Import the Stylus SDK along with alloy primitive types for use in our program.
use stylus_sdk::{alloy_primitives::{U256, Address}, prelude::*};


sol_storage! {
    #[entrypoint]
    pub struct WaitList {
        mapping(address => uint256) wait_list;
    }
}


#[external]
impl WaitList {
    /// Gets the number from storage.
    pub fn include(&mut self, from: Address) -> Result<U256, Vec<u8>> {
        let  sender_wait_list = self.wait_list.setter(from);
        Ok(sender_wait_list.get())
    }

    /// Sets a number in storage to a user-specified value.
    pub fn set_include(&mut self, from: Address) -> Result<(), Vec<u8>> {
        let mut sender_wait_list = self.wait_list.setter(from);
        let value :U256 = U256::from(1);
        sender_wait_list.set(value);
        Ok(())
    }

    /// Increments number and updates it values in storage.
    pub fn delete(&mut self, from: Address) -> Result<(), Vec<u8>> {
        let mut sender_wait_list = self.wait_list.setter(from);
        let value :U256 = U256::from(0);
        sender_wait_list.set(value);
        Ok(())
    }
}
