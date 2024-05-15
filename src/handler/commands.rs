use serenity::all::{Command, CommandInteraction, Context};

use crate::{
    commands::{misc, music},
    utils::response::{embed, response},
};

pub async fn register(ctx: &Context) {
    let mut commands = Vec::new();

    // Misc commands
    commands.push(misc::help::register());
    commands.push(misc::ping::register());

    // Music commands
    commands.push(music::clear::register());
    commands.push(music::join::register());
    commands.push(music::leave::register());
    commands.push(music::list::register());
    commands.push(music::r#loop::register());
    commands.push(music::pause::register());
    commands.push(music::play::register());
    commands.push(music::resume::register());
    commands.push(music::skip::register());

    for command in commands {
        Command::create_global_command(&ctx.http, command)
            .await
            .expect("Fallo al registrar un comando");
    }
}

pub async fn run(ctx: &Context, command: &CommandInteraction) {
    match command.data.name.as_str() {
        // Misc commands
        "help" => misc::help::run(&ctx, &command).await,
        "ping" => misc::ping::run(&ctx, &command).await,

        // Music commands
        "clear" => music::clear::run(&ctx, &command).await,
        "join" => music::join::run(&ctx, &command).await,
        "leave" => music::leave::run(&ctx, &command).await,
        "list" => music::list::run(&ctx, &command).await,
        "loop" => music::r#loop::run(&ctx, &command).await,
        "pause" => music::pause::run(&ctx, &command).await,
        "play" => music::play::run(&ctx, &command).await,
        "resume" => music::resume::run(&ctx, &command).await,
        "skip" => music::skip::run(&ctx, &command).await,

        _ => {
            command.defer_ephemeral(&ctx.http).await.expect(
                "Diferir una respuesta de un comando no deberia fallar. Posible cambio en la API",
            );

            response(ctx, command, embed(ctx).title("❌ | Comando desconocido")).await;
        }
    }
}
