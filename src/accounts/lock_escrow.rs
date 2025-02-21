//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use solana_program::pubkey::Pubkey;

/// State of lock escrow account

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LockEscrow {
    pub discriminator: [u8; 8],
    /// Pool address
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::DisplayFromStr>")
    )]
    pub pool: Pubkey,
    /// Owner address
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::DisplayFromStr>")
    )]
    pub owner: Pubkey,
    /// Vault address, store the lock user lock
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::DisplayFromStr>")
    )]
    pub escrow_vault: Pubkey,
    /// bump, used to sign
    pub bump: u8,
    /// Total locked amount
    pub total_locked_amount: u64,
    /// Lp per token, virtual price of lp token
    pub lp_per_token: u128,
    /// Unclaimed fee pending
    pub unclaimed_fee_pending: u64,
    /// Total a fee claimed so far
    pub a_fee: u64,
    /// Total b fee claimed so far
    pub b_fee: u64,
}

impl LockEscrow {
    pub const LEN: usize = 153;

    #[inline(always)]
    pub fn from_bytes(data: &[u8]) -> Result<Self, std::io::Error> {
        let mut data = data;
        Self::deserialize(&mut data)
    }
}

impl<'a> TryFrom<&solana_program::account_info::AccountInfo<'a>> for LockEscrow {
    type Error = std::io::Error;

    fn try_from(
        account_info: &solana_program::account_info::AccountInfo<'a>,
    ) -> Result<Self, Self::Error> {
        let mut data: &[u8] = &(*account_info.data).borrow();
        Self::deserialize(&mut data)
    }
}

#[cfg(feature = "fetch")]
pub fn fetch_lock_escrow(
    rpc: &solana_client::rpc_client::RpcClient,
    address: &Pubkey,
) -> Result<super::DecodedAccount<LockEscrow>, Error> {
    let accounts = fetch_all_lock_escrow(rpc, vec![address])?;
    Ok(accounts[0].clone())
}

#[cfg(feature = "fetch")]
pub fn fetch_all_lock_escrow(
    rpc: &solana_client::rpc_client::RpcClient,
    addresses: Vec<Pubkey>,
) -> Result<Vec<super::DecodedAccount<LockEscrow>>, Error> {
    let accounts = rpc.get_multiple_accounts(&addresses)?;
    let mut decoded_accounts: Vec<super::DecodedAccount<LockEscrow>> = Vec::new();
    for i in 0..addresses.len() {
        let address = addresses[i];
        let account = accounts[i]
            .as_ref()
            .ok_or(format!("Account not found: {}", address))?;
        let data = LockEscrow::from_bytes(&account.data)?;
        decoded_accounts.push(super::DecodedAccount {
            address,
            account: account.clone(),
            data,
        });
    }
    Ok(decoded_accounts)
}

#[cfg(feature = "fetch")]
pub fn fetch_maybe_lock_escrow(
    rpc: &solana_client::rpc_client::RpcClient,
    address: &Pubkey,
) -> Result<super::MaybeAccount<LockEscrow>, Error> {
    let accounts = fetch_all_maybe_lock_escrow(rpc, vec![address])?;
    Ok(accounts[0].clone())
}

#[cfg(feature = "fetch")]
pub fn fetch_all_maybe_lock_escrow(
    rpc: &solana_client::rpc_client::RpcClient,
    addresses: Vec<Pubkey>,
) -> Result<Vec<super::MaybeAccount<LockEscrow>>, Error> {
    let accounts = rpc.get_multiple_accounts(&addresses)?;
    let mut decoded_accounts: Vec<super::MaybeAccount<LockEscrow>> = Vec::new();
    for i in 0..addresses.len() {
        let address = addresses[i];
        if let Some(account) = accounts[i].as_ref() {
            let data = LockEscrow::from_bytes(&account.data)?;
            decoded_accounts.push(super::MaybeAccount::Exists(super::DecodedAccount {
                address,
                account: account.clone(),
                data,
            }));
        } else {
            decoded_accounts.push(super::MaybeAccount::NotFound(address));
        }
    }
    Ok(decoded_accounts)
}

#[cfg(feature = "anchor")]
impl anchor_lang::AccountDeserialize for LockEscrow {
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        Ok(Self::deserialize(buf)?)
    }
}

#[cfg(feature = "anchor")]
impl anchor_lang::AccountSerialize for LockEscrow {}

#[cfg(feature = "anchor")]
impl anchor_lang::Owner for LockEscrow {
    fn owner() -> Pubkey {
        crate::AMM_ID
    }
}

#[cfg(feature = "anchor-idl-build")]
impl anchor_lang::IdlBuild for LockEscrow {}

#[cfg(feature = "anchor-idl-build")]
impl anchor_lang::Discriminator for LockEscrow {
    const DISCRIMINATOR: [u8; 8] = [0; 8];
}
