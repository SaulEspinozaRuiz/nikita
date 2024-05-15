use serenity::all::{CommandInteraction, Context, CreateCommand};

use crate::utils::response::{embed, response};

pub fn register() -> CreateCommand {
    CreateCommand::new("help").description("🏓 | Muestra los comandos y como usarlos")
}

pub async fn run(ctx: &Context, command: &CommandInteraction) {
    command
        .defer_ephemeral(&ctx.http)
        .await
        .expect("Diferir una respuesta de un comando no deberia fallar. Posible cambio en la API");

    let description = String::from(
        "```
    # 🗿 Comandos de Nikita Buyanov 🗿 #
    \n Comandos miscelaneos.
    \n /ping - Responde con un Pong!
    \n /help - Muestra los comandos y como usarlos.
    \n Comandos de musica.
    \n /clear - Para la ejecucion actual y limpia la cola de reproducción.
    \n /join - Añade a Nikita Buyanov a tu canal de voz.
    \n /leave - Remueve a Nikita Buyanov de tu canal de voz.
    \n /list - Muestra la cola de reproducción actual.
    \n /loop - Activa/Desactiva el bucle de reproducción.
    \n /pause - Pausa la reproducción actual.
    \n /play - Reproduce un audio de Youtube, usando un titulo o link.
    \n /resume - Reanudar la reproducción actual.
    \n /skip - Salta la reproducción de audio actual.
        ```",
    );

    response(
        ctx,
        command,
        embed(ctx).title("🏓 | Ayuda").description(description),
    )
    .await;
}
