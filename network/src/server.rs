use std::net::SocketAddr;
use std::{io, thread};
use std::sync::mpsc::Sender;

use futures::Future;
use futures::future::result;
use tokio_proto::TcpServer;
use tokio_service::Service;

use util::config::ChitConfig;
use protocol::{ChitProto, ChitRequest, ChitResponse};
use msghandle::net_msg_handler;

#[derive(Clone)]
pub struct MySender {
    tx: Sender<(u32, ChitRequest)>,
}

impl MySender {
    pub fn new(tx: Sender<(u32, ChitRequest)>) -> Self {
        MySender { tx: tx }
    }

    pub fn send(&self, msg: (u32, ChitRequest)) {
        self.tx.send(msg).unwrap();
    }
}

unsafe impl Sync for MySender {}

struct Server {
    mysender: MySender,
}

impl Service for Server {
    type Request = ChitRequest;
    type Response = ChitResponse;
    type Error = io::Error;
    type Future = Box<Future<Item = Self::Response, Error = io::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        Box::new(result(net_msg_handler(req, &self.mysender)))
    }
}

pub fn start_server(config: &ChitConfig, tx: Sender<(u32, ChitRequest)>) {
    let mysender = MySender::new(tx);
    let addr = format!("0.0.0.0:{}", config.port);
    let addr = addr.parse::<SocketAddr>().unwrap();

    thread::spawn(move || {
                      info!("start server on {:?}!", addr);
                      TcpServer::new(ChitProto, addr)
                          .serve(move || Ok(Server { mysender: mysender.clone() }));
                  });
}
