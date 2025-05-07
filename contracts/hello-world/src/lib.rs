#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Symbol, String, symbol_short, Address, Vec, BytesN};

// Structure to store gas price data
#[contracttype]
#[derive(Clone)]
pub struct GasPrice {
    pub provider: Address,     // Address of the gas price provider
    pub price: u64,            // Current gas price
    pub timestamp: u64,        // When the price was updated
    pub location: String,      // Location identifier
}

// For tracking all registered providers
const PROVIDERS: Symbol = symbol_short!("PROVIDERS");

// For mapping location to the latest gas price
#[contracttype]
pub enum GasTracker {
    Location(String)
}

// For counting number of price updates
const UPDATE_COUNT: Symbol = symbol_short!("UPD_COUNT");

#[contract]
pub struct GasTrackerContract;

#[contractimpl]
impl GasTrackerContract {

    
    // Function to get all registered providers
    pub fn get_providers(env: Env) -> Vec<Address> {
        env.storage().instance().get(&PROVIDERS).unwrap_or(Vec::new(&env))
    }
    
    // Function to get the total count of price updates
    pub fn get_update_count(env: Env) -> u64 {
        env.storage().instance().get(&UPDATE_COUNT).unwrap_or(0)
    }
}