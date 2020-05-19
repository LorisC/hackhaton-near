use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId};
use near_sdk::collections::{Set, Map, Vector};


#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub type Location = String;
pub type Transformation = Set<String>;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct TrackTrash {
    owner: AccountId,
    location: Location,
    weight: u64,
    trash_type: String,
    transformation_by_owner: Map<AccountId, Transformation>,
}


impl Default for TrackTrash {
    fn default() -> Self {
        panic!("Tracker not initialized!")
    }
}

#[near_bindgen]
impl TrackTrash {
    #[init]
    pub fn new(creator: AccountId, location: Location, weight: u64, trash_type: String) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            weight,
            trash_type,
            owner: creator.clone(),
            location: location.clone(),
            transformation_by_owner: Map::new(env::current_account_id().into_bytes()),
        }
    }

    pub fn transfer_ownership(&mut self, new_owner: String) {
        self.assert_owner();
        self.owner = new_owner;
    }

    pub fn change_location(&mut self, location: Location) {
        self.assert_owner();
        self.location = location;
    }

    pub fn transform(&mut self, weight: u64, transformation: String) {
        self.assert_owner();
        assert!(weight <= self.weight, "Cannot transform more weight than the wight of the trash");
        self.weight -= weight;
        let mut transfo = self.get_transformation();
        transfo.insert(&transformation.clone());
        self.transformation_by_owner.insert(&self.owner.clone(), &transfo);
    }

    pub fn get_owner(&self) -> AccountId {
        self.owner.clone()
    }

    pub fn get_location(&self) -> Location {
        self.location.clone()
    }

    pub fn get_weight(&self) -> u64 {
        self.weight
    }

    pub fn get_type(&self) -> String {
        self.trash_type.clone()
    }

    pub fn get_transformation_by_owner(&self, owner: AccountId) -> Vec<String> {
        self.transformation_by_owner.get(&owner.clone()).unwrap_or_else(|| Set::new(b"a".to_vec())).to_vec()
    }

    pub fn get_owners(&self) -> Vec<String> {
        self.transformation_by_owner.keys().collect()
    }

    pub fn get_info(&self) -> Vec<String>{
        let mut vec = Vec::new();

        vec.push( self.owner.clone());
        vec.push(self.location.clone());
        vec.push(self.weight.to_string());
        vec.push(self.trash_type.clone());

        vec
    }
}

impl TrackTrash {
    fn get_transformation(&self) -> Set<String> {
        self.transformation_by_owner.get(&self.owner.clone()).unwrap_or_else(|| Set::new(env::current_account_id().into_bytes()))
    }

    fn assert_owner(&self) {
        assert_eq!(env::predecessor_account_id(), self.owner, "Only owner can transfer the tracker");
    }
}
