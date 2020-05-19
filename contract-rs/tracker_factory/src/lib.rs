use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, Promise};
use near_sdk::collections::{Set, Map};
use serde::Serialize;

const SINGLE_CALL_GAS: u64 = 100000000000000;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Account {
    address: String,
    contact: String,
    description: String,
    name: String,
    trackers: Set<AccountId>,
}

impl Default for Account {
    fn default() -> Self {
        panic!("account not initialized")
    }
}

impl Account {
    pub fn new(account_id: AccountId,
               address: String,
               contact: String,
               description: String,
               name: String, ) -> Self {
        Self {
            address,
            contact,
            description,
            name,
            trackers: Set::new(account_id.into_bytes()),
        }
    }

    pub fn get_trackers(&self) -> Vec<String> {
        self.trackers.iter().collect()
    }

    pub fn add_tracker(&mut self, tracker: AccountId) {
        self.trackers.insert(&tracker.clone());
    }
}


pub type Location = String;

#[derive(Serialize)]
struct TrackerArgs {
    creator: AccountId,
    location: Location,
    weight: u64,
    trash_type: String,
}

#[derive(Serialize)]
struct TrackerAccountNewArgs {}

#[derive(Serialize)]
struct TrackerAccountAddTrackerArgs {
    tracker: AccountId
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct TrackerFactory {
    tracker_created: Set<AccountId>,
    accounts: Map<AccountId, Account>,
}

impl Default for TrackerFactory {
    fn default() -> Self {
        panic!("Factory not initialized!")
    }
}

#[near_bindgen]
impl TrackerFactory {
    #[init]
    pub fn new() -> Self {
        Self {
            tracker_created: Set::new(env::current_account_id().into_bytes()),
            accounts: Map::new(b"a".to_vec()),
        }
    }

    pub fn create_tracker(&mut self,
                          location: Location,
                          weight: u64,
                          trash_type: String, ) -> AccountId {
        assert!(self.is_registered(env::predecessor_account_id()), "Account not registered");
        let tracker_id = format!("{}1.{}", self.tracker_created.len(), env::current_account_id());
        self.tracker_created.insert(&String::from(&tracker_id.clone()));

        let mut account = self.get_account(env::predecessor_account_id());

        account.add_tracker(tracker_id.clone());
        self.set_account(env::predecessor_account_id(), account);


        Promise::new(tracker_id.clone())
            .create_account()
            .deploy_contract(include_bytes!("../../track_contract/res/track_trash.wasm").to_vec())
            .transfer(env::account_balance() / 10)
            .function_call(b"new".to_vec(),
                           serde_json::to_vec(&TrackerArgs {
                               creator: env::predecessor_account_id(),
                               trash_type,
                               weight,
                               location,
                           }).unwrap(),
                           0,
                           SINGLE_CALL_GAS);
        tracker_id
    }

    pub fn get_tracker_created(&self) -> Vec<String> {
        self.tracker_created.to_vec()
    }

    pub fn get_account_tracker(&self, account_id: AccountId) -> Vec<AccountId> {
        let account = self.accounts.get(&account_id.clone());
        assert!(account.is_some(), "account doesn't exist");
        account.unwrap().get_trackers()
    }

    pub fn get_account_nb_tracker(&self, account_id: AccountId) -> u64 {
        self.get_account(account_id).trackers.len()
    }

    pub fn get_nb_accounts(&self) -> u64 {
        self.accounts.len()
    }

    pub fn is_registered(&self, account_id: AccountId) -> bool {
        self.accounts.get(&account_id.clone()).is_some()
    }

    pub fn register(&mut self, address: String,
                    contact: String,
                    description: String,
                    name: String) {
        let opt = self.accounts.get(&env::predecessor_account_id());
        assert!(opt.is_none(), "Account already created");
        let account = Account::new(env::predecessor_account_id(), address, contact, description, name);
        self.accounts.insert(&env::predecessor_account_id(), &account);
    }
}


impl TrackerFactory {
    fn get_account(&self, account_id: AccountId) -> Account {
        let opt = self.accounts.get(&account_id.clone());
        if opt.is_none() {
            return Account::new(account_id, String::from(""), String::from(""), String::from(""), String::from(""));
        }
        opt.unwrap()
    }
    fn set_account(&mut self, account_id: AccountId, account: Account) {
        self.accounts.insert(&account_id, &account);
    }

}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
    }

    #[test]
    fn set_get_message() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = TrackerFactory::new();
        let res = contract.create_tracker(String::from("paris"), 90, String::from("papier"));
        print!("{}", res)
    }
}

