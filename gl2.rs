// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::libc::*;
use std::cmp;
use std::cast;
use std::cast::transmute;
use std::ptr;
use std::ptr::to_unsafe_ptr;
use std::str;
use std::str::{as_c_str, from_bytes};
use std::str::raw::from_c_str;
use std::sys::size_of;
use std::vec::from_elem;
use std::vec::raw::to_ptr;

// Linking
#[nolink]
#[cfg(target_os = "macos")]
#[link_args="-framework OpenGL"]
pub extern { }

#[nolink]
#[cfg(target_os = "linux")]
#[link_args="-lGL"]
pub extern { }

// Constants

/* BeginMode */
pub static POINTS:         c_uint = 0x0000 as c_uint;
pub static LINES:          c_uint = 0x0001 as c_uint;
pub static LINE_LOOP:      c_uint = 0x0002 as c_uint;
pub static LINE_STRIP:     c_uint = 0x0003 as c_uint;
pub static TRIANGLES:      c_uint = 0x0004 as c_uint;
pub static TRIANGLE_STRIP: c_uint = 0x0005 as c_uint;
pub static TRIANGLE_FAN:   c_uint = 0x0006 as c_uint;

pub static DEPTH_BUFFER_BIT:   c_uint = 0x00000100 as c_uint;
pub static STENCIL_BUFFER_BIT: c_uint = 0x00000400 as c_uint;
pub static COLOR_BUFFER_BIT:   c_uint = 0x00004000 as c_uint;

/* BlendingFactorDest */
pub static ZERO:                     c_uint = 0      as c_uint;
pub static ONE:                      c_uint = 1      as c_uint;
pub static SRC_COLOR:                c_uint = 0x0300 as c_uint;
pub static ONE_MINUS_SRC_COLOR:      c_uint = 0x0301 as c_uint;
pub static SRC_ALPHA:                c_uint = 0x0302 as c_uint;
pub static ONE_MINUS_SRC_ALPHA:      c_uint = 0x0303 as c_uint;
pub static DST_ALPHA:                c_uint = 0x0304 as c_uint;
pub static ONE_MINUS_DST_ALPHA:      c_uint = 0x0305 as c_uint;

/* BlendingFactorSrc */
pub static DST_COLOR:                c_uint = 0x0306 as c_uint;
pub static ONE_MINUS_DST_COLOR:      c_uint = 0x0307 as c_uint;
pub static SRC_ALPHA_SATURATE:       c_uint = 0x0308 as c_uint;

/* BlendEquationSeparate */
pub static FUNC_ADD:                 c_uint = 0x8006 as c_uint;
pub static BLEND_EQUATION:           c_uint = 0x8009 as c_uint;
pub static BLEND_EQUATION_RGB:       c_uint = 0x8009 as c_uint;
pub static BLEND_EQUATION_ALPHA:     c_uint = 0x883D as c_uint;

/* BlendSubtract */
pub static FUNC_SUBTRACT:            c_uint = 0x800A as c_uint;
pub static FUNC_REVERSE_SUBTRACT:    c_uint = 0x800B as c_uint;

/* Separate Blend Functions */
pub static BLEND_DST_RGB:            c_uint = 0x80C8 as c_uint;
pub static BLEND_SRC_RGB:            c_uint = 0x80C9 as c_uint;
pub static BLEND_DST_ALPHA:          c_uint = 0x80CA as c_uint;
pub static BLEND_SRC_ALPHA:          c_uint = 0x80CB as c_uint;
pub static CONSTANT_COLOR:           c_uint = 0x8001 as c_uint;
pub static ONE_MINUS_CONSTANT_COLOR: c_uint = 0x8002 as c_uint;
pub static CONSTANT_ALPHA:           c_uint = 0x8003 as c_uint;
pub static ONE_MINUS_CONSTANT_ALPHA: c_uint = 0x8004 as c_uint;
pub static BLEND_COLOR:              c_uint = 0x8005 as c_uint;

/* Errors. */
pub static NO_ERROR: c_uint = 0 as c_uint;
pub static INVALID_ENUM: c_uint = 0x0500 as c_uint;
pub static INVALID_VALUE: c_uint = 0x0501 as c_uint;
pub static INVALID_OPERATION: c_uint = 0x0502 as c_uint;
pub static STACK_OVERFLOW: c_uint = 0x0503 as c_uint;
pub static STACK_UNDERFLOW: c_uint = 0x0504 as c_uint;
pub static OUT_OF_MEMORY: c_uint = 0x0505 as c_uint;
pub static INVALID_FRAMEBUFFER_OPERATION: c_uint = 0x0506 as c_uint;

/* DataType */
pub static BYTE:           c_uint = 0x1400 as c_uint;
pub static UNSIGNED_BYTE:  c_uint = 0x1401 as c_uint;
pub static SHORT:          c_uint = 0x1402 as c_uint;
pub static UNSIGNED_SHORT: c_uint = 0x1403 as c_uint;
pub static INT:            c_uint = 0x1404 as c_uint;
pub static UNSIGNED_INT:   c_uint = 0x1405 as c_uint;
pub static FLOAT:          c_uint = 0x1406 as c_uint;
pub static FIXED:          c_uint = 0x140C as c_uint;

/* EnableCap */
pub static TEXTURE_2D:               c_uint = 0x0DE1 as c_uint;
pub static CULL_FACE:                c_uint = 0x0B44 as c_uint;
pub static BLEND:                    c_uint = 0x0BE2 as c_uint;
pub static DITHER:                   c_uint = 0x0BD0 as c_uint;
pub static STENCIL_TEST:             c_uint = 0x0B90 as c_uint;
pub static DEPTH_TEST:               c_uint = 0x0B71 as c_uint;
pub static SCISSOR_TEST:             c_uint = 0x0C11 as c_uint;
pub static POLYGON_OFFSET_FILL:      c_uint = 0x8037 as c_uint;
pub static SAMPLE_ALPHA_TO_COVERAGE: c_uint = 0x809E as c_uint;
pub static SAMPLE_COVERAGE:          c_uint = 0x80A0 as c_uint;

/* Polygons */
pub static POINT: c_uint = 0x1B00 as c_uint;
pub static LINE: c_uint = 0x1B01 as c_uint;
pub static FILL: c_uint = 0x1B02 as c_uint;
pub static CW:  c_uint = 0x0900 as c_uint;
pub static CCW: c_uint = 0x0901 as c_uint;
pub static POLYGON_MODE: c_uint = 0x0B40 as c_uint;
pub static POLYGON_SMOOTH: c_uint = 0x0B41 as c_uint;
pub static POLYGON_STIPPLE: c_uint = 0x0B42 as c_uint;
pub static EDGE_FLAG: c_uint = 0x0B43 as c_uint;

