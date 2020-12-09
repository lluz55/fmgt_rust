#![allow(unused)]
use std::{ffi::{CStr, CString}, io::{stdin, stdout}};
use std::{thread, time};

use parking_lot::Mutex;
use libc::c_char;

mod api;
use crate::api::*;

mod engine;
use crate::engine::*;

mod utils;
use crate::utils::*;

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
    // let path = CString::new("user_db").unwrap();
    let path = CString::new(user_db).unwrap();
    let c_team_name = path.as_ptr() as *const c_char;
    let c_resp = unsafe { get_team_with_name(c_team_name) };
    let c_str = unsafe { CStr::from_ptr(c_resp)};
    println!("'get_team_with_name' response: {}", c_str.to_str().unwrap().trim());

    unsafe { quit() };
    println!("Exiting secondary thread");
    free_string(c_resp);
  });
  unsafe { start_game_thread(); }    
  println!("Exiting main thread");
  println!("duration: {:?}", now.elapsed());
}
