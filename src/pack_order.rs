#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub enum PackOrder{
    PackCustom = 100,
    Pack1bppK = 200,
    Pack8bppK = 300,
    Pack16bppRGB = 400,
    Pack16bppRGBX,
    Pack16bppXRGB,
    Pack16bppBGR,
    Pack16bppBGRX,
    Pack16bppXBGR,
    Pack16bppYCbCr,
    Pack24bppRGB = 500,
    Pack24bppBGR,
    Pack24bppYCbCr,
    Pack32bppRGBX = 600,
    Pack32bppXRGB,
    Pack32bppBGRX,
    Pack32bppCMYK,
    Pack32bppXBGR,
}