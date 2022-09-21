use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupSet, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{U128, U64};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, Balance};
use near_sdk::{AccountId, BorshStorageKey, Timestamp};

mod types;
use types::issue_ticket::*;
use types::member::*;
use types::storage_key::StorageKey;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct JoiningEvent {
    issue_tickets: UnorderedMap<TicketCode, IssueTicket>,
    members: UnorderedMap<AccountId, Member>,
    ticket_owners: UnorderedMap<AccountId, UnorderedSet<TicketCode>>,
    owner: AccountId,
    admin_accounts: LookupSet<AccountId>,
    price_ticket: Balance,
}

// using version for struct to update

impl Default for JoiningEvent {
    fn default() -> Self {
        Self {
            issue_tickets: UnorderedMap::new(StorageKey::IssueTicketKey),
            members: UnorderedMap::new(StorageKey::MemberKey),
            ticket_owners: UnorderedMap::new(StorageKey::TicketOwnerKey),
            owner: AccountId::new_unchecked(String::from("")),
            admin_accounts: LookupSet::new(StorageKey::OwnerKey),
            price_ticket: 0u128,
        }
    }
}

#[near_bindgen]
impl JoiningEvent {
    #[init]
    pub fn new() -> Self {
        let owner = env::signer_account_id();
        let mut admin_accounts = LookupSet::new(StorageKey::OwnerKey);
        admin_accounts.insert(&owner);
        JoiningEvent {
            issue_tickets: UnorderedMap::new(StorageKey::IssueTicketKey),
            members: UnorderedMap::new(StorageKey::MemberKey),
            ticket_owners: UnorderedMap::new(StorageKey::TicketOwnerKey),
            owner,
            admin_accounts,
            price_ticket: 0u128,
        }
    }

    pub fn get_owner(&self) -> AccountId {
        self.owner.clone()
    }

    pub fn set_admin_account(&mut self, account_id: AccountId) {
        let signer = env::signer_account_id();
        if self.owner != signer {
            env::panic_str("ERR_NOT_OWNER");
        }
        self.admin_accounts.insert(&account_id);
    }

    pub fn issue_ticket(&mut self, number_of_tickets: u8) {
        let min = 1u8;
        let max = 100u8;
        let last_ticket_number = self.issue_tickets.len();
        if number_of_tickets < min {
            env::panic_str("Number of tickets must be 1 or less than or equal 100");
        }
        if number_of_tickets > max {
            env::panic_str("Number of tickets must be less than or equal 100");
        }
        let mut from_number_ticket = last_ticket_number + 1;
        let to_number_ticket = last_ticket_number + number_of_tickets as u64;
        while from_number_ticket <= to_number_ticket {
            let new_ticket = IssueTicket {
                code: from_number_ticket,
                is_used: false,
                created_at: env::block_timestamp(),
            };
            self.issue_tickets.insert(&from_number_ticket, &new_ticket);
            from_number_ticket += 1;
        }
    }

    pub fn get_info_ticket(&self, ticket_code: U64) -> Option<IssueTicket> {
        let ticket = self.issue_tickets.get(&ticket_code.into());

        ticket
    }

    pub fn get_member(&self, account_id: AccountId) -> Option<Member> {
        let member = self.members.get(&account_id);

        member
    }

    pub fn register(&mut self, email: String) -> RegisterMemberResult {
        let signer = &env::signer_account_id();
        let member = self.members.get(&env::signer_account_id());
        match member {
            Some(_mb) => {
                return RegisterMemberResult {
                    success: false,
                    message: "This account has already resgistered exist".to_string(),
                }
            }
            None => {
                let new_member = Member {
                    account_id: signer.clone(),
                    email,
                    join_at: env::block_timestamp(),
                    is_active: false,
                };
                self.members.insert(signer, &new_member);
                return RegisterMemberResult {
                    success: true,
                    message: "Register successfully".to_string(),
                };
            }
        }
    }

    pub fn set_price_ticket(&mut self, price_ticket: U128) {
        let signer_id = env::signer_account_id();
        if signer_id != self.owner && !self.admin_accounts.contains(&signer_id) {
            env::panic_str("You can not call this method");
        }

        self.price_ticket = price_ticket.into();
    }

