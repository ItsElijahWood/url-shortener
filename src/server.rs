use std::{fs, io::{Read, Write}, net::{TcpListener, TcpStream}};

use rusqlite::{Connection, Result};

/// Initialises a new web server
pub fn server(server_port: &str) {
  let addr = format!("127.0.0.1{}", server_port);

  println!("   Started server on: {}", addr);
  let listener = TcpListener::bind(addr).unwrap();

  for stream in listener.incoming() {
    let mut stream = stream.unwrap();

    handle_stream(&mut stream);
  }
}

fn handle_stream(mut stream: &TcpStream) {
  let mut buffer = [0; 512];
  stream.read(&mut buffer).unwrap();

  let request = String::from_utf8_lossy(&buffer[..]);

  if request.starts_with("GET / HTTP/1.1") {
    let status = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("index.html").unwrap();
    let length = contents.len();

    let response = 
      format!("{status}\r\nContent-Length: {length}\r\n\r\n{contents}");

    return stream.write_all(response.as_bytes()).unwrap();
  }

  if let Some(path) = request.split_whitespace().nth(1) {
    if path.starts_with("/") && !path.contains("favicon") {
      let shortened_url = &path[1..6];
      match get_original_url(shortened_url) {
        Ok(redirect_url) => {
          let status = "HTTP/1.1 301 Moved Permanently";
          let body = "<html><body>Redirecting..</body></html>";
          let length = body.len();
          let response = format!("{status}\r\nLocation: {redirect_url}\r\nContent-Length: {length}\r\n\r\n{body}");

          return stream.write_all(response.as_bytes()).unwrap();
        }
        Err(e) => {
          println!("Error: {}", e);
          let status = "HTTP/1.1 200 OK";
          let contents = "404 Not Found - URL not found!";
          let length = contents.len();

          let response = 
            format!("{status}\r\nContent-Length: {length}\r\n\r\n{contents}");

            return stream.write_all(response.as_bytes()).unwrap();
        }
      }
    } 
  } 
    
  let status = "HTTP/1.1 200 OK";
  let contents = fs::read_to_string("index.html").unwrap();
  let length = contents.len();

  let response = 
    format!("{status}\r\nContent-Length: {length}\r\n\r\n{contents}");

  stream.write_all(response.as_bytes()).unwrap();
}

fn get_original_url(shortened_url: &str) -> Result<String, rusqlite::Error> {
  let connection = Connection::open("rustdb.db")?;
  let shortened_url = format!("http://localhost:3000/{}", shortened_url);

  println!("{}", shortened_url);

  let mut stmt = connection.prepare("select redirect_url from `url` where `url` = ?")?;
  let url_iter = 
    stmt.query_map([shortened_url], |row| row.get::<_, String>(0))?;

  for url in url_iter {
    println!("{:#?}", url);
    return Ok(url?);
  }

  Err(rusqlite::Error::QueryReturnedNoRows)
}
