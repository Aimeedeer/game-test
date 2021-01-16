#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;

#[ink::contract]
mod game {
    #[cfg(not(feature = "ink-as-dependency"))]
    use alloc::format;
    use ink_env::call::Selector;
    use ink_env::call::build_call;
    use ink_env::DefaultEnvironment;
    use ink_env::call::ExecutionInput;
    use ink_env::call::utils::ReturnType;
    use core::convert::TryFrom;
    

    #[derive(Debug, scale::Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))] 
    pub enum Error {
        ProgramNotExists,
    }

    #[ink(storage)]
    pub struct Game {
        value: AccountId,
    }

    impl Game {
        #[ink(constructor)]
        pub fn default() -> Self {
            let program_id = "4cfac7f74c6233449b5e54ba070231dd94c71b89505482cd910000656258d3ed";
            let program_id = hex::decode(program_id).unwrap();
            let program_id = AccountId::try_from(&program_id[..]).unwrap();
            //            ink_env::debug_println(&format!("AccountId {:?}", program_id));

            Game {           
                value: program_id,
            }
        }

        #[ink(message, payable)]
        pub fn run_game_test(&mut self, program_id: AccountId) -> Result<bool, Error> {
            let caller = self.env().caller();
            ink_env::debug_println(&format!("caller: {:?}", caller));
            ink_env::debug_println(&format!("calling flip on {:?}", program_id));
/*
            let return_value: () = build_call::<DefaultEnvironment>()
                .callee(program_id) 
                .gas_limit(50)
                .transferred_value(10)
                .exec_input(
                    ExecutionInput::new(Selector::new([0xDE, 0xAD, 0xBE, 0xEF]))
                )
                .returns::<ReturnType<()>>()
                .fire()
                .unwrap();

            */
            ink_env::debug_println(&format!("calling get on {:?}", program_id));

            let return_value: bool = build_call::<DefaultEnvironment>()
                .callee(program_id) 
                .gas_limit(50)
                .transferred_value(10)
                .exec_input(
                    ExecutionInput::new(Selector::new([0xDE, 0xAD, 0xBE, 0xFF]))
                )
                .returns::<ReturnType<bool>>()
                .fire()
                .unwrap();
            
            ink_env::debug_println(&format!("return value {}", return_value));
            Ok(return_value)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn default_works() {

        }
    }
}
