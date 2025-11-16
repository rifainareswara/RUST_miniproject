// src/handlers.rs

// 1. Impor makro 'service' dari actix_web
// Makro #[get], #[post], #[delete], #[put] adalah cara mudah
// untuk mendefinisikan HTTP method dan path (URL) untuk sebuah fungsi.
use actix_web::{get, post, delete, put, web, HttpResponse, Responder};
// Diperlukan untuk melakukan iterasi (looping) pada hasil 'find' dari MongoDB
use futures::stream::StreamExt;
// doc! adalah makro dari MongoDB untuk membuat filter (seperti { _id: ... })
// ObjectId adalah untuk mengubah String ID dari URL menjadi ObjectId MongoDB
use mongodb::bson::{doc, oid::ObjectId};

// Kita import struct dari 'models.rs' dan 'main.rs'
// 'crate::' berarti 'dari root proyek ini' (dimana main.rs berada)
use crate::models::{CreateTask, Task}; // Model data kita
use crate::AppState; // Struct state aplikasi (berisi koneksi DB)

// === READ (GET ALL) ===
// Makro ini mendaftarkan fungsi get_tasks ke path "/tasks"
#[get("/tasks")]
// 'data: web::Data<AppState>' adalah cara Actix 'menyuntikkan' (inject)
// state aplikasi (yang berisi koneksi DB) ke dalam fungsi handler ini.
pub async fn get_tasks(data: web::Data<AppState>) -> impl Responder {
    // Jalankan query 'find'. doc! {} artinya 'tanpa filter' (ambil semua)
    let mut cursor = match data.collection.find(doc! {}, None).await {
        Ok(cursor) => cursor,
        // Jika query gagal, kirim respons 500 Internal Server Error
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    // Siapkan Vec (Vector/Array) kosong untuk menampung hasil
    let mut tasks: Vec<Task> = Vec::new();
    // 'cursor' bersifat async, jadi kita loop pakai 'while let Some'
    while let Some(result) = cursor.next().await {
        match result {
            Ok(task) => tasks.push(task), // Masukkan task yg valid ke Vec
            Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
        }
    }

    // Kirim respons 200 OK dengan body berisi data 'tasks' yg di-serialize
    // menjadi JSON secara otomatis oleh Actix.
    HttpResponse::Ok().json(tasks)
}

// === CREATE (POST) ===
#[post("/tasks")]
pub async fn create_task(
    data: web::Data<AppState>, // Akses ke DB
    // 'new_task_json: web::Json<CreateTask>':
    // Actix secara otomatis mengambil body JSON dari request
    // dan men-deserialize-nya menjadi struct 'CreateTask' kita.
    new_task_json: web::Json<CreateTask>,
) -> impl Responder {
    // Buat struct 'Task' (model DB) dari struct 'CreateTask' (model input)
    let new_task = Task {
        id: None, // 'id' kita set 'None' agar MongoDB meng-generate-nya
        deskripsi: new_task_json.deskripsi.clone(),
    };

    // Masukkan 'new_task' ke database
    match data.collection.insert_one(new_task, None).await {
        Ok(result) => {
            // Jika berhasil, ambil kembali task yg baru dibuat itu
            // (lengkap dgn ID baru) untuk dikirim balik ke user.
            let created_task = data
                .collection
                .find_one(doc! {"_id": result.inserted_id}, None)
                .await;
            match created_task {
                // Kirim 200 OK + data task yg baru dibuat
                Ok(Some(task)) => HttpResponse::Ok().json(task),
                _ => HttpResponse::InternalServerError().body("Gagal mengambil task pasca-pembuatan"),
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

// === DELETE ===
// Path "/tasks/{id}" berarti {id} adalah parameter dinamis dari URL
#[delete("/tasks/{id}")]
pub async fn delete_task(
    data: web::Data<AppState>,
    // 'path: web::Path<String>' mengambil parameter {id} dari URL
    // dan menyimpannya sebagai String
    path: web::Path<String>,
) -> impl Responder {
    // Konversi 'id' (String) dari URL menjadi 'ObjectId' (tipe data MongoDB)
    let id = match ObjectId::parse_str(&path.into_inner()) {
        Ok(id) => id,
        // Jika format ID-nya salah (bukan ObjectId), kirim 400 Bad Request
        Err(_) => return HttpResponse::BadRequest().body("ID tidak valid"),
    };

    // Jalankan query delete_one dengan filter '_id'
    match data.collection.delete_one(doc! { "_id": id }, None).await {
        Ok(result) => {
            // Cek apakah ada dokumen yg terhapus
            if result.deleted_count == 1 {
                HttpResponse::Ok().body("Tugas berhasil dihapus")
            } else {
                // Jika tidak ada yg terhapus (krn ID-nya tidak ada)
                HttpResponse::NotFound().body("Tugas tidak ditemukan")
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

// === UPDATE (PUT) ===
#[put("/tasks/{id}")]
pub async fn update_task(
    data: web::Data<AppState>,
    path: web::Path<String>, // Ambil ID dari URL
    // Ambil data baru dari body JSON.
    // Kita bisa pakai ulang struct 'CreateTask' krn isinya sama ('deskripsi')
    update_data: web::Json<CreateTask>,
) -> impl Responder {
    // 1. Konversi ID dari String ke ObjectId
    let id = match ObjectId::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("ID tidak valid"),
    };

    // 2. Ambil deskripsi baru dari body JSON
    let new_deskripsi = update_data.deskripsi.clone();

    // 3. Buat "dokumen update" khusus MongoDB
    //    Ini memberitahu MongoDB: "Tolong $set (atur) field 'deskripsi'
    //    menjadi nilai 'new_deskripsi'"
    let update_doc = doc! {
        "$set": { "deskripsi": new_deskripsi }
    };

    // 4. Jalankan perintah update_one
    //    Argumen 1: Filter (dokumen mana yg mau di-update)
    //    Argumen 2: Perintah update (apa yg mau diubah)
    match data.collection.update_one(doc! { "_id": id }, update_doc, None).await {
        Ok(result) => {
            // Cek apakah ada dokumen yg cocok (matched)
            if result.matched_count == 1 {
                // Jika berhasil di-update, ambil versi terbaru dari task tsb
                match data.collection.find_one(doc! { "_id": id }, None).await {
                    // Kirim kembali data task yg sudah ter-update
                    Ok(Some(task)) => HttpResponse::Ok().json(task),
                    _ => HttpResponse::NotFound().body("Task tidak ditemukan pasca-update"),
                }
            } else {
                // ID tidak ditemukan di database
                HttpResponse::NotFound().body("Task tidak ditemukan untuk diupdate")
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}