/* GetPName */
pub static LINE_WIDTH:                    c_uint = 0x0B21 as c_uint;
pub static ALIASED_POINT_SIZE_RANGE:      c_uint = 0x846D as c_uint;
pub static ALIASED_LINE_WIDTH_RANGE:      c_uint = 0x846E as c_uint;
pub static CULL_FACE_MODE:                c_uint = 0x0B45 as c_uint;
pub static FRONT_FACE:                    c_uint = 0x0B46 as c_uint;
pub static DEPTH_RANGE:                   c_uint = 0x0B70 as c_uint;
pub static DEPTH_WRITEMASK:               c_uint = 0x0B72 as c_uint;
pub static DEPTH_CLEAR_VALUE:             c_uint = 0x0B73 as c_uint;
pub static DEPTH_FUNC:                    c_uint = 0x0B74 as c_uint;
pub static STENCIL_CLEAR_VALUE:           c_uint = 0x0B91 as c_uint;
pub static STENCIL_FUNC:                  c_uint = 0x0B92 as c_uint;
pub static STENCIL_FAIL:                  c_uint = 0x0B94 as c_uint;
pub static STENCIL_PASS_DEPTH_FAIL:       c_uint = 0x0B95 as c_uint;
pub static STENCIL_PASS_DEPTH_PASS:       c_uint = 0x0B96 as c_uint;
pub static STENCIL_REF:                   c_uint = 0x0B97 as c_uint;
pub static STENCIL_VALUE_MASK:            c_uint = 0x0B93 as c_uint;
pub static STENCIL_WRITEMASK:             c_uint = 0x0B98 as c_uint;
pub static STENCIL_BACK_FUNC:             c_uint = 0x8800 as c_uint;
pub static STENCIL_BACK_FAIL:             c_uint = 0x8801 as c_uint;
pub static STENCIL_BACK_PASS_DEPTH_FAIL:  c_uint = 0x8802 as c_uint;
pub static STENCIL_BACK_PASS_DEPTH_PASS:  c_uint = 0x8803 as c_uint;
pub static STENCIL_BACK_REF:              c_uint = 0x8CA3 as c_uint;
pub static STENCIL_BACK_VALUE_MASK:       c_uint = 0x8CA4 as c_uint;
pub static STENCIL_BACK_WRITEMASK:        c_uint = 0x8CA5 as c_uint;
pub static VIEWPORT:                      c_uint = 0x0BA2 as c_uint;
pub static SCISSOR_BOX:                   c_uint = 0x0C10 as c_uint;
/*      SCISSOR_TEST */
pub static COLOR_CLEAR_VALUE:             c_uint = 0x0C22 as c_uint;
pub static COLOR_WRITEMASK:               c_uint = 0x0C23 as c_uint;
pub static UNPACK_ALIGNMENT:              c_uint = 0x0CF5 as c_uint;
pub static PACK_ALIGNMENT:                c_uint = 0x0D05 as c_uint;
pub static MAX_TEXTURE_SIZE:              c_uint = 0x0D33 as c_uint;
pub static MAX_VIEWPORT_DIMS:             c_uint = 0x0D3A as c_uint;
pub static SUBPIXEL_BITS:                 c_uint = 0x0D50 as c_uint;
pub static RED_BITS:                      c_uint = 0x0D52 as c_uint;
pub static GREEN_BITS:                    c_uint = 0x0D53 as c_uint;
pub static BLUE_BITS:                     c_uint = 0x0D54 as c_uint;
pub static ALPHA_BITS:                    c_uint = 0x0D55 as c_uint;
pub static DEPTH_BITS:                    c_uint = 0x0D56 as c_uint;
pub static STENCIL_BITS:                  c_uint = 0x0D57 as c_uint;
pub static POLYGON_OFFSET_UNITS:          c_uint = 0x2A00 as c_uint;
/*      POLYGON_OFFSET_FILL */
pub static POLYGON_OFFSET_FACTOR:         c_uint = 0x8038 as c_uint;
pub static TEXTURE_BINDING_2D:            c_uint = 0x8069 as c_uint;
pub static SAMPLE_BUFFERS:                c_uint = 0x80A8 as c_uint;
pub static SAMPLES:                       c_uint = 0x80A9 as c_uint;
pub static SAMPLE_COVERAGE_VALUE:         c_uint = 0x80AA as c_uint;
pub static SAMPLE_COVERAGE_INVERT:        c_uint = 0x80AB as c_uint;

/* GetTarget */
pub static UNPACK_ROW_LENGTH: c_uint = 0x0CF2 as c_uint;

/* PixelFormat */
pub static DEPTH_COMPONENT: c_uint = 0x1902 as c_uint;
pub static RED:             c_uint = 0x1903 as c_uint;
pub static GREEN:           c_uint = 0x1904 as c_uint;
pub static BLUE:            c_uint = 0x1905 as c_uint;
pub static ALPHA:           c_uint = 0x1906 as c_uint;
pub static RGB:             c_uint = 0x1907 as c_uint;
pub static RGBA:            c_uint = 0x1908 as c_uint;

pub static BGRA:            c_uint = 0x80e1 as c_uint;   // NB: Not OpenGL ES!
pub static RGBA8:           c_uint = 0x8058 as c_uint;   // NB: Not OpenGL ES!

/* Packed Pixels */
pub static UNSIGNED_INT_8_8_8_8_REV: c_uint = 0x8367 as c_uint; // NB: Not OpenGL ES!

