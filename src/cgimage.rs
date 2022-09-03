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