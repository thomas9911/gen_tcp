use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;

use http_body_util::Full;
use hyper_util::server::conn::auto::Builder;
use hyper::body::Bytes;
use hyper::server::conn::http2;
use hyper::service::service_fn;
use hyper::{Request, Response};
use matchit::MatchError;
use rustler::{NifStruct, NifUnitEnum};
use smol::net::TcpListener;
use smol_hyper::rt::FuturesIo;
use smol_hyper::rt::SmolExecutor;
use smol::Executor;
use std::sync::Arc;
use handlebars::Handlebars;

#[rustler::nif]
fn add(a: i64, b: i64) -> i64 {
    a + b
}

#[rustler::nif(schedule = "DirtyIo")]
fn serve(router: Router) -> Result<(), String> {
    smol::block_on(xd(router))
}

async fn hello(
    req: Request<hyper::body::Incoming>,
    router: Arc<matchit::Router<String>>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    match router.at(req.uri().path()) {
        Ok(found) => {
            let value_map: HashMap<_, _> = found.params.iter().collect();
            if value_map.is_empty() {
                Ok(Response::new(Full::new(Bytes::from(found.value.to_string()))))
            } else {
                let reg = Handlebars::new();
                let out = reg.render_template(found.value, &value_map).unwrap_or(String::from("BIEM!!"));
                Ok(Response::new(Full::new(Bytes::from(out))))
            }
        }
        Err(MatchError::NotFound) => Ok(Response::builder().status(404).body(Full::new(Bytes::from(""))).unwrap()),
        Err(e) => Ok(Response::builder().status(400).body(Full::new(Bytes::from(e.to_string()))).unwrap())
    }
}


#[derive(Debug, NifUnitEnum)]
pub enum Method {
    GET,
    POST
}

#[derive(Debug, NifStruct)]
#[module = "GenTcp.Router.Route"]
pub struct Route {
    method: Method,
    path: String,
    response: String,
}

#[derive(Debug, NifStruct)]
#[module = "GenTcp.Router"]
pub struct Router {
    routes: Vec<Route>
}

async fn xd(router_info: Router) -> Result<(), String> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = TcpListener::bind(addr).await.map_err(|e| e.to_string())?;

    let mut router = matchit::Router::new();

    for route in router_info.routes {
        router.insert(route.path, route.response).map_err(|e| e.to_string())?;
    }

    let ex = Arc::new(Executor::new());
    let arc_router = Arc::new(router);

    // We start a loop to continuously accept incoming connections
    loop {
        let (stream, _) = listener.accept().await.map_err(|e| e.to_string())?;
        let io = FuturesIo::new(stream);
        let arc_router3 = arc_router.clone();

        let ex1 = ex.clone();
        let task = smol::spawn(async move {
            let arc_router2 = arc_router3.clone();
            let executor = SmolExecutor::new(ex1);
            if let Err(err) = Builder::new(executor)
                .serve_connection(
                    io,
                    service_fn(move |req| {
                        let router1 = arc_router2.clone();
                        async move { hello(req, router1).await }
                    }),
                ).await
            {
                println!("Error serving connection: {:?}", err);
            }
        });

        task.await;
    }
}

rustler::init!("Elixir.GenTcp.Native", [add, serve]);
