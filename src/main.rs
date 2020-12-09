#![allow(unused)]
use libc::c_char;
use std::{ffi::{CStr, CString}, io::{stdin, stdout}};
use lazy_static::lazy_static; // 1.4.0
use parking_lot::Mutex;

use std::{thread, time};

lazy_static! {
  static ref PROCESS: Mutex<Process> = Mutex::new(Process::Starting);
}

#[derive(Debug, Clone)]
enum Process {
  Starting,
  Waiting,
  Exit,
  LoadDB(String),
  Response(String),
  ResponseCode(i32),
}

enum LoadDBResult {
  Loaded,
  NotFound,
}

fn thr_sleep(t: u64) {
  thread::sleep(time::Duration::from_millis(t));
}

fn check_response(process: Process) -> String {
  let mut response = String::new();

  let mut proc = PROCESS.lock();
  *proc = process.clone();
  std::mem::drop(proc);

  loop {
    let mut proc = PROCESS.lock();
    match &*proc {
      Process::Response(resp) => {
        response = resp.clone();
        break;
      },
      _ => {}
    }
    std::mem::drop(proc);
    thr_sleep(1);
  }
  response
}

// Função exportada para ser chamada pelo frontend
unsafe extern "C" fn load_db_with_path(path: *const c_char) -> i32 {
  let c_str = unsafe {CStr::from_ptr(path)};
  let resp = check_response(Process::LoadDB(c_str.to_str().unwrap().to_string()));
  1
}

unsafe extern "C" fn quit() {
  *PROCESS.lock() = Process::Exit;
}

// Função local chamada internamento quando o frontend solicita ao backend
fn load_db(path: &String) {
  *PROCESS.lock() = Process::Response("Ok".to_string());
}

fn exit() {
  *PROCESS.lock() = Process::Exit;
}

unsafe extern "C" fn start_game_thread() {
  //Permanece em loop constantemente
  //Um enum global informa quando e qual ação será feita
  //Após o termino da ação o enum é retornado com uma resposta
	loop {
    let mut lock = PROCESS.lock();
    match &*lock {
      Process::Starting => {
        println!("Starting...");
        std::mem::drop(lock);
        *PROCESS.lock() = Process::Waiting;
      }, // Fazer configurações iniciais,
      Process::Waiting => (), // Aguardando por comando do frontend,
      Process::LoadDB(_path) => {
        let path = _path.clone();
        println!("calling loadDB");
        std::mem::drop(lock);
        load_db(&path); // Carrega o banco de dados
      },
      Process::Exit => {
        println!("Exiting game loop");
        break;
      }
      _ => continue,
    }
    thr_sleep(1);
	}
}
    

fn main() {
  let now = std::time::Instant::now();

  thread::spawn(move || {
    use std::io::{stdin, stdout, Write};
    print!("Choose DB name: ");
    let mut user_db = String::new();
    let _= stdout().flush();
    match stdin().read_line(&mut user_db) {
      _ => {}
    }
    // thread::sleep(time::Duration::from_secs(15));
    let path = CString::new("user_db").unwrap();
    let c_path = path.as_ptr() as *const c_char;
    let val = unsafe { load_db_with_path(c_path) };
    println!("'load_db_with_path' response: {}", val);
    unsafe { quit() };
    println!("Exiting secondary thread");
  });
  unsafe { start_game_thread(); }    
  println!("Exiting main thread");
  println!("duration: {:?}", now.elapsed());
}
