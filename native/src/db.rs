use mongodb::sync::{Client, Database};

pub struct WritaDB {
  pub client: Client,
  pub db: Option<Database>,
}

impl WritaDB {
  pub fn new(uri: &str) -> Self {
    WritaDB {
      client: WritaDB::create_mongodb_client(uri),
      db: None
    }
  }

  pub fn create_mongodb_client(uri: &str) -> Client {
    let result = Client::with_uri_str(uri);
    if let Ok(client) = result {
      client
    } else {
      panic!("connect mongodb failed");
    }
  }
}
