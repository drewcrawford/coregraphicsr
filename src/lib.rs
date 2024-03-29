extern crate core;

/**
Provides select Rust bindings for Apple [Core Graphics](https://developer.apple.com/documentation/coregraphics).  May be compared to [objrs_frameworks_core_graphics](https://crates.io/crates/objrs_frameworks_core_graphics)
and [core-graphics](https://crates.io/crates/core-graphics).

Part of the [objr expanded universe](https://github.com/drewcrawford/objr#objr-expanded-universe), distinctive features of coregraphicsr:

* Zero-cost abstractions.  Calling this library should perform identically to calling CoreGraphics from Swift/ObjC applications.
    * Most of the magic happens in [objr](https://github.com/drewcrawford/objr)
      which provide cutting-edge high-performance primitives which are used here extensively.
* Safe APIs.  Where possible APIs are designed with safe abstractions to provide familiar guarantees to Rust developers
* Low-level.  These bindings assume familiarity with bound APIs and are not documented separately.
* Free for noncommercial or "small commercial" use.

# Status
The following APIs are implemented
* CGPoint, CGFloat, CGSize, and CGRect
* CGColor (greyscale only currently)
* CGDirectDisplayID
* CGColorSpace
* CGImageAlphaInfo
* CGContext, CGBitmapContext
* CGDataProvider, CGDataProviderCreateDirect
* CGImage

*/
mod cgcolor;
mod cgdirectdisplay;
mod cgcolorspace;
mod cgimage;
mod cgbitmapcontext;
mod cgdataprovider;

use objr::bindings::*;

pub use cgcolor::*;
pub use cgdirectdisplay::CGDirectDisplayID;
pub use cgcolorspace::*;
pub use cgimage::*;
pub use cgbitmapcontext::*;
pub use cgdataprovider::*;

pub type CGFloat = f64;

#[repr(C)]
#[derive(Debug,Copy,Clone,PartialEq)]
pub struct CGPoint {
    pub x: CGFloat,
    pub y: CGFloat
}
impl CGPoint {
    pub const ZERO: CGPoint  = CGPoint { x: 0.0, y: 0.0 };
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
#[derive(Debug,Copy,Clone,PartialEq)]
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
    pub const ZERO: CGRect = CGRect {
        origin: CGPoint { x: 0.0, y: 0.0 },
        size: CGSize { width: 0.0, height: 0.0 }
    };
}

#[link(name="CoreGraphics", kind="framework")] extern "C" {}