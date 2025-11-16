# **📝 Aplikasi To-Do List Sederhana (CLI) dengan Rust**

Dokumentasi ini menjelaskan proyek aplikasi to-do list berbasis *Command-Line Interface* (CLI) yang dibuat menggunakan bahasa pemrograman Rust. Aplikasi ini dirancang sebagai contoh sederhana untuk pemula yang ingin memahami konsep-konsep dasar Rust seperti variabel, loop, input/output, vector, dan match.

## **📜 Deskripsi**

Aplikasi ini memungkinkan pengguna untuk mengelola daftar tugas langsung dari terminal. Pengguna dapat menambah, melihat, memperbarui, dan menghapus tugas. Data tugas disimpan di dalam memori dan akan hilang saat program ditutup.

## **✨ Fitur**

* ➕ **Menambah Tugas Baru**: Pengguna dapat menambahkan deskripsi tugas baru ke dalam daftar.
* 👀 **Melihat Daftar Tugas**: Menampilkan semua tugas yang ada dengan nomor urut untuk memudahkan referensi.
* 🔄 **Memperbarui Tugas**: Mengubah deskripsi tugas yang sudah ada berdasarkan nomor urutnya.
* ❌ **Menghapus Tugas**: Menghapus tugas dari daftar berdasarkan nomor urutnya.
* 🚪 **Keluar**: Menghentikan aplikasi dengan aman.

## **⚙️ Prasyarat**

Untuk dapat mengkompilasi dan menjalankan kode ini, Anda memerlukan:

1. **Rust Toolchain**: Pastikan Rust sudah terpasang di sistem Anda. Ini mencakup *compiler* rustc dan *build tool* cargo. Anda bisa memasangnya dari [situs resmi Rust](https://www.rust-lang.org/tools/install).
2. **Terminal / Command Prompt**: Aplikasi ini berjalan di lingkungan terminal.

## **🚀 Cara Menjalankan**

1. Buat Proyek Baru dengan Cargo  
   Cara terbaik untuk mengelola proyek Rust adalah dengan cargo. Buka terminal Anda dan jalankan perintah berikut:  
   cargo new todo\_cli  
   cd todo\_cli

   Perintah ini akan membuat direktori baru bernama todo\_cli dengan struktur proyek standar Rust.
2. Salin Kode  
   Buka file src/main.rs yang baru saja dibuat, hapus konten defaultnya, dan salin-tempel seluruh kode aplikasi to-do list ke dalam file tersebut.
3. Kompilasi dan Jalankan Proyek  
   Di dalam diretori todo\_cli, jalankan perintah:  
   cargo run

   cargo akan secara otomatis mengkompilasi kode Anda dan menjalankannya jika tidak ada eror.
4. (Opsional) Build untuk Produksi  
   Jika Anda ingin membuat file eksekusi (executable) yang teroptimasi, gunakan perintah:  
   cargo build \--release

   File eksekusi akan tersedia di direktori target/release/.

## **🖥️ Cara Menggunakan Aplikasi**

Setelah program berjalan, Anda akan disambut dengan menu utama:

\===== MENU TO-DO LIST \=====  
1\. Tambah Tugas  
2\. Lihat Daftar Tugas  
3\. Hapus Tugas  
4\. Perbarui Tugas  
5\. Keluar  
\===========================  
\>

* Untuk **menambah tugas**, ketik 1 lalu Enter. Program akan meminta Anda memasukkan deskripsi tugas.
* Untuk **melihat daftar**, ketik 2 lalu Enter.
* Untuk **menghapus tugas**, ketik 3 lalu Enter. Program akan meminta nomor tugas yang ingin dihapus.
* Untuk **memperbarui tugas**, ketik 4 lalu Enter. Program akan meminta nomor tugas dan deskripsi barunya.
* Untuk **keluar**, ketik 5 lalu Enter.

## **🔬 Membedah Arsitektur Kode**

* **fn main()**: Fungsi ini adalah titik masuk dan jantung dari aplikasi.
   * Ia menginisialisasi let mut tasks: Vec\<String\> sebagai tempat penyimpanan data utama.
   * Ia berisi loop utama yang membuat aplikasi terus berjalan.
   * Ia bertanggung jawab untuk menerima dan memproses semua input dari pengguna melalui struktur match.
* **fn menu()**: Fungsi pembantu sederhana yang hanya bertugas untuk mencetak opsi menu ke layar. Memisahkannya ke dalam fungsi sendiri membuat kode di main lebih bersih.
* **Struktur Data (Vec\<String\>)**:
   * Data tugas disimpan dalam Vec\<String\>, sebuah **vector** (list yang bisa tumbuh) yang berisi elemen String.
   * Ini adalah pilihan yang sederhana dan efektif untuk memulai, di mana setiap String merepresentasikan satu deskripsi tugas.
* **Alur Kontrol (loop dan match)**:
   * Aplikasi menggunakan loop tak terbatas untuk terus menerima perintah hingga pengguna memilih untuk keluar.
   * match adalah konstruksi utama yang mengarahkan input pengguna ("1", "2", dst.) ke blok logika yang sesuai. Ini adalah cara yang lebih aman dan jelas dibandingkan menggunakan if-else if berantai.

## **💡 Potensi Pengembangan**

Aplikasi ini dapat dikembangkan lebih lanjut dengan menambahkan fitur-fitur berikut:

1. **Persistensi Data**: Menyimpan daftar tugas ke dalam sebuah file (misalnya format JSON, CSV, atau database *embedded* seperti SQLite/Sled) sehingga data tidak hilang saat program ditutup, dan memuatnya kembali saat program dimulai.
2. **Struktur Data yang Lebih Baik**: Menggunakan struct Task { id: u32, description: String, completed: bool } daripada hanya String untuk menyimpan lebih banyak informasi tentang setiap tugas (misalnya status selesai).
3. **Error Handling yang Lebih Baik**: Mengganti .unwrap() dan .expect() dengan penanganan Result yang lebih eksplisit (misalnya dengan match atau if let Err(...)) untuk membuat program lebih tangguh dan tidak mudah *panic*.
4. **Fitur Tambahan**: Menambahkan fungsionalitas seperti menandai tugas sebagai selesai, *filtering* tugas (misalnya, tampilkan yang selesai saja), memberi prioritas, atau menetapkan tenggat waktu.

## **🤝 Kontribusi**

Saran dan kontribusi sangat diterima\! Jika Anda menemukan bug atau punya ide fitur, silakan buka *Issue* atau kirim *Pull Request*.