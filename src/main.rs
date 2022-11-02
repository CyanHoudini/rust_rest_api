use actix_web::{get,patch, post, put, delete, web, App, HttpServer};
use std::sync::Mutex;
mod warenkorb;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    
    let korb = web::Data::new(Mutex::new(warenkorb::warenkorb::Warenkorb::new())); //"korb" muss außerhalb von App::new() erstellt werden, da TODO
    //
    HttpServer::new(move || {
        App::new()
            .app_data(korb.clone()) 
            .route("/", web::get().to(warenkorb::handlers::index))
            .route("/waren", web::get().to(warenkorb::handlers::get_total_amount))
            .route("/ware/{id}", web::get().to(warenkorb::handlers::get_single_amount))
            .route("/change-amount/{id}", web::patch().to(warenkorb::handlers::change_amount))
            //.route("/remove-ware/{id}", web::get().to(warenkorb::handlers::remove_wares))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await


}
