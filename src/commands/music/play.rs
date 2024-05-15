use regex::Regex;
use serenity::all::{
    CommandDataOptionValue, CommandInteraction, CommandOptionType, Context, CreateCommand,
    CreateCommandOption,
};
use songbird::input::YoutubeDl;
use tokio::process::Command;
use tracing::error;

use crate::utils::{
    http_key::HttpKey,
    response::{embed, response},
};

use super::join::join;

pub fn register() -> CreateCommand {
    CreateCommand::new("play")
        .description("🎶 | Reproduce un audio de Youtube, con un link o titulo")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "musica",
                "🎶 | Musica a reproducir",
            )
            .required(true),
        )
}

pub async fn run(ctx: &Context, command: &CommandInteraction) {
    command
        .defer_ephemeral(&ctx.http)
        .await
        .expect("Diferir una respuesta de un comando no deberia fallar. Posible cambio en la API");

    let option = command.data.options.first();

    let option_value = match option {
        Some(data) => data.value.clone(),
        None => {
            return response(
                ctx,
                command,
                embed(ctx).title("❌ | Prove un valor a la opcion"),
            )
            .await;
        }
    };

    let string_option = match option_value {
        CommandDataOptionValue::String(option) => option.clone(),
        _ => {
            return response(
                ctx,
                command,
                embed(ctx).title("❌ | Favor de introducir un titulo o link valido"),
            )
            .await;
        }
    };

    join(ctx, command).await.unwrap();

    if string_option.contains("https://") {
        play_url(&ctx, &command, string_option).await;
    } else {
        play_title(&ctx, &command, string_option).await;
    }
}

async fn play_url(ctx: &Context, command: &CommandInteraction, url: String) {
    if !url.contains("youtube.com") && !url.contains("youtu.be") {
        return response(
            ctx,
            command,
            embed(ctx).title("❌ | Favor de introducir una URL de youtube valido"),
        )
        .await;
    }

    let manager = songbird::get(&ctx)
        .await
        .expect("Por diseño no puede fallar conseguir el manager");

    if let Some(call) = manager.get(command.guild_id.unwrap()) {
        let mut handler = call.lock().await;

        if url.contains("/playlist") {
            let playlist_result = Command::new("yt-dlp")
                .args(["-j", "--flat-playlist", &url])
                .output()
                .await;

            let playlist = match playlist_result {
                Ok(playlist) => {
                    let playlist_json_result = String::from_utf8(playlist.stdout);

                    match playlist_json_result {
                        Ok(playlist) => playlist,
                        Err(error) => {
                            error!("Error al conseguir la playlist json: error {}", error);

                            return response(
                                ctx,
                                command,
                                embed(ctx)
                                    .title("❌ | Error al conseguir la lista de reproduccion"),
                            )
                            .await;
                        }
                    }
                }
                Err(error) => {
                    error!("Error al obtener los datos de la playlist: {}", error);

                    return response(
                        ctx,
                        command,
                        embed(ctx).title("❌ | Error obteniendo la informacion de la playlist"),
                    )
                    .await;
                }
            };

            let regex =
                Regex::new(r#""url": "(https://www.youtube.com/watch\?v=[A-Za-z0-9]{11})""#)
                    .expect("Error building Youtube URL regex");

            let urls: Vec<String> = regex
                .captures_iter(&playlist)
                .map(|capture| capture[1].to_string())
                .collect();

            response(
                ctx,
                command,
                embed(ctx).title("⌛ | Agregando audios a la cola puede tomar algo de tiempo"),
            )
            .await;

            let mut num_queued_songs = 0;

            let http_client = {
                let data = ctx.data.read().await;

                data.get::<HttpKey>()
                    .cloned()
                    .expect("El tipo de dato existe por diseño")
            };

            for url in urls {
                let source = YoutubeDl::new(http_client.clone(), url);

                dbg!("Audio - {}", &source);

                handler.enqueue_input(source.into()).await;
                num_queued_songs += 1;
            }

            return response(
                ctx,
                command,
                embed(ctx)
                    .title("✅ | Reproduciendo playlist")
                    .description(format!(
                        "```{} de canciones en la cola de reproduccion```",
                        num_queued_songs
                    )),
            )
            .await;
        } else {
            let http_client = {
                let data = ctx.data.read().await;

                data.get::<HttpKey>()
                    .cloned()
                    .expect("El tipo de dato existe por diseño")
            };

            let source = YoutubeDl::new(http_client.clone(), url);

            handler.enqueue_input(source.into()).await;

            response(ctx, command, embed(ctx).title("✅ | Reproduciendo link")).await;
        }
    }
}

async fn play_title(ctx: &Context, command: &CommandInteraction, title: String) {
    let http_client = {
        let data = ctx.data.read().await;

        data.get::<HttpKey>()
            .cloned()
            .expect("El tipo de dato existe por diseño")
    };

    let manager = songbird::get(&ctx)
        .await
        .expect("Por diseño no puede fallar el obtener el manager");

    if let Some(call) = manager.get(command.guild_id.unwrap()) {
        let mut handler = call.lock().await;

        let source = YoutubeDl::new_search(http_client.clone(), title);

        handler.enqueue_input(source.into()).await;

        response(ctx, command, embed(ctx).title("✅ | Reproduciendo titulo")).await;
    } else {
        response(
            ctx,
            command,
            embed(ctx).title(
                "❌ | Error al reproducir, Asegurate que el bot este en el mismo canal de voz",
            ),
        )
        .await;
    }
}
