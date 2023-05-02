use rusttype::{Font, Point};
use svg::{node::element::Rectangle, Document};
use text_svg::Text;
fn main() {
    let font_data = include_bytes!("../AvenirNextCyr-Bold.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).expect("Error constructing Font");

    let x = 5.;
    let y = 10.;

    let text = Text::builder()
        .size(30.0)
        .start(Point { x, y })
        .build(&font, "&");

    let document = Document::new()
        .set("width", text.bounding_box.max.x + x)
        .set("height", text.bounding_box.max.y + y)
        .add(
            Rectangle::new()
                .set("fill", "#fff")
                .set("x", 0)
                .set("y", 0)
                .set("width", text.bounding_box.max.x + x)
                .set("height", text.bounding_box.max.y + y),
        )
        .add(text.path);

    svg::save("music.svg", &document).unwrap();
}
