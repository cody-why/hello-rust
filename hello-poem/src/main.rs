/*** 
 * @Author: plucky
 * @Date: 2022-07-10 13:48:01
 * @LastEditTime: 2022-07-11 18:20:00
 * @Description: 
 */

use poem::{listener::TcpListener, Route};

use hello_poem::*;
use poem_openapi::OpenApiService;



#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    init_log();
    
    info!("hello, world!");

    let api_service =
        OpenApiService::new(api::Api, "Hello World", "1.0").server("http://localhost:3000/api");
        // swager_ui,redoc,rapidoc
    let ui = api_service.rapidoc();
    let app = Route::new().nest("/api", api_service).nest("/", ui);

    poem::Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await

}

fn init_log(){
    use tracing_subscriber::fmt::time::OffsetTime;

    let local_time = OffsetTime::new(
        time::UtcOffset::from_hms(8, 0, 0).unwrap(),
        time::format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]").unwrap(),
    );
    tracing_subscriber::fmt().with_timer(local_time).init();
}