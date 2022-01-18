use decoder::ImageFileDecoder;
use r2ddoc::*;
mod decoder;

#[test]
fn test_image_to_document(){
    let data = decoder::png::Decoder::decode("./tests/images/specs-p141.PNG");

    let document = image_to_document(data.pixels.as_slice(), data.width.into(), data.height.into(), data.format).expect("Impossible: cannot happen in test");

    assert!(document.validity.valid);
}

//The following tests serve for json output, no code coverage implied
#[test]
fn test_image_to_json(){
    let data = decoder::png::Decoder::decode("./tests/images/img_3.png");

    let json = image_to_json(data.pixels.as_slice(), data.width.into(), data.height.into(), data.format);

    println!("{}", json);
    assert!(true);
}


#[test]
fn test_image_no_2ddoc(){
    let data = decoder::jpeg::Decoder::decode("./tests/images/img_1.jpg");

    let json = image_to_json(data.pixels.as_slice(), data.width.into(), data.height.into(), data.format);

    println!("{}", json);
    assert!(true);
}