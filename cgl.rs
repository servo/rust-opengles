// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Mac-specific OpenGL bindings.

#[allow(non_uppercase_statics)];

use gl2::{GLenum, GLint, GLsizei, GLuint};
use std::libc::{c_void, c_int};

pub type CGLContextObj = *c_void;
pub type CGLError = c_int;
pub type CGLPixelFormatAttribute = c_int;
pub type CGLPixelFormatObj = *c_void;
pub type IOSurfaceRef = *c_void;

pub static kCGLNoError: CGLError = 0;

pub static kCGLPFADoubleBuffer: CGLPixelFormatAttribute = 5;
pub static kCGLPFACompliant: CGLPixelFormatAttribute = 83;

#[nolink]
extern {
    // CGLCurrent.h

    pub fn CGLSetCurrentContext(ctx: CGLContextObj) -> CGLError;
    pub fn CGLGetCurrentContext() -> CGLContextObj;

    // OpenGL.h

    // Pixel format functions
    pub fn CGLChoosePixelFormat(attribs: *CGLPixelFormatAttribute, pix: *CGLPixelFormatObj,
                                npix: *GLint) -> CGLError;
    pub fn CGLDestroyPixelFormat(pix: CGLPixelFormatObj);

    // Context functions
    pub fn CGLCreateContext(pix: CGLPixelFormatObj, share: CGLContextObj, ctx: *CGLContextObj) ->
                            CGLError;

    // Locking functions
    pub fn CGLLockContext(ctx: CGLContextObj) -> CGLError;
    pub fn CGLUnlockContext(ctx: CGLContextObj) -> CGLError;

    // CGLIOSurface.h

    pub fn CGLTexImageIOSurface2D(ctx: CGLContextObj, target: GLenum, internal_format: GLenum,
                                  width: GLsizei, height: GLsizei, format: GLenum, ty: GLenum,
                                  ioSurface: IOSurfaceRef, plane: GLuint) -> CGLError;
}