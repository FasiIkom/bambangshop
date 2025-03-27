# BambangShop Publisher App
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
4.  `repository`: this module contains structs that serve as databases and methods to access the databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a basic functionality that makes BambangShop work: ability to create, read, and delete `Product`s.
This repository already contains a functioning `Product` model, repository, service, and controllers that you can try right away.

As this is an Observer Design Pattern tutorial repository, you need to implement another feature: `Notification`.
This feature will notify creation, promotion, and deletion of a product, to external subscribers that are interested of a certain product type.
The subscribers are another Rocket instances, so the notification will be sent using HTTP POST request to each subscriber's `receive notification` address.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Publisher" folder.
This Postman collection also contains endpoints that you need to implement later on (the `Notification` feature).

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    APP_INSTANCE_ROOT_URL="http://localhost:8000"
    ```
    Here are the details of each environment variable:
    | variable              | type   | description                                                |
    |-----------------------|--------|------------------------------------------------------------|
    | APP_INSTANCE_ROOT_URL | string | URL address where this publisher instance can be accessed. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)

## Mandatory Checklists (Publisher)
-   [ ] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [ ] Commit: `Create Subscriber model struct.`
    -   [ ] Commit: `Create Notification model struct.`
    -   [ ] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [ ] Commit: `Implement add function in Subscriber repository.`
    -   [ ] Commit: `Implement list_all function in Subscriber repository.`
    -   [ ] Commit: `Implement delete function in Subscriber repository.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [ ] Commit: `Create Notification service struct skeleton.`
    -   [ ] Commit: `Implement subscribe function in Notification service.`
    -   [ ] Commit: `Implement subscribe function in Notification controller.`
    -   [ ] Commit: `Implement unsubscribe function in Notification service.`
    -   [ ] Commit: `Implement unsubscribe function in Notification controller.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [ ] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [ ] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [ ] Commit: `Implement publish function in Program service and Program controller.`
    -   [ ] Commit: `Edit Product service methods to call notify after create/delete.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1

**Jawaban:**

1. **Interface (atau trait) vs Model Struct pada Subscriber**  
   Dalam diagram Observer pada buku Head First Design Patterns, Subscriber didefinisikan sebagai interface untuk memastikan fleksibilitas dan mengizinkan berbagai jenis implementasi yang dapat berubah secara dinamis. Di Rust, jika kita hanya membutuhkan satu model sederhana (Subscriber) dengan data statis, satu struct mungkin sudah cukup. Namun, jika di masa depan kita berencana untuk memiliki berbagai tipe Subscriber dengan perilaku berbeda, mendefinisikan sebuah trait akan memberikan keuntungan dengan mendukung polimorfisme dan loose coupling.

2. **Penggunaan Vec vs DashMap untuk data unik**  
   Karena `id` pada Program dan `url` pada Subscriber dimaksudkan untuk bersifat unik, menggunakan Vec saja tidak memadai karena pencarian dan pemeliharaan keunikan di dalam list membutuhkan iterasi manual yang berpotensi tidak efisien. Sebaliknya, DashMap (atau struktur data map/sejenis HashMap) lebih tepat karena secara otomatis menyediakan lookup dengan kompleksitas waktu konstan dan dapat membantu memastikan keunikan kunci.

3. **DashMap vs Singleton untuk akses thread-safe**  
   Meskipun pola Singleton bisa digunakan untuk mengelola instance tunggal, itu tidak secara otomatis mengatasi isu thread safety saat akses data secara konkuren. Di Rust, karena compiler menekankan keamanan thread, kita perlu menggunakan struktur seperti DashMap yang secara eksplisit dirancang untuk akses data yang aman dalam lingkungan multi-thread. Oleh karena itu, meskipun konsep Singleton bisa digunakan, DashMap merupakan pilihan yang lebih tepat untuk memastikan data (seperti list Subscribers) diakses dengan aman oleh beberapa thread.

#### Reflection Publisher-2

**Jawaban:**

