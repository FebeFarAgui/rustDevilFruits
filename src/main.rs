use actix_web::{web, App, HttpResponse, HttpServer};

mod routers;

use routers::{
    create_devilfruit, delete_devilfruit_by_id, get_all_devilfruits, get_devilfruit_by_id,
    update_devilfruit_by_id,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1";
    let port = 8080;
    let server = HttpServer::new(|| {
        App::new()
            .service(create_devilfruit)
            .service(get_all_devilfruits)
            .service(get_devilfruit_by_id)
            .service(update_devilfruit_by_id)
            .service(delete_devilfruit_by_id)
            .default_service(web::route().to(|| HttpResponse::NotFound()))
    })
    .bind((addr, port))?
    .run();
    println!("Server running at http://{}:{}", addr, port);
    server.await
}
