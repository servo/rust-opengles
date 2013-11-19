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

pub static kCGLPFAAllRenderers: CGLPixelFormatAttribute = 1;
pub static kCGLPFADoubleBuffer: CGLPixelFormatAttribute = 5;
pub static kCGLPFAStereo: CGLPixelFormatAttribute = 6;
pub static kCGLPFAAuxBuffers: CGLPixelFormatAttribute = 7;
pub static kCGLPFAColorSize: CGLPixelFormatAttribute = 8;
pub static kCGLPFAAlphaSize: CGLPixelFormatAttribute = 11;
pub static kCGLPFADepthSize: CGLPixelFormatAttribute = 12;
pub static kCGLPFAStencilSize: CGLPixelFormatAttribute = 13;
pub static kCGLPFAAccumSize: CGLPixelFormatAttribute = 14;
pub static kCGLPFAMinimumPolicy: CGLPixelFormatAttribute = 51;
pub static kCGLPFAMaximumPolicy: CGLPixelFormatAttribute = 52;
pub static kCGLPFAOffScreen: CGLPixelFormatAttribute = 53;
pub static kCGLPFAFullScreen: CGLPixelFormatAttribute = 54;
pub static kCGLPFASampleBuffers: CGLPixelFormatAttribute = 55;
pub static kCGLPFASamples: CGLPixelFormatAttribute = 56;
pub static kCGLPFAAuxDepthStencil: CGLPixelFormatAttribute = 57;
pub static kCGLPFAColorFloat: CGLPixelFormatAttribute = 58;
pub static kCGLPFAMultisample: CGLPixelFormatAttribute = 59;
pub static kCGLPFASupersample: CGLPixelFormatAttribute = 60;
pub static kCGLPFASampleAlpha: CGLPixelFormatAttribute = 61;
pub static kCGLPFARendererID: CGLPixelFormatAttribute = 70;
pub static kCGLPFASingleRenderer: CGLPixelFormatAttribute = 71;
pub static kCGLPFANoRecovery: CGLPixelFormatAttribute = 72;
pub static kCGLPFAAccelerated: CGLPixelFormatAttribute = 73;
pub static kCGLPFAClosestPolicy: CGLPixelFormatAttribute = 74;
pub static kCGLPFARobust: CGLPixelFormatAttribute = 75;
pub static kCGLPFABackingStore: CGLPixelFormatAttribute = 76;
pub static kCGLPFAMPSafe: CGLPixelFormatAttribute = 78;
pub static kCGLPFAWindow: CGLPixelFormatAttribute = 80;
pub static kCGLPFAMultiScreen: CGLPixelFormatAttribute = 81;
pub static kCGLPFACompliant: CGLPixelFormatAttribute = 83;
pub static kCGLPFADisplayMask: CGLPixelFormatAttribute = 84;
pub static kCGLPFAPBuffer: CGLPixelFormatAttribute = 90;
pub static kCGLPFARemotePBuffer: CGLPixelFormatAttribute = 91;
pub static kCGLPFAAllowOfflineRenderers: CGLPixelFormatAttribute = 96;
pub static kCGLPFAAcceleratedCompute: CGLPixelFormatAttribute = 97;
pub static kCGLPFAOpenGLProfile: CGLPixelFormatAttribute = 99;
pub static kCGLPFAVirtualScreenCount: CGLPixelFormatAttribute = 128;

pub static CORE_BOOLEAN_ATTRIBUTES: &'static [CGLPixelFormatAttribute] =
    &[kCGLPFAAllRenderers,
      kCGLPFADoubleBuffer,
      kCGLPFAStereo,
      kCGLPFAAuxBuffers,
      kCGLPFAMinimumPolicy,
      kCGLPFAMaximumPolicy,
      kCGLPFAOffScreen,
      kCGLPFAFullScreen,
      kCGLPFAAuxDepthStencil,
      kCGLPFAColorFloat,
      kCGLPFAMultisample,
      kCGLPFASupersample,
      kCGLPFASampleAlpha,
      kCGLPFASingleRenderer,
      kCGLPFANoRecovery,
      kCGLPFAAccelerated,
      kCGLPFAClosestPolicy,
      kCGLPFARobust,
      kCGLPFABackingStore,
      kCGLPFAMPSafe,
      kCGLPFAWindow,
      kCGLPFAMultiScreen,
      kCGLPFACompliant,
      kCGLPFAPBuffer,
      kCGLPFARemotePBuffer,
      kCGLPFAAllowOfflineRenderers,
      kCGLPFAAcceleratedCompute];

pub static CORE_INTEGER_ATTRIBUTES: &'static [CGLPixelFormatAttribute] =
    &[kCGLPFAColorSize,
      kCGLPFAAlphaSize,
      kCGLPFADepthSize,
      kCGLPFAStencilSize,
      kCGLPFAAccumSize,
      kCGLPFASampleBuffers,
      kCGLPFASamples,
      kCGLPFARendererID,
      kCGLPFADisplayMask,
      kCGLPFAOpenGLProfile,
      kCGLPFAVirtualScreenCount];

#[nolink]
extern {
    // CGLCurrent.h

    pub fn CGLSetCurrentContext(ctx: CGLContextObj) -> CGLError;
    pub fn CGLGetCurrentContext() -> CGLContextObj;

    // OpenGL.h

    // Pixel format functions
    pub fn CGLChoosePixelFormat(attribs: *CGLPixelFormatAttribute,
                                pix: *mut CGLPixelFormatObj,
                                npix: *mut GLint) -> CGLError;
    pub fn CGLDescribePixelFormat(pix: CGLPixelFormatObj,
                                  pix_num: GLint,
                                  attrib: CGLPixelFormatAttribute,
                                  value: *mut GLint) -> CGLError;
    pub fn CGLDestroyPixelFormat(pix: CGLPixelFormatObj);

    // Context functions
    pub fn CGLCreateContext(pix: CGLPixelFormatObj, share: CGLContextObj, ctx: *CGLContextObj) ->
                            CGLError;
    pub fn CGLGetPixelFormat(ctx: CGLContextObj) -> CGLPixelFormatObj;

    // Locking functions
    pub fn CGLLockContext(ctx: CGLContextObj) -> CGLError;
    pub fn CGLUnlockContext(ctx: CGLContextObj) -> CGLError;

    // CGLIOSurface.h

    pub fn CGLTexImageIOSurface2D(ctx: CGLContextObj, target: GLenum, internal_format: GLenum,
                                  width: GLsizei, height: GLsizei, format: GLenum, ty: GLenum,
                                  ioSurface: IOSurfaceRef, plane: GLuint) -> CGLError;
}
