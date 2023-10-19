// Copyright 2021-2022 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

use crate::error::Error;

use clap::*;
use serde::{Deserialize, Serialize};
use sp_npos_elections::BalancingConfig;
use sp_runtime::DeserializeOwned;
use std::{collections::HashMap, fmt, str::FromStr};

/// The type of solver to use.
// A common option across multiple commands.
#[derive(Debug, Clone, Parser)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Solver {
	SeqPhragmen {
		#[clap(long, default_value = "10")]
		iterations: usize,
	},
	PhragMMS {
		#[clap(long, default_value = "10")]
		iterations: usize,
	},
}

frame_support::parameter_types! {
	/// Number of balancing iterations for a solution algorithm. Set based on the [`Solvers`] CLI
	/// config.
	pub static BalanceIterations: usize = 10;
	pub static Balancing: Option<BalancingConfig> = Some( BalancingConfig { iterations: BalanceIterations::get(), tolerance: 0 } );
}

/// The chain being used.
#[derive(Debug, Copy, Clone)]
pub enum Chain {
	Westend,
	Kusama,
	Polkadot,
	Vara
}

impl fmt::Display for Chain {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let chain = match self {
			Self::Polkadot => "polkadot",
			Self::Kusama => "kusama",
			Self::Westend => "westend",
			Self::Vara => "vara",
		};
		write!(f, "{}", chain)
	}
}

impl std::str::FromStr for Chain {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Error> {
		match s {
			"polkadot" => Ok(Self::Polkadot),
			"kusama" => Ok(Self::Kusama),
			"westend" => Ok(Self::Westend),
			"vara" => Ok(Self::Vara),
			chain => Err(Error::InvalidChain(chain.to_string())),
		}
	}
}

impl TryFrom<subxt::backend::unstable::rpc_methods::RuntimeSpec> for Chain {
	type Error = Error;

	fn try_from(rv: subxt::backend::unstable::rpc_methods::RuntimeSpec) -> Result<Self, Error> {
		let spec_name = rv
			.spec_name
			.clone();
		let mut chain = spec_name;
		chain.make_ascii_lowercase();
		Chain::from_str(&chain)
	}
}

// This is infallible because all these field must exist on substrate-based chains
// and is based on <https://docs.rs/sp-version/latest/sp_version/struct.RuntimeVersion.html>
impl From<subxt::backend::unstable::rpc_methods::RuntimeSpec> for RuntimeVersion {
	fn from(rv: subxt::backend::unstable::rpc_methods::RuntimeSpec) -> Self {
		let mut spec_name: String = rv.spec_name;
		let impl_name: String = rv.impl_name;
		let impl_version: u32 = rv.impl_version;

		let spec_version = rv.spec_version;
		let transaction_version = rv.transaction_version;

		spec_name.make_ascii_lowercase();

		Self {
			spec_name,
			impl_name,
			impl_version,
			spec_version,
			transaction_version,
		}
	}
}

#[derive(Deserialize, Serialize)]
pub struct RuntimeVersion {
	pub spec_name: String,
	pub impl_name: String,
	pub spec_version: u32,
	pub impl_version: u32,
	pub transaction_version: u32,
}

fn get_val_unchecked<T: DeserializeOwned>(val: &str, rv: &HashMap<String, serde_json::Value>) -> T {
	let json = rv.get(val).expect("`{val}` must exist; qed").clone();
	serde_json::from_value::<T>(json).expect("T must be Deserialize; qed")
}
