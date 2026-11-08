// Replaces only the line Horizon puts at the top of its F3/debug overlay.
// Rendering, benchmarking and all timing data remain untouched.
fn fps_display_spoofer_f3(
    game: &crate::game::Game<'_>,
    d: &mut horizon_client::debug_overlay::Displayer,
) {
    use crate::game::fps_display_spoofer as m;

    let limit = game.framerate_limit();
    let present = game.renderer.as_ref().map_or("", |renderer| match renderer.config.present_mode {
        wgpu::PresentMode::Immediate => " (immediate)",
        wgpu::PresentMode::Mailbox => " (mailbox)",
        wgpu::PresentMode::Fifo => " (fifo)",
        wgpu::PresentMode::FifoRelaxed => " (fifo relaxed)",
        _ => "",
    });
    let refresh = game.debug_refresh_rate().map_or("0".into(), |mhz| {
        horizon_client::debug_overlay::refresh_rate_label(f64::from(mhz) / 1000.0)
    });

    let shown_fps = if game.native_on(m::ID) {
        m::fps() as f32
    } else {
        game.fps
    };
    let generated = game
        .renderer
        .as_ref()
        .and_then(|renderer| renderer.frame_generation_share())
        .map_or(String::new(), |share| {
            format!(" ({} fps FSR 3.1)", (shown_fps * (1.0 + share)).round() as i32)
        });

    d.priority(format!(
        "{} fps{generated} T: {}{present} @{refresh}Hz",
        shown_fps.round() as i32,
        if limit == 260 { "inf".into() } else { limit.to_string() },
    ));
}
