use actix::prelude::*;
use actix_web::{web,Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use bincode::deserialize;

use super::ws_message::InternodeMessage;
pub async fn ws_index(r: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
  info!("Websocket handshake");
    ws::start(Ws, &r, stream)
}


struct Ws;

impl Actor for Ws {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Ws {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
 if let Ok(msg) = msg {
             match msg {
            ws::Message::Ping(msg) => {
                ctx.pong(&msg);
            }
            ws::Message::Pong(msg) => {
                ctx.ping(&msg);
            }
            ws::Message::Text(text) => ctx.text(text),
            ws::Message::Binary(bin) => {
                let decoded: InternodeMessage = deserialize(&bin[..]).unwrap();
                info!("{:?}", decoded);
            }
            ws::Message::Close(_) => {
                ctx.stop();
            }
            _=>{}//actix_web_actors::ws::Message::Continuation(_)` and `actix_web_actors::ws::Message::Nop
        }
        } else {
            ctx.stop();
        }
      
    }
}
