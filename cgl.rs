// Mac-specific OpenGL bindings.

use gl2::{GLenum, GLsizei, GLuint};
use libc::*;

type CGLContextObj = *c_void;
type CGLError = c_int;
type IOSurfaceRef = *c_void;

const kCGLNoError: CGLError = 0;

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