    // buy ticket
    #[payable]
    pub fn buy_ticket(&mut self, ticket_code: U64) {
        // price per ticket is
        let account_id = env::signer_account_id();
        let price_ticket = self.price_ticket;
        let price_paid = env::attached_deposit();
        if price_ticket > price_paid {
            env::panic_str("The price paid is not enough");
        }
        let ticket_code_cvt: TicketCode = ticket_code.into();
        let ticket_info = self.issue_tickets.get(&ticket_code_cvt);
        match ticket_info {
            Some(ticket) => {
                if ticket.is_used {
                    env::panic_str("This ticket is used");
                }
                let mut ticket_updated = ticket.clone();
                ticket_updated.is_used = true;
                self.issue_tickets.insert(&ticket_code_cvt, &ticket_updated);

                match &mut self.ticket_owners.get(&account_id) {
                    Some(ticket_owner) => {
                        ticket_owner.insert(&ticket_code_cvt);
                    }
                    None => {
                        let mut new_owner_tickets = UnorderedSet::new(StorageKey::TicketOwnerKey);
                        new_owner_tickets.insert(&ticket_code_cvt);
                        self.ticket_owners.insert(&account_id, &new_owner_tickets);
                    }
                }
            }
            None => env::panic_str("Ticket code is not found"),
        }
    }

    #[result_serializer(borsh)]
    pub fn get_tickets(&self, account_id: AccountId) -> UnorderedSet<TicketCode> {
        let tickets = self.ticket_owners.get(&account_id);

        match tickets {
            Some(tks) => tks,
            None => UnorderedSet::new(StorageKey::TicketOwnerKey)
        }
    }

    // create event
    // join event
    // check event
    //
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, MockedBlockchain};

    fn get_context(is_view: bool, deposit: Balance) -> VMContextBuilder {
        let mut builder: VMContextBuilder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(accounts(0))
            .predecessor_account_id(accounts(0))
            .is_view(is_view);

        if deposit > 0 {
            builder.attached_deposit(deposit);
        }
        builder
    }

    #[test]
    fn test_init_contract() {
        let context = get_context(false, 0);
        testing_env!(context.build());

        let contract = JoiningEvent::new();

        assert_eq!(contract.get_owner(), accounts(0));
    }

    #[test]
    fn test_register() {
        let context = get_context(false, 0);
        testing_env!(context.build());

        let mut contract = JoiningEvent::new();
        env::log_str("Hello");

        contract.register(String::from("myemail@gmail.com"));
        let user = contract.get_member(accounts(0)).unwrap();
        assert_eq!(user.email, String::from("myemail@gmail.com"));
    }

    #[test]
    fn test_issue_ticket() {
        let context = get_context(false, 0);
        testing_env!(context.build());

        let mut contract = JoiningEvent::new();

        contract.issue_ticket(20);

        let mock_ticket = IssueTicket {
            code: 20,
            is_used: false,
            created_at: 0,
        };
        let issue_ticket = contract.get_info_ticket(U64(20)).unwrap();
        assert_eq!(mock_ticket, issue_ticket);
    }

    #[test]
    fn set_price_ticket() {
        let context = get_context(false, 0);
        testing_env!(context.build());

        let mut contract = JoiningEvent::new();
        let price_ticket = 10000000u128;
        contract.set_price_ticket(U128(price_ticket));

        assert_eq!(contract.price_ticket, price_ticket);
    }

    #[test]
    fn user_buy_ticket() {
        let price_ticket = 10000000u128;
        let context = get_context(false, price_ticket);
        testing_env!(context.build());

        let mut contract = JoiningEvent::new();
        contract.issue_ticket(20);
        contract.set_price_ticket(U128(price_ticket));
        contract.buy_ticket(U64(1u64));

        let info_ticket = contract.get_info_ticket(U64(1)).unwrap();
        let ticket_codes = contract.ticket_owners.get(&accounts(0)).unwrap();
        assert_eq!(true, ticket_codes.contains(&info_ticket.code));

        let tickets = contract.get_tickets(accounts(0));
        println!("tickets {:?}", tickets);
    }
}
