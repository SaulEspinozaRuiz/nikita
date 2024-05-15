use serenity::all::{CommandInteraction, Context, CreateCommand};
use songbird::tracks::PlayMode;
use tracing::error;

use crate::utils::response::{embed, response};

pub fn register() -> CreateCommand {
    CreateCommand::new("pause").description("🎶 | Pausa la reproduccion actual de audio")
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
                    error!("Error al conseguir el estado del audio: {}", error);
                    return response(
                        ctx,
                        command,
                        embed(ctx).title("❌ | Error al pausar el audio"),
                    )
                    .await;
                }
            },
            None => {
                return response(
                    ctx,
                    command,
                    embed(ctx).title("❌ | No hay un audio al que pausar"),
                )
                .await;
            }
        };

        match track_state {
            PlayMode::Play => match current_track {
                Some(track) => match track.pause() {
                    Ok(_) => {
                        response(ctx, command, embed(ctx).title("✅ | Audio pausado")).await;
                    }
                    Err(error) => {
                        error!("Error al pausar el audio, error: {}", error);
                        response(
                            ctx,
                            command,
                            embed(ctx).title("❌ | Error al pausar el audio"),
                        )
                        .await;
                    }
                },
                None => {
                    response(
                        ctx,
                        command,
                        embed(ctx).title("❌ | No hay nada al que pausar"),
                    )
                    .await;
                }
            },
            _ => {
                response(ctx, command, embed(ctx).title("❌ | El audio esta pausado")).await;
            }
        }
    } else {
        response(
            ctx,
            command,
            embed(ctx).title(
                "❌ | Error al pausar el audio. Asegurate de que el bot este en tu canal de voz",
            ),
        )
        .await;
    }
}
