use font_kit::{
    family_name::FamilyName, handle::Handle, properties::Properties, source::SystemSource,
};
use rusttype::Font;
use std::{fs::File, io::Read};
use svg::Document;
use text_svg::Glyph;

fn main() {
    let handle = SystemSource::new()
        .select_best_match(&[FamilyName::SansSerif], &Properties::new())
        .unwrap();

    let font = match handle {
        Handle::Path { path, font_index } => {
            println!("path:{:?}", path);
            println!("font_index:{}", font_index);
            let p2 = "C:\\WINDOWS\\FONTS\\ARIAL.TTF".to_string();
            let mut file = File::open(p2).unwrap();
            let mut buf = Vec::new();
            file.read_to_end(&mut buf).unwrap();
            Font::try_from_vec_and_index(buf, font_index).unwrap()
        }
        Handle::Memory { bytes, font_index } => {
            Font::try_from_vec_and_index(bytes.to_vec(), font_index).unwrap()
        }
    };

    let glyph = Glyph::new(&font, 'F', 100.);
    let document = Document::new()
        .set("width", 10. + glyph.bounding_box.width())
        .set("height", 10. + glyph.bounding_box.height())
        .add(glyph.path(10., 10.));

    svg::save("image.svg", &document).unwrap();
}
