/***
 * @Author: plucky
 * @Date: 2022-08-10 19:49:39
 * @LastEditTime: 2022-08-13 17:17:01
 * @Description: 
 */


use axum::http::{ StatusCode};
use axum::routing::get_service;
use axum::{Router};

use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

use super::Config;

pub fn app(c:&Config) -> Router {
    //let state = Arc::new(state::State { count: 0 });
    Router::new()
        // .route("/", get(|| async { "welcome !!!" }))
        // .nest("/api", short_links()
        // .merge(user_links()))
        .route("/",get_service(ServeDir::new(".")).handle_error(|e: std::io::Error| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", e),
                )
            }),
        )
        .merge(short_links(c))
        // 添加跟踪层,需要开启Debug日志
        .layer(TraceLayer::new_for_http())
        // 添加跨域层
        // .layer(CorsLayer::new().allow_origin("*".parse::<HeaderValue>().unwrap()))
        
       
       
}

#[allow(dead_code)]
fn short_links(c:&Config) -> Router {
    let mut r= Router::new();
    for i in 0..c.service.len() {
        r = r.nest(&c.service[i].path, get_service(ServeDir::new(&c.service[i].dir)).handle_error(|e:std::io::Error|async move{
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unhandled internal error: {}", e),
            )
        }));
    }
    r
}

