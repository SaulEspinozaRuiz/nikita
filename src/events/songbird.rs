use serenity::async_trait;
use songbird::{Event, EventContext, EventHandler};

pub struct TrackEndNotifier;

#[async_trait]
impl EventHandler for TrackEndNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx {
            for (state, handle) in *track_list {
                tracing::error!(
                    "Se encontro un error: {:?}\n en la cancion: {:?}",
                    state.playing,
                    handle.uuid()
                );
            }
        }

        None
    }
}
