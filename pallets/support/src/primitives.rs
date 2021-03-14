use sp_runtime::{
	RuntimeDebug,
	// traits::{
	// 	AtLeast32BitUnsigned, Zero, StaticLookup, Saturating, CheckedSub, CheckedAdd,
	// }
};
use sp_std::prelude::*;
use codec::{Encode, Decode};

// Asset 的组合特性
#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug)]
pub enum FeatureHue {
	Green,
	Yellow,
	White,
	Black,
	Blue,
	Red,
	Orange,
	Pink,
	Purple,
}
impl Into<u8> for FeatureHue {
	fn into(self) -> u8 {
		match self {
			Self::Green => 0x01,
			Self::Yellow => 0x02,
			Self::White => 0x03,
			Self::Black => 0x04,
			Self::Blue => 0x05,
			Self::Red => 0x06,
			Self::Orange => 0x07,
			Self::Pink => 0x08,
			Self::Purple => 0x09,
		}
	}
}
impl From<u8> for FeatureHue {
	fn from(num: u8) -> FeatureHue {
		match num {
			0x01 => Self::Green,
			0x02 => Self::Yellow,
			0x03 => Self::White,
			0x04 => Self::Black,
			0x05 => Self::Blue,
			0x06 => Self::Red,
			0x07 => Self::Orange,
			0x08 => Self::Pink,
			0x09 => Self::Purple,
			_ => Self::Green,
		}
	}
}
#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug)]
pub enum FeatureElements {
	One(FeatureHue),
	Two(FeatureHue, FeatureHue),
	Three(FeatureHue, FeatureHue, FeatureHue),
	Four(FeatureHue, FeatureHue, FeatureHue, FeatureHue),
}
impl From<u32> for FeatureElements {
	fn from(num: u32) -> FeatureElements {
		const BYTES_PER_U32: usize = 4;

		let mut bytes = [0u8; BYTES_PER_U32];
		for i in 0..bytes.len() {
			bytes[i] = (num >> (4 * i)) as u8;
		}
		FeatureElements::from(&bytes)
	}
}
impl From<&[u8;4]> for FeatureElements {
	fn from(bytes: &[u8;4]) -> FeatureElements {
		if bytes[3] == 0u8 && bytes[2] == 0u8 && bytes[1] == 0u8 {
			FeatureElements::One(FeatureHue::from(bytes[0]))
		} else if bytes[3] == 0u8 && bytes[2] == 0u8 {
			FeatureElements::Two(FeatureHue::from(bytes[0]), FeatureHue::from(bytes[1]))
		} else if bytes[3] == 0u8 {
			FeatureElements::Three(FeatureHue::from(bytes[0]), FeatureHue::from(bytes[1]), FeatureHue::from(bytes[2]))
		} else {
			FeatureElements::Four(FeatureHue::from(bytes[0]), FeatureHue::from(bytes[1]), FeatureHue::from(bytes[2]), FeatureHue::from(bytes[3]))
		}
	}
}

impl Default for FeatureElements {
	fn default() -> Self { Self::One(FeatureHue::Green) }
}
#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug)]
pub enum FeatureLevel {
	Lv0,
	Lv1,
	Lv2,
	Lv3,
	Lv4,
	Lv5,
}
impl From<u8> for FeatureLevel {
	fn from(num: u8) -> FeatureLevel {
		match num {
			0u8 => FeatureLevel::Lv0,
			1u8 => FeatureLevel::Lv1,
			2u8 => FeatureLevel::Lv2,
			3u8 => FeatureLevel::Lv3,
			4u8 => FeatureLevel::Lv4,
			5u8 => FeatureLevel::Lv5,
			_ => FeatureLevel::Lv0,
		}
	}
}
impl Into<u8> for FeatureLevel {
	fn into(self) -> u8 {
		match self {
			Self::Lv0 => 0u8,
			Self::Lv1 => 1u8,
			Self::Lv2 => 2u8,
			Self::Lv3 => 3u8,
			Self::Lv4 => 4u8,
			Self::Lv5 => 5u8,
		}
	}
}
impl Default for FeatureLevel {
	fn default() -> Self { Self::Lv0 }
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug)]
pub enum FeatureRankedLevel {
	Low(FeatureLevel),
	Middle(FeatureLevel),
	High(FeatureLevel),
}
impl Default for FeatureRankedLevel {
	fn default() -> Self { Self::Low(FeatureLevel::Lv0) }
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug)]
pub enum FeatureDestinyRank {
	Tian,
	Di,
	Xuan,
	Huang,
}
impl Default for FeatureDestinyRank {
	fn default() -> Self { Self::Huang }
}
