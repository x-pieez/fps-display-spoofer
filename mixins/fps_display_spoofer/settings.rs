fn fps_display_spoofer_settings(
    game: &mut crate::game::Game<'_>,
    mod_id: &str,
    buttons: &mut Vec<String>,
) {
    use crate::game::fps_display_spoofer as m;
    if mod_id == m::ID {
        let corner = m::corner(|| game.native_setting(m::ID, "corner"));
        buttons.push(format!("Corner: {}", m::CORNERS[corner].1));
    }
}
