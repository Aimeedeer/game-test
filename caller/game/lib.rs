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
    

    #[ink(storage)]
    pub struct Game { }

    impl Game {
        #[ink(constructor)]
        pub fn default() -> Self {
            Game { }
        }

        /*#[ink(message)]
        pub fn run_game_test(&mut self) {
            //ink_env::debug_println(&format!("env: {:?}", self.env()));
            ink_env::debug_println(&format!("weight_to_fee(gas_left): {}", self.env().weight_to_fee(self.env().gas_left() as _)));
        }*/

        //#[ink(message, payable)]
        //pub fn log_env(&mut self) {
            //ink_env::debug_println(&format!("account_id: {:?}", self.env().account_id()));
            //ink_env::debug_println(&format!("balance: {}", self.env().balance()));
            //ink_env::debug_println(&format!("caller: {:?}", self.env().caller()));
            //ink_env::debug_println(&format!("transferred_balance: {}", self.env().transferred_balance()));
            //ink_env::debug_println(&format!("block_timestamp: {}", self.env().block_timestamp()));
            //ink_env::debug_println(&format!("rent_allowance: {}", self.env().rent_allowance()));
            //ink_env::debug_println(&format!("minimum_balance: {}", self.env().minimum_balance()));
            //ink_env::debug_println(&format!("tombstone_deposit: {}", self.env().tombstone_deposit()));

            //ink_env::debug_println(&format!("gas_left: {}", self.env().gas_left()));
            //ink_env::debug_println(&format!("weight_to_fee(gas_left): {}", self.env().weight_to_fee(self.env().gas_left() as _)));
        //}

        #[ink(message, payable)]
        pub fn run_game_test(&mut self, program_id: AccountId) {
            /*ink_env::debug_println(&format!("caller: {:?}", env.caller()));
            ink_env::debug_println(&format!("transferred balance: {:?}", env.transferred_balance()));
            let gas_left = env.gas_left();
            ink_env::debug_println(&format!(*/
            /*ink_env::debug_println(&format!("env: {:?}", self.env()));
            ink_env::debug_println(&format!("weight_to_fee(gas_left): {}", self.env().weight_to_fee(self.env().gas_left() as _)));*/

            /*ink_env::debug_println(&format!("account_id: {:?}", self.env().account_id()));
            ink_env::debug_println(&format!("balance: {}", self.env().balance()));
            ink_env::debug_println(&format!("caller: {:?}", self.env().caller()));
            ink_env::debug_println(&format!("gas_left: {}", self.env().gas_left()));
            ink_env::debug_println(&format!("transferred_balance: {}", self.env().transferred_balance()));
            //ink_env::debug_println(&format!("weight_to_fee(gas_left): {}", self.env().weight_to_fee(self.env().gas_left() as _)));
            ink_env::debug_println(&format!("block_timestamp: {}", self.env().block_timestamp()));
            ink_env::debug_println(&format!("rent_allowance: {}", self.env().rent_allowance()));
            ink_env::debug_println(&format!("minimum_balance: {}", self.env().minimum_balance()));
            ink_env::debug_println(&format!("tombstone_deposit: {}", self.env().tombstone_deposit()));*/

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
                .exec_input(
                    ExecutionInput::new(Selector::new([0xDE, 0xAD, 0xBE, 0xFF]))
                )
                .returns::<ReturnType<bool>>()
                .fire();
            
            ink_env::debug_println(&format!("return value {:?}", return_value));
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
