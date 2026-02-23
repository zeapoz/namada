//! Airdrop storage functions.

use crate::storage_key::airdrop_nullifier_key;

/// Writes a nullifier to storage.
pub fn reveal_nullifier(
    storage: &mut impl namada_storage::StorageWrite,
    nullifier: &str,
) -> namada_storage::Result<()> {
    storage.write(&airdrop_nullifier_key(nullifier), ())
}

/// Checks if a nullifier has already been revealed.
pub fn is_nullifier_revealed(
    storage: &impl namada_storage::StorageRead,
    nullifier: &str,
) -> namada_storage::Result<bool> {
    let key = airdrop_nullifier_key(nullifier);
    storage.has_key(&key)
}
