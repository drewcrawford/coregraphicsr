use objr::bindings::*;
use crate::{CGColorSpace, CGFloat, Name};
objc_instance! {
    pub struct CGColor;
}
extern "C" {
    fn CGColorCreateGenericGray(grey: CGFloat, alpha: CGFloat) -> *const CGColor;
    fn CGColorCreate(colorSpace: *const CGColorSpace, components: *const CGFloat) -> *const CGColor;
}

#[allow(non_snake_case)]
impl CGColor {
    pub fn grey(grey: CGFloat, alpha: CGFloat) -> StrongCell<Self> {
        unsafe {
            Self::assume_nonnil(CGColorCreateGenericGray(grey, alpha)).assume_retained()
        }
    }
    pub fn create(colorSpace: &CGColorSpace, components: &[CGFloat]) -> Option<StrongCell<Self>> {
        unsafe {
            Self::nullable(CGColorCreate(colorSpace, components.as_ptr())).assume_retained()
        }
    }
}

#[test] fn test_cgcolor() {
    let color_space = CGColorSpace::with_name(Name::srgb());
    let _ = CGColor::create(&color_space, &[0.0, 0.0, 0.0, 1.0]).unwrap();
}

