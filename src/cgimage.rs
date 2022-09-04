use std::os::raw::c_void;
use crate::{CGColorRenderingIntent, CGColorSpace, CGDataProvider, CGFloat};
use core_foundationr::{StrongCell,CFType,StrongMutCell};
use objr::bindings::Arguable;

pub struct CGImageAlphaInfo(pub u32);
impl CGImageAlphaInfo {
    pub const NONE: CGImageAlphaInfo = CGImageAlphaInfo(0);
    pub const PREMULTIPLIED_LAST: CGImageAlphaInfo = CGImageAlphaInfo(1);
    pub const PREMULTIPLIED_FIRST: CGImageAlphaInfo = CGImageAlphaInfo(2);
    pub const LAST: CGImageAlphaInfo = CGImageAlphaInfo(3);
    pub const FIRST: CGImageAlphaInfo = CGImageAlphaInfo(4);
    pub const NONE_SKIP_LAST: CGImageAlphaInfo = CGImageAlphaInfo(5);
    pub const NONE_SKIP_FIRST: CGImageAlphaInfo = CGImageAlphaInfo(6);
    pub const ONLY: CGImageAlphaInfo = CGImageAlphaInfo(7);
}
#[repr(transparent)]
#[derive(Debug)]
pub struct CGImage(c_void);
unsafe impl Arguable for CGImage{}


extern "C" {
    fn CGImageCreate(width: usize, height: usize, bitsPerComponent: usize, bitsPerPixel: usize, bytesPerRow: usize, space: *const CGColorSpace, bitmapInfo: u32, provider: *const CGDataProvider, decode: *const CGFloat, shouldInterpolate: bool, intent: CGColorRenderingIntent) -> *const CGImage;
}
impl CFType for CGImage {}
#[allow(non_snake_case)]
impl CGImage {
    /**
    CGImageCreate.

    # Safety
    * Exceptions
    * The image might be bound to the lifetime of the provided data.  Currently this is not statically checked.
    */
    pub unsafe fn create(width: usize, height: usize, bitsPerComponent: usize, bitsPerPixel: usize, bytesPerRow: usize, space: &CGColorSpace, bitmapInfo: CGImageAlphaInfo, provider: &mut CGDataProvider, decode: Option<&[CGFloat]>, shouldInterpolate: bool, intent: CGColorRenderingIntent) -> Option<StrongMutCell<CGImage>> {
        let decode = decode.map(|x| x.as_ptr()).unwrap_or(std::ptr::null());
        let image = CGImageCreate(width, height, bitsPerComponent, bitsPerPixel, bytesPerRow, space, bitmapInfo.0, provider, decode, shouldInterpolate, intent);
        if image.is_null() {
            None
        } else {
            Some(StrongCell::assuming_retained_nonnull(image).assuming_mut())
        }
    }
}

#[cfg(test)] mod tests {
    use crate::{CGColorRenderingIntent, CGColorSpace, CGImage, CGImageAlphaInfo, Name};
    use crate::SliceProvider;

    #[test] fn smoke() {
        let mut bytes: [u8; 4] = [0; 4];
        let mut provider = SliceProvider::new(&mut bytes).unwrap();
        let _ = unsafe{CGImage::create(1, 1, 8, 32, 4, &CGColorSpace::with_name(Name::generic_rgb()), CGImageAlphaInfo::NONE,provider.as_provider_mut(), None, false, CGColorRenderingIntent::DEFAULT)}.unwrap();
    }
}