use rusqlite::{Connection, Result};

pub fn add_url(url: &str, redirect_url: &str) -> Result<()> {
  let connection = Connection::open("rustdb.db")?;

  connection.execute("INSERT INTO url (url, redirect_url) VALUES (?, ?)", [url, redirect_url])?;

  Ok(())
}
