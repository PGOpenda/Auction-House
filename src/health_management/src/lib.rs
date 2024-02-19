//Import dependencies
#[macro_use]
extern crate serde;
use candid::{Decode, Encode, Principal};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

//types to store our canisters state & generate unique IDs
type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

//Defining our structs
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Auction {
    id: u64,
    item_name: String,
    description: String,
    starting_bid: u64,
    current_bid: u64,
    auction_end_time: u64,
    winner: Option<Principal>,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct NewAuction {
    item_name: String,
    description: String,
    starting_bid: u64,
    auction_duration: u64,
}

//trait implemented for a struct that is stored in a stable struct
impl Storable for Auction {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Auction {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

//thread local variables that will hold canisters state
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static STORAGE: RefCell<StableBTreeMap<u64, Auction, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
}

#[ic_cdk::query]
fn get_auction(id: u64) -> Result<Auction, Error> {
    match _get_auction(&id) {
        Some(auction) => Ok(auction),
        None => Err(Error::NotFound {
            msg: format!("an auction with id={} not found", id),
        }),
    }
}

#[ic_cdk::update]
fn create_auction(new_auction: NewAuction) -> Option<Auction> {
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");
    let now = time();
    let auction_end_time = now + new_auction.auction_duration;
    let auction = Auction {
        id,
        item_name: new_auction.item_name,
        description: new_auction.description,
        starting_bid: new_auction.starting_bid,
        current_bid: new_auction.starting_bid,
        auction_end_time,
        winner: None,
    };
    do_insert(&auction);
    Some(auction)
}

// helper method to perform insert.
fn do_insert(auction: &Auction) {
    STORAGE.with(|service| service.borrow_mut().insert(auction.id, auction.clone()));
}

// a helper method to get an auction by id. used in get_auction/update_auction
fn _get_auction(id: &u64) -> Option<Auction> {
    STORAGE.with(|service| service.borrow().get(id))
}

// need this to generate candid
ic_cdk::export_candid!();