// This entry point initializes the donation system, setting up the fundraising purse
// and creating a dictionary to track the account hashes and the number of donations
// made.
#[no_mangle]
pub extern "C" fn init() {
    let fundraising_purse = system::create_purse();
    runtime::put_key(FUNDRAISING_PURSE, fundraising_purse.into());
    // Create a dictionary to track the mapping of account hashes to number of donations made.
    storage::new_dictionary(LEDGER).unwrap_or_revert();
}

// This is the donation entry point. When called, it records the caller's account
// hash and returns the donation purse, with add access, to the immediate caller.
#[no_mangle]
pub extern "C" fn donate() {
    let donating_account_key: Key = runtime::get_named_arg(DONATING_ACCOUNT_KEY);
    if let Key::Account(donating_account_hash) = donating_account_key {
        update_ledger_record(donating_account_hash.to_string())
    } else {
        runtime::revert(FundRaisingError::InvalidKeyVariant)
    }
    let donation_purse = *runtime::get_key(FUNDRAISING_PURSE)
        .unwrap_or_revert_with(FundRaisingError::MissingFundRaisingPurseURef)
        .as_uref()
        .unwrap_or_revert();
    // The return value is the donation_purse URef with `add` access only. As a result
    // the entity receiving this purse URef may only add to the purse, and cannot remove
    // funds.
    let value = CLValue::from_t(donation_purse.into_add()).unwrap_or_revert();
    runtime::ret(value)
}

// This entry point returns the amount of donations from the caller.
#[no_mangle]
pub extern "C" fn get_donation_count() {
    let donating_account_key: Key = runtime::get_named_arg(DONATING_ACCOUNT_KEY);
    if let Key::Account(donating_account_hash) = donating_account_key {
        let ledger_seed_uref = *runtime::get_key(LEDGER)
            .unwrap_or_revert_with(FundRaisingError::MissingLedgerSeedURef)
            .as_uref()
            .unwrap_or_revert();
        let donation_count = if let Some(donation_count) =
            storage::dictionary_get::<u64>(ledger_seed_uref, &donating_account_hash.to_string())
                .unwrap_or_revert()
        {
            donation_count
        } else {
            0u64
        };
        runtime::ret(CLValue::from_t(donation_count).unwrap_or_revert())
    } else {
        runtime::revert(FundRaisingError::InvalidKeyVariant)
    }
}

// This entry point returns the total funds raised.
#[no_mangle]
pub extern "C" fn get_funds_raised() {
    let donation_purse = *runtime::get_key(FUNDRAISING_PURSE)
        .unwrap_or_revert_with(FundRaisingError::MissingFundRaisingPurseURef)
        .as_uref()
        .unwrap_or_revert();
    let funds_raised = system::get_purse_balance(donation_purse)
        .unwrap_or_revert();
    runtime::ret(CLValue::from_t(funds_raised).unwrap_or_revert())
}


