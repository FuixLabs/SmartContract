//This is the full `call` function as defined within the donation contract.
#[no_mangle]
pub extern "C" fn call() {
    // This establishes the `init` entry point for initializing the contract's infrastructure.
    let init_entry_point = EntryPoint::new(
        ENTRY_POINT_INIT,
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    // This establishes the `donate` entry point for callers looking to donate.
    let donate_entry_point = EntryPoint::new(
        ENTRY_POINT_DONATE,
        vec![Parameter::new(DONATING_ACCOUNT_KEY, CLType::Key)],
        CLType::URef,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    // This establishes an entry point called `donation_count` that returns the amount of
    // donations from a specific account.
    let get_donation_count_entry_point = EntryPoint::new(
        ENTRY_POINT_GET_DONATION_COUNT,
        vec![Parameter::new(DONATING_ACCOUNT_KEY, CLType::Key)],
        CLType::U64,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    // This establishes an entry point called `funds_raised` that returns the total amount
    // donated by all participants.
    let funds_raised_entry_point = EntryPoint::new(
        ENTRY_POINT_GET_FUNDS_RAISED,
        vec![],
        CLType::U512,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
}

