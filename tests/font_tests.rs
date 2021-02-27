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
        // Get glyph outline as svg string
        struct SVGBuilder(String);
        impl ttf_parser::OutlineBuilder for SVGBuilder {
            fn move_to(&mut self, x: f32, y: f32) {
                write!(&mut self.0, "M {} {} ", x, y).expect("Couldn't write 'move'!");
            }
            fn line_to(&mut self, x: f32, y: f32) {
                write!(&mut self.0, "L {} {} ", x, y).expect("Couldn't write 'line'!");
            }
            fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
                write!(&mut self.0, "Q {} {} {} {} ", x1, y1, x, y).expect("Couldn't write 'quadratic curve'!");
            }
            fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
                write!(&mut self.0, "C {} {} {} {} {} {} ", x1, y1, x2, y2, x, y).expect("Couldn't write 'cubic curve'!");
            }
            fn close(&mut self) {
                write!(&mut self.0, "Z ").expect("Couldn't write 'close'!");
            }
        }
        let mut builder = SVGBuilder(String::new());
        let glyph_bounding = face.outline_glyph(glyph_index_i, &mut builder).expect("Glyph had no outline or parse error!");
        // Check glyph outline
        assert_eq!(
            builder.0,
            "M 136 1259 L 136 1466 L 316 1466 L 316 1259 L 136 1259 Z M 136 0 L 136 1062 L 316 1062 L 316 0 L 136 0 Z "
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
        assert!(face.ascender() >= glyph_bounding.y_max, "Glyph 'i' shouldn't be outside ascent space!");
    }
}