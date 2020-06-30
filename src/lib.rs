// Copyright © 2020
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

extern crate libc;

use std::ffi::c_void;

#[repr(C)]
struct CDisplay {
    fd: i32,
    connector_id: u32,
    mode: i32,
    width: u32,
    height: u32,
    crtc_id: u32,
}

pub struct Display {
    c_display: *mut CDisplay,
}

impl Display {
    pub fn new() -> Option<Display> {
        let c_display = unsafe { go2_display_create() };
        if c_display.is_null() {
            None
        } else {
            Some(Display { c_display })
        }
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        unsafe { go2_display_destroy(self.c_display) };
    }
}

#[repr(C)]
pub struct CSurface {
    display: *mut CDisplay,
    gem_handle: u32,
    size: u64,
    width: i32,
    height: i32,
    stride: i32,
    format: u32,
    prime_fd: i32,
    is_mapped: bool,
    map: *mut u8,
}

struct Surface {
    c_surface: *mut CSurface,
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe { go2_surface_destroy(self.c_surface) };
    }
}

#[repr(C)]
pub struct ContextAttributes {
    pub major: i32,
    pub minor: i32,
    pub red_bits: i32,
    pub green_bits: i32,
    pub blue_bits: i32,
    pub alpha_bits: i32,
    pub depth_bits: i32,
    pub stencil_bits: i32,
}

#[repr(C)]
struct BufferSurfacePair {
    gbm_buffer: *mut c_void,
    surface: *mut CSurface,
}

#[repr(C)]
struct CContext {
    c_display: *mut CDisplay,
    width: i32,
    height: i32,
    attributes: ContextAttributes,
    gbm_device: *mut c_void,
    egl_display: *mut c_void,
    gbm_surface: *mut c_void,
    egl_surface: *mut c_void,
    egl_context: *mut c_void,
    drm_four_cc: u32,
    buffer_map: [BufferSurfacePair; 3],
    buffer_count: i32,
}

pub struct Context {
    c_context: *mut CContext,
}

impl Context {
    pub fn new(
        display: &Display,
        width: i32,
        height: i32,
        attributes: &ContextAttributes,
    ) -> Option<Context> {
        let c_context = unsafe { go2_context_create(display.c_display, width, height, attributes) };
        if c_context.is_null() {
            None
        } else {
            Some(Context { c_context })
        }
    }

    pub fn make_current(&self) {
        unsafe { go2_context_make_current(self.c_context) };
    }

    pub fn swap_buffers(&self) {
        unsafe { go2_context_swap_buffers(self.c_context) };
    }

    pub fn surface_lock(&self) -> *mut CSurface {
        unsafe { go2_context_surface_lock(self.c_context) }
    }

    pub fn surface_unlock(&self, surface: *mut CSurface) {
        unsafe { go2_context_surface_unlock(self.c_context, surface) };
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { go2_context_destroy(self.c_context) };
    }
}

#[repr(C)]
struct CPresenter {
    display: *mut CDisplay,
    format: u32,
    background_color: u32,
    free_frame_buffers: *mut c_void,
    used_frame_buffers: *mut c_void,
}

pub struct Presenter {
    c_presenter: *mut CPresenter,
}

impl Presenter {
    pub fn new(display: &Display, format: u32, background_color: u32) -> Option<Presenter> {
        let c_presenter =
            unsafe { go2_presenter_create(display.c_display, format, background_color) };
        if c_presenter.is_null() {
            None
        } else {
            Some(Presenter { c_presenter })
        }
    }

    pub fn post(
        &self,
        surface: *mut CSurface,
        src_x: i32,
        src_y: i32,
        src_width: i32,
        src_height: i32,
        dst_x: i32,
        dst_y: i32,
        dst_width: i32,
        dst_height: i32,
        rotation: i32,
    ) {
        unsafe {
            go2_presenter_post(
                self.c_presenter,
                surface,
                src_x,
                src_y,
                src_width,
                src_height,
                dst_x,
                dst_y,
                dst_width,
                dst_height,
                rotation,
            )
        };
    }
}

impl Drop for Presenter {
    fn drop(&mut self) {
        unsafe { go2_presenter_destroy(self.c_presenter) };
    }
}

#[link(name = "EGL")]
extern "C" {
    pub fn eglGetProcAddress(procname: *const libc::c_char) -> extern "C" fn();
}

#[link(name = "go2")]
extern "C" {
    fn go2_display_create() -> *mut CDisplay;
    fn go2_display_destroy(display: *mut CDisplay);

    fn go2_surface_destroy(surface: *mut CSurface);

    fn go2_context_create(
        display: *mut CDisplay,
        width: i32,
        height: i32,
        attributes: *const ContextAttributes,
    ) -> *mut CContext;
    fn go2_context_destroy(context: *mut CContext);

    fn go2_context_make_current(context: *mut CContext);
    fn go2_context_swap_buffers(context: *mut CContext);
    fn go2_context_surface_lock(context: *mut CContext) -> *mut CSurface;
    fn go2_context_surface_unlock(context: *mut CContext, surface: *mut CSurface);

    fn go2_presenter_create(
        display: *mut CDisplay,
        format: u32,
        background_color: u32,
    ) -> *mut CPresenter;
    fn go2_presenter_destroy(presenter: *mut CPresenter);

    fn go2_presenter_post(
        presenter: *mut CPresenter,
        surface: *mut CSurface,
        src_x: i32,
        src_y: i32,
        src_width: i32,
        src_height: i32,
        dst_x: i32,
        dst_y: i32,
        dst_width: i32,
        dst_height: i32,
        rotation: i32,
    );
}
