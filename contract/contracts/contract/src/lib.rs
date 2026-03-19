#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol, Map, Address};

#[contract]
pub struct CharityDonationTracker;

#[contractimpl]
impl CharityDonationTracker {

    // Add donation for a charity
    pub fn donate(env: Env, donor: Address, charity: Symbol, amount: i128) {
        donor.require_auth();

        let mut donations: Map<Symbol, i128> = env
            .storage()
            .instance()
            .get(&symbol_short!("DONATIONS"))
            .unwrap_or(Map::new(&env));

        let current = donations.get(charity.clone()).unwrap_or(0);
        donations.set(charity.clone(), current + amount);

        env.storage().instance().set(&symbol_short!("DONATIONS"), &donations);
    }

    // Get total donation for a charity
    pub fn get_total(env: Env, charity: Symbol) -> i128 {
        let donations: Map<Symbol, i128> = env
            .storage()
            .instance()
            .get(&symbol_short!("DONATIONS"))
            .unwrap_or(Map::new(&env));

        donations.get(charity).unwrap_or(0)
    }
}