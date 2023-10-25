use actix_web::{web::{self,resource,Data},body::MessageBody, Error, dev::{ServiceFactory, ServiceRequest, ServiceResponse}, middleware, HttpServer, App};
use actix::prelude::*;
  

use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

mod heartbeat;
pub mod http_handler;
pub mod ws_handler;
pub mod ws_message;

use self::heartbeat::Heartbeat;
use self::http_handler::{check_transaction_status, get_peers, heartbeat, submit_transaction};
use self::ws_handler::ws_index;
pub struct Server;

#[derive(Clone)]
pub struct AppState {
    counter: Arc<AtomicUsize>,
    heartbeat_counter: Addr<Heartbeat>,
}

impl Server {
    pub fn create_app() -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Config = (),
        InitError = (),
        Error = Error,
    >,
>{
        let addr =  Heartbeat { count: 0 }.start();

        let counter = Arc::new(AtomicUsize::new(0));

App::new()
         .wrap(middleware::Logger::default())
        .app_data(Data::new(AppState {
            counter: counter.clone(),
            heartbeat_counter: addr,
        }))
            .service(resource("/transaction").route(web::post()).to( submit_transaction)
            )
            .service(resource("/transaction/{id}").route(web::get()).to(check_transaction_status)
            )
            .service(resource("/peer").route(web::get()).to(get_peers))
            .service(resource("/heartbeat").route(web::get()).to(heartbeat))
            .service(resource("/ws").route(web::get()).to(ws_index))
    }
//-> HttpServer<App<AppState>, impl Fn() -> App<AppState> + Send + Clone + 'static>
    pub async fn init() {
        let counter = Arc::new(AtomicUsize::new(0));

        let addr = Heartbeat { count: 0 }.start();

        let _=HttpServer::new(move ||  {
App::new()
            .wrap(middleware::Logger::default())
  .app_data(Data::new(AppState {
            counter: counter.clone(),
            heartbeat_counter: addr.clone(),
        }))
            .service(resource("/transaction").route(web::post()).to( submit_transaction)
            )
            .service(resource("/transaction/{id}").route(web::get()).to(check_transaction_status)
            )
            .service(resource("/peer").route(web::get()).to(get_peers))
            .service(resource("/heartbeat").route(web::get()).to(heartbeat))
            .service(resource("/ws").route(web::get()).to(ws_index))
        }).bind(format!("{}:{}", &"host", &1279)).expect("REASON")
    .run().await;
    }
}

#[cfg(test)]
mod tests {
    use super::http_handler::SubmitTransaction;

    use super::*;
    use actix_web::test::TestServer;
    use actix_web::HttpMessage;
    use futures::future::Future;

    #[test]
    fn test_submit_transaction() {
        let mut server = TestServer::with_factory(Server::create_app);

        let request = server
            .client(http::Method::POST, "/transaction")
            .json(SubmitTransaction {
                signature: "efwef".to_string(),
                payload: "WEfwef".to_string(),
            })
            .unwrap();

        let response = server.execute(request.send()).unwrap();
        assert!(response.status().is_success());
    }

    #[test]
    fn test_get_peers() {
        let mut server = TestServer::with_factory(Server::create_app);

        let request = server.client(http::Method::GET, "/peer").finish().unwrap();

        let response = server.execute(request.send()).unwrap();
        assert!(response.status().is_success());
    }

    #[test]
    fn test_check_transaction_status() {
        let mut server = TestServer::with_factory(Server::create_app);

        let request = server
            .client(http::Method::GET, "/transaction/0x81732be82h")
            .finish()
            .unwrap();

        let response = server.execute(request.send()).unwrap();
        println!("{:?}", response.body().wait());
        assert!(response.status().is_success());
    }
}
