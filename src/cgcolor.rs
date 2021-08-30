use objr::bindings::*;
use crate::CGFloat;
objc_instance! {
    pub struct CGColorRef;
}
extern "C" {
    fn CGColorCreateGenericGray(grey: CGFloat, alpha: CGFloat) -> *const CGColorRef;
}

impl CGColorRef {
    pub fn grey(grey: CGFloat, alpha: CGFloat) -> StrongCell<Self> {
        unsafe {
            Self::assume_nonnil(CGColorCreateGenericGray(grey, alpha)).assume_retained()
        }
    }
}

