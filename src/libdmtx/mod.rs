pub mod test_matrix;
mod libdmtx_sys;
#[link(name = "dmtx")] extern {}


use libdmtx_sys::*;

use std::{convert::TryInto, slice};

use crate::PackOrder;

impl From<PackOrder> for libdmtx_sys::DmtxPackOrder{
    fn from(pack_order: PackOrder) -> Self {
        match pack_order {
            PackOrder::PackCustom => DmtxPackOrder_DmtxPackCustom,
            PackOrder::Pack1bppK => DmtxPackOrder_DmtxPack1bppK,
            PackOrder::Pack8bppK => DmtxPackOrder_DmtxPack8bppK,
            PackOrder::Pack16bppRGB => DmtxPackOrder_DmtxPack16bppRGB,
            PackOrder::Pack16bppRGBX => DmtxPackOrder_DmtxPack16bppRGBX,
            PackOrder::Pack16bppXRGB => DmtxPackOrder_DmtxPack16bppXRGB,
            PackOrder::Pack16bppBGR => DmtxPackOrder_DmtxPack16bppBGR,
            PackOrder::Pack16bppBGRX => DmtxPackOrder_DmtxPack16bppBGRX,
            PackOrder::Pack16bppXBGR => DmtxPackOrder_DmtxPack16bppXBGR,
            PackOrder::Pack16bppYCbCr => DmtxPackOrder_DmtxPack16bppYCbCr,
            PackOrder::Pack24bppRGB => DmtxPackOrder_DmtxPack24bppRGB,
            PackOrder::Pack24bppBGR => DmtxPackOrder_DmtxPack24bppBGR,
            PackOrder::Pack24bppYCbCr => DmtxPackOrder_DmtxPack24bppYCbCr,
            PackOrder::Pack32bppRGBX => DmtxPackOrder_DmtxPack32bppRGBX,
            PackOrder::Pack32bppXRGB => DmtxPackOrder_DmtxPack32bppXRGB,
            PackOrder::Pack32bppBGRX => DmtxPackOrder_DmtxPack32bppBGRX,
            PackOrder::Pack32bppXBGR => DmtxPackOrder_DmtxPack32bppXBGR,
            PackOrder::Pack32bppCMYK => DmtxPackOrder_DmtxPack32bppCMYK,
        }
    }
}


/*
    Implementation of the demo example of libdmtx man page
    Returns an empty vec if no matrix is found
*/
pub fn dmtx_read(pxl: &[u8], width: u32, height: u32, pack: DmtxPackOrder) -> Vec<u8>{
    let mut output: Vec<u8> = Vec::new();

    //The raw FFI should be properly interfaced to rust code
    //https://gitlab.insa-rouen.fr/acamusat/lib2ddoc/-/issues/26
    //
    // For now, we bypass the borrow checker with a dirty hack
    let dirty = pxl as *const [u8] as *mut u8;

    unsafe {
        let mut image: *mut DmtxImage = dmtxImageCreate(
            dirty,
            width.try_into().expect("Can't fit width in int"),
            height.try_into().expect("Can't fit height in int"),
            pack.try_into().expect("Can't fit pack in int")
        );
    
        let mut decoded: *mut DmtxDecode = dmtxDecodeCreate(image, 1);

        let mut timeout = dmtxTimeAdd(dmtxTimeNow(), 1000);

        //timeout is a struct DmtxTime (sec, usec)
        let mut region: *mut DmtxRegion = dmtxRegionFindNext(decoded, &mut timeout);
    
        if !region.is_null() { 
            //fix = -1 ??
            let mut message: *mut DmtxMessage = dmtxDecodeMatrixRegion(decoded, region, -1);
            if !message.is_null() {
    
                let ptr = (*message).output;
                let len = (*message).outputIdx;

                output = Vec::with_capacity(len as usize);

                for i in slice::from_raw_parts(ptr, len as usize) {
                    output.push(*i);
                };
    
                dmtxMessageDestroy(&mut message);
            }
    
            dmtxRegionDestroy(&mut region);
        }
    
        dmtxDecodeDestroy(&mut decoded);
        dmtxImageDestroy(&mut image);
    }
    
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dmtx_example_decode() {
        assert_eq!(
            dmtx_read(&test_matrix::test_matrix(), 100, 100, DmtxPackOrder_DmtxPack24bppRGB),
            Vec::from("30Q324343430794<OQQ")
        );
    }
}