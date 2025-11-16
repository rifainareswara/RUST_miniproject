// src/models.rs

// Impor tipe data ObjectId, ini adalah ID unik yang dipakai MongoDB
use mongodb::bson::oid::ObjectId;
// Impor Serde, library untuk mengubah struct Rust ke JSON (Serialize)
// dan dari JSON ke struct Rust (Deserialize)
use serde::{Deserialize, Serialize};

// 'pub' berarti struct ini bisa diakses dari file lain (seperti handlers.rs)
// #[derive(...)] adalah macro yang otomatis menambahkan fungsionalitas:
// - Debug:      Agar bisa di-print ke konsol (untuk debugging)
// - Serialize:  Struct -> JSON (Saat mengirim data ke klien/frontend)
// - Deserialize:JSON -> Struct (Saat menerima data dari klien/frontend)
// - Clone:      Agar struct ini bisa diduplikasi
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    // Atribut 'serde' ini SANGAT PENTING untuk MongoDB
    // 1. rename = "_id": Di Rust kita pakai 'id', tapi di MongoDB namanya '_id'.
    //    Ini memberitahu Serde untuk memetakan nama tersebut.
    // 2. skip_serializing_if = "Option::is_none": Saat membuat task baru,
    //    'id' akan berisi 'None' (kosong). Perintah ini mencegah 'id'
    //    dikirim ke MongoDB saat nilainya 'None', sehingga MongoDB bisa
    //    meng-generate ID baru secara otomatis.
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub deskripsi: String,
}

// Ini adalah struct terpisah untuk MENANGKAP INPUT saat membuat task baru.
// Kenapa dipisah? Karena saat user membuat task, mereka HANYA mengirim
// 'deskripsi'. Mereka tidak mengirim 'id' (karena 'id' belum ada).
// Ini adalah praktik desain API yang sangat baik.
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTask {
    pub deskripsi: String,
}