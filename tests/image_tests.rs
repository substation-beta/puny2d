mod image_tests {
    // Imports
    use png::{Decoder, ColorType, OutputInfo, BitDepth};
    use std::fs::File;

    #[test]
    fn test_image_load() {
        // Load test image from disk
        let img = File::open(
            env!("CARGO_MANIFEST_DIR").to_owned() + "/tests/ayaya.png"
        ).expect("Test image should be there!");
        // Decode image as png
        let decoder = Decoder::new(img);
        let (out_info, mut reader) = decoder.read_info().expect("Couldn't read png file!");
        // Check image properties
        assert_eq!(
            out_info,
            OutputInfo {
                width: 128,
                height: 128,
                color_type: ColorType::RGBA,
                bit_depth: BitDepth::Eight,
                line_size: 128*4*1    // 128 pixels * 4 channels * 1 byte
            }
        );
        // Read one image frame
        let mut buffer = vec![0; reader.output_buffer_size()];
        reader.next_frame(&mut buffer).expect("Error on reading image data!");
        // Check first image pixel
        assert_eq!(&buffer[..4], &[255, 255, 188, 255]);
    }
}