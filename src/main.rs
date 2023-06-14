mod generic;
mod not_generic;
mod semi_generic;

fn main() {
    println!("View the results of each step below, or run cargo test :p");
    println!("=========================================================");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generic_zero_bal() {
        let bob = 0;

        let balances = generic::BalancePallet::<generic::GenericRuntime>::new();
        let bobs_balance = balances.balance(bob);

        assert_eq!(bobs_balance, 0)
    }

    #[test]
    fn generic_transfer() {
        let bob = 0;
        let alice = 1;

        let mut balances = generic::BalancePallet::<generic::GenericRuntime>::new();

        balances.set_balance(200, alice);
        balances.set_balance(100, bob);

        balances.transfer(alice, bob, 50).unwrap();

        let bobs_balance = balances.balance(bob);
        let alice_balance = balances.balance(alice);

        assert_eq!(bobs_balance, 150);
        assert_eq!(alice_balance, 150);
    }

    #[test]
    fn generic_set_balance() {
        let bob = 0;

        let mut balances = generic::BalancePallet::<generic::GenericRuntime>::new();
        balances.set_balance(100, bob);
        let bobs_balance = balances.balance(bob);

        assert_eq!(bobs_balance, 100)
    }

    // #[test]
    // fn generic_evm_balance() {
    //     let bob = 0;
    //     // Why wont this work?
    //     let bob_evm: [u8; 64] = [0; 64];

    //     let mut balances = generic::BalancePallet::<generic::EvmRuntime>::new();
    //     balances.set_balance(100, bob);
    //     let bobs_balance = balances.balance(bob);

    //     assert_eq!(bobs_balance, 100)
    // }
}
