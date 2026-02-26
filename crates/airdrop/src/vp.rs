//! Airdrop VP

use std::collections::BTreeSet;
use std::marker::PhantomData;

use bellman::groth16;
use namada_core::address::Address;
use namada_core::storage::Key;
use namada_tx::BatchedTxRef;
use namada_tx::action::{Action, AirdropAction, AirdropClaimData};
use namada_vp_env::{Error, Result, VpEnv};
use thiserror::Error;
use zair_sapling_proofs::verifier::{
    ValueCommitmentScheme, verify_claim_proof_bytes,
};

use crate::storage_key::{
    airdrop_nullifier_key, is_airdrop_nullifier_key,
    sapling_note_commitment_root_key, sapling_nullifier_gap_root_key,
    sapling_value_commitment_scheme_key, sapling_verifying_key,
};

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
    #[error("ZK proof verification failed: {0}")]
    ZkProofVerificationFailed(String),
    #[error("Missing sapling verifying key in storage")]
    MissingVerifyingKey,
    #[error("Missing sapling note commitment root in storage")]
    MissingNoteCommitmentRoot,
    #[error("Missing sapling nullifier gap root in storage")]
    MissingNullifierGapRoot,
    #[error("Missing sapling value commitment scheme in storage")]
    MissingValueCommitmentScheme,
    #[error("Invalid verifier key: {0}")]
    InvalidVerifierKey(String),
    #[error("Invalid hex encoding: {0}")]
    InvalidHex(String),
    #[error("Invalid value commitment scheme: {0}")]
    InvalidValueCommitmentScheme(String),
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
                claim_data,
                ..
            }) = action
            {
                if !verifiers.contains(target) {
                    return Err(VpError::Unauthorized(target.clone()).into());
                }

                let nullifier = claim_data.airdrop_nullifier;
                let nullifier_key = airdrop_nullifier_key(&nullifier);

                // Check if nullifier has already been used before.
                if ctx.has_key_pre(&nullifier_key)? {
                    return Err(VpError::NullifierAlreadyUsed(hex::encode(
                        nullifier,
                    ))
                    .into());
                }

                // Check if nullifier has already been used in this transaction.
                if claimed_nullifiers.contains(&nullifier) {
                    return Err(VpError::NullifierAlreadyUsed(hex::encode(
                        nullifier,
                    ))
                    .into());
                }

                ctx.read_bytes_post(&nullifier_key)?
                    .is_some_and(|value| value.is_empty())
                    .then_some(())
                    .ok_or(VpError::NullifierNotCommitted)?;

                // Verify the ZK proof.
                verify_zk_proof(ctx, claim_data)?;

                claimed_nullifiers.insert(nullifier);
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

/// Verify the ZK proof for a claim.
fn verify_zk_proof<'ctx, CTX>(
    ctx: &'ctx CTX,
    claim_data: &AirdropClaimData,
) -> Result<()>
where
    CTX: VpEnv<'ctx> + namada_tx::action::Read<Err = Error>,
{
    // Read verifying key from storage.
    let vk_bytes: Vec<u8> = ctx
        .read_bytes_pre(&sapling_verifying_key())?
        .ok_or(VpError::MissingVerifyingKey)?;

    let vk = groth16::VerifyingKey::read(&vk_bytes[..])
        .map_err(|e| VpError::InvalidVerifierKey(e.to_string()))?;
    let pvk = groth16::prepare_verifying_key(&vk);

    // Read note commitment root from storage.
    let note_commitment_root_bytes: [u8; 32] = ctx
        .read_bytes_pre(&sapling_note_commitment_root_key())?
        .ok_or(VpError::MissingNoteCommitmentRoot)?
        .try_into()
        .map_err(|_| VpError::InvalidHex("note_commitment_root".into()))?;

    // Read nullifier gap root from storage.
    let nullifier_gap_root_bytes: [u8; 32] = ctx
        .read_bytes_pre(&sapling_nullifier_gap_root_key())?
        .ok_or(VpError::MissingNullifierGapRoot)?
        .try_into()
        .map_err(|_| VpError::InvalidHex("nullifier_gap_root".into()))?;

    // Read value commitment scheme from storage.
    let scheme_bytes: u8 = ctx
        .read_pre(&sapling_value_commitment_scheme_key())?
        .ok_or(VpError::MissingValueCommitmentScheme)?;

    let scheme = match scheme_bytes {
        0 => ValueCommitmentScheme::Native,
        1 => ValueCommitmentScheme::Sha256,
        n => {
            return Err(
                VpError::InvalidValueCommitmentScheme(n.to_string()).into()
            );
        }
    };

    // Finally, verify the proof.
    verify_claim_proof_bytes(
        &pvk,
        &claim_data.zkproof,
        scheme,
        &claim_data.rk,
        claim_data.cv.as_ref(),
        claim_data.cv_sha256.as_ref(),
        &note_commitment_root_bytes,
        &claim_data.airdrop_nullifier,
        &nullifier_gap_root_bytes,
    )
    .map_err(|e| VpError::ZkProofVerificationFailed(e.to_string()))?;

    Ok(())
}
