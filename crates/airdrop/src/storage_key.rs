//! Airdrop storage keys.

use namada_core::storage::{self, DbKeySeg, KeySeg};

use crate::ADDRESS;

/// Key segment prefix for the airdrop nullifiers.
pub const AIRDROP_NULLIFIERS_KEY: &str = "nullifiers";
/// Key segment prefix for sapling configuration.
pub const AIRDROP_SAPLING_KEY: &str = "sapling";
/// Key segment for verifying key.
pub const VERIFYING_KEY_KEY: &str = "verifying_key";
/// Key segment for note commitment root.
pub const NOTE_COMMITMENT_ROOT_KEY: &str = "note_commitment_root";
/// Key segment for nullifier gap root.
pub const NULLIFIER_GAP_ROOT_KEY: &str = "nullifier_gap_root";
/// Key segment for value commitment scheme.
pub const VALUE_COMMITMENT_SCHEME_KEY: &str = "value_commitment_scheme";

/// Returns whether the given storage key is an airdrop nullifier key.
pub fn is_airdrop_nullifier_key(key: &storage::Key) -> bool {
    matches!(&key.segments[..],
        [DbKeySeg::AddressSeg(addr),
                 DbKeySeg::StringSeg(prefix),
                 DbKeySeg::StringSeg(_nullifier),
        ] if *addr == ADDRESS && prefix == AIRDROP_NULLIFIERS_KEY)
}

/// Gets a key for the airdrop nullifier storage.
pub fn airdrop_nullifier_key(nullifier: &[u8; 32]) -> storage::Key {
    storage::Key::from(ADDRESS.to_db_key())
        .push(&AIRDROP_NULLIFIERS_KEY.to_owned())
        .expect("Cannot obtain a storage key")
        .push(&hex::encode(nullifier))
        .expect("Cannot obtain a storage key")
}

/// Gets a key for the Sapling verifying key storage.
pub fn sapling_verifying_key() -> storage::Key {
    storage::Key::from(ADDRESS.to_db_key())
        .push(&AIRDROP_SAPLING_KEY.to_owned())
        .expect("Cannot obtain a storage key")
        .push(&VERIFYING_KEY_KEY.to_owned())
        .expect("Cannot obtain a storage key")
}

/// Gets a key for the Sapling note commitment root storage.
pub fn sapling_note_commitment_root_key() -> storage::Key {
    storage::Key::from(ADDRESS.to_db_key())
        .push(&AIRDROP_SAPLING_KEY.to_owned())
        .expect("Cannot obtain a storage key")
        .push(&NOTE_COMMITMENT_ROOT_KEY.to_owned())
        .expect("Cannot obtain a storage key")
}

/// Gets a key for the Sapling nullifier gap root storage.
pub fn sapling_nullifier_gap_root_key() -> storage::Key {
    storage::Key::from(ADDRESS.to_db_key())
        .push(&AIRDROP_SAPLING_KEY.to_owned())
        .expect("Cannot obtain a storage key")
        .push(&NULLIFIER_GAP_ROOT_KEY.to_owned())
        .expect("Cannot obtain a storage key")
}

/// Gets a key for the Sapling value commitment scheme storage.
pub fn sapling_value_commitment_scheme_key() -> storage::Key {
    storage::Key::from(ADDRESS.to_db_key())
        .push(&AIRDROP_SAPLING_KEY.to_owned())
        .expect("Cannot obtain a storage key")
        .push(&VALUE_COMMITMENT_SCHEME_KEY.to_owned())
        .expect("Cannot obtain a storage key")
}
