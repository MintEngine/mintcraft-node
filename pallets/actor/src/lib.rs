#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::prelude::*;
use sp_runtime::{
	RuntimeDebug, Percent,
	traits::{
		StaticLookup,
		// Zero,
		// Saturating, CheckedSub, CheckedAdd,
	},
};
use frame_support::{
	// ensure,
	traits::{ Get },
	// dispatch::DispatchError,
};
use codec::{Encode, Decode};
use mc_support::{
	traits::{ LifeTime }
};

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		weights::{DispatchClass, Pays},
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::*
	};
	use frame_system::pallet_prelude::*;
	use super::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// The module configuration trait.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// LifeTime calclator
		type ActorLifeTime: LifeTime<Self::BlockNumber>;
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		// TODO handle dying actors
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// generate an actor of the account
		#[pallet::weight((10_000 + T::DbWeight::get().writes(1), DispatchClass::Normal, Pays::No))]
		pub fn generate(
			origin: OriginFor<T>
		) -> DispatchResultWithPostInfo {
			let one = ensure_signed(origin)?;

			// TODO

			Ok(().into())
		}
	}

	#[pallet::storage]
	#[pallet::getter(fn actors)]
	pub type Actors<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		ActorInfo<T::BlockNumber>
	>;

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// An actor borned
		ActorBorn(T::AccountId),
		/// An actor dead
		ActorDead(T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Actor does't exist.
		NotExist,
		/// Actor was dead.
		Dead,
	}
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default)]
pub struct ActorInfo<BlockNumber> {
	/// The born time
	born_at: BlockNumber,
	/// The born age
	born_age: BlockNumber,
	/// actor dead time
	live_until: BlockNumber,
	// The rank level
	level: u8,
	/// The progress of actor level
	level_progress: Percent,
}

// The main implementation block for the module.
impl<T: Config> Pallet<T> {
	// Public immutables
	// TODO
}
