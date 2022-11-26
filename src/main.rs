use std::io;
use std::thread;
use std::time;
use std::io::Write;

fn main() {
    const ONE_MINUTE_MS : u64 = 60 * 1000;
    const TIMER : i32 = 25;

    listen_input();

    for minutes_left in 0 .. TIMER {
        update_time(TIMER - minutes_left);
        sleep(ONE_MINUTE_MS);
    }
}

fn sleep(ms: u64) {
  let thread = thread::spawn(move || {
      thread::sleep(time::Duration::from_millis(ms));
  });
  thread.join().expect("thread join failed");
}

fn update_time(time_left: i32) {
    print!("\r{} minutes left", time_left);
    io::stdout().flush().expect("flush failed");
}

fn listen_input() {
  thread::spawn(move || {
      let mut buffer = String::new();
      let stdin = io::stdin();
      stdin.read_line(&mut buffer);
      print!("\rTimer stopped, starting pause");
      io::stdout().flush().expect("flush failed");
      // stop timer thread (for loop should be in a thread that we pass to this function as
      // parameter)
      // start pause loop thread
      Ok::<(), ()>(());
  });
}
