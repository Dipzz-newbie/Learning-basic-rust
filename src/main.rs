use std::string;

fn main() {
    let x:i32 = 5;
    let mut y:i32 = 10;
    y += 5;  


    assert_eq!(x, 5);
    assert_eq!(y, 15);
    println!("x: {}, y: {}", x, y);

}


#[test]
fn fungsi_test() {
    println!("hello world");
}

#[test]
fn name() {

    // belajar mengenai tipe data muteble dan immutable
    let mut nama:&str = "Muhammad Nadhif Triyudo";
    println!("nama saya: {}", nama); 

    nama = "jojo";
    println!("nama saya: {}", nama);
}


#[test]
fn compared() {
    let a = 12;
    let b: i64 = 23;

    let result = a > b;
    println!("{}", result);
}

#[test] 
fn compared_real() {

    // belajar mengunakan tipe data boolean 
    let nilai = 80;

    // tipe data boolean adalah tipe data yang hanya memiliki dua nilai yaitu true dan false
    // untuk menentukan lulus atau tidak lulus
    let lulus = nilai  >= 75;
    let lulus_ujian_akhir = nilai >= 80;

    let lulus_sekolah = lulus && lulus_ujian_akhir;
    println!("{}", lulus_sekolah);

}

#[test] 
fn tuple() {
    // belajar mengenai tipe data tuple
    // tipe data tuple adalah tipe data yang dapat menyimpan beberapa nilai dengan tipe data yang berbeda
   let data: (i32, f64, &str) = (10, 40.5, "Nadhif");
   println!("data: {:?}", data);

   // untuk mengambil nilai dari tuple
   let a = data.0;
   let b = data.1;
   let c = data.2;

   println!("{} {} {}", a,b,c)

}


#[test] 
fn tuple_desc() {
   // initialsasi tentang materi tuple  
   let mut data: (i32, f64, &str) = (10, 40.5, "Nadhif");
   println!("data: {:?}", data);

   
   // dan untuk tidak mengambil data yang tidak diperlukan cukup tambahkan underscore
   /* let (a,b,_) = data;
   println!("{} {}", a,b); */

   // untuk mengubah data tuple menjadi data berbentuk variabel
   let (a,b,c) = data;
   println!("{} {} {}", a,b,c);

   // untuk mengubah data tuple di variable awal harus di didefinisikan sebagai mutable
   // karena data tuple tidak bisa diubah secara langsung
    data.0 = 20;
    data.1 = 50.4;
    data.2 = "Nadhif Triyudo";
    println!("data: {:?}", data);

}


#[test]
fn unit() {
    println!("Hello World");
}

#[test]
fn test_unit() {

    // belajar mengenai tipe data unit
    // tipe data unit adalah tipe data yang tidak memiliki nilai, biasanya digunakan untuk fungsi yang tidak mengembalikan nilai apapun
    
    let hello = unit();
    println!("Hello: {:?}", hello);

    // tipe data unit ditandai dengan tanda kurung
    let test = ();
    println!("Test: {:?}", test);
}

#[test] 

fn array() {

    // harus didefinisikan panjangnya dan tipe datanya 
    // dalam array hanya boleh satu tipe data saja semisal [i32;5] artinya array dengan tipe data i32 dan panjang 5
    let mut data:[i32; 5] = [1,2,3,4,5];
    println!("data: {:?}", data);

    // untuk mengambil data dari array
    // berbeda dengan tuple, untuk mengambil data dari array harus menggunangkan indeks secara langsung dan tidak menggunakan tanda titik
    let a = data[0];
    let b = data[1];
    println!("{} {}",a ,b);

    // untuk mengubah data di array harus didefinisikan sebagai mutable
    // karena data array tidak bisa diubah secara langsung
    // untuk mengubah data di array harus menggunakan indeks
    data[0] = 10;
    data[1] = 20;
    println!("data: {:?}", data);

    // untuk mendapatkan panjang data array
    // menggunakan method len()
    let panjang = data.len();
    println!("panjang data: {}", panjang);
}


#[test]
fn two_demiensional_array() {

    // belajar mengenai array dua dimensi
    // array dua dimensi adalah array yang memiliki lebih dari satu dimensi
    // dalam array dua dimensi harus didefinisikan panjangnya dan tipe datanya
    // semisal [[i32;2];2] artinya array dua dimensi dengan tipe data i32 dan panjang 2x2
    
    let data: [[i32;2]; 2] = [
        [1,2],
        [3,4],
    ];

    // untuk mengakses data di array dua dimensi
    // menggunakan indeks seperti array biasa
    // namun harus menggunakan dua indeks
    println!("data: {:?}", data);
    println!("data: {}", data[0][0]); // semisal data[0][0] untuk mengakses data di baris pertama kolom pertama
    println!("data: {}", data[0][1]);// data[0][1] untuk mengakses data di baris pertama kolom kedua
    println!("data: {}", data[1][0]);// data[1][0] untuk mengakses data di baris kedua kolom pertama
    println!("data: {}", data[1][1]); // data[1][1] untuk mengakses data di baris kedua kolom kedua
}


