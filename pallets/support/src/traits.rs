// use sp_std::prelude::*;

pub trait ModuleAccessor<AccountId>: Sized {
	fn get_owner_id() -> AccountId;
	// Default impls
	/// Can thaw tokens, force transfers and burn tokens from any account.
	fn is_admin(_: &AccountId) -> bool {
		false
	}
	/// Can mint tokens.
	fn is_issuer(_: &AccountId) -> bool {
		false
	}
	/// Can freeze tokens.
	fn is_freezer(_: &AccountId) -> bool {
		false
	}
}

/// default implement for test
impl ModuleAccessor<u64> for () {
	fn get_owner_id() -> u64 {
		0
	}
}

pub trait RandomNumber<T> {
	fn generate_random(seed: T) -> T;
	fn generate_random_in_range(total: T) -> T;
}
impl RandomNumber<u32> for () {
	fn generate_random(_: u32) -> u32 { 0 }
	fn generate_random_in_range(_: u32) -> u32 { 0 }
}

// Implication related traits
pub trait ImplicationSystem {

}

pub trait ImplicationEntity {

}

// Decay related traits
pub trait DecayingSystem {

}

pub trait DecayingEntity {

}
