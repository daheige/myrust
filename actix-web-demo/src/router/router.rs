use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

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
