
use axum::http::{ StatusCode};


use axum::routing::get_service;
use axum::{Router};



use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

pub fn app() -> Router {
    //let state = Arc::new(state::State { count: 0 });
    Router::new()
        // .route("/", get(|| async { "welcome !!!" }))
        // .nest("/api", short_links()
        // .merge(user_links()))
        .nest(
            "/",
            get_service(ServeDir::new(".")).handle_error(|error: std::io::Error| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                )
            }),
        )
        // 添加跟踪层,需要开启Debug日志
            .layer(TraceLayer::new_for_http())
            // 添加跨域层
            // .layer(CorsLayer::new().allow_origin("*".parse::<HeaderValue>().unwrap()))
        
       
}



