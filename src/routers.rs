use crate::db::MongoRepo;
use crate::models::DevilFruit;
use actix_web::{
    delete, get, patch, post,
    web::{Data, Json, Path, Query},
    HttpResponse, Responder,
};

#[get("/devilfruit")]
pub async fn get_all_devilfruits(db: Data<MongoRepo>) -> impl Responder {
    let devilfruits = db.get_all_devilfruits().await;
    match devilfruits {
        Ok(df) => HttpResponse::Ok().json(df),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/devilfruit/{id}")]
pub async fn get_devilfruit_by_id(db: Data<MongoRepo>, path: Path<String>) -> impl Responder {
    let id = path.into_inner();
    let devilfruit = db.get_devilfruit_by_id(id).await;
    match devilfruit {
        Ok(df) => HttpResponse::Ok().json(df),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/devilfruit")]
pub async fn get_devilfruit_by_name(
    db: Data<MongoRepo>,
    query: Query<DevilFruit>,
) -> impl Responder {
    let name = query.name.to_owned();
    let devilfruit = db.get_devilfruit_by_name(name).await;
    match devilfruit {
        Ok(df) => HttpResponse::Ok().json(df),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/devilfruit")]
pub async fn get_devilfruit_by_type(
    db: Data<MongoRepo>,
    query: Query<DevilFruit>,
) -> impl Responder {
    let df_type = query.df_type.to_owned();
    let devilfruit = db.get_devilfruit_by_type(df_type).await;
    match devilfruit {
        Ok(df) => HttpResponse::Ok().json(df),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/devilfruit")]
pub async fn get_devilfruit_by_user(
    db: Data<MongoRepo>,
    query: Query<DevilFruit>,
) -> impl Responder {
    let user = query.current_user.to_owned();
    let devilfruit = db.get_devilfruit_by_user(user).await;
    match devilfruit {
        Ok(df) => HttpResponse::Ok().json(df),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
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
pub async fn delete_devilfruit_by_id(db: Data<MongoRepo>, path: Path<String>) -> impl Responder {
    let id = path.into_inner();
    let devilfruit = db.delete_devilfruit_by_id(id).await;
    match devilfruit {
        Ok(_) => HttpResponse::Ok().body("Devilfruit deleted"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[patch("/devilfruit/{id}")]
pub async fn update_devilfruit_by_id(
    db: Data<MongoRepo>,
    path: Path<String>,
    new_df: Json<DevilFruit>,
) -> impl Responder {
    let id = path.into_inner();
    let data = DevilFruit {
        id: None,
        name: new_df.name.to_owned(),
        df_type: new_df.df_type.to_owned(),
        description: new_df.description.to_owned(),
        current_user: new_df.current_user.to_owned(),
        image_url: new_df.image_url.to_owned(),
    };
    let devilfruit = db.update_devilfruit_by_id(id, data).await;
    match devilfruit {
        Ok(df) => HttpResponse::Ok().json(df),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
