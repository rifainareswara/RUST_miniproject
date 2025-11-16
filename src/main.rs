// src/main.rs

// 1. DEKLARASI MODUL
pub mod handlers;
pub mod models;

// Impor 'Cors'
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use mongodb::{Client, Collection};
use std::io::Result; // Tipe data 'Result' standar untuk 'main'
use std::env; // <-- TAMBAHKAN: Impor modul 'env' standar Rust
use dotenvy::dotenv; // <-- TAMBAHKAN: Impor 'dotenv' dari crate 'dotenvy'

// 2. IMPOR FUNGSI DAN STRUCT
use handlers::{create_task, delete_task, get_tasks, update_task};
use models::Task;

// 3. DEFINISI APP STATE
pub struct AppState {
    collection: Collection<Task>,
}

// 'actix_web::main' adalah makro yg menyiapkan async runtime (Tokio)
#[actix_web::main]
async fn main() -> Result<()> {
    // TAMBAHKAN: Panggil fungsi dotenv() di paling atas 'main'
    // Ini akan membaca file .env dan memuatnya ke environment
    dotenv().expect("Gagal memuat file .env");

    // 4. KONEKSI DATABASE
    // UBAH: Ganti string yang di-hardcode dengan 'env::var()'
    // 'env::var' akan membaca variabel dari environment
    let uri = env::var("DATABASE_URL")
        .expect("DATABASE_URL harus di-set di environment");

    // Baris lama Anda:
    // let uri = "mongodb+srv://rnrifai12:test.,@rnrifai.sai9y.mongodb.net/";

    // UBAH: Gunakan '&uri' karena 'env::var' mengembalikan String
    let client = Client::with_uri_str(&uri)
        .await
        .expect("Gagal terhubung ke MongoDB."); // .expect akan 'panic' jika gagal

    // Tentukan database dan collection yg ingin dipakai
    let db = client.database("todo_db");
    let collection: Collection<Task> = db.collection("tasks");

    println!("✅ Server API murni berjalan di http://localhost:3000");

    // 5. MEMBUAT APP STATE
    let app_state = web::Data::new(AppState {
        collection: collection.clone(),
    });

    // 6. KONFIGURASI DAN JALANKAN SERVER
    HttpServer::new(move || {
        // Konfigurasi CORS
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(app_state.clone())

            // 7. REGISTRASI RUTE
            .service(get_tasks)
            .service(create_task)
            .service(delete_task)
            .service(update_task)
    })
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}