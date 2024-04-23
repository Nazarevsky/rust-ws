use actix::{Actor, StreamHandler};
use actix_web::{web, App, HttpServer, Responder, HttpRequest};
use actix_web_actors::ws;

async fn ws_index(req: HttpRequest, stream: web::Payload) -> impl Responder {
    // Establish WebSocket connection
    let resp = ws::start(WsActor {}, &req, stream);
    println!("WebSocket connection established: {:?}", resp);
    resp
}

struct WsActor;

impl Actor for WsActor {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("WebSocket connection started");
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        println!("WebSocket connection closed");
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsActor {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                println!("Received text message: {}", text);
                ctx.text(text);
            }
            Ok(ws::Message::Binary(bin)) => {
                println!("Received binary message: {:?}", bin);
                ctx.binary(bin);
            }
            _ => (),
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/ws/", web::get().to(ws_index))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
