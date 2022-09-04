use std::ffi::c_void;
use core_foundationr::CFString;

#[repr(transparent)]
pub struct CGColorSpace(c_void);
/*
Design note about wrapping types.

We generally use core_foundationr::StrongCell for CF types (although objr::bindings::StrongCell is also available for types that are bridged).

Another option is CGColorSpaceRetain and CGColorSpaceRelease.  The headers suggest that this is "equivalent to CFRelease" (that is, core_foundationr::StrongCell), "except it doesn't crash (as CFRelease
   does) if `space' is NULL."

   It's not immediately clear to me why we would be passing NULL to CGColorSpaceRelease, in a language with proper optionals in the typesystem.  So I think I am going to follow StrongCell convention rather
   than introducing yet a third convention.
 */
impl core_foundationr::CFType for CGColorSpace {

}
extern "C" {
    static  kCGColorSpaceGenericGray: &'static CFString;
    static kCGColorSpaceGenericRGB: &'static CFString;
    static kCGColorSpaceGenericCMYK: &'static CFString;
    static kCGColorSpaceDisplayP3: &'static CFString;
    static kCGColorSpaceGenericRGBLinear: &'static CFString;
    static kCGColorSpaceAdobeRGB1998: &'static CFString;
    static kCGColorSpaceSRGB: &'static CFString;
    static kCGColorSpaceGenericGrayGamma2_2: &'static CFString;
    static kCGColorSpaceGenericXYZ: &'static CFString;
    static kCGColorSpaceGenericLab: &'static CFString;
    static kCGColorSpaceACESCGLinear: &'static CFString;
    static kCGColorSpaceITUR_709: &'static CFString;
    static kCGColorSpaceITUR_709_PQ: &'static CFString;
    static kCGColorSpaceITUR_2020: &'static CFString;
    static kCGColorSpaceITUR_2020_sRGBGamma: &'static CFString;
    static kCGColorSpaceROMMRGB: &'static CFString;
    static kCGColorSpaceDCIP3: &'static CFString;
    static kCGColorSpaceLinearITUR_2020: &'static CFString;
    static kCGColorSpaceExtendedITUR_2020: &'static CFString;
    static kCGColorSpaceExtendedLinearITUR_2020: &'static CFString;
    static kCGColorSpaceLinearDisplayP3: &'static CFString;
    static kCGColorSpaceExtendedDisplayP3: &'static CFString;
    static kCGColorSpaceExtendedLinearDisplayP3: &'static CFString;
    static kCGColorSpaceITUR_2100_PQ: &'static CFString;
    static kCGColorSpaceITUR_2100_HLG: &'static CFString;
    static kCGColorSpaceDisplayP3_PQ: &'static CFString;
    static kCGColorSpaceDisplayP3_HLG: &'static CFString;
    static kCGColorSpaceITUR_2020_PQ: &'static CFString;
    static kCGColorSpaceITUR_2020_HLG: &'static CFString;
    static kCGColorSpaceDisplayP3_PQ_EOTF: &'static CFString;
    static kCGColorSpaceITUR_2020_PQ_EOTF: &'static CFString;
    static kCGColorSpaceExtendedSRGB: &'static CFString;
    static kCGColorSpaceLinearSRGB: &'static CFString;
    static kCGColorSpaceExtendedLinearSRGB: &'static CFString;
    static kCGColorSpaceExtendedGray: &'static CFString;
    static kCGColorSpaceExtendedLinearGray: &'static CFString;
}
pub struct Name;
impl Name {
    pub fn generic_gray() -> &'static CFString {
        unsafe { kCGColorSpaceGenericGray }
    }
    pub fn generic_rgb() -> &'static CFString {
        unsafe { kCGColorSpaceGenericRGB }
    }
    pub fn generic_cmyk() -> &'static CFString {
        unsafe { kCGColorSpaceGenericCMYK }
    }
    pub fn display_p3() -> &'static CFString {
        unsafe { kCGColorSpaceDisplayP3 }
    }
    pub fn generic_rgb_linear() -> &'static CFString {
        unsafe { kCGColorSpaceGenericRGBLinear }
    }
    pub fn adobe_rgb_1998() -> &'static CFString {
        unsafe { kCGColorSpaceAdobeRGB1998 }
    }
    pub fn srgb() -> &'static CFString {
        unsafe { kCGColorSpaceSRGB }
    }
    pub fn generic_gray_gamma_2_2() -> &'static CFString {
        unsafe { kCGColorSpaceGenericGrayGamma2_2 }
    }
    pub fn generic_xyz() -> &'static CFString {
        unsafe { kCGColorSpaceGenericXYZ }
    }
    pub fn generic_lab() -> &'static CFString {
        unsafe { kCGColorSpaceGenericLab }
    }
    pub fn acescg_linear() -> &'static CFString {
        unsafe { kCGColorSpaceACESCGLinear }
    }
    pub fn itu_r_709() -> &'static CFString {
        unsafe { kCGColorSpaceITUR_709 }
    }
    pub fn itu_r_709_pq() -> &'static CFString {
        unsafe { kCGColorSpaceITUR_709_PQ }
    }
    pub fn itu_r_2020() -> &'static CFString {
        unsafe { kCGColorSpaceITUR_2020 }
    }
    pub fn itu_r_2020_srgb_gamma() -> &'static CFString {
        unsafe { kCGColorSpaceITUR_2020_sRGBGamma }
    }
    pub fn rommrgb() -> &'static CFString {
        unsafe { kCGColorSpaceROMMRGB }
    }
    pub fn dcip3() -> &'static CFString {
        unsafe { kCGColorSpaceDCIP3 }
    }
    pub fn linear_itu_r_2020() -> &'static CFString {
        unsafe { kCGColorSpaceLinearITUR_2020 }
    }
    pub fn extended_itu_r_2020() -> &'static CFString {
        unsafe { kCGColorSpaceExtendedITUR_2020 }
    }
    pub fn extended_linear_itu_r_2020() -> &'static CFString {
        unsafe { kCGColorSpaceExtendedLinearITUR_2020 }
    }
    pub fn linear_display_p3() -> &'static CFString {
        unsafe { kCGColorSpaceLinearDisplayP3 }
    }
    pub fn extended_display_p3() -> &'static CFString {
        unsafe { kCGColorSpaceExtendedDisplayP3 }
    }
    pub fn extended_linear_display_p3() -> &'static CFString {
        unsafe { kCGColorSpaceExtendedLinearDisplayP3 }
    }
    pub fn itu_r_2100_pq() -> &'static CFString {
        unsafe { kCGColorSpaceITUR_2100_PQ }
    }
    pub fn itu_r_2100_hlg() -> &'static CFString {
        unsafe { kCGColorSpaceITUR_2100_HLG }
    }
    pub fn display_p3_pq() -> &'static CFString {
        unsafe { kCGColorSpaceDisplayP3_PQ }
    }
    pub fn display_p3_hlg() -> &'static CFString {
        unsafe { kCGColorSpaceDisplayP3_HLG }
    }
    pub fn itu_r_2020_pq() -> &'static CFString {
        unsafe { kCGColorSpaceITUR_2020_PQ }
    }
    pub fn itu_r_2020_hlg() -> &'static CFString {
        unsafe { kCGColorSpaceITUR_2020_HLG }
    }
    pub fn display_p3_pq_eotf() -> &'static CFString {
        unsafe { kCGColorSpaceDisplayP3_PQ_EOTF }
    }
    pub fn itu_r_2020_pq_eotf() -> &'static CFString {
        unsafe { kCGColorSpaceITUR_2020_PQ_EOTF }
    }
    pub fn extended_srgb() -> &'static CFString {
        unsafe { kCGColorSpaceExtendedSRGB }
    }
    pub fn linear_srgb() -> &'static CFString {
        unsafe { kCGColorSpaceLinearSRGB }
    }
    pub fn extended_linear_srgb() -> &'static CFString {
        unsafe { kCGColorSpaceExtendedLinearSRGB }
    }
    pub fn extended_gray() -> &'static CFString {
        unsafe { kCGColorSpaceExtendedGray }
    }
    pub fn extended_linear_gray() -> &'static CFString {
        unsafe { kCGColorSpaceExtendedLinearGray }
    }
}

#[repr(transparent)]
pub struct CGColorRenderingIntent(pub i32);

impl CGColorRenderingIntent {
    pub const DEFAULT: CGColorRenderingIntent = CGColorRenderingIntent(0);
    pub const ABSOLUTE_COLORIMETRIC: CGColorRenderingIntent = CGColorRenderingIntent(1);
    pub const RELATIVE_COLORIMETRIC: CGColorRenderingIntent = CGColorRenderingIntent(2);
    pub const PERCEPTUAL: CGColorRenderingIntent = CGColorRenderingIntent(3);
    pub const SATURATION: CGColorRenderingIntent = CGColorRenderingIntent(4);
}
extern "C" {
    fn CGColorSpaceCreateWithName(name: &CFString) -> *mut CGColorSpace;
}

impl CGColorSpace {
    ///CGColorSpaceCreateWithName
    pub fn with_name(name: &CFString) -> core_foundationr::StrongCell<Self> {
        unsafe {
            let color_space = CGColorSpaceCreateWithName(name);
            core_foundationr::StrongCell::assuming_retained_nonnull(color_space)
        }
    }
}

#[test] fn create() {
    let _ = CGColorSpace::with_name(&Name::generic_rgb());
}