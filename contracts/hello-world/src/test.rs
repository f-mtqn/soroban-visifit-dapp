#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test_workout_recording() {
    let env = Env::default();
    env.mock_all_auths(); // Mengizinkan simulasi otorisasi

    let contract_id = env.register(VisiFitContract, ());
    let client = VisiFitContractClient::new(&env, &contract_id);

    // Membuat alamat simulasi untuk Admin (n8n) dan Pengguna
    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    // 1. Inisialisasi
    client.initialize(&admin);

    // 2. Admin mencatat 15 repetisi untuk User
    client.record_workout(&admin, &user, &15);

    // 3. Verifikasi apakah data tersimpan
    let total_reps = client.get_total_reps(&user);
    assert_eq!(total_reps, 15);

    // 4. Admin mencatat sesi latihan kedua (10 repetisi)
    client.record_workout(&admin, &user, &10);
    
    // 5. Total harus menjadi 25
    let updated_reps = client.get_total_reps(&user);
    assert_eq!(updated_reps, 25);
}

#[test]
#[should_panic(expected = "Unauthorized: Only the AI Oracle (Admin) can record workouts")]
fn test_unauthorized_recording() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(VisiFitContract, ());
    let client = VisiFitContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let fake_admin = Address::generate(&env); // Seseorang yang mencoba meretas
    let user = Address::generate(&env);

    client.initialize(&admin);

    // fake_admin mencoba memasukkan data palsu (akan menyebabkan panic)
    client.record_workout(&fake_admin, &user, &100);
}