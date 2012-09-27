// Mac-specific OpenGL bindings.

use gl2::{GLenum, GLint, GLsizei, GLuint};
use libc::*;

pub type CGLContextObj = *c_void;
pub type CGLError = c_int;
pub type CGLPixelFormatAttribute = c_int;
pub type CGLPixelFormatObj = *c_void;
pub type IOSurfaceRef = *c_void;

pub const kCGLNoError: CGLError = 0;

pub const kCGLPFADoubleBuffer: CGLPixelFormatAttribute = 5;
pub const kCGLPFACompliant: CGLPixelFormatAttribute = 83;

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

#[nolink]
extern mod ll {
    // CGLContext.h
    fn CGLGetCurrentContext() -> CGLContextObj;

    // CGLIOSurface.h
    fn CGLTexImageIOSurface2D(ctx: CGLContextObj, target: GLenum, internal_format: GLenum,
                              width: GLsizei, height: GLsizei, format: GLenum, pixel_type: GLenum,
                              ioSurface: IOSurfaceRef, plane: GLuint)
                           -> CGLError;
}

