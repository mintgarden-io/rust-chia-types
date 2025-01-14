use crate::blockchain::vdf_proof::VdfProof;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SubSlotProofs {
    pub challenge_chain_slot_proof: VdfProof,
    pub infused_challenge_chain_slot_proof: Option<VdfProof>,
    pub reward_chain_slot_proof: VdfProof,
}
