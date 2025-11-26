use mongodb::Client;

pub struct DatabaseManager {
    client: Client,
    db_name: String,
}