/* Shaders */
pub static FRAGMENT_SHADER:                  c_uint = 0x8B30 as c_uint;
pub static VERTEX_SHADER:                    c_uint = 0x8B31 as c_uint;
pub static MAX_VERTEX_ATTRIBS:               c_uint = 0x8869 as c_uint;
pub static MAX_VERTEX_UNIFORM_VECTORS:       c_uint = 0x8DFB as c_uint;
pub static MAX_VARYING_VECTORS:              c_uint = 0x8DFC as c_uint;
pub static MAX_COMBINED_TEXTURE_IMAGE_UNITS: c_uint = 0x8B4D as c_uint;
pub static MAX_VERTEX_TEXTURE_IMAGE_UNITS:   c_uint = 0x8B4C as c_uint;
pub static MAX_TEXTURE_IMAGE_UNITS:          c_uint = 0x8872 as c_uint;
pub static MAX_FRAGMENT_UNIFORM_VECTORS:     c_uint = 0x8DFD as c_uint;
pub static SHADER_TYPE:                      c_uint = 0x8B4F as c_uint;
pub static DELETE_STATUS:                    c_uint = 0x8B80 as c_uint;
pub static LINK_STATUS:                      c_uint = 0x8B82 as c_uint;
pub static VALIDATE_STATUS:                  c_uint = 0x8B83 as c_uint;
pub static ATTACHED_SHADERS:                 c_uint = 0x8B85 as c_uint;
pub static ACTIVE_UNIFORMS:                  c_uint = 0x8B86 as c_uint;
pub static ACTIVE_UNIFORM_MAX_LENGTH:        c_uint = 0x8B87 as c_uint;
pub static ACTIVE_ATTRIBUTES:                c_uint = 0x8B89 as c_uint;
pub static ACTIVE_ATTRIBUTE_MAX_LENGTH:      c_uint = 0x8B8A as c_uint;
pub static SHADING_LANGUAGE_VERSION:         c_uint = 0x8B8C as c_uint;
pub static CURRENT_PROGRAM:                  c_uint = 0x8B8D as c_uint;

/* StencilFunction */
pub static NEVER:    c_uint = 0x0200 as c_uint;
pub static LESS:     c_uint = 0x0201 as c_uint;
pub static EQUAL:    c_uint = 0x0202 as c_uint;
pub static LEQUAL:   c_uint = 0x0203 as c_uint;
pub static GREATER:  c_uint = 0x0204 as c_uint;
pub static NOTEQUAL: c_uint = 0x0205 as c_uint;
pub static GEQUAL:   c_uint = 0x0206 as c_uint;
pub static ALWAYS:   c_uint = 0x0207 as c_uint;

pub static VENDOR:     c_uint = 0x1F00 as c_uint;
pub static RENDERER:   c_uint = 0x1F01 as c_uint;
pub static VERSION:    c_uint = 0x1F02 as c_uint;
pub static EXTENSIONS: c_uint = 0x1F03 as c_uint;

/* Shader Source */
pub static COMPILE_STATUS:       c_uint = 0x8B81 as c_uint;
pub static INFO_LOG_LENGTH:      c_uint = 0x8B84 as c_uint;
pub static SHADER_SOURCE_LENGTH: c_uint = 0x8B88 as c_uint;
pub static SHADER_COMPILER:      c_uint = 0x8DFA as c_uint;

/* Buffer Objects */
pub static ARRAY_BUFFER:                 c_uint = 0x8892 as c_uint;
pub static ELEMENT_ARRAY_BUFFER:         c_uint = 0x8893 as c_uint;
pub static ARRAY_BUFFER_BINDING:         c_uint = 0x8894 as c_uint;
pub static ELEMENT_ARRAY_BUFFER_BINDING: c_uint = 0x8895 as c_uint;

pub static STREAM_DRAW:  c_uint = 0x88E0 as c_uint;
pub static STATIC_DRAW:  c_uint = 0x88E4 as c_uint;
pub static DYNAMIC_DRAW: c_uint = 0x88E8 as c_uint;

/* CullFaceMode */
pub static FRONT: c_uint =           0x0404 as c_uint;
pub static BACK: c_uint =            0x0405 as c_uint;
pub static FRONT_AND_BACK: c_uint =  0x0408 as c_uint;

/* TextureMagFilter */
pub static NEAREST: c_uint = 0x2600 as c_uint;
pub static LINEAR:  c_uint = 0x2601 as c_uint;

/* TextureParameterName */
pub static TEXTURE_MAG_FILTER: c_uint = 0x2800 as c_uint;
pub static TEXTURE_MIN_FILTER: c_uint = 0x2801 as c_uint;
pub static TEXTURE_WRAP_S:     c_uint = 0x2802 as c_uint;
pub static TEXTURE_WRAP_T:     c_uint = 0x2803 as c_uint;

/* TextureUnit */
pub static TEXTURE0:       c_uint = 0x84C0 as c_uint;
pub static TEXTURE1:       c_uint = 0x84C1 as c_uint;
pub static TEXTURE2:       c_uint = 0x84C2 as c_uint;
pub static TEXTURE3:       c_uint = 0x84C3 as c_uint;
pub static TEXTURE4:       c_uint = 0x84C4 as c_uint;
pub static TEXTURE5:       c_uint = 0x84C5 as c_uint;
pub static TEXTURE6:       c_uint = 0x84C6 as c_uint;
pub static TEXTURE7:       c_uint = 0x84C7 as c_uint;
pub static TEXTURE8:       c_uint = 0x84C8 as c_uint;
pub static TEXTURE9:       c_uint = 0x84C9 as c_uint;
pub static TEXTURE10:      c_uint = 0x84CA as c_uint;
pub static TEXTURE11:      c_uint = 0x84CB as c_uint;
pub static TEXTURE12:      c_uint = 0x84CC as c_uint;
pub static TEXTURE13:      c_uint = 0x84CD as c_uint;
pub static TEXTURE14:      c_uint = 0x84CE as c_uint;
pub static TEXTURE15:      c_uint = 0x84CF as c_uint;
pub static TEXTURE16:      c_uint = 0x84D0 as c_uint;
pub static TEXTURE17:      c_uint = 0x84D1 as c_uint;
pub static TEXTURE18:      c_uint = 0x84D2 as c_uint;
pub static TEXTURE19:      c_uint = 0x84D3 as c_uint;
pub static TEXTURE20:      c_uint = 0x84D4 as c_uint;
pub static TEXTURE21:      c_uint = 0x84D5 as c_uint;
pub static TEXTURE22:      c_uint = 0x84D6 as c_uint;
pub static TEXTURE23:      c_uint = 0x84D7 as c_uint;
pub static TEXTURE24:      c_uint = 0x84D8 as c_uint;
pub static TEXTURE25:      c_uint = 0x84D9 as c_uint;
pub static TEXTURE26:      c_uint = 0x84DA as c_uint;
pub static TEXTURE27:      c_uint = 0x84DB as c_uint;
pub static TEXTURE28:      c_uint = 0x84DC as c_uint;
pub static TEXTURE29:      c_uint = 0x84DD as c_uint;
pub static TEXTURE30:      c_uint = 0x84DE as c_uint;
pub static TEXTURE31:      c_uint = 0x84DF as c_uint;
pub static ACTIVE_TEXTURE: c_uint = 0x84E0 as c_uint;

