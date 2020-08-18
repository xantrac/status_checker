use actix::prelude::*;

pub struct MonitorActor {
    listeners: Vec<Addr<super::websocket_actor::Websocket>>,
}

impl Actor for MonitorActor {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {}
}

pub struct WsRegistration {
    pub address: Addr<super::websocket_actor::Websocket>,
}

pub struct StatusUpdate {
    pub status: String,
}

impl Message for WsRegistration {
    type Result = ();
}

impl Message for StatusUpdate {
    type Result = ();
}

impl Handler<WsRegistration> for MonitorActor {
    type Result = ();

    fn handle(&mut self, msg: WsRegistration, _: &mut Context<Self>) {
        self.listeners.push(msg.address);
    }
}

impl Handler<StatusUpdate> for MonitorActor {
    type Result = ();

    fn handle(&mut self, msg: StatusUpdate, _: &mut Context<Self>) {
        for listener in &self.listeners {
            listener.do_send(super::websocket_actor::StatusEvent {
                status: (&msg.status).to_string(),
            });
        }
    }
}

impl MonitorActor {
    pub fn new() -> Self {
        Self { listeners: vec![] }
    }
}
