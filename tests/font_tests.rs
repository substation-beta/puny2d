#[cfg(feature = "text")]
mod font_tests {
    // Imports
    use font_loader::system_fonts;
    use std::fmt::Write;
    use ttf_parser::{Face, OutlineBuilder, Rect};

    // Native font (post-installed on linux)
    const TEST_FONT: &str = "Arial";

    #[test]
    fn test_has_system_fonts() {
        let fonts = system_fonts::query_all();
        assert!(fonts.len() > 0, "Fonts:\n{:?}", fonts);
    }

    #[test]
    fn test_find_font() {
        let fonts = system_fonts::query_specific(
            &mut system_fonts::FontPropertyBuilder::new().family(TEST_FONT).build()
        );
        assert!(fonts.contains(&TEST_FONT.to_owned()), "Found fonts: {:?}", fonts);
    }

    #[test]
    fn test_font_data() {
        // Load font (regular)
        let (data_regular, _) = system_fonts::get(
            &system_fonts::FontPropertyBuilder::new().family(TEST_FONT).build()
        ).expect("Regular font not loadable!");
        // Load font (bold)
        let (data_bold, _) = system_fonts::get(
            &system_fonts::FontPropertyBuilder::new().family(TEST_FONT).bold().build()
        ).expect("Bold font not loadable!");
        // Compare both font styles
        println!("{0} regular length: {1}\n{0} bold length: {2}", TEST_FONT, data_regular.len(), data_bold.len());
        assert!(data_regular.len() > 0 && data_regular.len() != data_bold.len());
    }

    #[test]
    fn test_glyph_outline() {
        // Load font (regular)
        let (data, _) = system_fonts::get(
            &system_fonts::FontPropertyBuilder::new().family(TEST_FONT).build()
        ).expect("Regular font not loadable!");
        // Load first face from font
        let face = Face::from_slice(&data, 0).expect("Font should have at least one face.");
        // Get glyph index of 'i' in face
        let glyph_index_i = face.glyph_index('i').expect("Font doesn't contain character 'i'!?");
        // Glyph outline builder
        struct SVGBuilder {
            svg: String,
            ascend: f32
        }
        impl SVGBuilder {
            pub fn new(ascend: i16) -> Self {
                Self {
                    svg: String::new(),
                    ascend: ascend as f32
                }
            }
            pub fn svg(&self) -> &str {
                &self.svg
            }
        }
        impl ttf_parser::OutlineBuilder for SVGBuilder {
            fn move_to(&mut self, x: f32, y: f32) {
                write!(&mut self.svg, "M {} {} ", x, -y + self.ascend).expect("Couldn't write 'move'!");
            }
            fn line_to(&mut self, x: f32, y: f32) {
                write!(&mut self.svg, "L {} {} ", x, -y + self.ascend).expect("Couldn't write 'line'!");
            }
            fn quad_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
                write!(&mut self.svg, "Q {} {} {} {} ", x1, -y1 + self.ascend, x2, -y2 + self.ascend).expect("Couldn't write 'quadratic curve'!");
            }
            fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {
                write!(&mut self.svg, "C {} {} {} {} {} {} ", x1, -y1 + self.ascend, x2, -y2 + self.ascend, x3, -y3 + self.ascend).expect("Couldn't write 'cubic curve'!");
            }
            fn close(&mut self) {
                write!(&mut self.svg, "Z ").expect("Couldn't write 'close'!");
            }
        }
        // Get glyph outline as svg string
        let mut builder = SVGBuilder::new(face.ascender());
        let glyph_bounding = face.outline_glyph(glyph_index_i, &mut builder).expect("Glyph had no outline or parse error!");
        // Check glyph outline
        assert_eq!(
            builder.svg(),
            "M 136 595 L 136 388 L 316 388 L 316 595 L 136 595 Z M 136 1854 L 136 792 L 316 792 L 316 1854 L 136 1854 Z "
        );
        assert_eq!(
            glyph_bounding,
            Rect {
                x_min: 136,
                y_min: 0,
                x_max: 316,
                y_max: 1466
            }
        );
        assert!(face.ascender() >= glyph_bounding.y_max, "Glyph 'i' shouldn't be outside ascent space: {}", face.ascender());
    }
}