/* TextureWrapMode */
pub static REPEAT:          c_uint = 0x2901 as c_uint;
pub static CLAMP_TO_EDGE:   c_uint = 0x812F as c_uint;
pub static MIRRORED_REPEAT: c_uint = 0x8370 as c_uint;

pub static COLOR_ATTACHMENT0: c_uint = 0x8CE0 as c_uint;

pub static FRAMEBUFFER_COMPLETE: c_uint = 0x8CD5 as c_uint;

// Framebuffer Object
pub static FRAMEBUFFER:  c_uint = 0x8D40 as c_uint;
pub static RENDERBUFFER: c_uint = 0x8D41 as c_uint;

// Extensions
pub static TEXTURE_RECTANGLE_ARB: c_uint = 0x84F5 as c_uint;         // NB: Not OpenGL ES!

pub static UNPACK_CLIENT_STORAGE_APPLE: c_uint = 0x85B2 as c_uint;   // NB: Not OpenGL ES!
pub static TEXTURE_STORAGE_HINT_APPLE: c_uint = 0x85BC as c_uint;    // NB: Not OpenGL ES!
pub static STORAGE_CACHED_APPLE: c_uint = 0x85BE as c_uint;          // NB: Not OpenGL ES!
pub static STORAGE_SHARED_APPLE: c_uint = 0x85BF as c_uint;          // NB: Not OpenGL ES!


// Types

pub type GLvoid = c_void /* unknown kind Void */;

pub type GLchar = c_char;

pub type GLenum = c_uint;

pub type GLboolean = c_uchar;

pub type GLbitfield = c_uint;

pub type GLbyte = int8_t;

pub type GLshort = c_short;

pub type GLint = c_int;

pub type GLsizei = c_int;

pub type GLubyte = uint8_t;

pub type GLushort = c_ushort;

pub type GLuint = c_uint;

pub type GLfloat = f32;

pub type GLclampf = f32;

pub type GLfixed = int32_t;

pub type GLintptr = intptr_t;

pub type GLsizeiptr = ssize_t;


// Helper functions

pub fn destroy<T>(_x: T) {
    // Just let the object drop.
}


// Exposed Rust API using Rust naming conventions

pub fn active_texture(texture: GLenum) {
    unsafe {
        glActiveTexture(texture);
    }
}

pub fn attach_shader(program: GLuint, shader: GLuint) {
    unsafe {
        glAttachShader(program, shader);
    }
}

pub fn bind_attrib_location(program: GLuint, index: GLuint, name: ~str) {
    unsafe {
        do str::as_c_str(name) |cstr| {
            glBindAttribLocation(program, index, cstr);
        }
    }
}

pub fn bind_buffer(target: GLenum, buffer: GLuint) {
    unsafe {
        glBindBuffer(target, buffer);
    }
}

pub fn bind_framebuffer(target: GLenum, framebuffer: GLuint) {
    unsafe {
        glBindFramebuffer(target, framebuffer);
    }
}

pub fn bind_texture(target: GLenum, texture: GLuint) {
    unsafe {
        glBindTexture(target, texture);
    }
}

#[cfg(not(target_os="macos"))]
#[cfg(not(mac_10_6))]
pub fn bind_vertex_array(array: GLuint) {
    unsafe {
        glBindVertexArray(array);
    }
}

pub fn blend_color(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf) {
    unsafe {
        glBlendColor(red, green, blue, alpha);
    }
}

pub fn blend_equation(mode: GLenum) {
    unsafe {
        glBlendEquation(mode);
    }
}

pub fn blend_equation_separate(mode_rgb: GLenum, mode_alpha: GLenum) {
    unsafe {
        glBlendEquationSeparate(mode_rgb, mode_alpha);
    }
}

pub fn blend_func(sfactor: GLenum, dfactor: GLenum) {
    unsafe {
        glBlendFunc(sfactor, dfactor);
    }
}

pub fn blend_func_separate(src_rgb: GLenum, dst_rgb: GLenum, src_alpha: GLenum, dst_alpha: GLenum) {
    unsafe {
        glBlendFuncSeparate(src_rgb, dst_rgb, src_alpha, dst_alpha);
    }
}

// FIXME: There should be some type-safe wrapper for this...
pub fn buffer_data<T>(target: GLenum, data: &[T], usage: GLenum) {
    unsafe {
        glBufferData(target,
                         (data.len() * size_of::<T>()) as GLsizeiptr,
                         to_ptr(data) as *GLvoid,
                         usage);
    }
}

// FIXME: As above
// Note: offset is the element offset index, not byte offset
pub fn buffer_sub_data<T>(target: GLenum, element_offset_index: uint, data: &[T]) {
    unsafe {
        let size = size_of::<T>();
        glBufferSubData(target,
                            (element_offset_index * size) as GLintptr,
                            (data.len() * size) as GLsizeiptr,
                            to_ptr(data) as *GLvoid);
    }
}

pub fn check_framebuffer_status(target: GLenum) -> GLenum {
    unsafe {
        glCheckFramebufferStatus(target)
    }
}

pub fn clear(mask: GLbitfield) {
    unsafe {
        glClear(mask);
    }
}

pub fn clear_color(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf) {
    unsafe {
        glClearColor(red, green, blue, alpha);
    }
}

pub fn compile_shader(shader: GLuint) {
    unsafe {
        glCompileShader(shader);
    }
}

pub fn create_program() -> GLuint {
    unsafe {
        return glCreateProgram();
    }
}

pub fn create_shader(shader_type: GLenum) -> GLuint {
    unsafe {
        return glCreateShader(shader_type);
    }
}

pub fn cull_face(mode: GLenum) {
    unsafe {
        glCullFace(mode);
    }
}

pub fn delete_buffers(buffers: &[GLuint]) {
    unsafe {
        glDeleteBuffers(buffers.len() as GLsizei, to_ptr(buffers));
    }
}

pub fn delete_frame_buffers(frame_buffers: &[GLuint]) {
    unsafe {
        glDeleteFramebuffers(frame_buffers.len() as GLsizei, to_ptr(frame_buffers));
    }
}

pub fn delete_program(program: GLuint) {
    unsafe {
        glDeleteProgram(program);
    }
}

pub fn delete_render_buffers(render_buffers: &[GLuint]) {
    unsafe {
        glDeleteRenderbuffers(render_buffers.len() as GLsizei, to_ptr(render_buffers));
    }
}

