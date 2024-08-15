use relm4::abstractions::DrawContext;

use std::f64::consts::PI;

use crate::generator::Star;

pub fn draw(app: &mut crate::App) {
    let cx: DrawContext = app.draw_handler.get_context();
    let width = app.draw_handler.width();
    let height = app.draw_handler.height();

    cx.set_source_rgb(0.0, 0.0, 0.0);
    cx.paint().unwrap();
    // cx.set_source_rgb(1.0, 1.0, 1.0);
    // cx.arc(width as f64 / 2.0, height as f64 / 2.0, height.min(width) as f64 / 2.0, 0.0, 2.0 * PI);
    // cx.fill().unwrap();
    // cx.set_font_size(40.0);
    // let family = app.font_dialog_button.font_desc().unwrap().family().unwrap();
    // cx.select_font_face(family.as_str(), cairo::FontSlant::Normal, cairo::FontWeight::Normal);
    // cx.move_to(width as f64 / 2.0, 50.0);
    // cx.show_text("HELLO WORLD").unwrap();

    for star in &app.stars[0..app.starcount as usize] {
        draw_star(&cx, width, height, star);
    }

}

fn draw_star(cx: &DrawContext, width: i32, height: i32, star: &Star) {
    cx.set_source_rgb(1.0, 0.5, 0.5);
    cx.arc(star.cords.0 * width as f64, star.cords.1 * height as f64, 4.0, 0.0, 2.0 * PI);
    cx.fill().unwrap();
}
