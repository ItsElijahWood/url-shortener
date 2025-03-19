use std::io;

use rand::{self, Rng};

mod modules;
mod server;

const SERVER_PORT: &str = ":3000";
fn main() {
   modules::export_modules(); 

   let listener = io::stdin();
   let mut url_redirect = String::new();
   let url = "http://localhost:3000/";

   println!("{}", ascii_pattern());
   println!("   Commands:
         start server
         delete database
         list urls
         <insert url>
   ");

   println!("   Input:");

   listener.read_line(&mut url_redirect).expect("Error reading line of variable `url` in `main.rs`.");

   if url_redirect.trim() == "start server" {
      return server::server(&SERVER_PORT);
   } else if url_redirect.trim() == "delete database" {
      return modules::delete_database();
   } else if url_redirect.trim() == "list urls" {
      return modules::list_url();
   }

   let url_redirect = url_redirect.trim().to_string();

   let url_redirect = if !url_redirect.starts_with("http://") && !url.starts_with("https://") {
      format!("https://{}", url_redirect)
   } else {
      url_redirect
   };

   if !url_redirect.contains(".") {
      panic!("Not a valid domain.");
   }

   let rng_url = generate_random_url(5);
   let url = format!("{url}{}", rng_url);

   println!("Redirect URL: {}", url);
   println!("Directed to: {}", url_redirect);
   modules::add_url_db(&url, &url_redirect);
}

fn generate_random_url(length: usize) -> String {
   const ALPHA_DOWN: &str = "abcdefghijklmnopqrstuvwxyz";
   const ALPHA_UP: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
   const NUMERIC: &str = "1234567890";

   let mut rng = rand::rng();
   let char = [ALPHA_DOWN, ALPHA_UP, NUMERIC].concat();

   let random_url: String = (0..length)
      .map(|_| {
         let random_index: usize = rng.random_range(0..char.len());
         char.chars().nth(random_index).unwrap()
      })
      .collect();

   random_url
}

fn ascii_pattern() -> String {
   return "
   ▗▖ ▗▖▗▄▄▖ ▗▖        ▗▄▄▖▗▖ ▗▖ ▗▄▖ ▗▄▄▖▗▄▄▄▖▗▄▄▄▖▗▖  ▗▖▗▄▄▄▖▗▄▄▖ 
   ▐▌ ▐▌▐▌ ▐▌▐▌       ▐▌   ▐▌ ▐▌▐▌ ▐▌▐▌ ▐▌ █  ▐▌   ▐▛▚▖▐▌▐▌   ▐▌ ▐▌
   ▐▌ ▐▌▐▛▀▚▖▐▌        ▝▀▚▖▐▛▀▜▌▐▌ ▐▌▐▛▀▚▖ █  ▐▛▀▀▘▐▌ ▝▜▌▐▛▀▀▘▐▛▀▚▖
   ▝▚▄▞▘▐▌ ▐▌▐▙▄▄▖    ▗▄▄▞▘▐▌ ▐▌▝▚▄▞▘▐▌ ▐▌ █  ▐▙▄▄▖▐▌  ▐▌▐▙▄▄▖▐▌ ▐▌  
   ".to_string();
}
