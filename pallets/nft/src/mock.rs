// Creating mock runtime here

use crate::{Module, Config};
use frame_support::{impl_outer_origin, parameter_types};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
  testing::Header,
  traits::{BlakeTwo256, IdentityLookup},
};

impl_outer_origin! {
    pub enum Origin for Test where system = frame_system {}
}

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
  type BaseCallFilter = ();
  type Origin = Origin;
  type Call = ();
  type Index = u64;
  type BlockNumber = u64;
  type Hash = H256;
  type Hashing = BlakeTwo256;
  type AccountId = u64;
  type Lookup = IdentityLookup<Self::AccountId>;
  type Header = Header;
  type Event = ();
  type BlockHashCount = BlockHashCount;
  type DbWeight = ();
  type Version = ();
  type AccountData = ();
  type OnNewAccount = ();
  type OnKilledAccount = ();
  type SystemWeightInfo = ();
  type BlockWeights = ();
  type BlockLength = ();
  type PalletInfo = PalletInfo;
  type SS58Prefix = SS58Prefix;
}

parameter_types! {
    pub const MaxCommodities: u128 = 5;
    pub const MaxCommoditiesPerUser: u64 = 2;
}

// For testing the pallet, we construct most of a mock runtime. This means
// first constructing a configuration type (`Test`) which `impl`s each of the
// configuration traits of pallets we want to use.
#[derive(Clone, Eq, PartialEq)]
pub struct Test;

impl Config for Test {
  type Event = ();
  type CommodityAdmin = frame_system::EnsureRoot<Self::AccountId>;
  type CommodityInfo = Vec<u8>;
  type CommodityLimit = MaxCommodities;
  type UserCommodityLimit = MaxCommoditiesPerUser;
}

// system under test
pub type SUT = Module<Test>;

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub fn new_test_ext() -> sp_io::TestExternalities {
  system::GenesisConfig::default()
    .build_storage::<Test>()
    .unwrap()
    .into()
}
