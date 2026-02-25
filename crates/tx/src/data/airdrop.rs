use namada_core::address::Address;
use namada_core::borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use namada_core::token::Amount;
use serde::{Deserialize, Serialize};

/// Airdrop claim data containing ZK proof information.
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[derive(
    Debug,
    Clone,
    PartialEq,
    BorshSerialize,
    BorshDeserialize,
    BorshSchema,
    Serialize,
    Deserialize
)]
pub struct AirdropClaimData {
    /// The Groth16 proof (192 bytes as hex string)
    pub zkproof: String,
    /// The re-randomized spend verification key (32 bytes as hex string)
    pub rk: String,
    /// Native value commitment (32 bytes as hex string), required for native scheme
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cv: Option<String>,
    /// SHA-256 value commitment (32 bytes as hex string), required for sha256 scheme
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cv_sha256: Option<String>,
    /// The airdrop nullifier (32 bytes as hex string)
    pub airdrop_nullifier: String,
}

/// A tx data type to hold airdrop claim data.
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[derive(
    Debug,
    Clone,
    PartialEq,
    BorshSerialize,
    BorshDeserialize,
    BorshSchema,
    Serialize,
    Deserialize,
)]
pub struct ClaimAirdrop {
    /// The target of the airdrop.
    pub target: Address,
    /// Token address to claim.
    pub token: Address,
    /// Amount to claim.
    pub amount: Amount,
    /// Claim data containing ZK proof information.
    pub claim_data: AirdropClaimData,
}

#[cfg(any(test, feature = "testing"))]
/// Tests and strategies for airdrop transactions.
pub mod tests {
    use namada_core::address::testing::arb_non_internal_address;
    use namada_core::token::testing::arb_amount;
    use proptest::prop_compose;

    use super::*;

    prop_compose! {
        /// Generate arbitrary airdrop claim data.
        pub fn arb_airdrop_claim_data()(
            zkproof in "[0-9a-fA-F]{384}", // 192 bytes = 384 hex chars
            rk in "[0-9a-fA-F]{64}",        // 32 bytes = 64 hex chars
            cv in "[0-9a-fA-F]{64}",        // 32 bytes = 64 hex chars
            airdrop_nullifier in "[0-9a-fA-F]{64}", // 32 bytes = 64 hex chars
        ) -> AirdropClaimData {
            AirdropClaimData {
                zkproof,
                rk,
                cv: Some(cv),
                cv_sha256: None,
                airdrop_nullifier,
            }
        }
    }

    prop_compose! {
        /// Generate an arbitrary airdrop claim.
        pub fn arb_airdrop_claim()(
            target in arb_non_internal_address(),
            token in arb_non_internal_address(),
            amount in arb_amount(),
            claim_data in arb_airdrop_claim_data(),
        ) -> ClaimAirdrop {
            ClaimAirdrop { target, token, amount, claim_data }
        }
    }
}
