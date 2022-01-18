use std::{convert::TryInto, ffi::CString, os::raw::c_char, slice};

use crate::{PackOrder, data_to_json, image_to_json};

#[no_mangle]
pub extern "C" fn lib2ddoc_data_to_json(data: *const u8, size: usize) -> *const c_char{

    let data= to_slide(data, size);

    let json = data_to_json(data);

    let c_string = CString::new(json).expect("Can't convert to c string, null in json ?");

    c_string.into_raw()
}

//image_to_json(pxls: &[u8], width: u32, height: u32, format: PackOrder) -> String {

#[no_mangle]
pub extern "C" fn lib2ddoc_image_to_json(pxls: *const u8, width: usize, height: usize, format: PackOrder) -> *const c_char{

    //We recover the number of bytes per pixel from the format (to later allocate the correct slide in Rust from C FFI)
    //To implement 1bppK, verify that width*height is multiple of 8
    let bytes_per_pixel = match format {
        PackOrder::PackCustom       => panic!("Must add bpp informations for custom pack"),
        PackOrder::Pack1bppK        => panic!("1bpp format not implemented (not multiple of 8)"),
        PackOrder::Pack8bppK        => 8,
        PackOrder::Pack16bppRGB   |
        PackOrder::Pack16bppRGBX  |
        PackOrder::Pack16bppXRGB  |
        PackOrder::Pack16bppBGR   |
        PackOrder::Pack16bppBGRX  |
        PackOrder::Pack16bppXBGR  |
        PackOrder::Pack16bppYCbCr   => 16,
        PackOrder::Pack24bppRGB   |
        PackOrder::Pack24bppBGR   |
        PackOrder::Pack24bppYCbCr   => 24,
        PackOrder::Pack32bppRGBX  |
        PackOrder::Pack32bppXRGB  |
        PackOrder::Pack32bppBGRX  |
        PackOrder::Pack32bppCMYK  |
        PackOrder::Pack32bppXBGR    => 32
    } / 8;

    let pixels= to_slide(pxls, width*height*bytes_per_pixel);

    let json = image_to_json(pixels, width.try_into().expect("width too big"), height.try_into().expect("height too big"), format);

    let c_string = CString::new(json).expect("Can't convert to c string, null in json ?");

    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn lib2ddoc_free_json(data: *mut c_char){
    unsafe { CString::from_raw(data) };
}


fn to_slide(ptr: *const u8, size: usize) -> &'static [u8] {
    unsafe{ slice::from_raw_parts(ptr, size)}
}
