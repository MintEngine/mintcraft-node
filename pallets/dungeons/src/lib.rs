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
		Get, Randomness, Currency, ReservableCurrency
	},
};
use codec::{Encode, Decode,HasCompact};
use mc_support::traits::{
	ManagerAccessor, RandomNumber
};

pub use pallet::*;

type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod pallet {
	use frame_system::pallet_prelude::*;
	use frame_support::{
		weights::{DispatchClass, Pays},
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::*
	};
	use super::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// The module configuration trait.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The arithmetic type of dungeon identifier.
		type DungeonId: Member + Parameter + Default + Copy + HasCompact;

		/// The units in which we record balances.
		type Balance: Member + Parameter + AtLeast32BitUnsigned + Default + Copy;

		/// The currency mechanism.
		type Currency: ReservableCurrency<Self::AccountId>;

		/// The manager origin.
		type ManagerOrigin: EnsureOrigin<Self::Origin>;

		/// Something that provides randomness in the runtime.
		type Randomness: Randomness<Self::Hash>;

		/// Number of time we should try to generate a random number that has no modulo bias.
		/// The larger this number, the more potential computation is used for picking the winner,
		/// but also the more likely that the chosen winner is done fairly.
		type MaxGenerateRandom: Get<u32>;
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		// TODO on finalized
	}

	#[pallet::call]
	impl<T:Config> Pallet<T> {
		/// create new dungeon
		#[pallet::weight((10_000 + T::DbWeight::get().writes(1), DispatchClass::Normal, Pays::No))]
		pub(super) fn create(
			origin: OriginFor<T>,
			#[pallet::compact] id: T::DungeonId,
			ticket_price: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			T::ManagerOrigin::ensure_origin(origin)?;

			// TODO create dungeon

			Self::deposit_event(Event::DungeonCreated(id, ticket_price));
			Ok(().into())
		}

		/// modify dungeon price
		#[pallet::weight((10_000 + T::DbWeight::get().writes(1), DispatchClass::Normal, Pays::No))]
		pub(super) fn modify_price(
			origin: OriginFor<T>,
			#[pallet::compact] id: T::DungeonId,
			ticket_price: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			T::ManagerOrigin::ensure_origin(origin)?;

			// TODO
			let old_ticket_price = ticket_price;

			Self::deposit_event(Event::DungeonTicketModified(id, old_ticket_price, ticket_price));
			Ok(().into())
		}

		/// modify assets supply
		#[pallet::weight((10_000 + T::DbWeight::get().writes(1), DispatchClass::Normal, Pays::No))]
		pub(super) fn modify_assets_supply(
			origin: OriginFor<T>,
			#[pallet::compact] id: T::DungeonId,
			// TODO
		) -> DispatchResultWithPostInfo {
			T::ManagerOrigin::ensure_origin(origin)?;

			// TODO

			Ok(().into())
		}

		/// modify final distribution
		#[pallet::weight((10_000 + T::DbWeight::get().writes(1), DispatchClass::Normal, Pays::No))]
		pub(super) fn modify_distribution_ratio(
			origin: OriginFor<T>,
			#[pallet::compact] id: T::DungeonId,
			// TODO
		) -> DispatchResultWithPostInfo {
			T::ManagerOrigin::ensure_origin(origin)?;

			// TODO

			Ok(().into())
		}

		/// buy dungeon ticket
		#[pallet::weight((10_000 + T::DbWeight::get().writes(1), DispatchClass::Normal, Pays::No))]
		pub(super) fn buy_ticket(
			origin: OriginFor<T>,
			#[pallet::compact] id: T::DungeonId,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			// TODO

			Ok(().into())
		}

		/// begin a dungeon instance
		#[pallet::weight((10_000 + T::DbWeight::get().writes(1), DispatchClass::Normal, Pays::No))]
		pub(super) fn start(
			origin: OriginFor<T>,
			ticket_id: T::Hash,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			// TODO

			Ok(().into())
		}

		/// end a dungeon instance
		#[pallet::weight((10_000 + T::DbWeight::get().writes(1), DispatchClass::Normal, Pays::No))]
		pub(super) fn end(
			origin: OriginFor<T>,
			ticket_id: T::Hash,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			// TODO

			Ok(().into())
		}
	}

	#[pallet::storage]
	#[pallet::getter(fn dungeons)]
	/// dungeon definations
	pub(super) type Dungeons<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::DungeonId,
		DungeonInfo<BalanceOf<T>>,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn dungeon_instances)]
	/// dungeon instances
	pub(super) type DungeonInstances<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::Hash,
		DungeonInstance<T::DungeonId>,
		ValueQuery,
	>;

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId", T::Balance = "Balance", T::DungeonId = "DungeonId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Some dungeon were created. \[dungeon_id, ticket_price\]
		DungeonCreated(T::DungeonId, BalanceOf<T>),
		/// Some dungeon's price were modified. \[dungeon_id, old_ticket_price, new_ticket_price\]
		DungeonTicketModified(T::DungeonId, BalanceOf<T>, BalanceOf<T>),
		// Some dungeon's price were modified. \[dungeon_id, old_ticket_price, new_ticket_price\]
		// DungeonInfoModified(T::DungeonId, BalanceOf<T>, BalanceOf<T>),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
	}
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default)]
pub struct DungeonInfo<
	Balance: Encode + Decode + Clone + Debug + Eq + PartialEq,
> {
	/// The balance.
	ticket_price: Balance,

}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default)]
pub struct DungeonInstance<
	DungeonId: Encode + Decode + Clone + Debug + Eq + PartialEq,
> {
	/// the id of dungeon
	id: DungeonId,
}

// The main implementation block for the module.
impl<T: Config> Pallet<T> {
	// Public immutables
	// TODO
}
