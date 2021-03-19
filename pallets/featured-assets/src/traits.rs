use frame_support::{
	dispatch::DispatchResultWithPostInfo,
	pallet_prelude::*
};

use mc_support::{
	primitives::{FeatureElements, FeatureLevel, FeatureDestinyRank, FeatureRankedLevel}
};

/// An interface over a set of unique assets.
/// Assets with equivalent attributes (as defined by the AssetInfo type) **must** have an equal ID
/// and assets with different IDs **must not** have equivalent attributes.
pub trait FeaturedAssets<AccountId> {
	/// The type used to identify unique assets.
	type AssetId;
	type Amount;
	type Balance;

	/// The total number of this type of asset that exists (minted - burned).
	fn total_supply() -> Self::Amount;
	/// The balance of this type of asset owned by an account.
	fn balance(id: Self::AssetId, who: &AccountId) -> Self::Balance;
	/// The feature of this type of asset
	fn feature(id: Self::AssetId) -> Option<AssetFeature>;

	/// Use the provided asset info to create a new unique asset for the specified user.
	fn mint(
		id: Self::AssetId,
		beneficiary: &AccountId,
		amount: Self::Balance,
	) -> DispatchResultWithPostInfo;

	/// Burn asset.
	fn burn(
		id: &Self::AssetId,
		who: &AccountId,
		amount: Self::Balance,
	) -> DispatchResultWithPostInfo;

	/// Transfer asset balance to another account.
	fn transfer(
		id: Self::AssetId,
		source: &AccountId,
		target: &AccountId,
		amount: Self::Balance,
	) -> DispatchResultWithPostInfo;
}

// Featured Part for asset
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default)]
pub struct AssetFeature {
	/// The level of this asset
	destiny: FeatureDestinyRank,
	/// The elements info of this asset
	elements: FeatureElements,
	/// The 'saturation' of this asset
	saturation: FeatureRankedLevel,
	/// The 'lightness' of this asset
	lightness: FeatureLevel
}

impl AssetFeature {
	/// create new Feature
	pub fn create (
		destiny: FeatureDestinyRank,
		elements: FeatureElements,
		saturation: FeatureRankedLevel,
		lightness: FeatureLevel
	) -> Self {
		AssetFeature {
			destiny,
			elements,
			saturation,
			lightness,
		}
	}
}
