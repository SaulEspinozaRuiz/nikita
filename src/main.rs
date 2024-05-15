mod commands;
mod events;
mod handler;
mod utils;

use events::serenity::SerenityHandler;
use utils::http_key::HttpKey;

use reqwest::Client as HttpClient;
use serenity::{all::GatewayIntents, Client};
use songbird::SerenityInit;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    info!("Inicializando...");

    let token = std::env::var("DISCORD_BOT_TOKEN")
        .expect("No se encontro el token del bot en las variables de entorno");
    let intents = GatewayIntents::all();

    let mut client = Client::builder(token, intents)
        .event_handler(SerenityHandler)
        .register_songbird()
        .type_map_insert::<HttpKey>(HttpClient::new())
        .await
        .expect("Ocurrio un error al momento de crear el cliente");

    tokio::spawn(async move {
        client
            .start()
            .await
            .map_err(|error| error!("Ocurrio un error en el cliente: error {}", error))
            .expect("No se pudo iniciar el cliente");
    });

    tokio::signal::ctrl_c()
        .await
        .expect("No puede fallar el `tokio::signal::ctrl_c()`");
    info!("Se recibio un Ctrl-C, apagando...");
}
