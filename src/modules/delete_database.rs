use std::{fs, path::Path};

use rusqlite::Result;

#[allow(dead_code)]
/// Deletes the rustdb database 
pub fn delete_database() -> Result<()> {
  let rustdb_file = "rustdb.db";

  if Path::new(rustdb_file).exists() {
    if let Err(e) = fs::remove_file(rustdb_file) {
      eprintln!("   Error deleting database: {}", e);
    } else {
      println!("   Successfully deleted database, a new one will generate when you run the application again.");
    };
  } else {
    panic!("   Cannot find database at: {}", rustdb_file);
  }

  Ok(())
}