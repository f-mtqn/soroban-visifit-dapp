#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env};

#[contract]
pub struct VisiFitContract;

#[contractimpl]
impl VisiFitContract {
    // 1. Inisialisasi kontrak dengan menetapkan alamat Admin (Server n8n/Oracle AI)
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&symbol_short!("admin")) {
            panic!("Contract already initialized");
        }
        env.storage().instance().set(&symbol_short!("admin"), &admin);
    }

    // 2. Oracle (n8n) mencatat jumlah repetisi olahraga yang sudah diverifikasi AI
    pub fn record_workout(env: Env, admin: Address, user: Address, reps: u32) {
        // Memastikan yang memanggil fungsi ini benar-benar entitas Admin
        admin.require_auth(); 

        let stored_admin: Address = env.storage().instance().get(&symbol_short!("admin")).unwrap();
        if admin != stored_admin {
            panic!("Unauthorized: Only the AI Oracle (Admin) can record workouts");
        }

        // Mengambil data reps sebelumnya, jika belum ada, mulai dari 0
        let mut current_reps: u32 = env.storage().persistent().get(&user).unwrap_or(0);
        
        // Menambahkan reps baru dari sesi latihan saat ini
        current_reps += reps;
        
        // Menyimpan total reps yang baru ke dalam blockchain
        env.storage().persistent().set(&user, &current_reps);
    }

    // 3. Fungsi publik untuk melihat total olahraga (reps) dari dompet pengguna tertentu
    pub fn get_total_reps(env: Env, user: Address) -> u32 {
        env.storage().persistent().get(&user).unwrap_or(0)
    }
}