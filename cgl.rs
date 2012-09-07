// Mac-specific OpenGL bindings.

use gl2::{GLenum, GLint, GLsizei, GLuint};
use libc::*;

type CGLContextObj = *c_void;
type CGLError = c_int;
type CGLPixelFormatAttribute = c_int;
type CGLPixelFormatObj = *c_void;
type IOSurfaceRef = *c_void;

const kCGLNoError: CGLError = 0;

const kCGLPFADoubleBuffer: CGLPixelFormatAttribute = 5;
const kCGLPFACompliant: CGLPixelFormatAttribute = 83;

#[nolink]
extern {
    // CGLCurrent.h

    fn CGLSetCurrentContext(ctx: CGLContextObj) -> CGLError;
    fn CGLGetCurrentContext() -> CGLContextObj;

    // OpenGL.h

    // Pixel format functions
    fn CGLChoosePixelFormat(attribs: *CGLPixelFormatAttribute, pix: *CGLPixelFormatObj,
                            npix: *GLint) -> CGLError;
    fn CGLDestroyPixelFormat(pix: CGLPixelFormatObj);

    // Context functions
    fn CGLCreateContext(pix: CGLPixelFormatObj, share: CGLContextObj, ctx: *CGLContextObj) ->
                        CGLError;

    // Locking functions
    fn CGLLockContext(ctx: CGLContextObj) -> CGLError;
    fn CGLUnlockContext(ctx: CGLContextObj) -> CGLError;

    // CGLIOSurface.h

    fn CGLTexImageIOSurface2D(ctx: CGLContextObj, target: GLenum, internal_format: GLenum,
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

