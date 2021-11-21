use std::{collections::HashMap, result, string, vec};

use actix_web::{
    get, http, post, web, App, Either, Error, HttpRequest, HttpResponse, HttpServer, Responder,
    Result, Scope,
};
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};
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

/*
差异化返回类型（Either 枚举）

有时，你需要返回不同类型的响应。比如，你可以检查错误和返回错误：
返回错误的异步响应，或者返回依赖于两个不同类型的任意结果（result）
 */
type RegisterResult = Either<HttpResponse, Result<&'static str, Error>>;

fn is_a_variant() -> bool {
    let a = 1;
    a > 0
}

#[get("/either")]
async fn either() -> RegisterResult {
    if is_a_variant() {
        // <- choose variant A
        Either::A(HttpResponse::BadRequest().body("Bad data"))
    } else {
        // <- variant B
        Either::B(Ok("Hello!"))
    }
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

pub async fn say(req: HttpRequest) -> Result<String> {
    println!("req.method: {}", req.method());
    Ok("abc".to_string())
}

#[derive(Serialize, Deserialize)]
struct Res<T> {
    code: i64,
    message: String,
    data: T,
}

#[derive(Serialize, Deserialize)]
struct EmptyObject {} // 空对象{}定义

// 采用空对象的模式，构建空数组，主要是兼容其他语言的[]
type EmptyArray<EmptyObject> = Vec<EmptyObject>;

pub async fn say2(req: HttpRequest) -> Result<HttpResponse> {
    println!("req.method: {}", req.method());
    let mut v: Vec<i32> = Vec::new();
    v.push(123);

    // let result = Res {
    //     code: 0,
    //     message: "ok".to_string(),
    //     data: v,
    // };
    let result = Res {
        code: 0,
        message: "ok".to_string(),
        data: v,
    };

    // let mut map = HashMap::new();
    // map.insert("list", v);
    // Ok(HttpResponse::Ok().json(map))
    Ok(HttpResponse::Ok().json(result))
}

pub async fn empty_obj(_req: HttpRequest) -> Result<HttpResponse> {
    let empty_obj = EmptyObject {};
    let result = Res {
        code: 0,
        message: "ok".to_string(),
        data: empty_obj, // to json的时候这个就是一个{}
    };

    Ok(HttpResponse::Ok().json(result))
}

pub async fn empty_array(_req: HttpRequest) -> Result<HttpResponse> {
    let empty_arr: EmptyArray<EmptyObject> = vec![];
    let result = Res {
        code: 0,
        message: "ok".to_string(),
        data: empty_arr, // to json的时候这个就是一个{}
    };

    Ok(HttpResponse::Ok().json(result))
}

pub async fn index3(_req: HttpRequest) -> impl Responder {
    web::Json(MyObj {
        name: "rust index3",
    })
    .with_header("x-version", "1.2.3")
}

/// 路由参数定义
/// http://localhost:8080/api/users/1/daheige
#[get("/users/{user_id}/{friend}")]
async fn get_user(web::Path((user_id, friend)): web::Path<(u32, String)>) -> Result<String> {
    Ok(format!("welcome {},uid:{}", friend, user_id))
}

// 将路径参数提取到指定的结构体中
#[derive(Deserialize)]
struct UserInfo {
    user_id: i32,
    friend: String,
}

// http://localhost:8080/api/users/read/1/daheige
#[get("/users/read/{user_id}/{friend}")]
async fn get_user2(info: web::Path<UserInfo>) -> Result<String> {
    Ok(format!("welcome {},uid:{}", info.friend, info.user_id))
}

#[get("/user/info/{user_id}/{friend}")]
async fn user_info(info: web::Path<UserInfo>) -> Result<HttpResponse> {
    println!("welcome {},uid:{}", info.friend, info.user_id);
    // 返回json格式
    Ok(HttpResponse::Ok().json(MyObj { name: "abc" }))
}

// http://localhost:8080/api/users/query?user_id=1&friend=daheige
#[get("/users/query")]
async fn query_user(info: web::Query<UserInfo>) -> Result<String> {
    Ok(format!("welcome {},uid:{}", info.friend, info.user_id))
}

// 接收json内容，post方法
/*
% curl --location --request POST 'localhost:8080/api/users/post-json' \
--header 'Content-Type: application/json' \
--data-raw '{"user_id":1,"friend":"daheige"}'
welcome daheige,uid:1
*/
#[post("/users/post-json")]
async fn post_json(info: web::Json<UserInfo>) -> Result<String> {
    Ok(format!("welcome {},uid:{}", info.friend, info.user_id))
}

// 以api作为前缀
pub fn run_api() -> Scope {
    // 路由前缀设置
    // http://localhost:8080/api/index
    // 自定义闭包形式的handler /api/info
    let v1 = web::scope("/api")
        .service(index)
        .service(either)
        .service(get_user)
        .service(get_user2)
        .service(query_user)
        .service(post_json)
        .service(user_info)
        .route(
            "/user/{name}",
            web::get().to(|| {
                println!("11");
                let body = serde_json::to_string(&MyObj { name: "abc" }).unwrap();
                HttpResponse::Ok().body(body)
            }),
        )
        .route(
            "/abc",
            web::post().to(|| HttpResponse::Ok().body("{\"code\":0}")),
        )
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
        .route("/my-object3", web::get().to(index3))
        .route("/say", web::get().to(say))
        .route("/say2", web::get().to(say2))
        .route("/empty-obj", web::get().to(empty_obj))
        .route("/empty-arr", web::get().to(empty_array));
    v1
}

// 单元测试
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("test");
    }

    #[test]
    fn test_str() {
        let s = String::from("abc");
        println!("s = {}", s.as_str());
        println!("s = {}", &s);
    }

    use actix_web::http;
    use actix_web::test;
    #[actix_rt::test]
    async fn test_index() {
        let req = test::TestRequest::with_header("content-type", "text/plain").to_http_request();
        let resp = super::home(req).await;
        println!("status:{:?}", resp);
        assert_eq!(resp, "rust web");
    }
}
