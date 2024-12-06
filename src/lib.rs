#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod swapx {
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct SwapX {
        fee_percentage: u128,
        protocol_fund: AccountId,
        liquidity_pool: Mapping<AccountId, u128>,
        user_liquidity: Mapping<(AccountId, AccountId), u128>,
        total_liquidity: Mapping<AccountId, u128>,
    }

    #[ink(event)]
    pub struct TokenDeposited {
        #[ink(topic)]
        user: AccountId,
        #[ink(topic)]
        token: AccountId,
        amount: u128,
    }

    #[ink(event)]
    pub struct TokensSwapped {
        #[ink(topic)]
        user: AccountId,
        #[ink(topic)]
        token_in: AccountId,
        amount_in: u128,
        #[ink(topic)]
        token_out: AccountId,
        amount_out: u128,
    }

    #[ink(event)]
    pub struct LiquidityAdded {
        #[ink(topic)]
        user: AccountId,
        #[ink(topic)]
        token: AccountId,
        amount: u128,
    }

    #[ink(event)]
    pub struct LiquidityRemoved {
        #[ink(topic)]
        user: AccountId,
        #[ink(topic)]
        token: AccountId,
        amount: u128,
    }

    impl SwapX {
        #[ink(constructor)]
        pub fn new(fee_percentage: u128, protocol_fund: AccountId) -> Self {
            Self {
                fee_percentage,
                protocol_fund,
                liquidity_pool: Mapping::default(),
                user_liquidity: Mapping::default(),
                total_liquidity: Mapping::default(),
            }
        }

        #[ink(message)]
        pub fn deposit_token(&mut self, token: AccountId, amount: u128) {
            assert!(amount > 0, "Amount must be greater than zero");
            let current = self.liquidity_pool.get(token).unwrap_or(0);
            let new_amount = current.checked_add(amount).expect("Overflow in deposit_token");
            self.liquidity_pool.insert(token, &new_amount);
            self.env().emit_event(TokenDeposited {
                user: self.env().caller(),
                token,
                amount,
            });
        }

        #[ink(message)]
        pub fn swap_tokens(
            &mut self,
            token_in: AccountId,
            amount_in: u128,
            token_out: AccountId,
            min_amount_out: u128,
        ) {
            assert!(amount_in > 0, "Amount must be greater than zero");
            let liquidity_in = self.liquidity_pool.get(token_in).unwrap_or(0);
            let liquidity_out = self.liquidity_pool.get(token_out).unwrap_or(0);
            assert!(liquidity_in >= amount_in, "Insufficient liquidity for tokenIn");
            assert!(liquidity_out > 0, "Insufficient liquidity for tokenOut");

            let swap_rate = self.get_swap_rate(token_in, token_out);
            let amount_out_pre = amount_in.checked_mul(swap_rate).expect("Overflow in swap calculation")
                .checked_div(1_000_000_000_000_000_000u128).expect("Division by zero");
            
            let fee = amount_out_pre.checked_mul(self.fee_percentage).expect("Overflow in fee calculation")
                .checked_div(100).expect("Division by zero");
            
            let protocol_fee = fee.checked_div(2).expect("Division by zero");
            let liquidity_fee = fee.checked_sub(protocol_fee).expect("Underflow in fee calculation");
            let amount_out = amount_out_pre.checked_sub(fee).expect("Underflow in amount calculation");

            assert!(amount_out >= min_amount_out, "Slippage tolerance exceeded");
            assert!(
                liquidity_out >= amount_out,
                "Insufficient liquidity for tokenOut after fee"
            );

            let new_liquidity_in = liquidity_in.checked_add(amount_in).expect("Overflow in liquidity calculation");
            let new_liquidity_out = liquidity_out.checked_sub(amount_out).expect("Underflow in liquidity calculation");
            let final_liquidity_out = new_liquidity_out.checked_add(liquidity_fee).expect("Overflow in final liquidity");

            self.liquidity_pool.insert(token_in, &new_liquidity_in);
            self.liquidity_pool.insert(token_out, &new_liquidity_out);
            self.liquidity_pool.insert(token_out, &final_liquidity_out);

            // Transfer protocol fee to protocol fund account
            // TODO: Implement protocol fee transfer
            let _ = protocol_fee; // Explicitly acknowledge protocol_fee usage

            self.env().emit_event(TokensSwapped {
                user: self.env().caller(),
                token_in,
                amount_in,
                token_out,
                amount_out,
            });
        }

        #[ink(message)]
        pub fn add_liquidity(&mut self, token: AccountId, amount: u128) {
            assert!(amount > 0, "Amount must be greater than zero");
            let user = self.env().caller();
            
            let current_liquidity = self.liquidity_pool.get(token).unwrap_or(0);
            let new_liquidity = current_liquidity.checked_add(amount).expect("Overflow in liquidity addition");
            self.liquidity_pool.insert(token, &new_liquidity);
            
            let current_user_liquidity = self.user_liquidity.get((user, token)).unwrap_or(0);
            let new_user_liquidity = current_user_liquidity.checked_add(amount).expect("Overflow in user liquidity");
            self.user_liquidity.insert((user, token), &new_user_liquidity);
            
            let current_total = self.total_liquidity.get(token).unwrap_or(0);
            let new_total = current_total.checked_add(amount).expect("Overflow in total liquidity");
            self.total_liquidity.insert(token, &new_total);

            self.env().emit_event(LiquidityAdded {
                user,
                token,
                amount,
            });
        }

        #[ink(message)]
        pub fn remove_liquidity(&mut self, token: AccountId, amount: u128) {
            assert!(amount > 0, "Amount must be greater than zero");
            let user = self.env().caller();
            let user_liquidity = self.user_liquidity.get((user, token)).unwrap_or(0);
            assert!(user_liquidity >= amount, "Insufficient liquidity");

            let fee_share = amount.checked_mul(self.fee_percentage).expect("Overflow in fee calculation")
                .checked_div(100).expect("Division by zero");
            let _protocol_fee = fee_share.checked_div(2).expect("Division by zero"); // Use underscore prefix
            let amount_after_fee = amount.checked_sub(fee_share).expect("Underflow in fee calculation");

            let current_liquidity = self.liquidity_pool.get(token).unwrap_or(0);
            let new_liquidity = current_liquidity.checked_sub(amount_after_fee).expect("Underflow in liquidity removal");
            self.liquidity_pool.insert(token, &new_liquidity);

            let new_user_liquidity = user_liquidity.checked_sub(amount).expect("Underflow in user liquidity");
            self.user_liquidity.insert((user, token), &new_user_liquidity);

            let current_total = self.total_liquidity.get(token).unwrap_or(0);
            let new_total = current_total.checked_sub(amount_after_fee).expect("Underflow in total liquidity");
            self.total_liquidity.insert(token, &new_total);

            self.env().emit_event(LiquidityRemoved {
                user,
                token,
                amount: amount_after_fee,
            });
        }

        #[ink(message)]
        pub fn get_swap_rate(
            &self,
            _token_in: AccountId,    // Prefix with underscore
            _token_out: AccountId,   // Prefix with underscore
        ) -> u128 {
            // Implement the logic to get the swap rate between token_in and token_out
            1 // Placeholder value
        }

        #[ink(message)]
        pub fn set_fee_percentage(&mut self, fee_percentage: u128) {
            self.fee_percentage = fee_percentage;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ink::test]
    fn default_works() {
        let swapx = SwapX::new(1, AccountId::from([0x1; 32]));
        assert_eq!(swapx.fee_percentage, 1);
    }
}