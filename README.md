# Health Management System Canister

This repository contains the code for a simple health management system implemented as an Internet Computer (IC) canister. This system allows users to manage patients, doctors, and rooms.

## Features:

- Add, update, and delete patients
- Add, update, and delete doctors
- Add, update, and delete rooms
- Assign patients to doctors
- Assign doctors to rooms
- Add diagnosis for a patient
- Search for all patients, doctors, and rooms

## Running the project locally

Clone the repository

```bash
git clone https://github.com/PGOpenda/Health-Canister.git
```

Change to the directory

```bash
cd health-canister
```

Start the replica

```bash
dfx start --background
```

Deploy your canisters to the replica and generates your candid interface

```bash
dfx deploy
```

If you have made changes to your backend canister, you can generate a new candid interface with

```bash
npm run generate
```