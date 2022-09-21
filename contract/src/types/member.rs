
use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Member {
    pub account_id: AccountId,
    pub email: String,
    pub join_at: Timestamp,
    pub is_active: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct RegisterMemberResult {
    pub success: bool,
    pub message: String,
}
