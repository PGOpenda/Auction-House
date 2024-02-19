// Importing neccessary dependencies
#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

//Use these types to store our canister's state and generate unique IDs
type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

//Define our Patient Struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Patient {
    id: u64,
    name: String,
    date_of_birth: String, //Format: DD-MM-YYYY
    gender: String,
    ethncity: String,
    address: String,
    phone_number: String,
    email: String, //Optional
    next_of_kin: String,
    kins_phone_number: String,
    registered_on: u64,
}

impl Storable for Patient {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Patient {
    const MAX_SIZE: u32 = 2048;
    const IS_FIXED_SIZE: bool = false;
}

//Define our Doctor Struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Doctor {
    id: u64,
    name: String,
    email: String,
    phone_number: String,
    speciality: String,
}

impl Storable for Doctor {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Doctor {
    const MAX_SIZE: u32 = 2048;
    const IS_FIXED_SIZE: bool = false;
}
