use actix_web::{
    http::StatusCode, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Scope,
};
use std::io::Result;

// 定义router module
mod router;

// http://localhost:8080/hey
// http://localhost:8080/

// 路由注册规则
// 对于使用了路由宏的 handler，使用 App::service 方法注册路由；
// 对不使用路由宏而注册自定义路由的情况，使用 App::route 方法。

#[actix_web::main]
async fn main() -> Result<()> {
    let address = "127.0.0.1:8080";
    println!("server run on: {}", address);

    // 创建一个http server
    // 默认采用多线程模式运行http server
    HttpServer::new(|| {
        App::new()
            .service(router::router::hello)
            .service(router::router::echo)
            .service(router::router::run_api())
            .route("/hey", web::get().to(router::router::manual_hello))
            .default_service(
                // 默认路由找不到的时候
                web::route().to(|| HttpResponse::NotFound().body("this page not found")),
            )
    })
    .bind(address)?
    .run()
    .await
}
