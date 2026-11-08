fn fps_display_spoofer_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use crate::game::fps_display_spoofer as m;
    if !game.native_on(m::ID) {
        return;
    }
    use horizon_client::extras::Corner;
    let corner = [Corner::TopLeft, Corner::TopRight, Corner::BottomLeft, Corner::BottomRight]
        [m::corner(|| game.native_setting(m::ID, "corner"))];
    if !game.native_corner_shown(corner) {
        return;
    }
    let text = format!("{} fps", m::fps());
    let width = game.native_text_width(&text) + 4.0;
    let (x, y) = corners.place(corner, width, 12.0);
    game.native_fill(batch, [x, y, width, 12.0], m::BACKGROUND);
    game.native_text(batch, &text, [x + 2.0, y + 2.0], m::INK, true);
}
