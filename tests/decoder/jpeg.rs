use r2ddoc::PackOrder;
use jpeg_decoder;
use super::{Image, ImageFileDecoder};


pub struct Decoder{}

impl ImageFileDecoder for Decoder {
    fn decode(path: &str) -> Image{
        let reader = Self::read_file(path);
        let mut decoder = jpeg_decoder::Decoder::new(reader);
        
        let pixels = decoder.decode().expect("Failed to decode the image");
        let metadata = decoder.info().expect("Impossible: cannot happen in test");
        
        let format = match metadata.pixel_format {
            jpeg_decoder::PixelFormat::L8 => PackOrder::Pack8bppK,
            jpeg_decoder::PixelFormat::RGB24 => PackOrder::Pack24bppRGB,
            jpeg_decoder::PixelFormat::CMYK32 => PackOrder::Pack32bppCMYK,
        };

        Image {
            pixels,
            width: metadata.width,
            height: metadata.height,
            format
        }
    }
}


mod tests{
    use super::*;

    fn image() -> Image {
        Decoder::decode("./tests/images/img_1.jpg")
    }
    
    #[test]	
    fn pixels() {
        let image = image();
        let slice = &image.pixels[0..8];
        assert_eq!(slice, &[17, 83, 159, 12, 78, 154, 9, 75]);
    }

    #[test]	
    fn width(){
        let image = image();
        assert_eq!(image.width, 235);
    }

    #[test]	
    fn height(){
        let image = image();
        assert_eq!(image.height, 156);
    }

    #[test]
    fn format(){
        let image = image();
        assert_eq!(image.format, PackOrder::Pack24bppRGB);
    }
}
