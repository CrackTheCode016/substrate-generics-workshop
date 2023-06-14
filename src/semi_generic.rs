use std::{collections::HashMap, hash::Hash};

/**
Here, we introduce the notion of 'semi' generic setup using associated types, or type aliases, at the top of the file.

It *kind* of solves the problem, but we can make this even better, as a pallet is defined once in runtime, as is the runtime itself.

It doesn't make sense to define generic things, which all of these pallets share, multiple times!

In fact, having one, central configuration for every pallet would solve this.  One that abides and takes full advantage of the Rust type system!
*/

pub struct BalancePallet<AccountId: Eq + Hash, Balance> {
    balances: HashMap<AccountId, Balance>,
}

pub struct ContractsPallet<AccountId: Eq, Balance> {
    contract_address: AccountId,
    code: [u8; 32],
    balance: Balance,
}

/**

Using just raw generics is also an approach, but it gets messy quickly with many trait bounds.

We also have to implement these generics everytime we use this api, which is no bueno!

*/

impl<AccountId: Eq + Hash, Balance> BalancePallet<AccountId, Balance> {}

impl<AccountId: Eq, Balance> ContractsPallet<AccountId, Balance> {}
