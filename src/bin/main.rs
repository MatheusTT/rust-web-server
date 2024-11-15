use std::{
  io::{prelude::*, BufReader},
  net::{TcpListener, TcpStream},
  thread,
  time::Duration,
};
use web_server::ThreadPool;

fn main() {
  let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
  let pool = ThreadPool::new(4);

  for stream in listener.incoming().take(2) {
    let stream = stream.unwrap();

    pool.execute(|| {
      handle_connection(stream);
    });
  }

  println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
  let buf_reader = BufReader::new(&mut stream);
  // let _ = stream.read(&mut buf_reader);
  let request_line = buf_reader.lines().next().unwrap().unwrap();

  let (status_line, filename) = match &request_line[..] {
    "GET / HTTP/1.1" => ("200 OK", "Hello World!"),
    "GET /sleep HTTP/1.1" => {
      thread::sleep(Duration::from_secs(5));
      ("200 OK", "Sleep")
    }
    _ => ("404 NOT FOUND", "404: Not Found"),
  };

  let contents = filename;
  let lenght = contents.len();

  let response = format!("HTTP/1.1 {status_line}\r\nContent-Length: {lenght}\r\n\r\n{contents}",);

  stream.write_all(response.as_bytes()).unwrap();
}
