use crate::blockchain::challenge_chain_subslot::ChallengeChainSubSlot;
use crate::blockchain::infused_challenge_chain_subslot::InfusedChallengeChainSubSlot;
use crate::blockchain::reward_chain_subslot::RewardChainSubSlot;
use crate::blockchain::subslot_proofs::SubSlotProofs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SubSlotBundle {
    pub challenge_chain: ChallengeChainSubSlot,
    pub infused_challenge_chain: Option<InfusedChallengeChainSubSlot>,
    pub reward_chain: RewardChainSubSlot,
    pub proofs: SubSlotProofs,
}
