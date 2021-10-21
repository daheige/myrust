use actix_web::{
    get, post, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder, Scope,
};
use futures::future::{ready, Ready};
use serde::Serialize;
use serde_json;

// 通过宏定义handler
// 请求handler使用异步函数，接受零个或多个参数，返回可以转换为HttpResponse类型
#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello world")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// 定义handler处理器
pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/index")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("index home")
}

// http://localhost:8080/api/home
pub async fn home(_req: HttpRequest) -> &'static str {
    "rust web"
}

// 返回String类型
pub async fn foo() -> String {
    "foo,ok".to_string()
}

// 自定义响应类型
// 要直接从 handler 函数返回自定义类型，则该类型需要实现 Responder trait。
// https://actix-web.budshome.com/handlers.html
#[derive(Serialize)]
struct MyObj {
    name: &'static str,
}

// 实现Responder 需要实现respond_to
impl Responder for MyObj {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;
    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        // Create response and set content type
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

pub async fn index2(_req: HttpRequest) -> impl Responder {
    MyObj {
        name: "rust develop",
    }
}

pub async fn index3(_req: HttpRequest) -> impl Responder {
    web::Json(MyObj {
        name: "rust index3",
    })
    .with_header("x-version", "1.2.3")
}

// 以api作为前缀
pub fn run_api() -> Scope {
    // 路由前缀设置
    // http://localhost:8080/api/index
    // 自定义闭包形式的handler /api/info
    let v1 = web::scope("/api")
        .service(index)
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
        .route("/home", web::get().to(home))
        .route("/foo", web::get().to(foo))
        .route("/my-object", web::get().to(index2))
        .route("/my-object3", web::get().to(index3));

    v1
}
