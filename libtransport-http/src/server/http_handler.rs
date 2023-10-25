use actix_web::{web, HttpRequest, HttpResponse};

use super::AppState;

use super::heartbeat::GetHeartbeatCount;

use std::sync::atomic::Ordering;

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitTransaction {
    pub signature: String,
    pub payload: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckTransactionStatus {
    id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Peer {
    id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeerList {
    peers: Vec<Peer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TransactionStatus {
    Complete,
    Pending,
    Failed,
}

pub async fn submit_transaction(
 web::Json(val): web::Json<SubmitTransaction>,
) ->  Result<HttpResponse, actix_web::Error> {
            debug!("model: {:?}", val);
        Ok(HttpResponse::Ok().json(val))
}

pub async fn heartbeat( app_state: web::Data<AppState>) ->  Result<HttpResponse, actix_web::Error>  {
    // debug!("{:?}", req);

    app_state.counter.fetch_add(1, Ordering::Relaxed);

    let res = app_state.heartbeat_counter.send(GetHeartbeatCount);
        // Box::pin(async move { res.await.unwrap() });
        let _=res.await.map(|res| match res {
            Ok(result) => info!("Got result: {:?}", result),
            Err(err) => error!("Got error: {:?}", err),
        })
        .map_err(|e| {
            debug!("Actor is probably dead: {}", e);
        });

    Ok(HttpResponse::Ok().body(format!(
        "Num of requests: {}",
        app_state.counter.load(Ordering::Relaxed),
    )))
}

pub async fn check_transaction_status(
    req: HttpRequest,
) ->   Result<HttpResponse, actix_web::Error>  {
    let _transaction_id = req.match_info().get("id").expect("no id provided");

    Ok(HttpResponse::Ok().json(TransactionStatus::Failed))
}

pub async fn get_peers() ->  Result<HttpResponse, actix_web::Error>  {
    let peers = vec![Peer {
        id: "wefwef".to_string(),
    }];

    Ok(HttpResponse::Ok().json(peers))
}
