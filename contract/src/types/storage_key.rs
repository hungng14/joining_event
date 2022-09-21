
use crate::*;

#[derive(BorshSerialize, BorshDeserialize, BorshStorageKey)]
pub enum StorageKey {
    IssueTicketKey,
    MemberKey,
    TicketOwnerKey,
    OwnerKey
}