const MINIMAL:i32 = 100;

#[test]
fn constant() {
    // belajar mengenai konstanta
    // konstanta adalah nilai yang tidak bisa diubah
    // konstanta harus didefinisikan dengan kata kunci const
    // dan harus didefinisikan tipe datanya
    const MAXIMAL:i32 = 1000;

    //MAXIMAL = 2000; ini akan error karena konstanta tidak bisa diubah
    println!("MINIMAL: {}, MAXIMAL: {}", MINIMAL, MAXIMAL);

}


#[test] 

fn variable_scope() {
    // belajar mengenai scope variabel
    // scope variabel adalah area dimana variabel dapat diakses
    
    println!("{}", MINIMAL);

    // variabel yang didefinisikan di luar blok bisa diakses di dalam blok tersebut
    let x = 10;

    {
        // variabel yang didefinisikan di dalam blok tidak bisa diakses di luar blok tersebut
        let y = 20;
        println!("x: {}, y: {}", x, y);
    }
    
    // println!("y: {}", y); ini akan error karena y hanya bisa diakses di dalam blok kecuali dengan variabel x yang mana variabel tersebut didefinisikan di luar blok
}

#[test] 
fn stack_heap() {
    let a = function_a();
    let b = function_b();

    println!("Stack and Heap functions called.");
    println!("function_a: {:?}, function_b: {:?}", a, b);
    // fungsi ini akan dipanggil di stack dan heap
}

#[test]
fn function_a() {
    // fungsi ini akan dipanggil di stack
    let a = 10; // tipe data i32 disimpan di stack
    let b = string::String::from("Hello");// tipe data String disimpan di heap
    println!("Function a: {}, Function b: {}", a, b);
}

#[test]
fn function_b() {
    // fungsi ini akan dipanggil di heap
    let a:i32 = 20;// tipe data i32 disimpan di stack
    let b = string::String::from("World");// tipe data String disimpan di heap
    println!("Function a: {}, Function b: {}", a, b);
}

#[test] 
fn string_slice() {

    // belajar mengenai string slice
    // string slice ditandai dengan tanda & dan tipe data str
    // string slice biasanya digunakan untuk mengambil bagian dari string yang lebih besar
    // string slice tidak memiliki kepemilikan, sehingga tidak perlu menghapusnya secara manual
    // string slice biasanya digunakan untuk mengambil bagian dari string yang lebih besar  
    let nama:&str = " Muhammad Nadhif Triyudo ";
    // string slice adalah tipe data yang merepresentasikan bagian dari string
    let trims:&str = nama.trim();

    println!("Nama: {}", nama);
    println!("Nama: {}", trims);
}


#[test] 
fn method() {
    // belajar mengenai method pada string
    // method adalah fungsi yang dimiliki oleh tipe data tertentu   
    // method pada string biasanya digunakan untuk mengubah atau mengambil bagian dari string
    let mut nama = String::from("Muhammad Nadhif");
    nama.push_str(" Triyudo"); // menambahkan string ke dalam string (push_str)
    println!("Nama: {}", nama);

    // method replace akan mengembalikan string baru yang sudah diganti
    // sehingga nama2 adalah string baru yang sudah diganti
    // nama tetap tidak berubah
    let nama2 = nama.replace("Muhammad", "Jojo"); // mengganti string di dalam string (replace)
    // mengganti string di dalam string
    println!("Nama2: {}", nama2);
}

#[test] 
fn ownership() {
    // ini hanya melakukan copy terhadap nilai dari variabel a ke variabel b
    let a = 10;
    let b = a;
    // karena tipe data i32 adalah tipe data yang disimpan di stack
    // sehingga tidak perlu menghapusnya secara manual
    println!("a: {}, b: {}", a, b);

    // belajar mengenai kepemilikan (ownership)
    // kepemilikan adalah konsep dimana variabel memiliki hak atas data yang disimpan di dalamnya
    // jika variabel memiliki hak atas data, maka variabel tersebut dapat mengubah
    // atau menghapus data tersebut secara manual
 
    let name = String::from("Muhammad Nadhif");
    let name2 = name;
    // tipe data String adalah tipe data yang disimpan di heap
    // sehingga jika kita mengubah atau menghapus data tersebut, maka kita harus menghapusnya
    // secara manual dengan menggunakan fungsi drop()

    println!("Name: {}", name2);
}

