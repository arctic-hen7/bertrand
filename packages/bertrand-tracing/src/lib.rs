use std::fmt;
use tracing::{field::Field, Subscriber};
use tracing_subscriber::Layer;

/// A [`Layer`] that will pick up events sent with the [`bstate!`] macro and send them to a
/// Bertrand instance. This allows connecting the logs of your application directly to Bertrand,
/// making bakcne demonstrations almost effortless.
///
/// Note that, under the hood, this makes synchronous requests, so it's recommended to have
/// Bertrand running on the same machine, or at least on the same local network.
pub struct BertrandLayer {
    url: String,
}
impl BertrandLayer {
    /// Constructs a new [`BertrandLayer`] with the given address on which a Bertrand server is
    /// running, like `localhost:8080`.
    pub fn new(addr: impl Into<String>) -> Self {
        Self {
            url: format!("http://{}/api/send", addr.into()),
        }
    }
}

impl<S> Layer<S> for BertrandLayer
where
    S: Subscriber,
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        if event.metadata().target() == "bertrand_state" {
            let mut state = None;
            event.record(&mut |field: &Field, value: &dyn fmt::Debug| {
                if field.name() == "state" {
                    // Irritatingly, we can only debug-format this, even though we know it's a
                    // string
                    let raw_str = format!("{:?}", value);
                    state = Some(raw_str.replace("\"", ""));
                }
            });
            if let Some(state) = state {
                // Send the state to Bertrand
                let _ = ureq::post(&self.url).send_string(&state);
            }
        }
    }
}

/// A macro for sending a state change to Bertrand.
#[macro_export]
macro_rules! bstate {
    ($state:expr) => {
        ::tracing::event!(
            target: "bertrand_state",
            ::tracing::Level::INFO,
            state = $state,
        );
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::subscriber::set_global_default;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::Registry;

    #[test]
    fn should_work() {
        let layer = BertrandLayer::new("localhost:8080");
        let subscriber = Registry::default().with(layer);
        set_global_default(subscriber).unwrap();

        bstate!("stateB");
        bstate!("stateC");
    }
}
