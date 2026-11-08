fn fps_display_spoofer_frame(_game: &mut crate::game::Game<'_>, elapsed: std::time::Duration) {
    crate::game::fps_display_spoofer::advance(elapsed);
}
