#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::{fmt::Debug, prelude::*};
use sp_runtime::{
	RuntimeDebug, DispatchResult, DispatchError,
	traits::{
		AtLeast32BitUnsigned, StaticLookup,
		// Zero,
		// Saturating, CheckedSub, CheckedAdd,
	},
};
use frame_support::{
	ensure,
	traits::{
		Randomness, //, Currency, ReservableCurrency
	},
};
use codec::{Encode, Decode, HasCompact};
pub use pallet::*;

use mc_support::{
	primitives::{Formula},
	traits::{RandomNumber, FeaturedAssets, UniqueAssets},
};

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

		/// The arithmetic type of formula identifier.
		type FormulaId: Member + Parameter + Default + Copy + HasCompact;

		/// The manager origin.
		type ManagerOrigin: EnsureOrigin<Self::Origin>;

		/// Something that provides randomness in the runtime.
		type RandomNumber: RandomNumber<u32>;

		/// The featured asset module
		type FeaturedAssets: FeaturedAssets<Self::AccountId>;

		/// NFT Assets
		type UniqueAssets: UniqueAssets<Self::AccountId>;
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T:Config> Pallet<T> {
		/// create a formula
		#[pallet::weight((10_000 + T::DbWeight::get().writes(1), DispatchClass::Normal, Pays::No))]
		pub fn create_formula(
			origin: OriginFor<T>,
			#[pallet::compact] id: T::FormulaId,
		) -> DispatchResultWithPostInfo {
			T::ManagerOrigin::ensure_origin(origin)?;

			// TODO

			Self::deposit_event(Event::FormulaCreated(id));
			Ok(().into())
		}

		/// modify a formula
		#[pallet::weight((10_000 + T::DbWeight::get().writes(1), DispatchClass::Normal, Pays::No))]
		pub fn modify_formula(
			origin: OriginFor<T>,
			#[pallet::compact] id: T::FormulaId,
		) -> DispatchResultWithPostInfo {
			T::ManagerOrigin::ensure_origin(origin)?;

			// TODO

			Self::deposit_event(Event::FormulaModified(id));
			Ok(().into())
		}

		/// execute a formula
		#[pallet::weight((10_000 + T::DbWeight::get().writes(1), DispatchClass::Normal, Pays::No))]
		pub fn excuete_formula(
			origin: OriginFor<T>,
			#[pallet::compact] id: T::FormulaId,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			// TODO

			Ok(().into())
		}
	}

	#[pallet::storage]
	#[pallet::getter(fn formulas)]
	/// formula definations
	pub(super) type Formulas<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::FormulaId,
		Formula<T::FormulaId>,
		ValueQuery,
	>;

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId",T::FormulaId = "FormulaId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Some formula were created. \[formula_id\]
		FormulaCreated(T::FormulaId),
		/// Some formula were modified. \[formula_id\]
		FormulaModified(T::FormulaId),
		/// Some formula were executed. \[formula_id, who, commodity_id\]
		FormulaExecuted(T::FormulaId, T::AccountId, T::Hash),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
	}
}

// The main implementation block for the module.
impl<T: Config> Pallet<T> {
	// Public immutables
	// TODO
}
