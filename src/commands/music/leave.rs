use serenity::all::{CommandInteraction, Context, CreateCommand};

use crate::utils::response::{embed, response};

pub fn register() -> CreateCommand {
    CreateCommand::new("leave").description("🎶 | Remueve a Nikita Buyanov de tu canal de voz")
}

pub async fn run(ctx: &Context, command: &CommandInteraction) {
    command
        .defer_ephemeral(&ctx.http)
        .await
        .expect("Diferir una respuesta de un comando no deberia fallar. Posible cambio en la API");

    let manager = songbird::get(&ctx)
        .await
        .expect("Por diseño no puede fallar el conseguir el manager");

    if let Ok(_) = manager.leave(command.guild_id.unwrap()).await {
        response(
            ctx,
            command,
            embed(ctx).title("✅ | Nikita Buyanov ha salido de tu canal de voz"),
        )
        .await;
    } else {
        response(ctx, command, embed(ctx).title("❌ | Error al dejar el canal de voz. Asegurate de que el bot este en tu canal de voz")).await;
    }
}
