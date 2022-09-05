use std::os::raw::c_void;
use crate::{CGColorRenderingIntent, CGColorSpace, CGDataProvider, CGFloat};
use core_foundationr::{StrongCell,CFType,StrongMutCell};
use objr::bindings::Arguable;

#[derive(PartialEq,Debug)]
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
    fn CGImageGetDataProvider(image: *const CGImage) -> *const CGDataProvider;
    fn CGImageGetHeight(image: *const CGImage) -> usize;
    fn CGImageGetWidth(image: *const CGImage) -> usize;
    fn CGImageGetBitsPerComponent(image: *const CGImage) -> usize;
    fn CGImageGetBitsPerPixel(image: *const CGImage) -> usize;
    fn CGImageGetBytesPerRow(image: *const CGImage) -> usize;
    fn CGImageGetColorSpace(image: *const CGImage) -> *const CGColorSpace;
    fn CGImageGetAlphaInfo(image: *const CGImage) -> u32;

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
    pub fn getDataProvider(&self) -> Option<StrongCell<CGDataProvider>> {
        let raw = unsafe{CGImageGetDataProvider(self)};
        if raw.is_null() {
            None
        } else {
            Some(unsafe{StrongCell::retain_assuming_nonnull(raw)})
        }
    }
    pub fn getWidth(&self) -> usize {
        unsafe{CGImageGetWidth(self)}
    }
    pub fn getHeight(&self) -> usize {
        unsafe{CGImageGetHeight(self)}
    }
    pub fn getBitsPerComponent(&self) -> usize {
        unsafe{CGImageGetBitsPerComponent(self)}
    }
    pub fn getBitsPerPixel(&self) -> usize {
        unsafe{CGImageGetBitsPerPixel(self)}
    }
    pub fn getBytesPerRow(&self) -> usize {
        unsafe{CGImageGetBytesPerRow(self)}
    }
    pub fn getColorSpace(&self) -> Option<StrongCell<CGColorSpace>> {
        let raw = unsafe{CGImageGetColorSpace(self)};
        if raw.is_null() {
            None
        } else {
            Some(unsafe{StrongCell::retain_assuming_nonnull(raw)})
        }
    }
    pub fn getAlphaInfo(&self) -> CGImageAlphaInfo {
        CGImageAlphaInfo(unsafe{CGImageGetAlphaInfo(self)})
    }
}

#[cfg(test)] mod tests {
    use crate::{CGColorRenderingIntent, CGColorSpace, CGImage, CGImageAlphaInfo, Name};
    use crate::SliceProvider;

    #[test] fn smoke() {
        let mut bytes: [u8; 4] = [0; 4];
        let mut provider = SliceProvider::new(&mut bytes).unwrap();
        let image = unsafe{CGImage::create(1, 1, 8, 32, 4, &CGColorSpace::with_name(Name::generic_rgb()), CGImageAlphaInfo::NONE_SKIP_LAST,provider.as_provider_mut(), None, false, CGColorRenderingIntent::DEFAULT)}.unwrap();
        let provider = image.getDataProvider().unwrap();
        assert_eq!(image.getWidth(), 1);
        assert_eq!(image.getHeight(), 1);
        assert_eq!(image.getBitsPerComponent(), 8);
        assert_eq!(image.getBitsPerPixel(), 32);
        assert_eq!(image.getBytesPerRow(), 4);
        assert!(image.getColorSpace().is_some());
        assert_eq!(image.getAlphaInfo(), CGImageAlphaInfo::NONE_SKIP_LAST);
        let _ = provider.copyData().unwrap();

    }
}