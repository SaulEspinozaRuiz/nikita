use std::ops::Deref;

use serenity::all::{ChannelType, Context, CreateChannel, Guild, VoiceState};

pub async fn run(ctx: &Context, old: &Option<VoiceState>, new: &VoiceState) {
    if new.channel_id.is_none() {
        return leave(ctx, old.as_ref().unwrap()).await;
    }

    if let Some(voice_state) = old {
        leave(ctx, voice_state).await;
    }

    join(ctx, new).await;
}

async fn join(ctx: &Context, state: &VoiceState) {
    let guild: Guild = ctx
        .cache
        .guild(state.guild_id.unwrap())
        .unwrap()
        .deref()
        .clone();
    let channels = guild.channels(&ctx.http).await.unwrap();
    let channel = channels.get(&state.channel_id.unwrap()).unwrap();
    let user = guild.member(&ctx.http, state.user_id).await.unwrap();

    if channel.name == "『🔘』..." {
        let new_channel = guild
            .create_channel(
                &ctx.http,
                CreateChannel::new(format!("『🔊』| Canal de {}", user.display_name()))
                    .category(channel.parent_id.unwrap())
                    .kind(ChannelType::Voice),
            )
            .await
            .unwrap();

        guild
            .move_member(&ctx.http, state.user_id, new_channel.id)
            .await
            .unwrap();
    }
}

async fn leave(ctx: &Context, state: &VoiceState) {
    let guild: Guild = ctx
        .cache
        .guild(state.guild_id.unwrap())
        .unwrap()
        .deref()
        .clone();
    let channels = guild.channels(&ctx.http).await.unwrap();
    let channel = channels.get(&state.channel_id.unwrap()).unwrap();

    let member_count = match channel.members(&ctx.cache) {
        Ok(v) => v.len(),
        _ => return,
    };

    if member_count != 0 || channel.name() == "『🔘』..." {
        return;
    }

    channel.delete(&ctx.http).await.unwrap();
}
