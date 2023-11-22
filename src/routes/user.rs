use core::fmt;

use actix_web::{HttpResponse, Responder};

use crate::respositories::user;

pub async fn get_user() -> impl Responder {
    let user_repo = user::UserRepository::create();

    let user = user_repo.get_user().await;
    if user.is_none() {
        return HttpResponse::Ok().body("User tidak ditemukan");
    }
    HttpResponse::Ok().body(format!("nama: {}", user.unwrap()))
}


