use crate::db::MongoRepo;
use crate::model::DevilFruit;
use actix_web::{
    delete, get, patch, post,
    web::{Data, Json},
    HttpResponse, Responder,
};

#[get("/devilfruit")]
pub async fn get_all_devilfruits() -> impl Responder {
    HttpResponse::Ok().body("All devilfruits")
}

#[get("/devilfruit/{id}")]
pub async fn get_devilfruit_by_id() -> impl Responder {
    HttpResponse::Ok().body("Devilfruit by id")
}

#[post("/devilfruit")]
pub async fn create_devilfruit(db: Data<MongoRepo>, new_df: Json<DevilFruit>) -> impl Responder {
    let data = DevilFruit {
        id: None,
        name: new_df.name.to_owned(),
        df_type: new_df.df_type.to_owned(),
        description: new_df.description.to_owned(),
        current_user: new_df.current_user.to_owned(),
        image_url: new_df.image_url.to_owned(),
    };
    let df_detail = db.create_devilfruit(data).await;
    match df_detail {
        Ok(df) => HttpResponse::Ok().json(df),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/devilfruit/{id}")]
pub async fn delete_devilfruit_by_id() -> impl Responder {
    HttpResponse::Ok().body("Delete devilfruit by id")
}

#[patch("/devilfruit/{id}")]
pub async fn update_devilfruit_by_id() -> impl Responder {
    HttpResponse::Ok().body("Update devilfruit by id")
}
