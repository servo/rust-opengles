// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Functions for X11 support.

use libc::{c_char, c_int, c_ulong};

// Types

pub struct Display {
    opaque: (),
}

pub type GLXDrawable = c_ulong;

pub struct __GLXFBConfig {
    opaque: (),
}

pub type GLXFBConfig = *const __GLXFBConfig;

pub type GLXPixmap = c_ulong;

pub type Pixmap = c_ulong;

pub struct __XVisualInfo {
    opaque: (),
}

pub type XVisualInfo = *const __XVisualInfo;

// Constants

pub static GLX_BIND_TO_TEXTURE_RGB_EXT: c_int     = 0x20d0;
pub static GLX_BIND_TO_TEXTURE_RGBA_EXT: c_int    = 0x20d1;
pub static GLX_BIND_TO_MIPMAP_TEXTURE_EXT: c_int  = 0x20d2;
pub static GLX_BIND_TO_TEXTURE_TARGETS_EXT: c_int = 0x20d3;
pub static GLX_Y_INVERTED_EXT: c_int              = 0x20d4;

pub static GLX_TEXTURE_FORMAT_EXT: c_int = 0x20d5;
pub static GLX_TEXTURE_TARGET_EXT: c_int = 0x20d6;
pub static GLX_MIPMAP_TEXTURE_EXT: c_int = 0x20d7;

pub static GLX_TEXTURE_FORMAT_NONE_EXT: c_int = 0x20d8;
pub static GLX_TEXTURE_FORMAT_RGB_EXT: c_int  = 0x20d9;
pub static GLX_TEXTURE_FORMAT_RGBA_EXT: c_int = 0x20da;

pub static GLX_TEXTURE_1D_BIT_EXT: c_int        = 0x1;
pub static GLX_TEXTURE_2D_BIT_EXT: c_int        = 0x2;
pub static GLX_TEXTURE_RECTANGLE_BIT_EXT: c_int = 0x4;

pub static GLX_TEXTURE_1D_EXT: c_int        = 0x20db;
pub static GLX_TEXTURE_2D_EXT: c_int        = 0x20dc;
pub static GLX_TEXTURE_RECTANGLE_EXT: c_int = 0x20dd;

pub static GLX_FRONT_LEFT_EXT: c_int  = 0x20de;
pub static GLX_FRONT_RIGHT_EXT: c_int = 0x20df;
pub static GLX_BACK_LEFT_EXT: c_int   = 0x20e0;
pub static GLX_BACK_RIGHT_EXT: c_int  = 0x20e1;
pub static GLX_FRONT_EXT: c_int       = GLX_FRONT_LEFT_EXT;
pub static GLX_BACK_EXT: c_int        = GLX_BACK_LEFT_EXT;
pub static GLX_AUX0_EXT: c_int        = 0x20e2;
pub static GLX_AUX1_EXT: c_int        = 0x20e3;
pub static GLX_AUX2_EXT: c_int        = 0x20e4;
pub static GLX_AUX3_EXT: c_int        = 0x20e5;
pub static GLX_AUX4_EXT: c_int        = 0x20e6;
pub static GLX_AUX5_EXT: c_int        = 0x20e7;
pub static GLX_AUX6_EXT: c_int        = 0x20e8;
pub static GLX_AUX7_EXT: c_int        = 0x20e9;
pub static GLX_AUX8_EXT: c_int        = 0x20ea;
pub static GLX_AUX9_EXT: c_int        = 0x20eb;

pub static GLX_DRAWABLE_TYPE: c_int   = 0x8010;
pub static GLX_RENDER_TYPE: c_int     = 0x8011;

pub static GLX_VENDOR: c_int          = 1;

pub static GLX_RGBA: c_int            = 4;
pub static GLX_DOUBLEBUFFER: c_int    = 5;
pub static GLX_ALPHA_SIZE: c_int      = 11;
pub static GLX_DEPTH_SIZE: c_int      = 12;

pub static GLX_WINDOW_BIT: c_int      = 0x01;
pub static GLX_PIXMAP_BIT: c_int      = 0x02;
pub static GLX_PBUFFER_BIT: c_int     = 0x04;
pub static GLX_AUX_BUFFERS_BIT: c_int = 0x10;

pub static GLX_RGBA_BIT: c_int        = 0x00000001;
// Functions

extern {
    pub fn glXQueryVersion(dpy: *const Display, major: *mut c_int, minor: *mut c_int) -> bool;

    pub fn glXGetProcAddress(procName: *const c_char) -> extern "C" fn();

    pub fn glXReleaseTexImageEXT(dpy: *const Display, drawable: GLXDrawable, buffer: c_int);

    pub fn glXChooseFBConfig(dpy: *const Display,
                             screen: c_int,
                             attrib_list: *const c_int,
                             n_elements: *mut c_int)
                             -> *const GLXFBConfig;

    pub fn glXChooseVisual(dpy: *const Display, screen: c_int, attribList: *const c_int) -> *const XVisualInfo;

    // For GLX 1.3+
    pub fn glXCreatePixmap(dpy: *const Display, config: GLXFBConfig, pixmap: Pixmap, attribList: *const c_int)
                           -> GLXPixmap;

    pub fn glXDestroyPixmap(dpy: *const Display, pixmap: GLXPixmap);

    // For GLX < 1.3. Use only to match behavior with other libraries (i.e. Skia) that
    // access GLX pixmaps using the visual instead of fbconfig.
    pub fn glXCreateGLXPixmap(dpy: *const Display, visual: *const XVisualInfo, pixmap: Pixmap) -> GLXPixmap;

    pub fn glXDestroyGLXPixmap(dpy: *const Display, pix: GLXPixmap);

    pub fn glXGetFBConfigAttrib(dpy: *const Display,
                                config: GLXFBConfig,
                                attribute: c_int,
                                value: *mut c_int)
                                -> c_int;

    pub fn glXGetFBConfigs(dpy: *const Display, screen: c_int, nelements: *mut c_int) -> *const GLXFBConfig;

    pub fn glXGetVisualFromFBConfig(dpy: *const Display, config: GLXFBConfig) -> *const XVisualInfo;
}

pub fn get_version(display: *const Display) -> (int, int) {
    unsafe {
        let mut major = 0;
        let mut minor = 0;
        glXQueryVersion(display, &mut major, &mut minor);
        (major as int, minor as int)
    }
}
