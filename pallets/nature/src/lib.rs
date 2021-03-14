#![cfg_attr(not(feature = "std"), no_std)]

use sp_runtime::{
	ModuleId,
	traits::{ AccountIdConversion, }
};
use frame_support::traits::{
	Get, Randomness
};
use codec::{Encode, Decode};
use mc_support::traits::{
	ModuleAccessor, RandomNumber
};

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_system::pallet_prelude::*;
	use frame_support::{
		pallet_prelude::*,
		dispatch::DispatchResultWithPostInfo
	};

	use super::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// The module configuration trait.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		#[pallet::constant]
		/// The Lottery's module id
		type ModuleId: Get<ModuleId>;

		/// Something that provides randomness in the runtime.
		type Randomness: Randomness<Self::Hash>;

		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The manager origin.
		type ManagerOrigin: EnsureOrigin<Self::Origin>;

		/// Number of time we should try to generate a random number that has no modulo bias.
		/// The larger this number, the more potential computation is used for picking the winner,
		/// but also the more likely that the chosen winner is done fairly.
		type MaxGenerateRandom: Get<u32>;
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T:Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResultWithPostInfo {
			// ensure_signed(origin)?;

			// let who = T::ModuleId::get().into_account();

			// // Update storage.
			// Something::<T>::put(something);

			// // Emit an event.
			// Self::deposit_event(Event::SomethingStored(something, who));
			// // Return a successful DispatchResultWithPostInfo
			Ok(().into())
		}
	}

	// The pallet's runtime storage items.
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn managers)]
	pub(super) type Managers<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		u32,
		OptionQuery
	>;

	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Manager was added. \[who\]
		ManagerAdded(T::AccountId),
		/// Manager was removed. \[who\]
		ManagerRemoved(T::AccountId),
	}

	#[deprecated(note = "use `Event` instead")]
	pub type RawEvent<T> = Event<T>;


	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// The given account id is unknown.
		Unknown,
		/// The account id is already a manager.
		InUse,
		/// The account id is not a manager.
		NotManager,
	}
}

// The main implementation block for the module.
impl<T: Config> Pallet<T> {
	// Public immutables

	/// The account ID of Nature.
	///
	/// This actually does computation. If you need to keep using it, then make sure you cache the
	/// value and only call this once.
	pub fn account_id() -> T::AccountId {
		T::ModuleId::get().into_account()
	}
}

impl<T: Config> ModuleAccessor<T::AccountId> for Pallet<T> {
	fn get_owner_id() -> T::AccountId {
		Self::account_id()
	}
	/// Can thaw tokens, force transfers and burn tokens from any account.
	fn is_admin(_: &T::AccountId) -> bool {
		// TODO
		false
	}
	/// Can mint tokens.
	fn is_issuer(_: &T::AccountId) -> bool {
		// TODO
		false
	}
	/// Can freeze tokens.
	fn is_freezer(_: &T::AccountId) -> bool {
		// TODO
		false
	}
}

impl<T: Config> RandomNumber<u32> for Pallet<T> {
	// Generate a random number from a given seed.
	// Note that there is potential bias introduced by using modulus operator.
	// You should call this function with different seed values until the random
	// number lies within `u32::MAX - u32::MAX % n`.
	fn generate_random(seed: u32) -> u32 {
		let random_seed = T::Randomness::random(&(T::ModuleId::get(), seed).encode());
		let random_number = <u32>::decode(&mut random_seed.as_ref())
			.expect("secure hashes should always be bigger than u32; qed");
		random_number
	}
	fn generate_random_in_range(total: u32) -> u32 {
		let mut random_number = Self::generate_random(0);

		// Best effort attempt to remove bias from modulus operator.
		for i in 1 .. T::MaxGenerateRandom::get() {
			if random_number < u32::MAX - u32::MAX % total {
				break;
			}
			random_number = Self::generate_random(i);
		}
		random_number % total
	}
}
