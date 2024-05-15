use std::ops::Deref;

use serenity::all::{CommandInteraction, Context, CreateCommand, Guild};
use songbird::{Event, TrackEvent};

use crate::{
    events::songbird::TrackEndNotifier,
    utils::response::{embed, response},
};

pub fn register() -> CreateCommand {
    CreateCommand::new("join").description("🎶 | Añadir a Nikita Buyanov a tu canal de voz")
}

pub async fn run(ctx: &Context, command: &CommandInteraction) {
    command
        .defer_ephemeral(&ctx.http)
        .await
        .expect("Diferir una respuesta de un comando no deberia fallar. Posible cambio en la API");

    let result = join(ctx, command).await;

    match result {
        Ok(success) => {
            response(ctx, command, embed(ctx).title(success)).await;
        }
        Err(error) => {
            response(ctx, command, embed(ctx).title(error)).await;
        }
    }
}

pub async fn join(ctx: &Context, command: &CommandInteraction) -> Result<String, String> {
    let (guild_id, channel_id) = {
        let guild: Guild = ctx
            .cache
            .guild(command.guild_id.unwrap())
            .unwrap()
            .deref()
            .clone();
        let channel_id = guild
            .voice_states
            .get(&command.user.id)
            .and_then(|voice_state| voice_state.channel_id);

        (command.guild_id.unwrap(), channel_id)
    };

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => return Err("❌ | No te encuentras en un canal de voz".to_string()),
    };

    let manager = songbird::get(&ctx)
        .await
        .expect("Por diseño no puede fallar el conseguir el manager");

    if let Ok(call) = manager.join(guild_id, connect_to).await {
        let mut handler = call.lock().await;

        handler.remove_all_global_events();
        handler.add_global_event(Event::Track(TrackEvent::End), TrackEndNotifier);

        Ok("✅ | Nikita se ha unido a tu canal de voz".to_string())
    } else {
        Err("❌ | Nikita no se pudo unir a tu canal de voz".to_string())
    }
}
