use sdl2::{Sdl, VideoSubsystem, AudioSubsystem, EventSubsystem, TimerSubsystem};

pub struct ContextManager {
    pub sdl_context: Sdl,
    pub video_subsystem: VideoSubsystem,
    pub audio_subsystem: AudioSubsystem,
    pub event_subsystem: EventSubsystem,
    pub timer_subsystem: TimerSubsystem,
}
impl ContextManager {
    pub fn new() -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let audio_subsystem = sdl_context.audio()?;
        let event_subsystem = sdl_context.event()?;
        let timer_subsystem = sdl_context.timer()?;
        Ok(Self {
            sdl_context,
            video_subsystem,
            audio_subsystem,
            event_subsystem,
            timer_subsystem,
        })
    }
}