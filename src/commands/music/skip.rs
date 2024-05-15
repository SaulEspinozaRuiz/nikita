use serenity::all::{CommandInteraction, Context, CreateCommand};
use tracing::error;

use crate::utils::response::{embed, response};

pub fn register() -> CreateCommand {
    CreateCommand::new("skip").description("🎶 | Salta la reproduccion actual de audio")
}

pub async fn run(ctx: &Context, command: &CommandInteraction) {
    command
        .defer_ephemeral(&ctx.http)
        .await
        .expect("Diferir una respuesta de un comando no deberia fallar. Posible cambio en la API");

    let manager = songbird::get(&ctx)
        .await
        .expect("Por diseño no puede fallar el conseguir el manager");

    if let Some(call) = manager.get(command.guild_id.unwrap()) {
        let handler = call.lock().await;

        let skip_result = match handler.queue().current() {
            Some(track) => track.stop(),
            None => {
                return response(
                    ctx,
                    command,
                    embed(ctx).title("❌ | No hay un audio en reproduccion para saltar"),
                )
                .await;
            }
        };

        match skip_result {
            Ok(_) => {
                response(ctx, command, embed(ctx).title("✅ | Audio saltado")).await;
            }
            Err(error) => {
                error!("Error al saltar la cancion: {}", error);
                response(
                    ctx,
                    command,
                    embed(ctx).title("❌ | Error al saltar el audio"),
                )
                .await;
            }
        }
    } else {
        response(
            ctx,
            command,
            embed(ctx).title(
                "❌ | Error al saltar el audio. Asegurate de que el bot este en tu canal de voz",
            ),
        )
        .await;
    }
}
