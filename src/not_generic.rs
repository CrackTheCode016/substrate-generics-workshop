use std::collections::HashMap;

/**
This represents a non-generic version of how a Runtime may look, namely with two modules: "Balance" and "Contracts".
Each require various types, some which are common with each other! This will show how it is quite hard to manage
a lot of types without generics or associated types.
*/

/**

As we can see, it can be a little hard to distinguish what type is used for what.
For example, in 'BalancesPallet', it is not clear what the key and values are supposed to represent (account address -> balance).
In 'ContractsPallet' pallet, we also define 'contract_address' as something different from the account address of a Balance.

Surely this cannot be maintainable!

*/

pub struct BalancePallet {
    balances: HashMap<u32, u32>,
}

pub struct ContractsPallet {
    contract_address: String,
    code: [u8; 32],
}

impl BalancePallet {
    pub fn new() -> Self {
        BalancePallet {
            balances: HashMap::new(),
        }
    }

    pub fn set_balance(&mut self, amount: u32, who: u32) {
        self.balances.insert(who, amount);
    }

    pub fn balance(&self, who: u32) -> u32 {
        *self.balances.get(&who).unwrap_or(&0)
    }

    pub fn transfer(
        &mut self,
        sender: u32,
        recipient: u32,
        amount: u32,
    ) -> Result<(), &'static str> {
        // Example; Take from one, give to another, etc.
        let sender_bal = *self.balances.get(&sender).ok_or("User doesn't exist")?;
        let recipient_bal = *self.balances.get(&recipient).ok_or("User doesn't exist")?;

        let new_sender_bal = sender_bal.checked_sub(amount).ok_or("Not enough money!")?;
        let new_recipient_bal = recipient_bal
            .checked_add(amount)
            .ok_or("Integer Overflow!")?;

        self.balances.insert(sender, new_sender_bal);
        self.balances.insert(recipient, new_recipient_bal);

        Ok(())
    }
}
