use serenity::{
    all::{ActivityData, Context, EventHandler},
    async_trait,
    model::prelude::*,
};
use tracing::info;

use crate::handler;

pub struct SerenityHandler;

#[async_trait]
impl EventHandler for SerenityHandler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        handler::commands::register(&ctx).await;

        ctx.set_activity(Some(ActivityData::custom("Dame 250 dolares.")));

        info!("¡{} esta conectado!", ready.user.name);
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction.clone() {
            handler::commands::run(&ctx, &command).await;
        }
    }

    async fn voice_state_update(&self, ctx: Context, old: Option<VoiceState>, new: VoiceState) {
        handler::events::join_to_create::run(&ctx, &old, &new).await;
    }

    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        handler::events::guild_member_add::run(&ctx, new_member).await;
    }

    async fn guild_member_removal(
        &self,
        ctx: Context,
        guild_id: GuildId,
        user: User,
        _member_data_if_available: Option<Member>,
    ) {
        handler::events::guild_member_remove::run(&ctx, guild_id, user).await;
    }
}
