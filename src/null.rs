use crate::{AsJRef, JavaObject};

#[derive(Copy, Clone)]
pub struct Null;

impl<J: JavaObject> AsJRef<J> for Null {
    fn as_jref(&self) -> crate::Nullable<&J> {
        let null_ptr: *const () = std::ptr::null();
        let null_ref: &J = unsafe { std::mem::transmute(null_ptr) };
        Ok(null_ref)
    }
}
