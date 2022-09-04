use std::os::raw::c_void;
use crate::CGColorSpace;
use core_foundationr::{StrongCell,CFType};
#[repr(transparent)]
pub struct CGContext(c_void);
extern "C" {
    fn CGBitmapContextCreate(data: *mut c_void, width: usize, height: usize, bitsPerComponent: usize, bytesPerRow: usize, space: *const CGColorSpace, bitmapInfo: u32) -> *const CGContext;
}
impl CFType for CGContext {

}

#[allow(non_snake_case)]
impl CGContext {
    /**
    See docs for CGBitmapContextCreate

    # Safety
    * bounds check
    * exceptions, particularly around invalid parameter combinations.  See also environment variable `CGBITMAP_CONTEXT_LOG_ERRORS=1`.
    */
    pub unsafe fn createBitmap(data: *mut c_void, width: usize, height: usize, bitsPerComponent: usize, bytesPerRow: usize, space: &CGColorSpace, bitmapInfo: u32) -> Option<StrongCell<Self>> {
        //in https://developer.apple.com/library/archive/documentation/GraphicsImaging/Conceptual/drawingwithquartz2d/dq_context/dq_context.html#//apple_ref/doc/uid/TP30001066-CH203-TPXREF101, listing 2-5,
        //they check this for null
        let context = CGBitmapContextCreate(data, width, height, bitsPerComponent, bytesPerRow, space, bitmapInfo);
        if context.is_null() {
            None
        } else {
            Some(StrongCell::assuming_retained_nonnull(context))
        }
    }
}

#[cfg(test)] mod tests {
    use std::os::raw::c_void;
    use crate::{CGColorSpace, CGContext, CGImageAlphaInfo, Name};

    #[test] fn smoke() {
        let color_space = CGColorSpace::with_name(&Name::generic_rgb());
        let mut vec: Vec<u8> = vec![0; 4]; //8-bit color, 4 channels, 1x1, skip last
        let _ = unsafe{CGContext::createBitmap(vec.as_mut_ptr() as *mut c_void, 1, 1, 8, 4, &color_space, CGImageAlphaInfo::NONE_SKIP_LAST.0)}.unwrap();
    }
}