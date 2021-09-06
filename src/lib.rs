mod cgcolor;
mod cgdirectdisplay;

use objr::bindings::*;

pub use cgcolor::CGColorRef;
pub use cgdirectdisplay::CGDirectDisplayID;

pub type CGFloat = f64;

#[repr(C)]
#[derive(Debug)]
pub struct CGPoint {
    pub x: CGFloat,
    pub y: CGFloat
}
unsafe impl Primitive for CGPoint {}
unsafe impl Arguable for CGPoint {}
#[repr(C)]
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy,Clone)]
pub struct CGSize {
    pub width: CGFloat,
    pub height: CGFloat
}
unsafe impl Primitive for CGSize {}
unsafe impl Arguable for CGSize {}

#[repr(C)]
#[derive(Debug)]
pub struct CGRect {
    pub origin: CGPoint,
    pub size: CGSize
}
unsafe impl Primitive for CGRect {}
unsafe impl Arguable for CGRect{}
impl CGRect {
    pub fn make(x: CGFloat, y: CGFloat, width: CGFloat, height: CGFloat) -> CGRect {
        CGRect {
            origin: CGPoint { x, y },
            size: CGSize { width, height}
        }
    }
}
