use {
    hyper::{
        // Hyper中的其他类型，用于构建HTTP
        Body, Client, Request, Response, Server, Uri,
        service::service_fn_ok,
        // 使用Hyper运行时可以运行future到完成的函数。
        rt::run,
    },
    futures::{
        // futures 0.1版本的一个扩展trait，添加了'.compat()'方法
        // 允许我们在0.1版本的futures中使用'.await'语法
        compat::Future01CompatExt,
        // 扩展trait在futures上提供了额外的方法在
        // `FutureExt` 添加了适用于所有futures的方法,
        // `TryFutureExt` 给futures添加了可以放回‘Result’类型的方法
        future::{FutureExt, TryFutureExt},
    },
    std::net::SocketAddr,
};

fn main() {
    println!("Hello, world!");
    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    let server_future = run_server(address);
    let run_rt = server_future.unit_error().boxed().compat();
    // 用Hyper提供的run方法运行我们的future直到完成
    run(run_rt);
}

fn server_req(req: Request<Body>) -> Response<Body> {
    println!("request method:{},uri: {}", req.method(), req.uri());

    // let url_str = "http://www.baidu.com";
    // let url = url_str.parse::<Uri>().expect("failed to parse URL");
    // let res = Client::new().get(url).compat();
    // // Return the result of the request directly to the user
    // println!("request finished --returning response");
    // println!("res = {:?}",res);
    //
    Response::new(Body::from("hello world"))
}

async fn run_server(address: SocketAddr) {
    println!("listen on: {}", address);
    let server_future = Server::bind(&address).serve(|| service_fn_ok(|req: Request<Body>| {
        server_req(req)
    }));

    if let Err(e) = server_future.compat().await {
        eprintln!("server run error:{}", e);
    }
}

