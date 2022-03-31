
//! Autogenerated weights for `pallet_data_store`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-03-31, STEPS: `1`, REPEAT: 50, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/node-template
// benchmark
// --chain
// dev
// --pallet
// pallet_data_store
// --extrinsic
// *
// --repeat
// 50
// --output=./pallets/data-store/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_data_store`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_data_store::WeightInfo for WeightInfo<T> {
	// Storage: DataStore FileStorage (r:1 w:1)
	fn store(_s: u32, ) -> Weight {
		(10_207_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: DataStore FileStorage (r:1 w:0)
	fn retrieve(_s: u32, ) -> Weight {
		(9_499_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	// Storage: DataStore FileStorage (r:2 w:1)
	fn replace(_s: u32, ) -> Weight {
		(13_311_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: DataStore FileStorage (r:1 w:0)
	fn delete(_s: u32, ) -> Weight {
		(9_343_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	// Storage: DataStore FileStorage (r:1 w:1)
	fn edit(_s: u32, ) -> Weight {
		(12_192_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}