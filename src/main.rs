mod commands;
mod events;
mod handler;
mod utils;

use events::serenity::SerenityHandler;
use shuttle_runtime::SecretStore;
use utils::http_key::HttpKey;

use reqwest::Client as HttpClient;
use serenity::{all::GatewayIntents, Client};
use songbird::SerenityInit;
use tracing::{error, info};

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    info!("Inicializando...");

    let token = secret_store
        .get("DISCORD_BOT_TOKEN")
        .expect("No se encontro el token del bot");
    let intents = GatewayIntents::all();

    let mut client = Client::builder(token, intents)
        .event_handler(SerenityHandler)
        .register_songbird()
        .type_map_insert::<HttpKey>(HttpClient::new())
        .await
        .expect("Ocurrio un error al momento de crear el cliente");

    client
        .start()
        .await
        .map_err(|error| error!("Ocurrio un error en el cliente: error {}", error))
        .expect("No se pudo iniciar el cliente");

    {
        tokio::signal::ctrl_c()
            .await
            .expect("No puede fallar el `tokio::signal::ctrl_c()`");
        info!("Se recibio un Ctrl-C, apagando...");
    }

    Ok(client.into())
}
