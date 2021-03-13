// use sp_std::prelude::*;

pub trait ModuleAccessor<AccountId>: Sized {
	fn get_owner_id() -> AccountId;
}

/// default implement for test
impl ModuleAccessor<u64> for () {
	fn get_owner_id() -> u64 {
		0
	}
}
