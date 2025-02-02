//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use num_derive::FromPrimitive;

/// Type of depeg pool
#[derive(
    BorshSerialize,
    BorshDeserialize,
    Clone,
    Debug,
    Eq,
    PartialEq,
    Copy,
    PartialOrd,
    Hash,
    FromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DepegType {
    None,
    Marinade,
    Lido,
    SplStake,
}
