use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId};
use near_sdk::collections::{Set};


#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct TrashTrackAccount {
    trackers: Set<AccountId>
}


impl Default for TrashTrackAccount {
    fn default() -> Self {
        panic!("Account not initialized");
    }
}

#[near_bindgen]
impl TrashTrackAccount {
    #[init]
    pub fn  new() -> Self{
        Self {
            trackers: Set::new(env::current_account_id().into_bytes())
        }
    }

    pub fn get_trackers(&self) -> Vec<String>{
        self.trackers.to_vec()
    }

    pub fn add_tracker(&mut self, tracker: AccountId) {
        self.trackers.insert(&tracker.clone());
    }
}
