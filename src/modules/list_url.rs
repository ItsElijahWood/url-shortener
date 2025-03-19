use rusqlite::{Result, Connection};

pub fn list_url() -> Result<()> {
    let connection = Connection::open("rustdb.db")?;

    let mut stmt = connection.prepare("SELECT * FROM url")?;

    let url_iter = stmt.query_map([], |row| {
        let url: String = row.get(0)?;
        let redirect_url: String = row.get(1)?;

        Ok((url, redirect_url))
    })?;

    for url in url_iter {
      match url {
        Ok((url, redirect_url)) => {
          println!("URL: {} REDIRECT_URL: {}", url, redirect_url)
        },
        Err(e) => {
          println!("Error listing url's from database: {}", e);
        }
      }
    }

    Ok(())
}
