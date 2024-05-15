use serenity::all::{CommandInteraction, Context, CreateCommand};
use songbird::tracks::PlayMode;
use tracing::error;

use crate::utils::response::{embed, response};

pub fn register() -> CreateCommand {
    CreateCommand::new("resume").description("🎶 | Reanuda la reproduccion actual de audio")
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

        let current_track = handler.queue().current();

        let track_state = match &current_track {
            Some(track) => match track.get_info().await {
                Ok(state) => state.playing,
                Err(error) => {
                    error!("Error al obtener el estado del audio: {}", error);
                    return response(
                        ctx,
                        command,
                        embed(ctx).title("❌ | Error al reanudar el audio"),
                    )
                    .await;
                }
            },
            None => {
                return response(
                    ctx,
                    command,
                    embed(ctx).title("❌ | No hay un audio al que reanudar"),
                )
                .await;
            }
        };

        match track_state {
            PlayMode::Pause => match current_track {
                Some(track) => match track.play() {
                    Ok(_) => {
                        response(ctx, command, embed(ctx).title("✅ | Reanudando el audio")).await;
                    }
                    Err(error) => {
                        error!("Error al reanudar el audio: {}", error);
                        response(
                            ctx,
                            command,
                            embed(ctx).title("❌ | Error al reanudar el audio"),
                        )
                        .await;
                    }
                },
                None => {
                    response(
                        ctx,
                        command,
                        embed(ctx).title("❌ | No hay audio al que reanudar"),
                    )
                    .await;
                }
            },
            _ => {
                response(
                    ctx,
                    command,
                    embed(ctx).title("❌ | No hay un audio en reproduccion"),
                )
                .await;
            }
        }
    } else {
        response(
            ctx,
            command,
            embed(ctx)
                .title("❌ | Error al reanudar. Asegurate de que el bot este en tu canal de voz"),
        )
        .await;
    }
}
