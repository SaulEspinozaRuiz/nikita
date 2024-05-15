use std::ops::Deref;

use serenity::all::{ChannelId, Context, CreateMessage, Guild, GuildId, Timestamp, User};
use tracing::error;

use crate::utils::response::embed;

pub async fn run(ctx: &Context, guild_id: GuildId, user: User) {
    let guild: Guild = ctx.cache.guild(guild_id).unwrap().deref().clone();

    let embed = embed(ctx)
        .title(format!("👋 | Adios de {}", guild.name))
        .description(format!("```{} Ha sido removido del servidor```", user.name))
        .thumbnail(user.face())
        .timestamp(Timestamp::now());

    let goodbay_channel = ChannelId::new(1125842067961172099);

    let message_builder = CreateMessage::new().embed(embed);

    if let Err(why) = goodbay_channel
        .send_message(&ctx.http, message_builder)
        .await
    {
        error!("Cannot send goodbay message, why: {}", why);
    }
}
