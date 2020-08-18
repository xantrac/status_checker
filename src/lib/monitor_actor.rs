use actix::prelude::*;
use std::time::Duration;

pub struct MonitorActor {
    listeners: Vec<Addr<super::websocket_actor::Websocket>>,
}

impl Actor for MonitorActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.run_interval(Duration::from_secs(5), |act, _| {
            for listener in &act.listeners {
                listener.do_send(super::websocket_actor::StatusEvent {
                    status: String::from("COCONUT"),
                });
            }
        });
    }
}

pub struct WsRegistration {
    pub address: Addr<super::websocket_actor::Websocket>,
}

impl Message for WsRegistration {
    type Result = ();
}

impl Handler<WsRegistration> for MonitorActor {
    type Result = ();

    fn handle(&mut self, msg: WsRegistration, _: &mut Context<Self>) {
        self.listeners.push(msg.address);
    }
}

impl MonitorActor {
    pub fn new() -> Self {
        Self { listeners: vec![] }
    }
}