pub fn delete_shader(shader: GLuint) {
    unsafe {
        glDeleteShader(shader);
    }
}

pub fn delete_textures(textures: &[GLuint]) {
    unsafe {
        return glDeleteTextures(textures.len() as GLsizei, to_ptr(textures));
    }
}

pub fn depth_func(func: GLenum) {
    unsafe {
        glDepthFunc(func);
    }
}

pub fn depth_mask(flag: bool) {
    unsafe {
        glDepthMask(flag as GLboolean);
    }
}

pub fn detach_shader(program: GLuint, shader: GLuint) {
    unsafe {
        glDetachShader(program, shader);
    }
}

pub fn draw_arrays(mode: GLenum, first: GLint, count: GLsizei) {
    unsafe {
        return glDrawArrays(mode, first, count);
    }
}

pub fn draw_elements(mode: GLenum, count: GLsizei, element_type: GLenum, indices: Option<&[u8]>) {
    unsafe {
        return glDrawElements(mode,
                                  match indices {
                                    Some(ref i) => cmp::min(count, i.len() as GLsizei),
                                    None => count,
                                  },
                                  element_type,
                                  match indices {
                                    Some(ref i) => cast::transmute(&i[0]),
                                    None => ptr::null(),
                                  })
    }
}

#[cfg(not(target_os="macos"))]
#[cfg(not(mac_10_6))]
pub fn draw_arrays_instanced(mode: GLenum, first: GLint, count: GLsizei, primcount: GLsizei) {
    unsafe {
        glDrawArraysInstanced(mode, first, count, primcount);
    }
}

#[cfg(not(target_os="macos"))]
#[cfg(not(mac_10_6))]
pub fn draw_elements_instanced(mode: GLenum, count: GLsizei, element_type: GLenum, indices: Option<&[u8]>, primcount: GLsizei) {
    unsafe {
        glDrawElementsInstanced(mode,
                                    match indices {
                                      Some(ref i) => cmp::min(count, i.len() as GLsizei),
                                      None => count,
                                    },
                                    element_type,
                                    match indices {
                                      Some(ref i) => cast::transmute(&i[0]),
                                      None => ptr::null(),
                                    }, 
                                    primcount);
    }
}

pub fn enable(cap: GLenum) {
    unsafe {
        glEnable(cap);
    }
}

pub fn disable(cap: GLenum) {
    unsafe {
        glDisable(cap);
    }
}

pub fn enable_vertex_attrib_array(index: GLuint) {
    unsafe {
        glEnableVertexAttribArray(index);
    }
}

pub fn disable_vertex_attrib_array(index: GLuint) {
    unsafe {
        glDisableVertexAttribArray(index);
    }
}

pub fn finish() {
    unsafe {
        return glFinish();
    }
}

pub fn flush() {
    unsafe {
        return glFlush();
    }
}

pub fn framebuffer_texture_2d(target: GLenum,
                              attachment: GLenum,
                              textarget: GLenum,
                              texture: GLuint,
                              level: GLint) {
    unsafe {
        glFramebufferTexture2D(target, attachment, textarget, texture, level);
    }
}

pub fn front_face(mode: GLenum) {
    unsafe {
        glFrontFace(mode);
    }
}

pub fn gen_buffers(n: GLsizei) -> ~[GLuint] {
    unsafe {
        let result = from_elem(n as uint, 0 as GLuint);
        glGenBuffers(n, to_ptr(result));
        return result;
    }
}

pub fn gen_framebuffers(n: GLsizei) -> ~[GLuint] {
    unsafe {
        let result = from_elem(n as uint, 0);
        glGenFramebuffers(n, to_ptr(result));
        return result;
    }
}

pub fn gen_textures(n: GLsizei) -> ~[GLuint] {
    unsafe {
        let result = from_elem(n as uint, 0 as GLuint);
        glGenTextures(n, to_ptr(result));
        return result;
    }
}

#[cfg(not(target_os="macos"))] #[cfg(not( mac_10_6))]
pub fn gen_vertex_arrays(n: GLsizei) -> ~[GLuint] {
    unsafe {
        let result = from_elem(n as uint, 0 as GLuint);
        glGenVertexArrays(n, to_ptr(result));
        return result;
    }
}

pub fn get_attrib_location(program: GLuint, name: ~str) -> c_int {
    unsafe {
        return as_c_str(name, |name_bytes|
            glGetAttribLocation(program, name_bytes as *GLchar));
    }
}

pub fn get_error() -> GLenum {
    unsafe {
        return glGetError();
    }
}

pub fn get_program_info_log(program: GLuint) -> ~str {
    unsafe {
        let mut result = from_elem(1024u, 0u8);
        let result_len: GLsizei = 0 as GLsizei;
        glGetProgramInfoLog(program,
                               1024 as GLsizei,
                               to_unsafe_ptr(&result_len),
                               to_ptr(result) as *GLchar);
        result.truncate(if result_len > 0 {result_len-1} else {0} as uint);
        return from_bytes(result);
    }
}

pub fn get_program_iv(program: GLuint, pname: GLenum) -> GLint {
    unsafe {
        let result: GLint = 0 as GLint;
        glGetProgramiv(program, pname, to_unsafe_ptr(&result));
        return result;
    }
}

pub fn get_shader_info_log(shader: GLuint) -> ~str {
    unsafe {
        let mut result = from_elem(1024u, 0u8);
        let result_len: GLsizei = 0 as GLsizei;
        glGetShaderInfoLog(shader,
                               1024 as GLsizei,
                               to_unsafe_ptr(&result_len),
                               to_ptr(result) as *GLchar);
        result.truncate(if result_len > 0 {result_len-1} else {0} as uint);
        return from_bytes(result);
    }
}

pub fn get_string(which: GLenum) -> ~str {
    unsafe {
        let llstr = glGetString(which);
        if !ptr::is_null(llstr) {
            return from_c_str(llstr as *c_char);
        } else {
            return ~"";
        }
    }
}

pub fn get_shader_iv(shader: GLuint, pname: GLenum) -> GLint {
    unsafe {
        let result: GLint = 0 as GLint;
        glGetShaderiv(shader, pname, to_unsafe_ptr(&result));
        return result;
    }
}

pub fn get_uniform_location(program: GLuint, name: ~str) -> c_int {
    unsafe {
        do as_c_str(name) |name_bytes| {
            glGetUniformLocation(program, name_bytes as *GLchar)
        }
    }
}

