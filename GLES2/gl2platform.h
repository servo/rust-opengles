// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#ifndef GL2PLATFORM_H
#define GL2PLATFORM_H

#include <sys/types.h>
#include <stdint.h>

#define GL_APICALL
#define GL_APIENTRY

typedef int8_t khronos_int8_t;
typedef uint8_t khronos_uint8_t;
typedef float khronos_float_t;
typedef int32_t khronos_int32_t;
typedef intptr_t khronos_intptr_t;
typedef ssize_t khronos_ssize_t;

#endif

