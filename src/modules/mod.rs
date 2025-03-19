pub mod database_init;
pub mod delete_database;
pub mod add_url;
pub mod list_url;

pub fn export_modules() {
  if let Err(e) = database_init::database_init() {
    eprintln!("Error initializing the database: {}", e);
  };
}

pub fn add_url_db(url: &str, redirect_url: &str) {
  if let Err(e) = add_url::add_url(url, redirect_url) {
    eprintln!("Error adding url into the database: {}", e);
  } else {
    println!("Successfully added url into the database.");
  };
}

pub fn delete_database() {
  if let Err(e) = delete_database::delete_database() {
    eprintln!("{}", e);
  } else {
    println!("");
  };
}

pub fn list_url() {
  if let Err(e) = list_url::list_url() {
    eprintln!("Error fetching url's from database: {}", e);
  } 
}
