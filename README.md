# BambangShop Receiver App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a Rocket web framework skeleton that you can work with.

As this is an Observer Design Pattern tutorial repository, you need to implement a feature: `Notification`.
This feature will receive notifications of creation, promotion, and deletion of a product, when this receiver instance is subscribed to a certain product type.
The notification will be sent using HTTP POST request, so you need to make the receiver endpoint in this project.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Receiver" folder.

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    ROCKET_PORT=8001
    APP_INSTANCE_ROOT_URL=http://localhost:${ROCKET_PORT}
    APP_PUBLISHER_ROOT_URL=http://localhost:8000
    APP_INSTANCE_NAME=Safira Sudrajat
    ```
    Here are the details of each environment variable:
    | variable                | type   | description                                                     |
    |-------------------------|--------|-----------------------------------------------------------------|
    | ROCKET_PORT             | string | Port number that will be listened by this receiver instance.    |
    | APP_INSTANCE_ROOT_URL   | string | URL address where this receiver instance can be accessed.       |
    | APP_PUUBLISHER_ROOT_URL | string | URL address where the publisher instance can be accessed.       |
    | APP_INSTANCE_NAME       | string | Name of this receiver instance, will be shown on notifications. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)
3.  To simulate multiple instances of BambangShop Receiver (as the tutorial mandates you to do so),
    you can open new terminal, then edit `ROCKET_PORT` in `.env` file, then execute another `cargo run`.

    For example, if you want to run 3 (three) instances of BambangShop Receiver at port `8001`, `8002`, and `8003`, you can do these steps:
    -   Edit `ROCKET_PORT` in `.env` to `8001`, then execute `cargo run`.
    -   Open new terminal, edit `ROCKET_PORT` in `.env` to `8002`, then execute `cargo run`.
    -   Open another new terminal, edit `ROCKET_PORT` in `.env` to `8003`, then execute `cargo run`.

## Mandatory Checklists (Subscriber)
-   [ ] Clone https://gitlab.com/ichlaffterlalu/bambangshop-receiver to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [ ] Commit: `Create Notification model struct.`
    -   [ ] Commit: `Create SubscriberRequest model struct.`
    -   [ ] Commit: `Create Notification database and Notification repository struct skeleton.`
    -   [ ] Commit: `Implement add function in Notification repository.`
    -   [ ] Commit: `Implement list_all_as_string function in Notification repository.`
    -   [ ] Write answers of your learning module's "Reflection Subscriber-1" questions in this README.
-   **STAGE 3: Implement services and controllers**
    -   [ ] Commit: `Create Notification service struct skeleton.`
    -   [ ] Commit: `Implement subscribe function in Notification service.`
    -   [ ] Commit: `Implement subscribe function in Notification controller.`
    -   [ ] Commit: `Implement unsubscribe function in Notification service.`
    -   [ ] Commit: `Implement unsubscribe function in Notification controller.`
    -   [ ] Commit: `Implement receive_notification function in Notification service.`
    -   [ ] Commit: `Implement receive function in Notification controller.`
    -   [ ] Commit: `Implement list_messages function in Notification service.`
    -   [ ] Commit: `Implement list function in Notification controller.`
    -   [ ] Write answers of your learning module's "Reflection Subscriber-2" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Subscriber) Reflections

#### Reflection Subscriber-1
1. Penggunaan RwLock<Vec<Notification>> sangat diperlukan untuk menangani sinkronisasi data dalam lingkungan multithreading di mana banyak thread dapat mengakses daftar notifikasi secara bersamaan. Alasan utama kita tidak menggunakan Mutex adalah karena RwLock memberikan efisiensi yang jauh lebih baik untuk skenario yang didominasi oleh operasi pembacaan data. Tidak seperti Mutex yang hanya mengizinkan satu thread untuk mengakses data baik saat membaca maupun menulis, RwLock memungkinkan banyak thread untuk melakukan pembacaan (shared read) secara serentak selama tidak ada thread yang sedang melakukan modifikasi atau penulisan. Mengingat aplikasi ini memiliki karakteristik di mana pengguna akan jauh lebih sering memanggil endpoint untuk melihat atau menampilkan daftar pesan dibandingkan frekuensi masuknya notifikasi baru, RwLock menjadi pilihan yang lebih optimal untuk menjaga performa aplikasi agar tidak terhambat (blocked) saat banyak permintaan akses baca terjadi di waktu yang sama.

2. Terkait penggunaan lazy_static, Rust tidak mengizinkan modifikasi variabel statis secara langsung seperti pada Java karena keamanan memori (memory safety) yang sangat ketat untuk mencegah data race. Di Java, variabel statis yang dapat diubah (mutable) bisa diakses oleh banyak thread tanpa perlindungan wajib dari kompilator, yang sering kali menyebabkan perilaku program yang tidak terprediksi. Rust memaksa kita menggunakan mekanisme seperti lazy_static digabung dengan thread-safe wrapper (RwLock, Mutex, atau DashMap) untuk memastikan bahwa setiap upaya modifikasi pada data statis harus melalui prosedur penguncian yang aman. Hal ini menjamin bahwa pada satu waktu, akses terhadap data tersebut selalu tervalidasi oleh sistem ownership dan borrowing Rust, sehingga keamanan program tetap terjaga bahkan dalam kondisi konkuren yang kompleks.

#### Reflection Subscriber-2
1. Dalam pengerjaan tutorial ini, saya telah mengeksplorasi bagian lain di luar langkah-langkah instruksi, salah satunya adalah file src/lib.rs dan src/config.rs. Dari eksplorasi tersebut, saya mempelajari bagaimana aplikasi ini mengelola konfigurasi global menggunakan singleton AppConfig yang dibungkus dalam OnceCell untuk memastikan inisialisasi yang aman dan hanya terjadi satu kali. Saya juga memahami peran REQWEST_CLIENT yang didefinisikan secara statis agar koneksi HTTP dapat digunakan kembali (reused) oleh berbagai service, yang jauh lebih efisien dibandingkan membuat klien baru di setiap permintaan. Selain itu, pemisahan modul di lib.rs membantu saya memahami bagaimana Rust mengorganisir kode agar controller, model, repository, dan service dapat saling mengenali satu sama lain melalui deklarasi pub mod.

2. Penerapan pola Observer sangat memudahkan penambahan subscriber (Receiver) karena Main App dirancang untuk tidak bergantung pada siapa saja yang menerima notifikasi. Kita cukup mendaftarkan URL baru ke dalam sistem, dan notifikasi akan terkirim secara otomatis tanpa mengubah kode inti. Namun, kendala nyata muncul jika kita menjalankan lebih dari satu instansi Main App, karena daftar subscriber saat ini hanya disimpan di dalam memori lokal masing-masing instansi melalui variabel lazy_static. Jika seorang subscriber melakukan registrasi ke instansi Main App yang berjalan di port 8000, data tersebut tidak akan tersinkronisasi ke instansi Main App yang berjalan di port 8001. Akibatnya, ketika instansi di port 8001 mengirimkan notifikasi, subscriber tersebut tidak akan menerima pesan apa pun karena namanya tidak tercatat dalam memori lokal instansi tersebut. Untuk menjaga skalabilitas dan konsistensi data pada banyak instansi Main App, kita memerlukan mekanisme penyimpanan eksternal yang terpusat, seperti basis data, sehingga semua instansi aplikasi utama merujuk pada daftar subscriber yang sama secara real-time.

3. Mengenai penggunaan fitur pengujian di Postman, saya telah mencoba membuat Tests otomatis untuk memvalidasi respon dari endpoint subscribe dan list_messages. Fitur ini sangat bermanfaat karena saya dapat memastikan secara instan apakah status kode yang dikembalikan adalah 200 OK atau 201 Created dan apakah struktur JSON yang diterima sudah sesuai tanpa harus memeriksanya secara manual setiap kali melakukan perubahan kode. Dokumentasi pada koleksi Postman juga sangat membantu, terutama dalam pengerjaan proyek kelompok, karena rekan tim dapat langsung memahami parameter apa saja yang dibutuhkan oleh suatu endpoint dan melihat contoh respon sukses maupun gagal secara jelas, sehingga mengurangi risiko miskomunikasi saat proses integrasi.