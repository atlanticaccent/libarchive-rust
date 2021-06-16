use libarchive3_sys::ffi;

use crate::archive::{ArchiveHandle, Handle};

pub struct Writer {
    handle: ArchiveHandle,
}

impl Writer {
    pub(crate) fn new(handle: ArchiveHandle) -> Self {
        Writer { handle: handle }
    }
}

impl Handle for Writer {
    unsafe fn handle(&self) -> &mut ffi::Struct_archive {
        self.handle.handle()
    }
}