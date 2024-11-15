use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid:: Uuid;
use crate::schema::users; // Pastikan path ke schema sudah benar

// Struktur untuk user yang sudah ada di database (untuk query)
#[derive(Debug, Deserialize, Queryable, Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String
}

// Struktur untuk user baru yang akan dimasukkan ke database (untuk insert)
#[derive(Queryable, Insertable, Deserialize, Serialize)]
#[diesel(table_name = users)]  // Pastikan `users` sesuai dengan nama tabel di schema
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

// Jika ingin mendefinisikan update dengan perubahan opsional, gunakan struct seperti ini:
// #[derive(Deserialize, AsChangeset)]
// #[diesel(table_name = users)]
// pub struct UpdateUser {
//     pub name: Option<String>,
//     pub email: Option<String>,
//     pub password: Option<String>,
// }
