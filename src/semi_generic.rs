use std::collections::HashMap;

/**
Here, we introduce the notion of 'semi' generic setup using associated types, or type aliases, at the top of the file.

It *kind* of solves the problem, but what if  but we can make this even better...
*/

type AccountId = u32;
type Balance = u32;

pub struct BalancePallet {
    balances: HashMap<AccountId, Balance>,
}

pub struct ContractsPallet {
    contract_address: AccountId,
    code: [u8; 32],
}
