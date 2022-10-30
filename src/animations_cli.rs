use std::io::{self, Write};
use std::{thread, time};

pub fn play_an_animations(msgs: Vec<&str>, repeat_times: u32, sleep_millis: u64) {
  let mut i = 0;
  loop {
    if i == repeat_times {
      break;
    }
    for msg in &msgs {
      print!("{}\r", msg);
      io::stdout().flush().unwrap();
      thread::sleep(time::Duration::from_millis(sleep_millis));
    }
    i += 1;
  }
  clearscreen::clear().expect("failed to clear screen");
}

pub fn play_text_input_animation(text: &str) {
  let mut i = 0;
  loop {
    if i == text.len() {
      break;
    }

    print!("{} \r", &text[0..i]);
    io::stdout().flush().unwrap();
    thread::sleep(time::Duration::from_millis(100));
    i += 1;
  }
  println!("");
}

pub fn play_text_input_animation_base_text_front(text: &str, base_text_front: &str) {
  let mut i = 0;
  loop {
    if i > text.len() {
      break;
    }

    print!("{}{}\r", &base_text_front, &text[0..i]);
    io::stdout().flush().unwrap();
    thread::sleep(time::Duration::from_millis(100));
    i += 1;
  }
  println!("");
}
