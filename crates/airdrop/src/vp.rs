//! Airdrop VP

use std::collections::BTreeSet;
use std::marker::PhantomData;

use namada_core::address::Address;
use namada_core::storage::Key;
use namada_tx::BatchedTxRef;
use namada_tx::action::{Action, AirdropAction};
use namada_vp_env::{Error, Result, VpEnv};
use thiserror::Error;

use crate::storage_key::{airdrop_nullifier_key, is_airdrop_nullifier_key};

#[derive(Error, Debug)]
pub enum VpError {
    #[error("Airdrop action not authorized by {0}")]
    Unauthorized(Address),
    #[error("No Airdrop action found")]
    NoAction,
    #[error("Nullifier already used: {0}")]
    NullifierAlreadyUsed(String),
    #[error("Nullifier not properly committed")]
    NullifierNotCommitted,
    #[error("Unexpected nullifier key changed: {0}")]
    UnexpectedNullifierKey(Key),
}

impl From<VpError> for Error {
    fn from(value: VpError) -> Self {
        Error::new(value)
    }
}

/// Airdrop VP
pub struct AirdropVp<'ctx, CTX> {
    pub _marker: PhantomData<&'ctx CTX>,
}

impl<'ctx, CTX> AirdropVp<'ctx, CTX>
where
    CTX: VpEnv<'ctx> + namada_tx::action::Read<Err = Error>,
{
    /// Run the validity predicate
    pub fn validate_tx(
        ctx: &'ctx CTX,
        _batched_tx: &BatchedTxRef<'_>,
        keys_changed: &BTreeSet<Key>,
        verifiers: &BTreeSet<Address>,
    ) -> Result<()> {
        let actions = ctx.read_actions()?;

        let mut claimed_nullifiers = BTreeSet::new();

        for action in &actions {
            if let Action::Airdrop(AirdropAction::Claim {
                target,
                message,
                ..
            }) = action
            {
                if !verifiers.contains(target) {
                    return Err(VpError::Unauthorized(target.clone()).into());
                }

                let nullifier_key = airdrop_nullifier_key(message);

                // Check if nullifier has already been used before.
                if ctx.has_key_pre(&nullifier_key)? {
                    return Err(
                        VpError::NullifierAlreadyUsed(message.clone()).into()
                    );
                }

                // Check if nullifier has already been used in this transaction.
                if claimed_nullifiers.contains(message) {
                    return Err(
                        VpError::NullifierAlreadyUsed(message.clone()).into()
                    );
                }

                ctx.read_bytes_post(&nullifier_key)?
                    .is_some_and(|value| value.is_empty())
                    .then_some(())
                    .ok_or(VpError::NullifierNotCommitted)?;

                claimed_nullifiers.insert(message.clone());
            }
        }

        if claimed_nullifiers.is_empty() {
            return Err(VpError::NoAction.into());
        }

        for nullifier_key in keys_changed
            .iter()
            .filter(|key| is_airdrop_nullifier_key(key))
        {
            let expected_key = claimed_nullifiers
                .iter()
                .find(|msg| &airdrop_nullifier_key(msg) == nullifier_key);
            if expected_key.is_none() {
                return Err(VpError::UnexpectedNullifierKey(
                    nullifier_key.clone(),
                )
                .into());
            }
        }

        Ok(())
    }
}
