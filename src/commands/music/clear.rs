use serenity::all::{CommandInteraction, Context, CreateCommand};

use crate::utils::response::{embed, response};

pub fn register() -> CreateCommand {
    CreateCommand::new("clear").description("🎶 | Limpia y para la cola de reproduccion actual")
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

        let queue_length = handler.queue().len();

        if queue_length == 0 {
            return response(
                ctx,
                command,
                embed(ctx).title("❌ | No hay nada que limpiar"),
            )
            .await;
        }

        handler.queue().stop();

        response(
            ctx,
            command,
            embed(ctx).title("✅ | Lista de reproduccion limpiado"),
        )
        .await;
    } else {
        response(ctx, command, embed(ctx).title("❌ | Error al limpiar la lista de reproduccion. Asegurate de que el bot este en tu canal de voz")).await;
    }
}
