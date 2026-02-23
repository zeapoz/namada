//! Airdrop storage keys.

use namada_core::storage::{self, DbKeySeg, KeySeg};

use crate::ADDRESS;

/// Key segment prefix for the airdrop nullifiers.
pub const AIRDROP_NULLIFIERS_KEY: &str = "nullifiers";

/// Returns whether the given storage key is an airdrop nullifier key.
pub fn is_airdrop_nullifier_key(key: &storage::Key) -> bool {
    matches!(&key.segments[..],
        [DbKeySeg::AddressSeg(addr),
                 DbKeySeg::StringSeg(prefix),
                 DbKeySeg::StringSeg(_nullifier),
        ] if *addr == ADDRESS && prefix == AIRDROP_NULLIFIERS_KEY)
}

/// Gets a key for the airdrop nullifier storage.
pub fn airdrop_nullifier_key(nullifier: &str) -> storage::Key {
    storage::Key::from(ADDRESS.to_db_key())
        .push(&AIRDROP_NULLIFIERS_KEY.to_owned())
        .expect("Cannot obtain a storage key")
        .push(&nullifier.to_owned())
        .expect("Cannot obtain a storage key")
}
