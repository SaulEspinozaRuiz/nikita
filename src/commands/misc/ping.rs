use serenity::all::{CommandInteraction, Context, CreateCommand};

use crate::utils::response::{embed, response};

pub fn register() -> CreateCommand {
    CreateCommand::new("ping").description("🏓 | Responder con un Pong!")
}

pub async fn run(ctx: &Context, command: &CommandInteraction) {
    command
        .defer_ephemeral(&ctx.http)
        .await
        .expect("Diferir una respuesta de un comando no deberia fallar. Posible cambio en la API");

    response(ctx, command, embed(ctx).title("🏓 | Pong!")).await;
}
