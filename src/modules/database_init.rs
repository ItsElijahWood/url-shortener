use rusqlite::{Connection, Result};

/// Initializes the database and its table
pub fn database_init() -> Result<()> {
  let connection = Connection::open("rustdb.db")?;  

  connection.execute(
    "create table if not exists url (
      url text not null primary key,
      redirect_url text not null
    )", (),
  )?;

  Ok(())
}
