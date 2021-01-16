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
    pub struct Game { }

    impl Game {
        #[ink(constructor)]
        pub fn default() -> Self {
            Game { }
        }

        #[ink(message, payable)]
        pub fn run_game_test(&mut self, program_id: AccountId) -> Result<bool, Error> {
            let env = self.env();
            /*ink_env::debug_println(&format!("caller: {:?}", env.caller()));
            ink_env::debug_println(&format!("transferred balance: {:?}", env.transferred_balance()));
            let gas_left = env.gas_left();
            ink_env::debug_println(&format!(*/
            ink_env::debug_println(&format!("env: {:?}", env));

            //ink_env::debug_println(&format!("calling flip on {:?}", program_id));
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

            let return_value = build_call::<DefaultEnvironment>()
                .callee(program_id) 
                .gas_limit(20035585459) 
                .transferred_value(0)
                .exec_input(
                    ExecutionInput::new(Selector::new([0xDE, 0xAD, 0xBE, 0xFF]))
                )
                .returns::<ReturnType<bool>>()
                .fire();
                //.unwrap();
            
            ink_env::debug_println(&format!("return value {:?}", return_value));
            Ok(true)
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
