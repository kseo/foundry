// Copyright 2018-2020 Kodebox, Inc.
// This file is part of CodeChain.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

extern crate codechain_crypto as ccrypto;
extern crate codechain_db as cdb;
#[macro_use]
extern crate codechain_logger as clogger;
extern crate codechain_key as ckey;
extern crate codechain_types as ctypes;
extern crate codechain_vm as cvm;
#[macro_use]
extern crate log;
#[cfg(test)]
extern crate rustc_hex;
#[macro_use]
extern crate rlp_derive;

mod action_handler;
mod cache;
mod checkpoint;
mod db;
mod error;
mod impls;
mod item;
mod traits;

pub mod tests;

pub use crate::action_handler::{ActionDataKeyBuilder, ActionHandler, FindActionHandler, HitHandler};
pub use crate::checkpoint::{CheckpointId, StateWithCheckpoint};
pub use crate::db::StateDB;
pub use crate::error::Error as StateError;
pub use crate::impls::{ShardLevelState, TopLevelState};
pub use crate::item::account::Account;
pub use crate::item::action_data::ActionData;
pub use crate::item::dummy_shard_text::{ShardText, ShardTextAddress};
pub use crate::item::metadata::{Metadata, MetadataAddress};
pub use crate::item::regular_account::{RegularAccount, RegularAccountAddress};
pub use crate::item::shard::{Shard, ShardAddress};
pub use crate::traits::{ShardState, ShardStateView, StateWithCache, TopState, TopStateView};

use crate::cache::CacheableItem;

pub type StateResult<T> = Result<T, StateError>;
