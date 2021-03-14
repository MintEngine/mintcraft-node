use mc_support::traits::{ LifeTime };
use super::primitives::{ BlockNumber };

pub struct Demo;

impl LifeTime<BlockNumber> for Demo {
	fn base_age(level: u32) -> BlockNumber {
		// TODO
		0
	}
}
