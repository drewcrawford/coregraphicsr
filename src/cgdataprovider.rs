use std::marker::PhantomData;
use std::os::raw::{c_uint, c_void};
use core_foundationr::{StrongCell,StrongMutCell,CFType};

#[allow(non_snake_case)] pub type CGDataProviderGetBytePointerCallback = extern "C" fn(nullable_info: *mut c_void) -> *mut c_void; //nullable return
#[allow(non_snake_case)] pub type CGDataProviderReleaseBytePointerCallback = extern "C" fn(nullable_info: *mut c_void, pointer: *const c_void);
#[allow(non_snake_case)] pub type CGDataProviderGetBytesAtPositionCallback = extern "C" fn(nullable_info: *mut c_void, buffer: *mut c_void, position: usize, count: usize) -> usize;
#[allow(non_snake_case)] pub type CGDataProviderReleaseInfoCallback = extern "C" fn(nullable_info: *mut c_void);

#[allow(non_snake_case)]
#[repr(C)]
pub struct CGDataProviderDirectCallbacks {
    pub version: c_uint,
    pub getBytePointer: Option<CGDataProviderGetBytePointerCallback>,
    pub releaseBytePointer: Option<CGDataProviderReleaseBytePointerCallback>,
    pub getBytesAtPosition: Option<CGDataProviderGetBytesAtPositionCallback>,
    pub releaseInfo: Option<CGDataProviderReleaseInfoCallback>,
}
#[repr(transparent)]
pub struct CGDataProvider(c_void);
extern "C" {
    fn CGDataProviderCreateDirect(nullable_info: *mut c_void, size: usize, callbacks: *const CGDataProviderDirectCallbacks) -> *const CGDataProvider;
    fn CGDataProviderCopyData(provider: *const CGDataProvider) -> *const core_foundationr::CFData;
}
impl CFType for CGDataProvider {}

extern "C" fn raw_get_pointer(info: *mut c_void) -> *mut c_void {
    info
}

/**
A custom [CGDataProvider] that wraps a slice of bytes.
*/
#[repr(transparent)]
pub struct SliceProvider<'s>(CGDataProvider, PhantomData<&'s mut ()>);
impl<'s> CFType for SliceProvider<'s> {}

#[allow(non_snake_case)]
impl CGDataProvider {
    /**
    # Safety
    * Bounds
    * Among other issues, they document that "The underlying data must not change for the life of the data provider."
    */
    pub unsafe fn createDirect(nullable_info: *mut c_void, size: usize, callbacks: &CGDataProviderDirectCallbacks) -> Option<StrongCell<Self>> {
        let context = CGDataProviderCreateDirect(nullable_info, size, callbacks);
        if context.is_null() {
            None
        } else {
            Some(StrongCell::assuming_retained_nonnull(context))
        }
    }
    pub fn copyData(&self) -> Option<StrongCell<core_foundationr::CFData>> {
        unsafe {
            let data = CGDataProviderCopyData(self);
            if data.is_null() {
                None
            } else {
                Some(StrongCell::assuming_retained_nonnull(data))
            }
        }
    }
}
impl<'s> SliceProvider<'s> {
    pub fn new(slice: &'s mut [u8]) -> Option<StrongMutCell<Self>> {
        let callbacks = CGDataProviderDirectCallbacks {
            version: 0,
            getBytePointer: Some(raw_get_pointer),
            releaseBytePointer: None,
            getBytesAtPosition: None,
            releaseInfo: None,
        };
        unsafe{CGDataProvider::createDirect(slice.as_mut_ptr() as *mut c_void, slice.len(), &callbacks).map(|provider| {
            use core_foundationr::CFTypeBehavior;
            let ptr = provider.as_ptr() as *const Self;
            std::mem::forget(provider);
            StrongCell::assuming_retained_nonnull(ptr).assuming_mut()
        })}
    }
    pub fn as_provider_mut(&mut self) -> &mut CGDataProvider {
        &mut self.0
    }
}



#[cfg(test)] mod tests {
    #[test] fn create_direct() {
        let mut slice = [1,2,3,4,5,6];
        let _ = super::SliceProvider::new(&mut slice).unwrap();
    }
}