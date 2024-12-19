use std::io;

fn main() {
    loop {
        println!("\n -----Selamat Datang di Aplikasi Konversi Suhu-----");
        println!("1. Celcius (C°)");
        println!("2. Fahrenheit (F°)");
        println!("3. kelvin (k°)");
        println!("4. keluar");
        println!("Silahkan PIlih menu yang tersedia");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Gagal membaca input");

        let choice: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input tidak valid. Harap masukkan pilihan 1-4.");
                continue;
            }
        };

        match choice {
            1 => {
                println!("\n Selamat Datang di Menu konversi Celcius");
                println!("1. Celcius ke Fahrenheit");
                println!("2. Celcius ke Kelvin");
                println!("3. Kembali");
                println!("Silahkan PIlih menu yang tersedia");

                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Gagal membaca input");

                let choice: u32 = match input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Input tidak valid. Harap masukkan pilihan 1-6.");
                        continue;
                    }
                };

                match choice {
                    1 => {
                        celcius_fahrenheit();
                    }
                    2 => {
                        celcius_kelvin();
                    }
                    3 => {
                        println!("Keluar Dari Program");
                        break;
                    }
                    _ => {
                        println!("Pilihan tidak valid, silahkan pilih sesuai menu yang tersedia.");
                    }
                }
            }
            2 => {
                println!("\n Selamat Datang di Menu Konversi Fahrenheit");
                println!("1. Fahrenheit ke Celcius");
                println!("2. Fahrenheit ke Kelvin");
                println!("3. Kembali");
                println!("Silahkan PIlih menu yang tersedia");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Gagal membaca input");

                let choice: u32 = match input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Input tidak valid. Harap masukkan pilihan 1-6.");
                        continue;
                    }
                };

                match choice {
                    1 => {
                        fahrenheit_celcius();
                    }
                    2 => {
                        fahrenheit_kelvin();
                    }
                    3 => {
                        println!("Keluar Dari Program");
                        break;
                    }
                    _ => {
                        println!("Pilihan tidak valid, silahkan pilih sesuai menu yang tersedia.");
                    }
                }
            }
            3 => {
                println!("\n Selamat Datang di Menu Konversi Kelvin");
                println!("1. Kelvin ke Celcius");
                println!("2. Kelvin ke Fahrenheit");
                println!("3. Kembali");
                println!("Silahkan PIlih menu yang tersedia");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Gagal membaca input");

                let choice: u32 = match input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Input tidak valid. Harap masukkan pilihan 1-6.");
                        continue;
                    }
                };

                match choice {
                    1 => {
                        kelvin_celcius();
                    }
                    2 => {
                        kelvin_fahrenheit();
                    }
                    3 => {
                        println!("Keluar Dari Program");
                        break;
                    }
                    _ => {
                        println!("Pilihan tidak valid, silahkan pilih sesuai menu yang tersedia.");
                    }
                }
            }
            4 => {
                println!("Terima kasih telah menggunakan aplikasi!");
                break;
            }
            _ => {
                println!("Pilihan tidak valid. Harap pilih antara 1-3.");
            }
        }
    }
}

fn celcius_fahrenheit() {
    println!("Masukkan Suhu :");
    let input = baca_input_suhu();
    let hitung = input * 1.8 + 32.0;
    println!("Hasil konversi: {:.2} °F", hitung);
}

fn celcius_kelvin() {
    println!("Masukkan Suhu :");
    let input = baca_input_suhu();
    let hitung = input + 273.15;
    println!("Hasil konversi: {:.2} °K", hitung);
}

fn fahrenheit_celcius() {
    println!("Masukkan Suhu :");
    let input = baca_input_suhu();
    let hitung = (input - 32.0) / 1.8;
    println!("Hasil konversi: {:.2} °C", hitung);
}
fn fahrenheit_kelvin() {
    println!("Masukkan Suhu :");
    let input = baca_input_suhu();
    let hitung = (input - 32.0) / 1.8 + 273.15;
    println!("Hasil konversi: {:.2} °F", hitung);
}
fn kelvin_celcius() {
    println!("Masukkan Suhu :");
    let input = baca_input_suhu();
    let hitung = input - 273.15;
    println!("Hasil konversi: {:.2} °F", hitung);
}
fn kelvin_fahrenheit() {
    println!("Masukkan Suhu :");
    let input = baca_input_suhu();
    let hitung = (input - 273.15) * 1.8 + 32.0;
    println!("Hasil konversi: {:.2} °F", hitung);
}

fn baca_input_suhu() -> f64 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Gagal membaca input");
    match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Input tidak valid. Harap masukkan angka.");
            0.0
        }
    }
}
