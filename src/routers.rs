use actix_web::{delete, get, patch, post, HttpResponse, Responder};

#[get("/devilfruit")]
pub async fn get_all_devilfruits() -> impl Responder {
    HttpResponse::Ok().body("All devilfruits")
}

#[get("/devilfruit/{id}")]
pub async fn get_devilfruit_by_id() -> impl Responder {
    HttpResponse::Ok().body("Devilfruit by id")
}

#[post("/devilfruit")]
pub async fn create_devilfruit() -> impl Responder {
    HttpResponse::Ok().body("Create devilfruit")
}

#[delete("/devilfruit/{id}")]
pub async fn delete_devilfruit_by_id() -> impl Responder {
    HttpResponse::Ok().body("Delete devilfruit by id")
}

#[patch("/devilfruit/{id}")]
pub async fn update_devilfruit_by_id() -> impl Responder {
    HttpResponse::Ok().body("Update devilfruit by id")
}
