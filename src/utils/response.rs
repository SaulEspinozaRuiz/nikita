use rand::Rng;
use serenity::all::{
    Colour, CommandInteraction, Context, CreateEmbed, CreateEmbedFooter,
    CreateInteractionResponseFollowup,
};

pub async fn response(ctx: &Context, command: &CommandInteraction, embed: CreateEmbed) {
    command
        .create_followup(
            &ctx.http,
            CreateInteractionResponseFollowup::new()
                .embed(embed)
                .ephemeral(true),
        )
        .await
        .expect("Enviar una respuesta a un comando no deberia fallar. Posible cambio en la API");
}

pub fn embed(ctx: &Context) -> CreateEmbed {
    let mut rng = rand::thread_rng();

    CreateEmbed::new()
        .color(Colour::from_rgb(
            rng.gen_range(0..255),
            rng.gen_range(0..255),
            rng.gen_range(0..255),
        ))
        .footer(
            CreateEmbedFooter::new(format!(
                "Traido a ti por {}",
                &ctx.cache.current_user().name
            ))
            .icon_url(&ctx.cache.current_user().avatar_url().unwrap()),
        )
}