pub fn is_buffer(buffer: GLuint) -> bool {
  unsafe {
    glIsBuffer(buffer) > 0
  }
}

pub fn is_enabled(cap: GLenum) -> bool {
  unsafe {
    glIsEnabled(cap) > 0
  }
}

pub fn is_framebuffer(framebuffer: GLuint) -> bool {
  unsafe {
    glIsFramebuffer(framebuffer) > 0
  }
}

pub fn is_program(program: GLuint) -> bool {
  unsafe {
    glIsProgram(program) > 0
  }
}

pub fn is_renderbuffer(renderbuffer: GLuint) -> bool {
  unsafe {
    glIsRenderbuffer(renderbuffer) > 0
  }
}

pub fn is_shader(shader: GLuint) -> bool {
  unsafe {
    glIsShader(shader) > 0
  }
}

pub fn is_texture(texture: GLuint) -> bool {
  unsafe {
    glIsTexture(texture) > 0
  }
}

pub fn line_width(width: GLfloat) {
  unsafe {
    glLineWidth(width);
  }
}

pub fn link_program(program: GLuint) {
    unsafe {
        return glLinkProgram(program);
    }
}

pub fn pixel_store_i(pname: GLenum, param: GLint) {
    unsafe {
        glPixelStorei(pname, param);
    }
}

pub fn polygon_mode(face: GLenum, mode: GLenum) {
    unsafe {
        glPolygonMode(face, mode);
    }
}

pub fn shader_source(shader: GLuint, strings: &[~[u8]]) {
    unsafe {
        let pointers = strings.map(|string| to_ptr(*string));
        let lengths = strings.map(|string| string.len() as GLint);
        glShaderSource(shader, pointers.len() as GLsizei,
                           to_ptr(pointers) as **GLchar, to_ptr(lengths));
        destroy(lengths);
        destroy(pointers);
    }
}

// FIXME: Does not verify buffer size -- unsafe!
pub fn tex_image_2d(target: GLenum,
                    level: GLint,
                    internal_format: GLint,
                    width: GLsizei,
                    height: GLsizei,
                    border: GLint,
                    format: GLenum,
                    ty: GLenum,
                    opt_data: Option<&[u8]>) {
    match opt_data {
        Some(data) => {
            unsafe {
                let pdata = transmute(to_ptr(data));
                glTexImage2D(target, level, internal_format, width, height, border, format, ty,
                                 pdata);
            }
        }
        None => {
            unsafe {
                glTexImage2D(target, level, internal_format, width, height, border, format, ty,
                                 ptr::null());
            }
        }
    }
}

// FIXME: Does not verify buffer size -- unsafe!
pub fn tex_sub_image_2d(target: GLenum,
                        level: GLint,
                        xoffset: GLint,
                        yoffset: GLint,
                        width: GLsizei,
                        height: GLsizei,
                        format: GLenum,
                        ty: GLenum,
                        opt_data: Option<&[u8]>) {
    match opt_data {
        Some(data) => {
            unsafe {
                let pdata = transmute(to_ptr(data));
                glTexSubImage2D(target, level, xoffset, yoffset, width, height, format, ty,
                                   pdata);
            }
        }
        None => {
            unsafe {
                glTexSubImage2D(target, level, xoffset, yoffset, width, height, format, ty,
                                   ptr::null());
            }
        }
    }
}

pub fn tex_parameter_i(target: GLenum, pname: GLenum, param: GLint) {
    unsafe {
        glTexParameteri(target, pname, param);
    }
}

pub fn uniform_1f(location: GLint, x: GLfloat) {
    unsafe {
        glUniform1f(location, x);
    }
}

pub fn uniform_1i(location: GLint, x: GLint) {
    unsafe {
        glUniform1i(location, x);
    }
}

pub fn uniform_2f(location: GLint, x: GLfloat, y: GLfloat) {
    unsafe {
        glUniform2f(location, x, y);
    }
}

pub fn uniform_3f(location: GLint, x: GLfloat, y: GLfloat, z: GLfloat) {
    unsafe {
        glUniform3f(location, x, y, z);
    }
}

pub fn uniform_4f(location: GLint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) {
    unsafe {
        glUniform4f(location, x, y, z, w);
    }
}

pub fn uniform_matrix_4fv(location: GLint, transpose: bool, value: &[f32]) {
    unsafe {
        glUniformMatrix4fv(location,
                               1 as GLsizei,
                               transpose as GLboolean,
                               cast::transmute(&value[0]));
    }
}

pub fn use_program(program: GLuint) {
    unsafe {
        glUseProgram(program);
    }
}

pub fn validate_program(program: GLuint) {
    unsafe {
        glValidateProgram(program);
    }
}

pub fn vertex_attrib_pointer_f32(index: GLuint,
                                 size: GLint,
                                 normalized: bool,
                                 stride: GLsizei,
                                 offset: GLuint) {
    unsafe {
        glVertexAttribPointer(index,
                                  size,
                                  FLOAT,
                                  normalized as GLboolean,
                                  stride,
                                  transmute(offset as uint));
    }
}

pub fn vertex_attrib_pointer_i8(index: GLuint,
                                 size: GLint,
                                 normalized: bool,
                                 stride: GLsizei,
                                 offset: GLuint) {
    unsafe {
        glVertexAttribPointer(index,
                                  size,
                                  BYTE,
                                  normalized as GLboolean,
                                  stride,
                                  transmute(offset as uint));
    }
}

pub fn vertex_attrib_pointer_i32(index: GLuint,
                                 size: GLint,
                                 normalized: bool,
                                 stride: GLsizei,
                                 offset: GLuint) {
    unsafe {
        glVertexAttribPointer(index,
                                  size,
                                  INT,
                                  normalized as GLboolean,
                                  stride,
                                  transmute(offset as uint));
    }
}

pub fn vertex_attrib_pointer_u8(index: GLuint,
                                 size: GLint,
                                 normalized: bool,
                                 stride: GLsizei,
                                 offset: GLuint) {
    unsafe {
        glVertexAttribPointer(index,
                                  size,
                                  UNSIGNED_BYTE,
                                  normalized as GLboolean,
                                  stride,
                                  transmute(offset as uint));
    }
}

#[cfg(not(target_os="macos"))]
#[cfg(not(mac_10_6), not(mac_10_7))]
pub fn vertex_attrib_divisor(index: GLuint, divisor: GLuint) {
    unsafe {
        glVertexAttribDivisor(index, divisor);
    }
}

pub fn viewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    unsafe {
        glViewport(x, y, width, height);
    }
}

