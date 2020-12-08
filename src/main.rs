#![allow(unused)]
use libc::c_char;
use std::ffi::CStr;
use lazy_static::lazy_static; // 1.4.0
use std::sync::Mutex;

use std::{thread, time};

lazy_static! {
    static ref PROCESS: Mutex<Process> = Mutex::new(Process::LoadDB("testing".to_string()));
    static ref IN_PROCESS: Mutex<bool> = Mutex::new(false);
}

#[derive(Debug)]
enum Process {
    Starting,
    Waiting,
    LoadDB(String),
    Response(String),
    ResponseCode(i32),
}

fn thr_sleep() {
    thread::sleep(time::Duration::from_millis(10));
}

fn check_response(process: Process) -> String {
    let mut response = String::new();
    if *IN_PROCESS.lock().unwrap() {
        //retorna uma mensagem informando que a fila está ocupada
        return "aborted".to_string()
    }
    loop {
        match process {
            Process::LoadDB(ref path) => load_db(path),
            Process::Response(resp) => {
                response = resp;
                break;
            },
            _ => {}
        }
    	thr_sleep();
    }
    *IN_PROCESS.lock().unwrap() = false;
    response
}

// Função exportada para ser chamada pelo frontend
unsafe extern "C" fn load_db_with_path(path: *const c_char) -> i32 {
    let c_str = unsafe {CStr::from_ptr(path)};
    let resp = check_response(Process::LoadDB(c_str.to_str().unwrap().to_string()));
    0
}

// Função local chamada internamento quando o frontend solicita ao backend
fn load_db(path: &String) {
    *PROCESS.lock().unwrap() = Process::ResponseCode(0);
}

unsafe extern "C" fn start_game_thread() {
    //Permanece em loop constantemente
    //Um enum global informa quando e qual ação será feita
    //Após o termino da ação o enum é retornado com uma resposta
	loop {
    	match &*PROCESS.lock().unwrap() {
            Process::Starting => (), // Fazer configurações iniciais,
            Process::Waiting => (), // Aguardando por comando do frontend,
            Process::LoadDB(path) => {
                *IN_PROCESS.lock().unwrap() = true;
                load_db(path); // Carrega o banco de dados
            },
            _ => {}
    	
    	}
    	thr_sleep();
	}
}
    

fn main() {
    //unsafe { start_game_thread(); }
    let p = PROCESS.lock().unwrap();
    println!("Response: {:?}", (&*p));
}
