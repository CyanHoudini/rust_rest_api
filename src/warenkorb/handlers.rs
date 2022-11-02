use actix_web::{web, HttpResponse, Responder};
use std::sync::Mutex;
use crate::warenkorb;

pub async fn index() -> &'static str {
    "Hallo"
}
//Aufrufe mit Put müssen mit z.B. curl -X PATCH --header 'Content-Type: application/json' -d '{"amount_apples":1,"amount_bananas":0,"amount_potatoes":0}' '127.0.0.1:8080/add-ware/1
//bzw. Postman/Insomnia
pub async fn change_amount(id : web::Path<u32>, app_state : web::Data<Mutex<warenkorb::warenkorb::Warenkorb>>, req : web::Json<warenkorb::warenkorb::ResponseStruct>) -> impl Responder {
    
    
    match id.into_inner(){
        1 => {
            web::Json(app_state.lock().unwrap().amount_apples = req.into_inner().amount);
            HttpResponse::Ok().json(web::Json(app_state))
        }
        2 => {
            web::Json(app_state.lock().unwrap().amount_bananas = req.into_inner().amount);
        
            HttpResponse::Ok().json(web::Json(app_state))
        }
        3 => {
            web::Json(app_state.lock().unwrap().amount_potatoes = req.into_inner().amount);
            HttpResponse::Ok().json(web::Json(app_state))
        } 
        _ => HttpResponse::InternalServerError().finish()
    }
}

/*
pub async fn remove_wares(id : web::Path<u32>, app_state : web::Json<Mutex<warenkorb::warenkorb::Warenkorb>>) -> impl Responder {
    match id.into_inner(){
        1 => {
            if app_state.lock().unwrap().amount_apples>=1 { //wenn Mutex panic!, dann wird returned Mutex "poison"
                app_state.lock().unwrap().amount_apples-=1; //damit die ANzahl nicht kleiner 0 sein kann (da sonst Mutext sonst zu PoisonErr wird) benutze einen Check
                HttpResponse::Ok().json(web::Json(app_state))
            } else { HttpResponse::Ok().json(web::Json(app_state)) }
        }
        2 => { if app_state.lock().unwrap().amount_bananas>=1 {
                app_state.lock().unwrap().amount_apples-=1;
                HttpResponse::Ok().json(web::Json(app_state))
            } else { HttpResponse::Ok().json(web::Json(app_state)) }
        }
        3 => {
            if app_state.lock().unwrap().amount_potatoes>=1 {
                app_state.lock().unwrap().amount_potatoes-=1;
                HttpResponse::Ok().json(web::Json(app_state))
            } else { HttpResponse::Ok().json(web::Json(app_state)) }
        } 
        _ => HttpResponse::InternalServerError().finish()
    }
} */
//verwende Mutex
pub async fn get_total_amount( app_state : web::Data<Mutex<warenkorb::warenkorb::Warenkorb>>) -> impl Responder {
    HttpResponse::Ok().json(app_state)
}
//web::Data<> wird als Wrapper um Paramter verwendet da dieses dann automatisch FromRequest implementiert (anstatt es selbst zu machen)
//wird benötigt da .to() einen Handler benötigt (welcher FromRequest impl)
//response struct,
pub async fn get_single_amount(id : web::Path<u32>, app_state : web::Data<Mutex<warenkorb::warenkorb::Warenkorb>>) -> impl Responder {
    match id.into_inner(){
        1 => {
            let response =warenkorb::warenkorb::ResponseStruct { amount : app_state.lock().unwrap().amount_apples};
            HttpResponse::Ok().json(web::Json(response))
        }
        2 => {
            let response =warenkorb::warenkorb::ResponseStruct { amount : app_state.lock().unwrap().amount_bananas};
            HttpResponse::Ok().json(web::Json(response))
        }
        3 => {
            let response =warenkorb::warenkorb::ResponseStruct { amount : app_state.lock().unwrap().amount_potatoes};
            HttpResponse::Ok().json(web::Json(response))
        } 
        _ => HttpResponse::InternalServerError().finish()
    }
}