// Apple extensions
#[cfg(target_os="macos")]
pub mod apple {
    use super::{GLenum, GLsizei};
    use core::cast::transmute;
    use core::vec::raw::to_ptr;

    pub unsafe fn texture_range(target: GLenum, buffer: &[u8]) {
        super::glTextureRangeAPPLE(target, buffer.len() as GLsizei, transmute(to_ptr(buffer)));
    }
}

#[nolink]
extern {

// Lower-level API

pub fn glActiveTexture(texture: GLenum);

pub fn glAttachShader(program: GLuint, shader: GLuint);

pub fn glBindAttribLocation(program: GLuint, index: GLuint, name: *GLchar);

pub fn glBindBuffer(target: GLenum, buffer: GLuint);

pub fn glBindFramebuffer(target: GLenum, framebuffer: GLuint);

pub fn glBindRenderbuffer(target: GLenum, renderbuffer: GLuint);

pub fn glBindTexture(target: GLenum, texture: GLuint);

#[cfg(not(target_os="macos"))] #[cfg(not( mac_10_6))]
pub fn glBindVertexArray(array: GLuint);

pub fn glBlendColor(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf);

pub fn glBlendEquation(mode: GLenum);

pub fn glBlendEquationSeparate(modeRGB: GLenum, modeAlpha: GLenum);

pub fn glBlendFunc(sfactor: GLenum, dfactor: GLenum);

pub fn glBlendFuncSeparate(srcRGB: GLenum, dstRGB: GLenum, srcAlpha: GLenum, dstAlpha: GLenum);

pub fn glBufferData(target: GLenum, size: GLsizeiptr, data: *GLvoid, usage: GLenum);

pub fn glBufferSubData(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *GLvoid);

pub fn glCheckFramebufferStatus(target: GLenum) -> GLenum;

pub fn glClear(mask: GLbitfield);

pub fn glClearColor(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf);

// Unsupported on Mac:
//fn glClearDepthf(depth: GLclampf);

pub fn glClearStencil(s: GLint);

pub fn glColorMask(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean);

pub fn glCompileShader(shader: GLuint);

pub fn glCompressedTexImage2D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *GLvoid);

pub fn glCompressedTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *GLvoid);

pub fn glCopyTexImage2D(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint);

pub fn glCopyTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

pub fn glCreateProgram() -> GLuint;

pub fn glCreateShader(_type: GLenum) -> GLuint;

pub fn glCullFace(mode: GLenum);

pub fn glDeleteBuffers(n: GLsizei, buffers: *GLuint);

pub fn glDeleteFramebuffers(n: GLsizei, framebuffers: *GLuint);

pub fn glDeleteProgram(program: GLuint);

pub fn glDeleteRenderbuffers(n: GLsizei, renderbuffers: *GLuint);

pub fn glDeleteShader(shader: GLuint);

pub fn glDeleteTextures(n: GLsizei, textures: *GLuint);

pub fn glDepthFunc(func: GLenum);

pub fn glDepthMask(flag: GLboolean);

// Unsupported on Mac:
//fn glDepthRangef(zNear: GLclampf, zFar: GLclampf);

pub fn glDetachShader(program: GLuint, shader: GLuint);

pub fn glDisable(cap: GLenum);

pub fn glDisableVertexAttribArray(index: GLuint);

pub fn glDrawArrays(mode: GLenum, first: GLint, count: GLsizei);

pub fn glDrawElements(mode: GLenum, count: GLsizei, _type: GLenum, indices: *GLvoid);

#[cfg(not(target_os="macos"))]
#[cfg(not(mac_10_6))]
pub fn glDrawArraysInstanced(mode: GLenum, first: GLint, count: GLsizei, primcount: GLsizei);

#[cfg(not(target_os="macos"))]
#[cfg(not(mac_10_6))]
pub fn glDrawElementsInstanced(mode: GLenum, count: GLsizei, _type: GLenum, indices: *GLvoid, primcount: GLsizei);

pub fn glEnable(cap: GLenum);

pub fn glEnableVertexAttribArray(index: GLuint);

pub fn glFinish();

pub fn glFlush();

pub fn glFramebufferRenderbuffer(target: GLenum, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint);

pub fn glFramebufferTexture2D(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint);

pub fn glFrontFace(mode: GLenum);

pub fn glGenBuffers(n: GLsizei, buffers: *GLuint);

pub fn glGenerateMipmap(target: GLenum);

pub fn glGenFramebuffers(n: GLsizei, framebuffers: *GLuint);

pub fn glGenRenderbuffers(n: GLsizei, renderbuffers: *GLuint);

pub fn glGenTextures(n: GLsizei, textures: *GLuint);

#[cfg(not(target_os="macos"))] #[cfg(not( mac_10_6))]
pub fn glGenVertexArrays(n: GLsizei, arrays: *GLuint);

pub fn glGetActiveAttrib(program: GLuint, index: GLuint, bufsize: GLsizei, length: *GLsizei, size: *GLint, _type: *GLenum, name: *GLchar);

pub fn glGetActiveUniform(program: GLuint, index: GLuint, bufsize: GLsizei, length: *GLsizei, size: *GLint, _type: *GLenum, name: *GLchar);

pub fn glGetAttachedShaders(program: GLuint, maxcount: GLsizei, count: *GLsizei, shaders: *GLuint);

pub fn glGetAttribLocation(program: GLuint, name: *GLchar) -> c_int;

pub fn glGetBooleanv(pname: GLenum, params: *GLboolean);

pub fn glGetBufferParameteriv(target: GLenum, pname: GLenum, params: *GLint);

pub fn glGetError() -> GLenum;

pub fn glGetFloatv(pname: GLenum, params: *GLfloat);

pub fn glGetFramebufferAttachmentParameteriv(target: GLenum, attachment: GLenum, pname: GLenum, params: *GLint);

pub fn glGetIntegerv(pname: GLenum, params: *GLint);

pub fn glGetProgramiv(program: GLuint, pname: GLenum, params: *GLint);

pub fn glGetProgramInfoLog(program: GLuint, bufsize: GLsizei, length: *GLsizei, infolog: *GLchar);

pub fn glGetRenderbufferParameteriv(target: GLenum, pname: GLenum, params: *GLint);

pub fn glGetShaderiv(shader: GLuint, pname: GLenum, params: *GLint);

pub fn glGetShaderInfoLog(shader: GLuint, bufsize: GLsizei, length: *GLsizei, infolog: *GLchar);

