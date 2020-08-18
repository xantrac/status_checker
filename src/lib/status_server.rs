use actix::prelude::*;
use rand::seq::SliceRandom;
use std::thread;
use std::time::Duration;

pub fn start_polling(monitor_address: &Addr<super::monitor_actor::MonitorActor>) -> () {
    // thread::spawn(move || loop {
    // let status = super::clients::github_status().await;
    thread::spawn(|| loop {
        let list = vec!["BANANA", "COCONUT", "MANGO", "WATERMELON"];
        let fruit = list.choose(&mut rand::thread_rng()).unwrap();
        let status = fruit.to_string();
        monitor_address.do_send(super::monitor_actor::StatusUpdate { status: status });
        thread::sleep(Duration::from_secs(5));
    });
}

fn list(addr: &Addr<super::monitor_actor::MonitorActor>) {}
