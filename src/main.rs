//service::service_fn是一个方便创建服务实例的函数
use hyper::{Body, Method, Request, Response, StatusCode, header, service::service_fn};
//这行代码使用了Rust的use关键字，将Hyper库中的server模块下的Server结构体引入到当前作用域中。
//这意味着在后续的代码中可以直接使用Server，而不需要每次都写完整的路径hyper::server::Server
use hyper::server::Server;
use std::convert::Infallible;
use std::net::SocketAddr;

//这行代码声明了一个名为 service 的模块。模块可以包含其他模块、函数、类型、常量等。声明模块后，Rust编译器会在当前文件所在的目录下查找一个名为 service.rs 的文件，
//或者在一个名为 service 的目录下查找一个名为 mod.rs 的文件。这些文件中包含的代码将成为 service 模块的一部分。
mod service;

//Result<Response<Body>, hyper::Error>是一个枚举类型，表示函数的返回值有两种可能的情况：
//Ok(Response<Body>)：表示请求处理成功，返回一个 Response<Body> 类型的HTTP响应。
pub async fn handle_request(req: Request<Body>) ->Result<Response<Body>, hyper::Error>{
    //创建响应构建器:以便支持跨域请求
    let response_builder = Response::builder()
        .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .header(header::ACCESS_CONTROL_ALLOW_METHODS, "GET,POST,OPTIONS")
        .header(header::ACCESS_CONTROL_ALLOW_HEADERS, "Content-Type");

    //匹配请求方法和路径：
    match (req.method(), req.uri().path()) {
        (&Method::OPTIONS, _) => {
            //由于 response_builder.body 可能会失败（例如，如果响应构建器的状态不正确），我们需要处理潜在的错误
           Ok(response_builder.body(Body::empty()).unwrap())
        }
        (&Method::GET, "/api/hello") => {
            let result = service::get_hello();
            Ok(response_builder.body(Body::from(result)).unwrap())
        }
        (&Method::POST, "/api/echo") => {
            //req.into_body()：将Request对象转换为Body对象。Body是Hyper库中表示HTTP请求或响应体的类型，可以是一个流式的字节序列
            //hyper::body::to_bytes(...)：将Body对象转换为一个完整的字节数组。to_bytes函数会等待整个请求体被读取完毕
            //.await?：这是一个异步操作，使用await关键字等待异步任务完成
            let body_bytes = hyper::body::to_bytes(req.into_body()).await?;
           //这一行调用了一个名为service::post_echo的函数，将读取到的请求体字节数组传递给该函数。service::post_echo函数的具体实现不在这段代码中展示，但可以假设它负责处理请求体并生成响应内容。result变量将存储处理后的结果。
           let result = service::post_echo(body_bytes);
            Ok(response_builder.body(Body::from(result)).unwrap())
        }
        _ => {
            Ok(response_builder
                //设置响应的状态码为 404 Not Found
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("Not Found"))
                .unwrap())
        }
    }
}


#[tokio::main]
async fn main() {
    // 将服务器绑定到本地的3000端口
    let addr: SocketAddr = ([0, 0, 0, 0], 3000).into();

    // 创建服务工厂，在每次有新的连接时被调用，用于创建一个新的 Service 实例来处理请求
    let make_svc = hyper::service::make_service_fn(|_conn| {
        async {
            Ok::<_, Infallible>(service_fn(handle_request))
        }
    });

    // 创建并启动服务器,绑定到指定的地址和端口，并使用提供的服务工厂来处理请求
    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    // 运行服务器
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

