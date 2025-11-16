// src/main.rs

// 1. DEKLARASI MODUL
// Memberitahu Rust untuk mencari file 'src/handlers.rs' dan 'src/models.rs'
// dan menyertakan isinya ke dalam kompilasi.
pub mod handlers;
pub mod models;

// Impor 'Cors' untuk mengizinkan frontend (React/Vue/dll)
// mengakses API ini dari domain yg berbeda.
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use mongodb::{Client, Collection};
use std::io::Result; // Tipe data 'Result' standar untuk 'main'

// 2. IMPOR FUNGSI DAN STRUCT
// Setelah modul dideklarasikan, kita bisa mengimpor fungsi spesifik darinya
use handlers::{create_task, delete_task, get_tasks, update_task};
// Kita juga butuh struct 'Task' untuk mendefinisikan tipe Collection
use models::Task;

// 3. DEFINISI APP STATE
// Struct ini akan menyimpan data yg ingin kita 'share' ke semua handler.
// Dalam kasus ini, adalah koneksi ke 'collection' MongoDB.
pub struct AppState {
    collection: Collection<Task>,
}

// 'actix_web::main' adalah makro yg menyiapkan async runtime (Tokio)
#[actix_web::main]
async fn main() -> Result<()> {
    // 4. KONEKSI DATABASE
    // Ganti string koneksi ini dengan milik Anda
    let uri = "mongodb+srv://rnrifai12:test.,@rnrifai.sai9y.mongodb.net/";
    let client = Client::with_uri_str(uri)
        .await
        .expect("Gagal terhubung ke MongoDB."); // .expect akan 'panic' jika gagal

    // Tentukan database dan collection yg ingin dipakai
    let db = client.database("todo_db");
    let collection: Collection<Task> = db.collection("tasks");

    println!("✅ Server API murni berjalan di http://localhost:3000");

    // 5. MEMBUAT APP STATE
    // Kita bungkus 'AppState' kita dalam 'web::Data'.
    // 'web::Data' adalah 'smart pointer' (seperti Arc) yg memungkinkan
    // 'AppState' di-share secara aman antar-thread oleh Actix.
    let app_state = web::Data::new(AppState {
        collection: collection.clone(), // .clone() krn 'collection' akan di-pindah
    });

    // 6. KONFIGURASI DAN JALANKAN SERVER
    HttpServer::new(move || {
        // 'move' di sini penting agar 'closure' ini mengambil
        // kepemilikan 'app_state' yg kita definisikan di luar.

        // Konfigurasi CORS (Cross-Origin Resource Sharing)
        // Ini WAJIB agar frontend Anda (misal di localhost:5173)
        // diizinkan memanggil API ini (di localhost:3000).
        let cors = Cors::default()
            .allow_any_origin() // Izinkan panggilan dari domain manapun
            .allow_any_method() // Izinkan metode (GET, POST, PUT, DELETE)
            .allow_any_header() // Izinkan header apapun
            .max_age(3600); // Durasi 'preflight' request di-cache

        App::new()
            .wrap(cors) // Terapkan konfigurasi CORS sebagai 'middleware'
            .app_data(app_state.clone()) // Berikan 'app_state' ke aplikasi

            // 7. REGISTRASI RUTE
            // Kita 'mendaftarkan' fungsi-fungsi dari 'handlers.rs'.
            // Actix akan otomatis mencocokkan makro #[get], #[post], dll
            // yg ada di dalam file 'handlers.rs'.
            .service(get_tasks)
            .service(create_task)
            .service(delete_task)
            .service(update_task)
    })
        .bind(("127.0.0.1", 3000))? // Ikat server ke alamat dan port
        .run() // Jalankan server
        .await // Tunggu sampai server berhenti
}


// // src/main.rs
//
// pub mod handlers;
// pub mod models;
//
// use actix_cors::Cors;
// use actix_files::Files;
// use actix_web::{web, App, HttpServer};
// use mongodb::{Client, Collection};
// use std::io::Result;
//
// // 1. Impor 'update_task'
// use handlers::{create_task, delete_task, get_tasks, update_task};
// use models::Task;
//
// pub struct AppState {
//     collection: Collection<Task>,
// }
//
// #[actix_web::main]
// async fn main() -> Result<()> {
//     let uri = "mongodb+srv://rnrifai12:Ngatini12.,@rnrifai.sai9y.mongodb.net/";
//     let client = Client::with_uri_str(uri)
//         .await
//         .expect("Gagal terhubung ke MongoDB.");
//
//     let db = client.database("todo_db");
//     let collection: Collection<Task> = db.collection("tasks");
//
//     // Pastikan port 3000
//     println!("✅ Server berjalan di http://127.0.0.1:3000");
//
//     let app_state = web::Data::new(AppState {
//         collection: collection.clone(),
//     });
//
//     HttpServer::new(move || {
//         let cors = Cors::default()
//             .allow_any_origin()
//             .allow_any_method()
//             .allow_any_header()
//             .max_age(3600);
//
//         App::new()
//             .wrap(cors)
//             .app_data(app_state.clone())
//             .service(
//                 web::scope("/api")
//                     .service(get_tasks)
//                     .service(create_task)
//                     .service(delete_task)
//                     .service(update_task), // <-- 2. TAMBAHKAN BARIS INI
//             )
//             .service(
//                 Files::new("/", "./static")
//                     .index_file("index.html")
//                     .default_handler(web::route().to(move || async {
//                         let s = std::fs::read_to_string("./static/index.html").unwrap();
//                         actix_web::HttpResponse::Ok().content_type("text/html").body(s)
//                     })),
//             )
//     })
//         .bind(("127.0.0.1", 3000))? // Pastikan port 3000
//         .run()
//         .await
// }
