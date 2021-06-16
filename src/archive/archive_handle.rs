use libarchive3_sys::ffi;

pub struct ArchiveHandle {
    handle: *mut ffi::Struct_archive,
}

impl ArchiveHandle {
    pub unsafe fn from_raw(handle: *mut ffi::Struct_archive) -> Option<Self> {
        handle.as_mut().map(|handle| ArchiveHandle { handle: handle } )
    }
}

impl Drop for ArchiveHandle {
    fn drop(&mut self) {
        unsafe {
            // It doesn't matter whether to call read or write variants
            // of the following functions, because since libarchive-2.7.0
            // they are implemented identically and know which kind of
            // archive struct they deal with.
            // The documentation suggests not calling close(),
            // because free() calls it automatically, but actually
            // free() doesn't call it for fatally failed archives,
            // which apparently leads to file descriptors leaks.
            ffi::archive_read_close(self.handle);
            ffi::archive_read_free(self.handle);
        }
    }
}

impl crate::archive::Handle for ArchiveHandle {
    unsafe fn handle(&self) -> &mut ffi::Struct_archive {
        &mut *self.handle
    }
}
