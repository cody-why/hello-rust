/***
 * @Author: plucky
 * @Date: 2022-08-10 19:42:45
 * @LastEditTime: 2022-08-13 16:58:13
 * @Description: 
 */

use std::error::Error;
use std::net::SocketAddr;
use tracing::{debug, info};

mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    let config = config::load_config();

    config::init_log(&config.log);
    info!("{:#?}", config);

    debug!("debug is ok");

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    info!("Server bind at: {:?}", addr);

    axum::Server::bind(&addr)
        .serve(config::routes::app(&config).into_make_service())
        .await?;
    
    Ok(())
}


