use crate::*;

#[allow(dead_code)]
pub type TicketCode = u64;


#[derive(BorshDeserialize, BorshSerialize, Debug, Deserialize, Serialize, PartialEq, Clone, Copy)]
#[serde(crate = "near_sdk::serde")]
pub struct IssueTicket {
    pub code: TicketCode,
    pub is_used: bool,
    pub created_at: Timestamp,
}