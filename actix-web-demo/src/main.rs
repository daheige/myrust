use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
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
            .service(
                // 路由前缀设置
                // http://localhost:8080/api/index
                // 自定义闭包形式的handler
                web::scope("/api")
                    .service(router::router::index)
                    .route(
                        "/info",
                        web::get().to(|req: HttpRequest| {
                            println!("req:{:?}", req);
                            println!("req method:{},uri:{}", req.method(), req.uri());
                            let headers = req.headers();
                            println!("req header:{:?}", headers);
                            HttpResponse::Ok().body("api/info ok")
                        }),
                    )
                    .route("/home", web::get().to(router::router::home))
                    .route("/foo", web::get().to(router::router::foo)),
            )
            .route("/hey", web::get().to(router::router::manual_hello))
    })
    .bind(address)?
    .run()
    .await
}
