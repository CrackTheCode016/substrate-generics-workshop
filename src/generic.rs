use num::{traits::Zero, CheckedAdd, CheckedSub};
use std::{collections::HashMap, hash::Hash};

/**
This represents a generic version of how a Runtime may look, namely with two modules: "Balance" and "Contracts".
Using a common configuration trait, it becomes quite trivial to define a common grounds for all relevant types used.
*/
pub trait RuntimeConfig {
    type AccountId: Eq + Hash;
    type Balance: Copy + Zero + CheckedAdd + CheckedSub;
    type ExecutableCode;
}

/**
This is our "generic" runtime.  We can use associated types to define our common types *once*.
Now, we can use this concept of a common runtime to call our pallets with the types in place!
*/
pub struct GenericRuntime;

impl RuntimeConfig for GenericRuntime {
    type Balance = u32;
    type AccountId = u32;
    type ExecutableCode = [u8; 32];
}

/**
Let's define and configure a second runtime, this one can be "evm" based:
*/
pub struct EvmRuntime;

impl RuntimeConfig for EvmRuntime {
    type Balance = u128;
    type AccountId = [u8; 64];
    type ExecutableCode = String;
}

/**

Now, both pallets can take full advantage of these common types, as defined above.
Remember, the *generics* define our input, meaning that our runtime serves as the input, or context, for each pallet to operate on.
The associated types, on the other hand, define outputs for each respective operation within the pallet, irrespective of whatever runtime they're running on.

*/

pub struct BalancePallet<Config: RuntimeConfig> {
    balances: HashMap<Config::AccountId, Config::Balance>,
}

impl<Config: RuntimeConfig> BalancePallet<Config> {
    pub fn new() -> Self {
        BalancePallet {
            balances: HashMap::new(),
        }
    }

    pub fn set_balance(&mut self, amount: Config::Balance, who: Config::AccountId) {
        self.balances.insert(who, amount);
    }

    pub fn balance(&self, who: Config::AccountId) -> Config::Balance {
        *self.balances.get(&who).unwrap_or(&Config::Balance::zero())
    }

    pub fn transfer(
        &mut self,
        sender: Config::AccountId,
        recipient: Config::AccountId,
        amount: Config::Balance,
    ) -> Result<(), &'static str> {
        // Example; Take from one, give to another, etc.
        let sender_bal = *self.balances.get(&sender).ok_or("User doesn't exist")?;
        let recipient_bal = *self.balances.get(&recipient).ok_or("User doesn't exist")?;

        let new_sender_bal = sender_bal.checked_sub(&amount).ok_or("Not enough money!")?;
        let new_recipient_bal = recipient_bal
            .checked_add(&amount)
            .ok_or("Integer Overflow!")?;

        self.balances.insert(sender, new_sender_bal);
        self.balances.insert(recipient, new_recipient_bal);

        Ok(())
    }
}

pub struct ContractsPallet<Config: RuntimeConfig> {
    contract_address: Config::AccountId,
    code: Config::ExecutableCode,
}