// Unsupported on Mac:
//fn glGetShaderPrecisionFormat(shadertype: GLenum, precisiontype: GLenum, range: *GLint, precision: *GLint);

pub fn glGetShaderSource(shader: GLuint, bufsize: GLsizei, length: *GLsizei, source: *GLchar);

pub fn glGetString(name: GLenum) -> *GLubyte;

pub fn glGetTexParameterfv(target: GLenum, pname: GLenum, params: *GLfloat);

pub fn glGetTexParameteriv(target: GLenum, pname: GLenum, params: *GLint);

pub fn glGetUniformfv(program: GLuint, location: GLint, params: *GLfloat);

pub fn glGetUniformiv(program: GLuint, location: GLint, params: *GLint);

pub fn glGetUniformLocation(program: GLuint, name: *GLchar) -> c_int;

pub fn glGetVertexAttribfv(index: GLuint, pname: GLenum, params: *GLfloat);

pub fn glGetVertexAttribiv(index: GLuint, pname: GLenum, params: *GLint);

pub fn glGetVertexAttribPointerv(index: GLuint, pname: GLenum, pointer: **GLvoid);

pub fn glHint(target: GLenum, mode: GLenum);

pub fn glIsBuffer(buffer: GLuint) -> GLboolean;

pub fn glIsEnabled(cap: GLenum) -> GLboolean;

pub fn glIsFramebuffer(framebuffer: GLuint) -> GLboolean;

pub fn glIsProgram(program: GLuint) -> GLboolean;

pub fn glIsRenderbuffer(renderbuffer: GLuint) -> GLboolean;

pub fn glIsShader(shader: GLuint) -> GLboolean;

pub fn glIsTexture(texture: GLuint) -> GLboolean;

pub fn glLineWidth(width: GLfloat);

pub fn glLinkProgram(program: GLuint);

pub fn glPixelStorei(pname: GLenum, param: GLint);

pub fn glPolygonOffset(factor: GLfloat, units: GLfloat);

pub fn glPolygonMode(face: GLenum, mode: GLenum);

pub fn glReadPixels(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, _type: GLenum, pixels: *GLvoid);

// Unsupported on Mac:
// fn glReleaseShaderCompiler();

pub fn glRenderbufferStorage(target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei);

pub fn glSampleCoverage(value: GLclampf, invert: GLboolean);

pub fn glScissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei);

// Unsupported on Mac:
//fn glShaderBinary(n: GLsizei, shaders: *GLuint, binaryformat: GLenum, binary: *GLvoid, length: GLsizei);

pub fn glShaderSource(shader: GLuint, count: GLsizei, string: **GLchar, length: *GLint);

pub fn glStencilFunc(func: GLenum, reference: GLint, mask: GLuint);

pub fn glStencilFuncSeparate(face: GLenum, func: GLenum, reference: GLint, mask: GLuint);

pub fn glStencilMask(mask: GLuint);

pub fn glStencilMaskSeparate(face: GLenum, mask: GLuint);

pub fn glStencilOp(_fail: GLenum, zfail: GLenum, zpass: GLenum);

pub fn glStencilOpSeparate(face: GLenum, _fail: GLenum, zfail: GLenum, zpass: GLenum);

pub fn glTexImage2D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, _type: GLenum, pixels: *GLvoid);

pub fn glTexParameterf(target: GLenum, pname: GLenum, param: GLfloat);

pub fn glTexParameterfv(target: GLenum, pname: GLenum, params: *GLfloat);

pub fn glTexParameteri(target: GLenum, pname: GLenum, param: GLint);

pub fn glTexParameteriv(target: GLenum, pname: GLenum, params: *GLint);

pub fn glTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, _type: GLenum, pixels: *GLvoid);

pub fn glUniform1f(location: GLint, x: GLfloat);

pub fn glUniform1fv(location: GLint, count: GLsizei, v: *GLfloat);

pub fn glUniform1i(location: GLint, x: GLint);

pub fn glUniform1iv(location: GLint, count: GLsizei, v: *GLint);

pub fn glUniform2f(location: GLint, x: GLfloat, y: GLfloat);

pub fn glUniform2fv(location: GLint, count: GLsizei, v: *GLfloat);

pub fn glUniform2i(location: GLint, x: GLint, y: GLint);

pub fn glUniform2iv(location: GLint, count: GLsizei, v: *GLint);

pub fn glUniform3f(location: GLint, x: GLfloat, y: GLfloat, z: GLfloat);

pub fn glUniform3fv(location: GLint, count: GLsizei, v: *GLfloat);

pub fn glUniform3i(location: GLint, x: GLint, y: GLint, z: GLint);

pub fn glUniform3iv(location: GLint, count: GLsizei, v: *GLint);

pub fn glUniform4f(location: GLint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

pub fn glUniform4fv(location: GLint, count: GLsizei, v: *GLfloat);

pub fn glUniform4i(location: GLint, x: GLint, y: GLint, z: GLint, w: GLint);

pub fn glUniform4iv(location: GLint, count: GLsizei, v: *GLint);

pub fn glUniformMatrix2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat);

pub fn glUniformMatrix3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat);

pub fn glUniformMatrix4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat);

pub fn glUseProgram(program: GLuint);

pub fn glValidateProgram(program: GLuint);

pub fn glVertexAttrib1f(indx: GLuint, x: GLfloat);

pub fn glVertexAttrib1fv(indx: GLuint, values: *GLfloat);

pub fn glVertexAttrib2f(indx: GLuint, x: GLfloat, y: GLfloat);

pub fn glVertexAttrib2fv(indx: GLuint, values: *GLfloat);

pub fn glVertexAttrib3f(indx: GLuint, x: GLfloat, y: GLfloat, z: GLfloat);

pub fn glVertexAttrib3fv(indx: GLuint, values: *GLfloat);

pub fn glVertexAttrib4f(indx: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

pub fn glVertexAttrib4fv(indx: GLuint, values: *GLfloat);

pub fn glVertexAttribPointer(indx: GLuint, size: GLint, _type: GLenum, normalized: GLboolean, stride: GLsizei, ptr: *GLvoid);

#[cfg(not(target_os="macos"))]
#[cfg(not(mac_10_6), not(mac_10_7))]
pub fn glVertexAttribDivisor(indx: GLuint, divisor: GLuint);

pub fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei);


}

// Apple extensions
#[cfg(target_os="macos")]
extern {

pub fn glTextureRangeAPPLE(target: GLenum, length: GLsizei, pointer: *GLvoid);

}