1. **Pemisahan Service dan Repository dari Model**  
   Dalam pola MVC tradisional, Model menangani representasi data dan sebagian logika bisnis dasar. Namun, untuk memastikan _separation of concerns_ dan menerapkan prinsip Single Responsibility, Service dan Repository dipisahkan dari Model.  
   - **Repository** bertanggung jawab khusus untuk interaksi dengan penyimpanan data (misal: database), sehingga semua logika terkait akses data terpusat pada satu tempat.  
   - **Service** mengenkapsulasi logika bisnis yang memanfaatkan data dari Model dan Repository, memungkinkan modifikasi logika bisnis tanpa mengganggu struktur data.  
   Pemisahan ini membuat kode menjadi lebih modular, mudah diuji, dan lebih siap untuk perubahan di masa depan.

2. **Dampak jika Hanya Menggunakan Model**  
   Jika kita hanya mengandalkan Model untuk menangani segala hal:  
   - Setiap Model (seperti Program, Subscriber, Notification) harus menanggung tanggung jawab ganda antara representasi data dan logika bisnis, yang berujung pada _tight coupling_.  
   - Perubahan dalam logika bisnis salah satu Model berpotensi mempengaruhi Model lain karena tidak ada pemisahan yang jelas antara pengelolaan data dan operasional.  
   - Akibatnya, kompleksitas kode meningkat, pengujian menjadi lebih sulit, dan pemeliharaan proyek bisa terhambat seiring bertambahnya fitur dan interaksi antar model.

3. **Manfaat Postman dalam Pengujian**  
   Postman adalah alat yang sangat berguna untuk menguji API karena:  
   - Memungkinkan pengiriman request HTTP dengan berbagai metode dan memeriksa respons secara real-time, sehingga memudahkan debugging endpoint API.  
   - Fitur Collection, Environment, dan Pre-request/Post-request Scripts memungkinkan otomasi pengujian dan pengelolaan variabel lingkungan yang dinamis.  
   - Collection Runner dan kemampuan menulis skrip test mendukung pengujian otomatis (regression testing) yang sangat bermanfaat dalam pengembangan berkelanjutan (CI/CD).  
   - Bagi proyek kelompok atau proyek di masa depan, Postman membantu dalam pemantauan, dokumentasi, dan kolaborasi sehingga memastikan integrasi antar komponen API bekerja dengan baik.

#### Reflection Publisher-3

**Jawaban:**

1. **Observer Pattern Variations (Push vs. Pull):**  
   Dalam tutorial ini, kita menggunakan variasi **Push model**, di mana publisher langsung mengirim data notifikasi (misalnya status "CREATED", "DELETED", "PROMOTION" beserta data produk) kepada setiap subscriber yang telah terdaftar melalui pemanggilan method `NotificationService.notify()`. Publisher mendorong data ke subscribers tanpa meminta data secara eksplisit.

2. **Keunggulan dan Kekurangan jika Menggunakan Pull Model:**  
   - **Keunggulan (Pull Model):**  
     - Subscriber memiliki kontrol penuh untuk mengambil (pull) data yang diperlukan pada saat yang tepat, sehingga bisa mengurangi pengiriman data yang tidak dibutuhkan.
     - Dapat mengurangi beban komunikasi jika subscriber hanya tertarik pada subset data tertentu dan memilih frekuensi polling yang optimal.
   - **Kekurangan (Pull Model):**  
     - Latensi yang lebih tinggi karena subscriber harus secara periodik mengecek update, sehingga notifikasi tidak real-time.
     - Lebih kompleks untuk mengelola mekanisme polling dan penjadwalan, terutama jika jumlah subscriber meningkat.
     - Dalam sistem yang mengutamakan responsivitas seperti contoh tutorial ini, pull model dapat membuat proses update dan sinkronisasi data menjadi kurang efisien.

3. **Tanpa Multi-threading dalam Proses Notifikasi:**  
   Jika proses notifikasi dijalankan secara sinkron (tanpa multi-threading), maka:  
   - Proses utama (main thread) akan menunggu hingga seluruh notifikasi terkirim, yang dapat menyebabkan penundaan (blocking) dalam pemrosesan request baru.  
   - Kinerja sistem dapat menurun, terutama jika ada subscriber yang lambat merespons atau terdapat banyak subscriber, sehingga waktu tunggu untuk notifikasi meningkat.  
   - Secara keseluruhan, tidak menggunakan multi-threading bisa mengakibatkan aplikasi menjadi tidak responsif dan efisiensi throughput menurun.