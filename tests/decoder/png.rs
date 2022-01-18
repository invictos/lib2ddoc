use r2ddoc::PackOrder;
use png;
use super::{Image, ImageFileDecoder};

pub struct Decoder{}

impl ImageFileDecoder for Decoder {
    fn decode(path: &str) -> Image{
        let decoder = png::Decoder::new(Self::read_file(path));
        let mut reader = decoder.read_info().expect("Impossible: cannot happen in test");
        
        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf).expect("Impossible: cannot happen in test");
        let bytes = &buf[..info.buffer_size()];

        let format = match info.bit_depth {
            png::BitDepth::Eight => match info.color_type {
                png::ColorType::Rgb => PackOrder::Pack24bppRGB,
                png::ColorType::Grayscale => PackOrder::Pack8bppK,
                png::ColorType::Indexed => todo!(),
                png::ColorType::GrayscaleAlpha =>  panic!(),
                png::ColorType::Rgba => PackOrder::Pack32bppRGBX,   
            },
            png::BitDepth::One => todo!(),
            png::BitDepth::Two => todo!(),
            png::BitDepth::Four => todo!(),
            png::BitDepth::Sixteen => todo!(),     
        };

        Image {
            pixels: bytes.to_vec(),
            width: info.width as u16,
            height: info.height as u16,
            format,

        }
    }
}


mod tests{
    use super::*;

    fn image() -> Image {
        Decoder::decode("./tests/images/img_3.png")
    }
    
    #[test]	
    fn pixels() {
        let image = image();
        let slice = &image.pixels[0..8];
        assert_eq!(slice, &[255, 255, 255, 255, 255, 255, 255, 255]);
    }

    #[test]	
    fn width(){
        let image = image();
        assert_eq!(image.width, 246);
    }

    #[test]	
    fn height(){
        let image = image();
        assert_eq!(image.height, 397);
    }

    #[test]
    fn format(){
        let image = image();
        assert_eq!(image.format, PackOrder::Pack32bppRGBX);
    }

    #[test]
    #[ignore]
    fn export(){
        let i = image();
        println!("{}x{} ; fmt={}", i.width, i.height, i.format as i32);
        println!("{:#02x?}", i.pixels);
    }
}
