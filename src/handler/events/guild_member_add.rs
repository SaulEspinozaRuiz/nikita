use std::ops::Deref;

use serenity::all::{ChannelId, Context, CreateMessage, Guild, Member, RoleId, Timestamp};
use tracing::error;

use crate::utils::response::embed;

pub async fn run(ctx: &Context, new_member: Member) {
    let guild: Guild = ctx
        .cache
        .guild(new_member.guild_id)
        .unwrap()
        .deref()
        .clone();

    let member_role = RoleId::new(1042301131377872907);

    new_member.add_role(&ctx.http, member_role).await.unwrap();

    let embed = embed(ctx)
        .title(format!("👋 | Bienvenido a {}", guild.name))
        .description(format!(
            "```{} Se ha unido al servidor```",
            new_member.user.name
        ))
        .thumbnail(new_member.face())
        .timestamp(Timestamp::now());

    let welcome_channel = ChannelId::new(1125842038622003251);

    let message_builder = CreateMessage::new().embed(embed);

    if let Err(why) = welcome_channel
        .send_message(&ctx.http, message_builder)
        .await
    {
        error!("No se pudo enviar el mensaje, por que: {}", why);
    }
}
