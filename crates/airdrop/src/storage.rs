//! Airdrop storage functions.

use std::path::Path;

use namada_storage::StorageWrite;
use zair_core::schema::config::{AirdropConfiguration, ValueCommitmentScheme};

use crate::storage_key::{
    airdrop_nullifier_key, sapling_note_commitment_root_key,
    sapling_nullifier_gap_root_key, sapling_value_commitment_scheme_key,
    sapling_verifying_key,
};

/// Initialize airdrop sapling configuration from files in the airdrop
/// directory.
///
/// Reads:
/// - `<airdrop_dir>/config.json` - contains note_commitment_root,
///   nullifier_gap_root, value_commitment_scheme
/// - `<airdrop_dir>/setup-sapling-vk.params` - the Groth16 verifying key
///
/// # Panics
/// Panics if the airdrop directory or required files are missing.
pub fn init_storage<S: StorageWrite>(
    storage: &mut S,
    airdrop_dir: &Path,
) -> namada_storage::Result<()> {
    // Read Airdrop config.
    let config_path = airdrop_dir.join("config.json");
    let config_content = std::fs::read_to_string(&config_path)
        .expect("Failed to read airdrop config.json");
    let config: AirdropConfiguration = serde_json::from_str(&config_content)
        .expect("Failed to parse airdrop config.json");

    let sapling = config
        .sapling
        .expect("Airdrop configuration did not contain a sapling snapshot");

    // Read and write verifying key.
    let vk_path = airdrop_dir.join("setup-sapling-vk.params");
    let vk_bytes =
        std::fs::read(&vk_path).expect("Failed to read airdrop verifying key");

    storage.write_bytes(&sapling_verifying_key(), vk_bytes)?;

    // Write note commitment root.
    storage.write_bytes(
        &sapling_note_commitment_root_key(),
        &sapling.note_commitment_root,
    )?;

    // Write nullifier gap root.
    storage.write_bytes(
        &sapling_nullifier_gap_root_key(),
        &sapling.nullifier_gap_root,
    )?;

    // Write value commitment scheme
    let scheme = match sapling.value_commitment_scheme {
        ValueCommitmentScheme::Native => 0u8,
        ValueCommitmentScheme::Sha256 => 1u8,
    };
    storage.write(&sapling_value_commitment_scheme_key(), scheme)?;

    Ok(())
}

/// Writes a nullifier to storage.
pub fn reveal_nullifier(
    storage: &mut impl namada_storage::StorageWrite,
    nullifier: &[u8; 32],
) -> namada_storage::Result<()> {
    storage.write(&airdrop_nullifier_key(nullifier), ())
}

/// Checks if a nullifier has already been revealed.
pub fn is_nullifier_revealed(
    storage: &impl namada_storage::StorageRead,
    nullifier: &[u8; 32],
) -> namada_storage::Result<bool> {
    let key = airdrop_nullifier_key(nullifier);
    storage.has_key(&key)
}
