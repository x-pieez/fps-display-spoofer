fn fps_display_spoofer_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::fps_display_spoofer as m;
    if mod_id == m::ID && button == 0 {
        let corner = m::cycle_corner();
        game.native_set_setting(m::ID, "corner", m::CORNERS[corner].0);
    }
}
