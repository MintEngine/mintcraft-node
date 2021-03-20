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
use codec::{HasCompact};
pub use pallet::*;

use mc_support::{
	primitives::{
		FeatureDestinyRank, Formula
	},
	traits::{
		ManagerAccessor, RandomNumber, FeaturedAssets, UniqueAssets,
	},
};

type AssetBalance<T> = <<T as Config>::FeaturedAssets as FeaturedAssets<<T as frame_system::Config>::AccountId>>::Balance;

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

		/// Asset Admin is outer module
		type FormulaManager: ManagerAccessor<Self::AccountId>;

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
			formula: Formula<T::FormulaId, AssetBalance<T>>,
		) -> DispatchResultWithPostInfo {
			// T::ManagerOrigin::ensure_origin(origin)?;
			let origin = ensure_signed(origin)?;
			ensure!(T::FormulaManager::is_admin(&origin), Error::<T>::NoPermission);

			let formula_id = formula.id;
			ensure!(!Formulas::<T>::contains_key(&formula_id), Error::<T>::IdExists);

			// create formulas
			Formulas::<T>::insert(&formula_id, formula);

			Self::deposit_event(Event::FormulaCreated(formula_id));
			Ok(().into())
		}

		/// modify a formula
		#[pallet::weight((10_000 + T::DbWeight::get().writes(1), DispatchClass::Normal, Pays::No))]
		pub fn modify_formula_required_rank(
			origin: OriginFor<T>,
			#[pallet::compact] id: T::FormulaId,
			required_rank: FeatureDestinyRank,
		) -> DispatchResultWithPostInfo {
			// T::ManagerOrigin::ensure_origin(origin)?;
			let origin = ensure_signed(origin)?;
			ensure!(T::FormulaManager::is_admin(&origin), Error::<T>::NoPermission);

			Formulas::<T>::try_mutate(id, |maybe| {
				let formula = maybe.as_mut().ok_or(Error::<T>::Unknown)?;

				formula.required_rank = required_rank.clone();

				Self::deposit_event(Event::FormulaRequiredRankModified(id, required_rank));
				Ok(().into())
			})
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
		Formula<T::FormulaId, AssetBalance<T>>
	>;

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId", T::FormulaId = "FormulaId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Some formula were created. \[formula_id\]
		FormulaCreated(T::FormulaId),
		/// Some formula were modified. \[formula_id, required_rank\]
		FormulaRequiredRankModified(T::FormulaId, FeatureDestinyRank),
		/// Some formula were executed. \[formula_id, who\]
		FormulaExecuted(T::FormulaId, T::AccountId, T::Hash),
		/// Unique asset were minted. \[formula_id, who\]
		MintUniqueAssetSucceeded(T::FormulaId, T::AccountId),
		/// Unique asset were minted. \[formula_id, who, commodity_id\]
		MintUniqueAssetFailed(T::FormulaId, T::AccountId, T::Hash),
	}

	#[pallet::error]
	pub enum Error<T> {
		NoPermission,
		IdExists,
		Unknown,
	}
}

// The main implementation block for the module.
impl<T: Config> Pallet<T> {
	// Public immutables
	// TODO
}
