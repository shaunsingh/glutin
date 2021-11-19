#![cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd",
))]

use crate::platform::ContextTraitExt;
pub use crate::platform_impl::{HeadlessContextExt, RawContextExt, RawHandle};
use crate::{Context, ContextCurrentState};
pub use glutin_egl_sys::EGLContext;
#[cfg(feature = "x11")]
pub use glutin_glx_sys::GLXContext;

pub use winit::platform::unix::*;

#[cfg(feature = "x11")]
pub mod x11 {
    use crate::event_loop::EventLoopWindowTarget;
    use crate::platform::unix::EventLoopWindowTargetExtUnix;
    use std::fmt::{Debug, Formatter};
    use std::sync::Arc;

    pub struct XConnection {
        pub(crate) display: *mut x11_dl::xlib::Display,
        pub(crate) xlib: x11_dl::xlib::Xlib,
        pub(crate) xrender: x11_dl::xrender::Xrender,
    }

    impl Debug for XConnection {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("XConnection").field("display", &self.display).finish_non_exhaustive()
        }
    }

    impl XConnection {
        pub unsafe fn from_event_loop<T>(e: &EventLoopWindowTarget<T>) -> Option<Arc<Self>> {
            let display = e.xlib_display()? as _;
            let xlib = x11_dl::xlib::Xlib::open().ok()?;
            let xrender = x11_dl::xrender::Xrender::open().ok()?;
            let (mut major, mut minor) = (0, 8);
            if (xrender.XRenderQueryExtension)(display, &mut major, &mut minor) != 1 {
                return None;
            }
            Some(Arc::new(Self { display, xlib, xrender }))
        }
    }
}

use std::os::raw;

impl<T: ContextCurrentState> ContextTraitExt for Context<T> {
    type Handle = RawHandle;

    #[inline]
    unsafe fn raw_handle(&self) -> Self::Handle {
        self.context.raw_handle()
    }

    #[inline]
    unsafe fn get_egl_display(&self) -> Option<*const raw::c_void> {
        self.context.get_egl_display()
    }
}
