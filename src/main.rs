use std::io;
use std::thread;
use std::time;
use std::io::Write;
use std::sync::mpsc;
use std::sync::mpsc::Sender;

const ONE_MINUTE_MS : u64 = 60 * 1000;
const TIMER : i32 = 25;

enum Status {
    Timer,
    Pause
}

// https://doc.rust-lang.org/rust-by-example/std_misc/channels.html

fn main() {
    let mut status = Status::Timer;
    let (tx, rx) = mpsc::channel::<i32>();

    listen_input(tx);
    let timer_thread = start_timer();

    // memory leak, from that loop i guess
    loop {
        match rx.recv() {
            Ok(_) => {
                match status {
                    Status::Timer => {
                        print!("\rTimer stopped, starting pause");
                        io::stdout().flush().expect("flush failed");
                        status = Status::Pause;
                    },
                    Status::Pause => {
                        print!("\rPause stopped, starting timer");
                        io::stdout().flush().expect("flush failed");
                        status = Status::Timer;
                    }
                }
            }
            Err(_) => {
                println!("Terminating.");
            }
        }
    }
}

fn start_timer() -> thread::JoinHandle<()> {
  let thread = thread::spawn(move || {
    for minutes_left in 0 .. TIMER {
        update_time(TIMER - minutes_left);
        sleep(ONE_MINUTE_MS);
    }
  });

  return thread
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

fn listen_input(tx: Sender<i32>) {
  thread::spawn(move || loop {
      let mut buffer = String::new();
      let stdin = io::stdin();
      stdin.read_line(&mut buffer);
      tx.send(1);
      Ok::<(), ()>(());
  });
}
