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
    diagnostics: String,
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
    current_patient: u64,
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

//Represents payload for adding a patient
#[derive(candid::CandidType, Serialize, Deserialize)]
struct PatientPayLoad {
    name: String,
    date_of_birth: String, //Format: DD-MM-YYYY
    gender: String,
    ethncity: String,
    address: String,
    phone_number: String,
    email: String, //Optional
    next_of_kin: String,
    kins_phone_number: String,
}

impl Default for PatientPayLoad {
    fn default() -> Self {
        PatientPayLoad{
            name: String::default(),
            date_of_birth: String::default(), //Format: DD-MM-YYYY
            gender: String::default(),
            ethncity: String::default(),
            address: String::default(),
            phone_number: String::default(),
            email: String::default(), //Optional
            next_of_kin: String::default(),
            kins_phone_number: String::default(),
        }
    }
}

//Represents payload for adding a Doctor
#[derive(candid::CandidType, Serialize, Deserialize)]
struct DoctorPayLoad{
    name: String,
    email: String,
    phone_number: String,
    speciality: String,
}

impl Default for DoctorPayLoad {
    fn default() -> Self {
        DoctorPayLoad{
            name: String::default(),
            email: String::default(),
            phone_number: String::default(),
            speciality: String::default(),
        }
    }
}

//thread-local variables that will hold our canister's state
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static PATIENT_STORAGE: RefCell<StableBTreeMap<u64, Patient, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static DOCTOR_STORAGE: RefCell<StableBTreeMap<u64, Doctor, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));
}
