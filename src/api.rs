use std::ffi::{CStr, CString};
use std::{thread, time};

use libc::c_char;

use crate::engine::*;
use crate::utils::*;

// Função exportada para ser chamada pelo frontend
pub unsafe extern "C" fn load_db_with_path(path: *const c_char) -> i32 {
  let c_str = unsafe {CStr::from_ptr(path)};
  let resp = check_response(Process::LoadDB(c_str.to_str().unwrap().to_string()));
  *PROCESS.lock() = Process::Waiting;
  1
}

pub unsafe extern "C" fn quit() {
  *PROCESS.lock() = Process::Exit;
}

pub unsafe extern "C" fn get_team_with_name(team_name: *const c_char) -> *mut c_char {
  let c_str = unsafe { CStr::from_ptr(team_name)};
  let resp = check_response(Process::GetTeam(c_str.to_str().unwrap().to_string()));
  let c_string = CString::new(resp).unwrap();
  *PROCESS.lock() = Process::Waiting;
  c_string.into_raw()  
}


pub unsafe extern "C" fn start_game_thread() {
  //Permanece em loop constantemente
  //Um enum global informa quando e qual ação será feita
  //Após o termino da ação o enum é retornado com uma resposta
	loop {
    let mut lock = PROCESS.lock();
    match &*lock {
      Process::Starting => { // Fazer configurações iniciais,
        println!("Starting...");
        drop(lock);
        *PROCESS.lock() = Process::Waiting;
      },
      Process::Waiting => (), // Aguardando por comando do frontend,
      Process::LoadDB(_path) => { // Carrega o banco de dados
        let path = _path.clone();
        println!("calling loadDB");
        drop(lock);
        load_db(&path); 
      },
      Process::GetTeam(team_name) => {
        let _team_name = team_name.clone();
        println!("calling get_team");
        drop(lock);
        get_team(_team_name);
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

pub extern "C" fn free_string(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}