use std::{thread, time};

pub fn thr_sleep(t: u64) {
  thread::sleep(time::Duration::from_millis(t));
}

pub fn drop<T>(lock: T) {
  std::mem::drop(lock);
}