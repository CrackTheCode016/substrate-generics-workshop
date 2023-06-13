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
