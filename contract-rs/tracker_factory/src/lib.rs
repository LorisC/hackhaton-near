use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, Promise};
use near_sdk::collections::{Set};
use serde::Serialize;

const SINGLE_CALL_GAS: u64 = 100000000000000;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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
            tracker_created: Set::new(env::current_account_id().into_bytes())
        }
    }

    pub fn create_tracker(&mut self,
                          location: Location,
                          weight: u64,
                          trash_type: String, ) {
        let tracker_id = format!("{}.{}", self.tracker_created.len(), env::current_account_id());
        let create_tracker = Promise::new(tracker_id.clone())
            .create_account()
            .function_call(b"new".to_vec(),
                           serde_json::to_vec(&TrackerArgs {
                               creator: env::predecessor_account_id(),
                               trash_type,
                               weight,
                               location,
                           }).unwrap(),
                           0,
                           SINGLE_CALL_GAS);
        let create_tracker_account = Promise::new(env::predecessor_account_id())
            .deploy_contract(include_bytes!("../../track_trash_account/res/track_trash_account.wasm").to_vec())
            .function_call(b"new".to_vec(),
                           serde_json::to_vec(&TrackerAccountNewArgs {}).unwrap(),
                           0,
                           SINGLE_CALL_GAS);

        let add_tracker_to_account = Promise::new(env::predecessor_account_id())
            .function_call(b"add_tracker".to_vec(),
                           serde_json::to_vec(&TrackerAccountAddTrackerArgs {
                               tracker: tracker_id.clone()
                           }).unwrap(),
                           0,
                           SINGLE_CALL_GAS);

        self.tracker_created.insert(&tracker_id.clone());

        create_tracker
            .then(create_tracker_account)
            .then(add_tracker_to_account);
    }
}
