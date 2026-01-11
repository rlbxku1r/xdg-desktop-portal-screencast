mod display;
mod display_config;
mod screencast;
mod screencast_session;
mod screencast_stream;

pub use display::DisplayProxy as Display;
pub use display_config::DisplayConfigProxy as DisplayConfig;
pub use screencast::ScreenCastProxy as ScreenCast;
pub use screencast_session::SessionProxy as ScreenCastSession;
pub use screencast_stream::StreamProxy as ScreenCastStream;
