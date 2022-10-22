/**
 * @Author: plucky
 * @Date: 2022-08-13 18:15:19
 * @LastEditTime: 2022-09-16 18:13:33
 * @Description: 
 */


use axum::http::{ StatusCode};
use axum::response::IntoResponse;
use axum::routing::{get_service, get};
use axum::{Router};

use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

use super::Config;

pub fn app(_c:&Config) -> Router {
    //let state = Arc::new(state::State { count: 0 });
    // #[cfg(not(debug_assertions))]
    // let path = std::env::current_exe().unwrap().parent().unwrap().join("");
    // #[cfg(debug_assertions)]
    // let path = ".";
    Router::new()
        .route("/foo", get(|| async { "welcome !!!" }))
        .fallback(get_service(ServeDir::new(".")).handle_error(handle_error))
       
        .merge(short_links(_c))
        // 添加跟踪层,需要开启Debug日志
        .layer(TraceLayer::new_for_http())
        // 添加跨域层
        // .layer(CorsLayer::new().allow_origin("*".parse::<HeaderValue>().unwrap()))
        
       
       
}

#[allow(dead_code)]
fn short_links(c:&Config) -> Router {
    let mut r= Router::new();
    for i in 0..c.service.len() {
        r = r.nest(&c.service[i].path, get_service(ServeDir::new(&c.service[i].dir)).handle_error(handle_error));
        // r=r.merge(axum_extra::routing::SpaRouter::new(&c.service[i].path, &c.service[i].dir))
    }
    r
}

async fn handle_error(_err: std::io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}