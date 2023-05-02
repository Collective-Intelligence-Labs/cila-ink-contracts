#![cfg_attr(not(feature = "std"), no_std)]


pub mod cila;


#[ink::contract]
mod cqrs {

    use ink::prelude::vec::Vec;

    use crate::cila::cila::EventStore;
    
    
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct CQRS {
      
    }

    impl CQRS {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { }
        }

        
        #[ink(message)]
        pub fn dipatch(&mut self, data: Vec<u8>) {
            //TODO:: implement dispatch
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::{*};

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            //let Dispatcher = Eventst::default();
           // assert_eq!(Dispatcher.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            //let mut Dispatcher = Dispatcher::new(false);
            //assert_eq!(Dispatcher.get(), false);
           // Dispatcher.flip();
           // assert_eq!(Dispatcher.get(), true);
        }

        /// Event Store Tests

        #[ink::test]
        fn event_store_initializes(){
            //let mut EventStore = EventStore::new();
            

        }
    }
}
