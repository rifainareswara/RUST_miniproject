use std::io::{stdin, stdout, Write};

fn main() {
    let mut tasks: Vec<String> = Vec::new();
    loop {
        // Tampilkan menu di setiap iterasi loop agar pengguna selalu tahu opsinya
        menu();

        // Minta input dari pengguna
        print!("> ");
        // Flush stdout untuk memastikan "> " langsung tampil sebelum input
        stdout().flush().unwrap();

        let mut choice: String = String::new();
        if stdin().read_line(&mut choice).is_ok() {
            match choice.trim() {
                "1" => {
                    println!("\n== Tambah Tugas Baru ==");
                    print!("Masukkan deskripsi tugas: ");
                    stdout().flush().unwrap();

                    let mut task: String = String::new();
                    if stdin().read_line(&mut task).is_ok() {
                        tasks.push(task.trim().to_string());
                        println!("✅ Tugas berhasil ditambahkan!\n");
                    }
                }
                "2" => {
                    println!("\n== Daftar Tugas ==");
                    if tasks.is_empty() {
                        println!("Belum ada tugas.\n");
                    } else {
                        for (index, task) in tasks.iter().enumerate() {
                            println!("{}. {}", index + 1, task);
                        }
                        println!(); // Baris baru untuk spasi
                    }
                }
                "3" => {
                    println!("\n== Hapus Tugas ==");
                    print!("Masukkan nomor tugas yang akan dihapus: ");
                    stdout().flush().unwrap();

                    let mut id_str = String::new();
                    if stdin().read_line(&mut id_str).is_ok() {
                        if let Ok(id) = id_str.trim().parse::<usize>() {
                            // Cek apakah id valid (lebih besar dari 0 dan tidak melebihi panjang vector)
                            if id > 0 && id <= tasks.len() {
                                // Kurangi 1 karena user melihat index 1-based, tapi vector 0-based
                                let removed_task = tasks.remove(id - 1);
                                println!("✅ Tugas '{}' berhasil dihapus.\n", removed_task);
                            } else {
                                println!("❌ Nomor tugas tidak valid.\n");
                            }
                        } else {
                            println!("❌ Input harus berupa angka.\n");
                        }
                    }
                }
                "4" => {
                    println!("\n== Perbarui Tugas ==");
                    print!("Masukkan nomor tugas yang akan diperbarui: ");
                    stdout().flush().unwrap();

                    let mut id_str = String::new();
                    if stdin().read_line(&mut id_str).is_ok() {
                        if let Ok(id) = id_str.trim().parse::<usize>() {
                            // Cek apakah id valid
                            if id > 0 && id <= tasks.len() {
                                print!("Masukkan deskripsi tugas yang baru: ");
                                stdout().flush().unwrap();

                                let mut new_task = String::new();
                                if stdin().read_line(&mut new_task).is_ok() {
                                    // Update task pada index yang sesuai (id - 1)
                                    tasks[id - 1] = new_task.trim().to_string();
                                    println!("✅ Tugas berhasil diperbarui.\n");
                                }
                            } else {
                                println!("❌ Nomor tugas tidak valid.\n");
                            }
                        } else {
                            println!("❌ Input harus berupa angka.\n");
                        }
                    }
                }
                "5" => {
                    println!("Terima kasih! Program berhenti.");
                    break; // Keluar dari loop
                }
                _ => {
                    // Memberi tahu pengguna jika input tidak ada di menu
                    println!("❌ Pilihan tidak valid. Silakan masukkan angka dari 1 sampai 5.\n");
                }
            }
        }
    }
}

fn menu() {
    println!("===== MENU TO-DO LIST =====");
    println!("1. Tambah Tugas");
    println!("2. Lihat Daftar Tugas");
    println!("3. Hapus Tugas");
    println!("4. Perbarui Tugas");
    println!("5. Keluar");
    println!("===========================");
}