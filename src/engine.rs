use std::{thread, time};

use lazy_static::lazy_static; // 1.4.0
use parking_lot::Mutex;

use crate::utils::*;

pub enum LoadDBResult {
  Loaded,
  NotFound,
}
#[derive(Debug, Clone)]
pub enum Process {
  Starting,
  Waiting,
  Exit,
  LoadDB(String),
  Response(String),
  ResponseCode(i32),
  GetTeam(String),
}

lazy_static! {
  pub static ref PROCESS: Mutex<Process> = Mutex::new(Process::Starting);
}

pub fn check_response(process: Process) -> String {
  let mut response = String::new();

  let mut lock = PROCESS.lock();
  *lock = process.clone();
  drop(lock);

  loop {
    let mut lock = PROCESS.lock();
    match &*lock {
      Process::Response(resp) => {
        response = resp.clone();
        break;
      },
      _ => {}
    }
    drop(lock);
    thr_sleep(1);
  }
  response
}


// Função local chamada internamente quando o frontend solicita o carregamento do DB ao backend
pub fn load_db(path: &String) {
  *PROCESS.lock() = Process::Response("Ok".to_string());
}

pub fn exit() {
  *PROCESS.lock() = Process::Exit;
}

pub fn get_team(team_name: String) {
  *PROCESS.lock() = Process::Response(format!("Team is: {}", team_name).to_string());
}