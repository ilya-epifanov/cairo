// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#![allow(non_camel_case_types)]
#![cfg_attr(feature = "cargo-clippy", allow(unreadable_literal, write_literal))]

extern crate libc;

#[cfg(feature = "use_glib")]
extern crate glib_sys as glib_ffi;

#[cfg(any(feature = "xlib", feature = "dox"))]
extern crate x11;

#[cfg(windows)]
extern crate winapi as winapi_orig;

#[cfg(windows)]
pub mod winapi {
    pub use winapi_orig::shared::windef::HDC;
}

#[cfg(all(not(windows), feature = "dox"))]
pub mod winapi {
   use libc::c_void;

   #[repr(C)]
   pub struct HDC(c_void);
}

use libc::{c_void, c_int, c_uint, c_char, c_uchar, c_double, c_ulong};

#[cfg(any(feature = "xlib", feature = "dox"))]
use x11::xlib;

pub type cairo_antialias_t = c_int;
pub type cairo_content_t = c_int;
pub type cairo_extend_t = c_int;
pub type cairo_fill_rule_t = c_int;
pub type cairo_filter_t = c_int;
pub type cairo_font_slant_t = c_int;
pub type cairo_font_type_t = c_int;
pub type cairo_font_weight_t = c_int;
pub type cairo_format_t = c_int;
pub type cairo_hint_metrics_t = c_int;
pub type cairo_hint_style_t = c_int;
pub type cairo_line_cap_t = c_int;
pub type cairo_line_join_t = c_int;
pub type cairo_operator_t = c_int;
pub type cairo_pattern_type_t = c_int;
pub type cairo_path_data_type_t = c_int;
pub type cairo_region_overlap_t = c_int;
pub type cairo_status_t = c_int;
pub type cairo_subpixel_order_t = c_int;
pub type cairo_surface_type_t = c_int;
pub type cairo_svg_unit_t = c_int;
pub type cairo_text_cluster_flags_t = c_int;

#[cfg(any(feature = "pdf", feature = "dox"))]
pub type cairo_pdf_version_t = c_int;
#[cfg(any(feature = "svg", feature = "dox"))]
pub type cairo_svg_version_t = c_int;
#[cfg(any(feature = "ps", feature = "dox"))]
pub type cairo_ps_level_t = c_int;

pub type cairo_mesh_corner_t = c_uint;

macro_rules! debug_impl {
    ($name:ty) => {
        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{} @ {:?}", stringify!($name), self as *const _)
            }
        }
    }
}

#[repr(C)]
pub struct cairo_t(c_void);
debug_impl!(cairo_t);

#[repr(C)]
pub struct cairo_surface_t(c_void);
debug_impl!(cairo_surface_t);

#[repr(C)]
#[derive(Copy,Clone,Debug)]
pub struct cairo_pattern_t(u8);

#[cfg(any(feature = "xcb", feature = "dox"))]
#[repr(C)]
pub struct cairo_device_t(c_void);
#[cfg(any(feature = "xcb", feature = "dox"))]
debug_impl!(cairo_device_t);

#[cfg(any(feature = "xcb", feature = "dox"))]
#[repr(C)]
pub struct xcb_connection_t(c_void);
#[cfg(any(feature = "xcb", feature = "dox"))]
debug_impl!(xcb_connection_t);

#[cfg(any(feature = "xcb", feature = "dox"))]
pub type xcb_drawable_t = u32;

#[cfg(any(feature = "xcb", feature = "dox"))]
pub type xcb_pixmap_t = u32;

#[cfg(any(feature = "xcb", feature = "dox"))]
#[repr(C)]
pub struct xcb_visualtype_t(c_void);
#[cfg(any(feature = "xcb", feature = "dox"))]
debug_impl!(xcb_visualtype_t);

#[cfg(any(feature = "xcb", feature = "dox"))]
#[repr(C)]
pub struct xcb_screen_t(c_void);
#[cfg(any(feature = "xcb", feature = "dox"))]
debug_impl!(xcb_screen_t);

#[cfg(any(feature = "xcb", feature = "dox"))]
#[repr(C)]
pub struct xcb_render_pictforminfo_t(c_void);
#[cfg(any(feature = "xcb", feature = "dox"))]
debug_impl!(xcb_render_pictforminfo_t);

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct cairo_rectangle_t {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct cairo_rectangle_int_t {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct cairo_rectangle_list_t {
    pub status: cairo_status_t,
    pub rectangles: *mut cairo_rectangle_t,
    pub num_rectangles: c_int,
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct cairo_path_t {
    pub status: cairo_status_t,
    pub data: *mut cairo_path_data,
    pub num_data: c_int,
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct cairo_path_data_header{
    pub data_type: cairo_path_data_type_t,
    pub length:    c_int,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union cairo_path_data{
    pub header: cairo_path_data_header,
    pub point: [f64; 2],
}
#[repr(C)]
pub struct cairo_glyph_t(c_void);
debug_impl!(cairo_glyph_t);

#[repr(C)]
pub struct cairo_region_t(c_void);
debug_impl!(cairo_region_t);

#[repr(C)]
pub struct cairo_font_face_t(c_void);
debug_impl!(cairo_font_face_t);

#[repr(C)]
pub struct cairo_scaled_font_t(c_void);
debug_impl!(cairo_scaled_font_t);

#[repr(C)]
pub struct cairo_font_options_t(c_void);
debug_impl!(cairo_font_options_t);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct FontExtents {
    pub ascent: c_double,
    pub descent: c_double,
    pub height: c_double,
    pub max_x_advance: c_double,
    pub max_y_advance: c_double,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Glyph {
    pub index: c_ulong,
    pub x: c_double,
    pub y: c_double,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TextCluster {
    pub num_bytes: c_int,
    pub num_glyphs: c_int,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TextExtents {
    pub x_bearing: c_double,
    pub y_bearing: c_double,
    pub width: c_double,
    pub height: c_double,
    pub x_advance: c_double,
    pub y_advance: c_double,
}
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix {
    pub xx: c_double,
    pub yx: c_double,

    pub xy: c_double,
    pub yy: c_double,

    pub x0: c_double,
    pub y0: c_double,
}
#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct cairo_user_data_key_t {
    pub unused: c_int,
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct cairo_bool_t{
    value: c_int
}

impl cairo_bool_t {
    pub fn as_bool(&self) -> bool{
        self.value != 0
    }
}

pub type CGContextRef = *mut c_void;

pub type cairo_destroy_func_t = Option<unsafe extern fn (*mut c_void)>;
pub type cairo_read_func_t = Option<unsafe extern fn (*mut c_void, *mut c_uchar, c_uint) -> cairo_status_t>;
pub type cairo_write_func_t = Option<unsafe extern fn (*mut c_void, *mut c_uchar, c_uint) -> cairo_status_t>;

extern "C" {
    //CAIRO CONTEXT
    pub fn cairo_create (target: *mut cairo_surface_t) -> *mut cairo_t;
    pub fn cairo_reference (cr: *mut cairo_t) -> *mut cairo_t;
    pub fn cairo_destroy (cr: *mut cairo_t);
    pub fn cairo_status (cr: *mut cairo_t) -> cairo_status_t;
    pub fn cairo_save (cr: *mut cairo_t);
    pub fn cairo_restore (cr: *mut cairo_t);
    pub fn cairo_get_target (cr: *mut cairo_t) -> *mut cairo_surface_t;
    pub fn cairo_push_group (cr: *mut cairo_t);
    pub fn cairo_push_group_with_content (cr: *mut cairo_t, content: cairo_content_t);
    pub fn cairo_pop_group (cr: *mut cairo_t) -> *mut cairo_pattern_t;
    pub fn cairo_pop_group_to_source (cr: *mut cairo_t);
    pub fn cairo_get_group_target (cr: *mut cairo_t) -> *mut cairo_surface_t;
    pub fn cairo_set_source_rgb (cr: *mut cairo_t, red: c_double, green: c_double, blue: c_double);
    pub fn cairo_set_source_rgba (cr: *mut cairo_t, red: c_double, green: c_double, blue: c_double, alpha: c_double);
    pub fn cairo_set_source (cr: *mut cairo_t, source: *mut cairo_pattern_t);
    pub fn cairo_set_source_surface (cr: *mut cairo_t, surface: *mut cairo_surface_t, x: c_double, y: c_double);
    pub fn cairo_get_source (cr: *mut cairo_t) -> *mut cairo_pattern_t;
    pub fn cairo_set_antialias (cr: *mut cairo_t, antialias: cairo_antialias_t);
    pub fn cairo_get_antialias (cr: *mut cairo_t) -> cairo_antialias_t;
    pub fn cairo_set_dash (cr: *mut cairo_t, dashes : *const c_double, num_dashes: c_int, offset: c_double);
    pub fn cairo_get_dash_count (cr: *mut cairo_t) -> c_int;
    pub fn cairo_get_dash (cr: *mut cairo_t, dashes: *mut c_double, offset: *mut c_double);
    pub fn cairo_set_fill_rule (cr: *mut cairo_t, fill_rule: cairo_fill_rule_t);
    pub fn cairo_get_fill_rule (cr: *mut cairo_t) -> cairo_fill_rule_t;
    pub fn cairo_set_line_cap (cr: *mut cairo_t, line_cap: cairo_line_cap_t);
    pub fn cairo_get_line_cap (cr: *mut cairo_t) -> cairo_line_cap_t;
    pub fn cairo_set_line_join (cr: *mut cairo_t, line_join: cairo_line_join_t);
    pub fn cairo_get_line_join (cr: *mut cairo_t) -> cairo_line_join_t;
    pub fn cairo_set_line_width (cr: *mut cairo_t, width: c_double);
    pub fn cairo_get_line_width (cr: *mut cairo_t) -> c_double;
    pub fn cairo_set_miter_limit (cr: *mut cairo_t, limit: c_double);
    pub fn cairo_get_miter_limit (cr: *mut cairo_t) -> c_double;
    pub fn cairo_set_operator (cr: *mut cairo_t, op: cairo_operator_t);
    pub fn cairo_get_operator (cr: *mut cairo_t) -> cairo_operator_t;
    pub fn cairo_set_tolerance (cr: *mut cairo_t, tolerance: c_double);
    pub fn cairo_get_tolerance (cr: *mut cairo_t) -> c_double;
    pub fn cairo_clip (cr: *mut cairo_t);
    pub fn cairo_clip_preserve (cr: *mut cairo_t);
    pub fn cairo_clip_extents (cr: *mut cairo_t, x1: *mut c_double, y1: *mut c_double, x2: *mut c_double, y2: *mut c_double);
    pub fn cairo_in_clip (cr: *mut cairo_t, x: c_double, y: c_double) -> cairo_bool_t;
    pub fn cairo_reset_clip (cr: *mut cairo_t);
    pub fn cairo_rectangle_list_destroy (rectangle_list: *mut cairo_rectangle_list_t);
    pub fn cairo_copy_clip_rectangle_list (cr: *mut cairo_t) -> *mut cairo_rectangle_list_t;
    pub fn cairo_fill (cr: *mut cairo_t);
    pub fn cairo_fill_preserve (cr: *mut cairo_t);
    pub fn cairo_fill_extents (cr: *mut cairo_t, x1: *mut c_double, y1: *mut c_double, x2: *mut c_double, y2: *mut c_double);
    pub fn cairo_in_fill (cr: *mut cairo_t, x: c_double, y: c_double) -> cairo_bool_t;
    pub fn cairo_mask (cr: *mut cairo_t, pattern: *mut cairo_pattern_t);
    pub fn cairo_mask_surface (cr: *mut cairo_t, surface: *mut cairo_surface_t, surface_x: c_double, surface_y: c_double);
    pub fn cairo_paint (cr: *mut cairo_t);
    pub fn cairo_paint_with_alpha (cr: *mut cairo_t, alpha: c_double);
    pub fn cairo_stroke (cr: *mut cairo_t);
    pub fn cairo_stroke_preserve (cr: *mut cairo_t);
    pub fn cairo_stroke_extents (cr: *mut cairo_t, x1: *mut c_double, y1: *mut c_double, x2: *mut c_double, y2: *mut c_double);
    pub fn cairo_in_stroke (cr: *mut cairo_t, x: c_double, y: c_double) -> cairo_bool_t;
    pub fn cairo_copy_page (cr: *mut cairo_t);
    pub fn cairo_show_page (cr: *mut cairo_t);
    pub fn cairo_get_reference_count (cr: *mut cairo_t) -> c_uint;

    //CAIRO UTILS: Error handling
    pub fn cairo_status_to_string (status : cairo_status_t) -> *const c_char;


    //CAIRO PATHS
    pub fn cairo_copy_path(cr: *mut cairo_t) -> *mut cairo_path_t;
    pub fn cairo_copy_path_flat(cr: *mut cairo_t) -> *mut cairo_path_t;
    pub fn cairo_path_destroy(path: *mut cairo_path_t);
    pub fn cairo_append_path(cr: *mut cairo_t, path: *mut cairo_path_t);
    pub fn cairo_has_current_point(cr: *mut cairo_t) -> cairo_bool_t;
    pub fn cairo_get_current_point(cr: *mut cairo_t, x: *mut c_double, y: *mut c_double);
    pub fn cairo_new_path(cr: *mut cairo_t);
    pub fn cairo_new_sub_path(cr: *mut cairo_t);
    pub fn cairo_close_path(cr: *mut cairo_t);
    pub fn cairo_arc(cr: *mut cairo_t, xc: c_double, yc: c_double, radius: c_double, angle1: c_double, angle2: c_double);
    pub fn cairo_arc_negative(cr: *mut cairo_t, xc: c_double, yc: c_double, radius: c_double, angle1: c_double, angle2: c_double);
    pub fn cairo_curve_to(cr: *mut cairo_t, x1: c_double, y1: c_double, x2: c_double, y2: c_double, x3: c_double, y3: c_double);
    pub fn cairo_line_to(cr: *mut cairo_t, x: c_double, y: c_double);
    pub fn cairo_move_to(cr: *mut cairo_t, x: c_double, y: c_double);
    pub fn cairo_rectangle(cr: *mut cairo_t, x: c_double, y: c_double, width: c_double, height: c_double);
    pub fn cairo_glyph_path(cr: *mut cairo_t, glyphs: *const Glyph, num_glyphs: c_int);
    pub fn cairo_text_path(cr: *mut cairo_t, utf8: *const c_char);
    pub fn cairo_rel_curve_to(cr: *mut cairo_t, dx1: c_double, dy1: c_double, dx2: c_double, dy2: c_double, dx3: c_double, dy3: c_double);
    pub fn cairo_rel_line_to(cr: *mut cairo_t, dx: c_double, dy: c_double);
    pub fn cairo_rel_move_to(cr: *mut cairo_t, dx: c_double, dy: c_double);
    pub fn cairo_path_extents(cr: *mut cairo_t, x1: *mut c_double, y1: *mut c_double, x2: *mut c_double, y2: *mut c_double);

    //CAIRO TRANSFORMATIONS
    pub fn cairo_translate(cr: *mut cairo_t, tx: c_double, ty: c_double);
    pub fn cairo_scale(cr: *mut cairo_t, sx: c_double, sy: c_double);
    pub fn cairo_rotate(cr: *mut cairo_t, angle: c_double);
    pub fn cairo_transform(cr: *mut cairo_t, matrix: *const Matrix);
    pub fn cairo_set_matrix(cr: *mut cairo_t, matrix: *const Matrix);
    pub fn cairo_get_matrix(cr: *mut cairo_t, matrix: *mut Matrix);
    pub fn cairo_identity_matrix(cr: *mut cairo_t);
    pub fn cairo_user_to_device(cr: *mut cairo_t, x: *mut c_double, y: *mut c_double);
    pub fn cairo_user_to_device_distance(cr: *mut cairo_t, dx: *mut c_double, dy: *mut c_double);
    pub fn cairo_device_to_user(cr: *mut cairo_t, x: *mut c_double, y: *mut c_double);
    pub fn cairo_device_to_user_distance(cr: *mut cairo_t, dx: *mut c_double, dy: *mut c_double);

    //CAIRO PATTERNS
    pub fn cairo_pattern_add_color_stop_rgb(pattern: *mut cairo_pattern_t, offset: c_double, red: c_double, green: c_double, blue: c_double);
    pub fn cairo_pattern_add_color_stop_rgba(pattern: *mut cairo_pattern_t, offset: c_double, red: c_double, green: c_double, blue: c_double, alpha: c_double);
    pub fn cairo_pattern_get_color_stop_count(pattern: *mut cairo_pattern_t, count: *mut c_int) -> cairo_status_t;
    pub fn cairo_pattern_get_color_stop_rgba(pattern: *mut cairo_pattern_t, index: c_int, offset: *mut c_double, red: *mut c_double, green: *mut c_double, blue: *mut c_double, alpha: *mut c_double) -> cairo_status_t;
    pub fn cairo_pattern_create_rgb(red: c_double, green: c_double, blue: c_double) -> *mut cairo_pattern_t;
    pub fn cairo_pattern_create_rgba(red: c_double, green: c_double, blue: c_double, alpha: c_double) -> *mut cairo_pattern_t;
    pub fn cairo_pattern_get_rgba(pattern: *mut cairo_pattern_t, red: *mut c_double, green: *mut c_double, blue: *mut c_double, alpha: *mut c_double) -> cairo_status_t;
    pub fn cairo_pattern_create_for_surface(surface: *mut cairo_surface_t) -> *mut cairo_pattern_t;
    pub fn cairo_pattern_get_surface(pattern: *mut cairo_pattern_t, surface: *mut *mut cairo_surface_t) -> cairo_status_t;
    pub fn cairo_pattern_create_linear(x0: c_double, y0: c_double, x1: c_double, y1: c_double) -> *mut cairo_pattern_t;
    pub fn cairo_pattern_get_linear_points(pattern: *mut cairo_pattern_t, x0: *mut c_double, y0: *mut c_double, x1: *mut c_double, y1: *mut c_double) -> cairo_status_t;
    pub fn cairo_pattern_create_radial(cx0: c_double, cy0: c_double, radius0: c_double, cx1: c_double, cy1: c_double, radius1: c_double) -> *mut cairo_pattern_t;
    pub fn cairo_pattern_get_radial_circles(pattern: *mut cairo_pattern_t, x0: *mut c_double, y0: *mut c_double, r0: *mut c_double, x1: *mut c_double, y1: *mut c_double, r1: *mut c_double) -> cairo_status_t;
    pub fn cairo_pattern_create_mesh() -> *mut cairo_pattern_t;
    pub fn cairo_mesh_pattern_begin_patch(pattern: *mut cairo_pattern_t);
    pub fn cairo_mesh_pattern_end_patch(pattern: *mut cairo_pattern_t);
    pub fn cairo_mesh_pattern_move_to(pattern: *mut cairo_pattern_t, x: c_double, y: c_double);
    pub fn cairo_mesh_pattern_line_to(pattern: *mut cairo_pattern_t, x: c_double, y: c_double);
    pub fn cairo_mesh_pattern_curve_to(pattern: *mut cairo_pattern_t, x1: c_double, y1: c_double, x2: c_double, y2: c_double, x3: c_double, y3: c_double);
    pub fn cairo_mesh_pattern_set_control_point(pattern: *mut cairo_pattern_t, point_num: cairo_mesh_corner_t, x: c_double, y: c_double);
    pub fn cairo_mesh_pattern_set_corner_color_rgb(pattern: *mut cairo_pattern_t, corner_num: cairo_mesh_corner_t, red: c_double, green: c_double, blue: c_double);
    pub fn cairo_mesh_pattern_set_corner_color_rgba(pattern: *mut cairo_pattern_t, corner_num: cairo_mesh_corner_t, red: c_double, green: c_double, blue: c_double, alpha: c_double);
    pub fn cairo_mesh_pattern_get_patch_count(pattern: *mut cairo_pattern_t, count: *mut c_uint) -> cairo_status_t;
    pub fn cairo_mesh_pattern_get_path(pattern: *mut cairo_pattern_t, patch_num: c_uint) -> *mut cairo_path_t;
    pub fn cairo_mesh_pattern_get_control_point(pattern: *mut cairo_pattern_t, patch_num: c_uint, point_num: cairo_mesh_corner_t, x: *mut c_double, y: *mut c_double) -> cairo_status_t;
    pub fn cairo_mesh_pattern_get_corner_color_rgba(pattern: *mut cairo_pattern_t, patch_num: c_uint, corner_num: cairo_mesh_corner_t, red: *mut c_double, green: *mut c_double, blue: *mut c_double, alpha: *mut c_double) -> cairo_status_t;
    pub fn cairo_pattern_reference(pattern: *mut cairo_pattern_t) -> *mut cairo_pattern_t;
    pub fn cairo_pattern_destroy(pattern: *mut cairo_pattern_t);
    pub fn cairo_pattern_status(pattern: *mut cairo_pattern_t) -> cairo_status_t;
    pub fn cairo_pattern_set_extend(pattern: *mut cairo_pattern_t, extend: cairo_extend_t);
    pub fn cairo_pattern_get_extend(pattern: *mut cairo_pattern_t) -> cairo_extend_t;
    pub fn cairo_pattern_set_filter(pattern: *mut cairo_pattern_t, filter: cairo_filter_t);
    pub fn cairo_pattern_get_filter(pattern: *mut cairo_pattern_t) -> cairo_filter_t;
    pub fn cairo_pattern_set_matrix(pattern: *mut cairo_pattern_t, matrix: *const Matrix);
    pub fn cairo_pattern_get_matrix(pattern: *mut cairo_pattern_t, matrix: *mut Matrix);
    pub fn cairo_pattern_get_type(pattern: *mut cairo_pattern_t) -> cairo_pattern_type_t;
    pub fn cairo_pattern_get_reference_count(pattern: *mut cairo_pattern_t) -> c_uint;
    //pub fn cairo_pattern_set_user_data(pattern: *mut cairo_pattern_t, key: *mut cairo_user_data_key_t, user_data: *mut void, destroy: cairo_destroy_func_t) -> cairo_status_t;
    //pub fn cairo_pattern_get_user_data(pattern: *mut cairo_pattern_t, key: *mut cairo_user_data_key_t) -> *mut void;

    //CAIRO REGIONS
    pub fn cairo_region_create() -> *mut cairo_region_t;
    pub fn cairo_region_create_rectangle(rectangle: *mut cairo_rectangle_int_t) -> *mut cairo_region_t;
    pub fn cairo_region_create_rectangles(rects: *mut cairo_rectangle_int_t, count: c_int) -> *mut cairo_region_t;
    pub fn cairo_region_copy(original: *mut cairo_region_t) -> *mut cairo_region_t;
    pub fn cairo_region_reference(region: *mut cairo_region_t) -> *mut cairo_region_t;
    pub fn cairo_region_destroy(region: *mut cairo_region_t);
    pub fn cairo_region_status(region: *mut cairo_region_t) -> cairo_status_t;
    pub fn cairo_region_get_extents(region: *mut cairo_region_t, extents: *mut cairo_rectangle_int_t);
    pub fn cairo_region_num_rectangles(region: *mut cairo_region_t) -> c_int;
    pub fn cairo_region_get_rectangle(region: *mut cairo_region_t, nth: c_int, rectangle: *mut cairo_rectangle_int_t);
    pub fn cairo_region_is_empty(region: *mut cairo_region_t) -> cairo_bool_t;
    pub fn cairo_region_contains_point(region: *mut cairo_region_t, x: c_int, y: c_int) -> cairo_bool_t;
    pub fn cairo_region_contains_rectangle(region: *mut cairo_region_t, rectangle: *mut cairo_rectangle_int_t) -> cairo_region_overlap_t;
    pub fn cairo_region_equal(a: *mut cairo_region_t, b: *mut cairo_region_t) -> cairo_bool_t;
    pub fn cairo_region_translate(region: *mut cairo_region_t, dx: c_int, dy: c_int);
    pub fn cairo_region_intersect(dst: *mut cairo_region_t, other: *mut cairo_region_t) -> cairo_status_t;
    pub fn cairo_region_intersect_rectangle(dst: *mut cairo_region_t, rectangle: *mut cairo_rectangle_int_t) -> cairo_status_t;
    pub fn cairo_region_subtract(dst: *mut cairo_region_t, other: *mut cairo_region_t) -> cairo_status_t;
    pub fn cairo_region_subtract_rectangle(dst: *mut cairo_region_t, rectangle: *mut cairo_rectangle_int_t) -> cairo_status_t;
    pub fn cairo_region_union(dst: *mut cairo_region_t, other: *mut cairo_region_t) -> cairo_status_t;
    pub fn cairo_region_union_rectangle(dst: *mut cairo_region_t, rectangle: *mut cairo_rectangle_int_t) -> cairo_status_t;
    pub fn cairo_region_xor(dst: *mut cairo_region_t, other: *mut cairo_region_t) -> cairo_status_t;
    pub fn cairo_region_xor_rectangle(dst: *mut cairo_region_t, rectangle: *mut cairo_rectangle_int_t) -> cairo_status_t;

    //text
    pub fn cairo_select_font_face(cr: *mut cairo_t, family: *const c_char, slant: cairo_font_slant_t, weight: cairo_font_weight_t);
    pub fn cairo_set_font_size(cr: *mut cairo_t, size: c_double);
    pub fn cairo_set_font_matrix(cr: *mut cairo_t, matrix: *const Matrix);
    pub fn cairo_get_font_matrix(cr: *mut cairo_t, matrix: *mut Matrix);
    pub fn cairo_set_font_options(cr: *mut cairo_t, options: *const cairo_font_options_t);
    pub fn cairo_get_font_options(cr: *mut cairo_t, options: *mut cairo_font_options_t);
    pub fn cairo_set_font_face(cr: *mut cairo_t, font_face: *mut cairo_font_face_t);
    pub fn cairo_get_font_face(cr: *mut cairo_t) -> *mut cairo_font_face_t;
    pub fn cairo_set_scaled_font(cr: *mut cairo_t, scaled_font: *mut cairo_scaled_font_t);
    pub fn cairo_get_scaled_font(cr: *mut cairo_t) -> *mut cairo_scaled_font_t;
    pub fn cairo_show_text(cr: *mut cairo_t, utf8: *const c_char);
    pub fn cairo_show_glyphs(cr: *mut cairo_t, glyphs: *const Glyph, num_glyphs: c_int);
    pub fn cairo_show_text_glyphs(cr: *mut cairo_t, utf8: *const c_char, utf8_len: c_int, glyphs: *const Glyph, num_glyphs: c_int, clusters: *const TextCluster, num_clusters: c_int, cluster_flags: cairo_text_cluster_flags_t);
    pub fn cairo_font_extents(cr: *mut cairo_t, extents: *mut FontExtents);
    pub fn cairo_text_extents(cr: *mut cairo_t, utf8: *const c_char, extents: *mut TextExtents);
    pub fn cairo_glyph_extents(cr: *mut cairo_t, glyphs: *const Glyph, num_glyphs: c_int, extents: *mut TextExtents);
    pub fn cairo_toy_font_face_create(family: *const c_char, slant: cairo_font_slant_t, weight: cairo_font_weight_t) -> *mut cairo_font_face_t;
    pub fn cairo_toy_font_face_get_family(font_face: *mut cairo_font_face_t) -> *const c_char;
    pub fn cairo_toy_font_face_get_slant(font_face: *mut cairo_font_face_t) -> cairo_font_slant_t;
    pub fn cairo_toy_font_face_get_weight(font_face: *mut cairo_font_face_t) -> cairo_font_weight_t;
    pub fn cairo_glyph_allocate(num_glyphs: c_int) -> *mut Glyph;
    pub fn cairo_glyph_free(glyphs: *mut Glyph);
    pub fn cairo_text_cluster_allocate(num_clusters: c_int) -> *mut TextCluster;
    pub fn cairo_text_cluster_free(clusters: *mut TextCluster);

    //CAIRO RASTER
    //pub fn cairo_pattern_create_raster_source(user_data: *mut void, content: Content, width: c_int, height: c_int) -> *mut cairo_pattern_t;
    //pub fn cairo_raster_source_pattern_set_callback_data(pattern: *mut cairo_pattern_t, data: *mut void);
    //pub fn cairo_raster_source_pattern_get_callback_data(pattern: *mut cairo_pattern_t) -> *mut void;
    /* FIXME how do we do these _func_t types?
    pub fn cairo_raster_source_pattern_set_acquire(pattern: *mut cairo_pattern_t, acquire: cairo_raster_source_acquire_func_t, release: cairo_raster_source_release_func_t);
    pub fn cairo_raster_source_pattern_get_acquire(pattern: *mut cairo_pattern_t, acquire: *mut cairo_raster_source_acquire_func_t, release: *mut cairo_raster_source_release_func_t);
    pub fn cairo_raster_source_pattern_set_snapshot(pattern: *mut cairo_pattern_t, snapshot: cairo_raster_source_snapshot_func_t);
    pub fn cairo_raster_source_pattern_get_snapshot(pattern: *mut cairo_pattern_t) -> cairo_raster_source_snapshot_func_t;
    pub fn cairo_raster_source_pattern_set_copy(pattern: *mut cairo_pattern_t, copy: cairo_raster_source_copy_func_t);
    pub fn cairo_raster_source_pattern_get_copy(pattern: *mut cairo_pattern_t) -> cairo_raster_source_copy_func_t;
    pub fn cairo_raster_source_pattern_set_finish(pattern: *mut cairo_pattern_t, finish: cairo_raster_source_finish_func_t);
    pub fn cairo_raster_source_pattern_get_finish(pattern: *mut cairo_pattern_t) -> cairo_raster_source_finish_func_t;
    */

    //cairo_surface_t     (*mut cairo_raster_source_acquire_func_t)
    //                                                        (pattern: *mut cairo_pattern_t, callback_data: *mut void, target: *mut cairo_surface_t, extents: *mut cairo_rectangle_int_t);
    //void                (*mut cairo_raster_source_release_func_t)
    //                                                        (pattern: *mut cairo_pattern_t, callback_data: *mut void, surface: *mut cairo_surface_t);
    //Status      (*mut cairo_raster_source_snapshot_func_t)
    //                                                        (pattern: *mut cairo_pattern_t, callback_data: *mut void);
    //Status      (*mut cairo_raster_source_copy_func_t)  (pattern: *mut cairo_pattern_t, callback_data: *mut void, other: *mut cairo_pattern_t);
    //void                (*mut cairo_raster_source_finish_func_t)
    //                                                        (pattern: *mut cairo_pattern_t, callback_data: *mut void);

    //CAIRO FONT
    pub fn cairo_font_face_reference(font_face: *mut cairo_font_face_t) -> *mut cairo_font_face_t;
    pub fn cairo_font_face_destroy(font_face: *mut cairo_font_face_t);
    pub fn cairo_font_face_status(font_face: *mut cairo_font_face_t) -> cairo_status_t;
    pub fn cairo_font_face_get_type(font_face: *mut cairo_font_face_t) -> cairo_font_type_t;
    pub fn cairo_font_face_get_reference_count(font_face: *mut cairo_font_face_t) -> c_uint;
    //pub fn cairo_font_face_set_user_data(font_face: *mut cairo_font_face_t, key: *mut cairo_user_data_key_t, user_data: *mut void, destroy: cairo_destroy_func_t) -> cairo_status_t;
    //pub fn cairo_font_face_get_user_data(font_face: *mut cairo_font_face_t, key: *mut cairo_user_data_key_t) -> *mut void;

    //CAIRO SCALED FONT
    pub fn cairo_scaled_font_create(font_face: *mut cairo_font_face_t, font_matrix: *const Matrix, ctm: *const Matrix, options: *const cairo_font_options_t) -> *mut cairo_scaled_font_t;
    pub fn cairo_scaled_font_reference(scaled_font: *mut cairo_scaled_font_t) -> *mut cairo_scaled_font_t;
    pub fn cairo_scaled_font_destroy(scaled_font: *mut cairo_scaled_font_t);
    pub fn cairo_scaled_font_status(scaled_font: *mut cairo_scaled_font_t) -> cairo_status_t;
    pub fn cairo_scaled_font_extents(scaled_font: *mut cairo_scaled_font_t, extents: *mut FontExtents);
    pub fn cairo_scaled_font_text_extents(scaled_font: *mut cairo_scaled_font_t, utf8: *const c_char, extents: *mut TextExtents);
    pub fn cairo_scaled_font_glyph_extents(scaled_font: *mut cairo_scaled_font_t, glyphs: *const Glyph, num_glyphs: c_int, extents: *mut TextExtents);
    pub fn cairo_scaled_font_text_to_glyphs(scaled_font: *mut cairo_scaled_font_t, x: c_double, y: c_double, utf8: *const c_char, utf8_len: c_int, glyphs: *mut *mut Glyph, num_glyphs: *mut c_int, clusters: *mut *mut TextCluster, num_clusters: *mut c_int, cluster_flags: *mut cairo_text_cluster_flags_t) -> cairo_status_t;
    pub fn cairo_scaled_font_get_font_face(scaled_font: *mut cairo_scaled_font_t) -> *mut cairo_font_face_t;
    pub fn cairo_scaled_font_get_font_options(scaled_font: *mut cairo_scaled_font_t, options: *mut cairo_font_options_t);
    pub fn cairo_scaled_font_get_font_matrix(scaled_font: *mut cairo_scaled_font_t, font_matrix: *mut Matrix);
    pub fn cairo_scaled_font_get_ctm(scaled_font: *mut cairo_scaled_font_t, ctm: *mut Matrix);
    pub fn cairo_scaled_font_get_scale_matrix(scaled_font: *mut cairo_scaled_font_t, scale_matrix: *mut Matrix);
    pub fn cairo_scaled_font_get_type(scaled_font: *mut cairo_scaled_font_t) -> cairo_font_type_t;
    pub fn cairo_scaled_font_get_reference_count(font_face: *mut cairo_scaled_font_t) -> c_uint;
    //pub fn cairo_scaled_font_set_user_data(scaled_font: *mut cairo_scaled_font_t, key: *mut cairo_user_data_key_t, user_data: *mut void, destroy: cairo_destroy_func_t) -> cairo_status_t;
    //pub fn cairo_scaled_font_get_user_data(scaled_font: *mut cairo_scaled_font_t, key: *mut cairo_user_data_key_t) -> *mut void;

    //CAIRO FONT OPTIONS
    pub fn cairo_font_options_create() -> *mut cairo_font_options_t;
    pub fn cairo_font_options_copy(original: *const cairo_font_options_t) -> *mut cairo_font_options_t;
    pub fn cairo_font_options_destroy(options: *mut cairo_font_options_t);
    pub fn cairo_font_options_status(options: *mut cairo_font_options_t) -> cairo_status_t;
    pub fn cairo_font_options_merge(options: *mut cairo_font_options_t, other: *const cairo_font_options_t);
    pub fn cairo_font_options_hash(options: *const cairo_font_options_t) -> c_ulong;
    pub fn cairo_font_options_equal(options: *const cairo_font_options_t, other: *const cairo_font_options_t) -> cairo_bool_t;
    pub fn cairo_font_options_set_antialias(options: *mut cairo_font_options_t, antialias: cairo_antialias_t);
    pub fn cairo_font_options_get_antialias(options: *const cairo_font_options_t) -> cairo_antialias_t;
    pub fn cairo_font_options_set_subpixel_order(options: *mut cairo_font_options_t, subpixel_order: cairo_subpixel_order_t);
    pub fn cairo_font_options_get_subpixel_order(options: *const cairo_font_options_t) -> cairo_subpixel_order_t;
    pub fn cairo_font_options_set_hint_style(options: *mut cairo_font_options_t, hint_style: cairo_hint_style_t);
    pub fn cairo_font_options_get_hint_style(options: *const cairo_font_options_t) -> cairo_hint_style_t;
    pub fn cairo_font_options_set_hint_metrics(options: *mut cairo_font_options_t, hint_metrics: cairo_hint_metrics_t);
    pub fn cairo_font_options_get_hint_metrics(options: *const cairo_font_options_t) -> cairo_hint_metrics_t;

    // CAIRO MATRIX
    pub fn cairo_matrix_multiply(matrix: *mut Matrix, left: *const Matrix, right: *const Matrix);
    pub fn cairo_matrix_init(matrix: *mut Matrix, xx: f64, yx: f64, xy: f64, yy: f64, x0: f64, y0: f64);
    pub fn cairo_matrix_init_identity(matrix: *mut Matrix);
    pub fn cairo_matrix_translate(matrix: *mut Matrix, tx: f64, ty: f64);
    pub fn cairo_matrix_scale(matrix: *mut Matrix, sx: f64, sy: f64);
    pub fn cairo_matrix_rotate(matrix: *mut Matrix, angle: f64);
    pub fn cairo_matrix_invert(matrix: *mut Matrix) -> cairo_status_t;
    pub fn cairo_matrix_transform_distance(matrix: *const Matrix, dx: *mut f64, dy: *mut f64);
    pub fn cairo_matrix_transform_point(matrix: *const Matrix, x: *mut f64, y: *mut f64);

    // CAIRO SURFACE
    pub fn cairo_surface_destroy(surface: *mut cairo_surface_t);
    pub fn cairo_surface_flush(surface: *mut cairo_surface_t);
    pub fn cairo_surface_finish(surface: *mut cairo_surface_t);
    pub fn cairo_surface_status(surface: *mut cairo_surface_t) -> cairo_status_t;
    pub fn cairo_surface_get_type(surface: *mut cairo_surface_t) -> cairo_surface_type_t;
    pub fn cairo_surface_reference(surface: *mut cairo_surface_t) -> *mut cairo_surface_t;
    pub fn cairo_surface_get_user_data(surface: *mut cairo_surface_t, key: *mut cairo_user_data_key_t) -> *mut c_void;
    pub fn cairo_surface_set_user_data(surface: *mut cairo_surface_t, key: *mut cairo_user_data_key_t, user_data: *mut c_void, destroy: cairo_destroy_func_t) -> cairo_status_t;
    pub fn cairo_surface_get_reference_count(surface: *mut cairo_surface_t) -> c_uint;
    pub fn cairo_surface_mark_dirty(surface: *mut cairo_surface_t);
    pub fn cairo_surface_create_similar(surface: *mut cairo_surface_t, content: cairo_content_t, width: c_int, height: c_int) -> *mut cairo_surface_t;

    // CAIRO IMAGE SURFACE
    pub fn cairo_image_surface_create(format: cairo_format_t, width: c_int, height: c_int) -> *mut cairo_surface_t;
    pub fn cairo_image_surface_create_for_data(data: *mut u8, format: cairo_format_t, width: c_int, height: c_int, stride: c_int) -> *mut cairo_surface_t;
    pub fn cairo_image_surface_get_data(surface: *mut cairo_surface_t) -> *mut u8;
    pub fn cairo_image_surface_get_format(surface: *mut cairo_surface_t) -> cairo_format_t;
    pub fn cairo_image_surface_get_height(surface: *mut cairo_surface_t) -> c_int;
    pub fn cairo_image_surface_get_stride(surface: *mut cairo_surface_t) -> c_int;
    pub fn cairo_image_surface_get_width(surface: *mut cairo_surface_t) -> c_int;
    pub fn cairo_format_stride_for_width(format: cairo_format_t, width: c_int) -> c_int;
    #[cfg(any(feature = "png", feature = "dox"))]
    pub fn cairo_image_surface_create_from_png_stream(read_func: cairo_read_func_t, closure: *mut c_void) -> *mut cairo_surface_t;
    #[cfg(any(feature = "png", feature = "dox"))]
    pub fn cairo_surface_write_to_png_stream(surface: *mut cairo_surface_t, write_func: cairo_write_func_t, closure: *mut c_void) -> cairo_status_t;

    // CAIRO PDF
    #[cfg(any(feature = "pdf", feature = "dox"))]
    pub fn cairo_pdf_surface_create (filename: *const c_char,
                                     width_in_points: c_double,
                                     height_in_points: c_double) -> *mut cairo_surface_t;
    #[cfg(any(feature = "pdf", feature = "dox"))]
    pub fn cairo_pdf_surface_create_for_stream (write_func: cairo_write_func_t,
                                                closure: *mut c_void,
                                                width_in_points: c_double,
                                                height_in_points: c_double) -> *mut cairo_surface_t;
    #[cfg(any(feature = "pdf", feature = "dox"))]
    pub fn cairo_pdf_surface_restrict_to_version (surface: *mut cairo_surface_t, version: cairo_pdf_version_t);
    // CAIRO SVG
    #[cfg(any(feature = "svg", feature = "dox"))]
    pub fn cairo_svg_surface_create (filename: *const c_char,
                                     width_in_points: c_double,
                                     height_in_points: c_double) -> *mut cairo_surface_t;
    #[cfg(any(feature = "svg", feature = "dox"))]
    pub fn cairo_svg_surface_create_for_stream (write_func: cairo_write_func_t,
                                                closure: *mut c_void,
                                                width_in_points: c_double,
                                                height_in_points: c_double) -> *mut cairo_surface_t;
    #[cfg(any(feature = "svg", feature = "dox"))]
    pub fn cairo_svg_surface_restrict_to_version (surface: *mut cairo_surface_t, version: cairo_svg_version_t);
    #[cfg(any(feature = "svg", feature = "dox"))]
    pub fn cairo_svg_surface_get_document_unit(surface: *const cairo_surface_t) -> cairo_svg_unit_t;
    #[cfg(any(feature = "svg", feature = "dox"))]
    pub fn cairo_svg_surface_set_document_unit(surface: *mut cairo_surface_t, unit: cairo_svg_unit_t);
    // CAIRO PS
    #[cfg(any(feature = "ps", feature = "dox"))]
    pub fn cairo_ps_surface_create (filename: *const c_char,
                                    width_in_points: c_double,
                                    height_in_points: c_double) -> *mut cairo_surface_t;
    #[cfg(any(feature = "ps", feature = "dox"))]
    pub fn cairo_ps_surface_create_for_stream (write_func: cairo_write_func_t,
                                               closure: *mut c_void,
                                               width_in_points: c_double,
                                               height_in_points: c_double) -> *mut cairo_surface_t;
    #[cfg(any(feature = "ps", feature = "dox"))]
    pub fn cairo_ps_surface_restrict_to_level (surface: *mut cairo_surface_t, version: cairo_ps_level_t);
    #[cfg(any(feature = "ps", feature = "dox"))]
    pub fn cairo_ps_surface_set_eps (surface: *mut cairo_surface_t, eps: cairo_bool_t);
    #[cfg(any(feature = "ps", feature = "dox"))]
    pub fn cairo_ps_surface_get_eps (surface: *mut cairo_surface_t) -> cairo_bool_t;
    #[cfg(any(feature = "ps", feature = "dox"))]
    pub fn cairo_ps_surface_set_size (surface: *mut cairo_surface_t,
                                      width_in_points: f64,
                                      height_in_points: f64);
    #[cfg(any(feature = "ps", feature = "dox"))]
    pub fn cairo_ps_surface_dsc_begin_setup (surface: *mut cairo_surface_t);
    #[cfg(any(feature = "ps", feature = "dox"))]
    pub fn cairo_ps_surface_dsc_begin_page_setup (surface: *mut cairo_surface_t);
    #[cfg(any(feature = "ps", feature = "dox"))]
    pub fn cairo_ps_surface_dsc_comment (surface: *mut cairo_surface_t, comment: *const c_char);

    // CAIRO XCB
    #[cfg(any(feature = "xcb", feature = "dox"))]
    pub fn cairo_xcb_surface_create(connection: *mut xcb_connection_t,
                                    drawable: xcb_drawable_t,
                                    visual: *mut xcb_visualtype_t,
                                    width: c_int,
                                    height: c_int) -> *mut cairo_surface_t;
    #[cfg(any(feature = "xcb", feature = "dox"))]
    pub fn cairo_xcb_surface_create_for_bitmap(connection: *mut xcb_connection_t,
                                               screen: *mut xcb_screen_t,
                                               bitmap: xcb_pixmap_t,
                                               width: c_int,
                                               height: c_int) -> *mut cairo_surface_t;
    #[cfg(any(feature = "xcb", feature = "dox"))]
    pub fn cairo_xcb_surface_create_with_xrender_format(connection: *mut xcb_connection_t,
                                                        screen: *mut xcb_screen_t,
                                                        drawable: xcb_drawable_t,
                                                        format: *mut xcb_render_pictforminfo_t,
                                                        width: c_int,
                                                        height: c_int) -> *mut cairo_surface_t;
    #[cfg(any(feature = "xcb", feature = "dox"))]
    pub fn cairo_xcb_surface_set_size(surface: *mut cairo_surface_t,
                                      width: c_int,
                                      height: c_int);
    #[cfg(any(feature = "xcb", feature = "dox"))]
    pub fn cairo_xcb_surface_set_drawable(surface: *mut cairo_surface_t,
                                          drawable: xcb_drawable_t,
                                          width: c_int,
                                          height: c_int);
    #[cfg(any(feature = "xcb", feature = "dox"))]
    pub fn cairo_xcb_device_get_connection(device: *mut cairo_device_t) -> *mut xcb_connection_t;
    #[cfg(any(feature = "xcb", feature = "dox"))]
    pub fn cairo_xcb_device_debug_cap_xrender_version(device: *mut cairo_device_t,
                                                      major_version: c_int,
                                                      minor_version: c_int);
    #[cfg(any(feature = "xcb", feature = "dox"))]
    pub fn cairo_xcb_device_debug_cap_xshm_version(device: *mut cairo_device_t,
                                                   major_version: c_int,
                                                   minor_version: c_int);
    #[cfg(any(feature = "xcb", feature = "dox"))]
    pub fn cairo_xcb_device_debug_get_precision(device: *mut cairo_device_t) -> c_int;
    #[cfg(any(feature = "xcb", feature = "dox"))]
    pub fn cairo_xcb_device_debug_set_precision(device: *mut cairo_device_t,
                                                precision: c_int);

    // CAIRO XLIB SURFACE
    #[cfg(any(feature = "xlib", feature = "dox"))]
    pub fn cairo_xlib_surface_create(dpy: *mut xlib::Display,
                                     drawable: xlib::Drawable,
                                     visual: *mut xlib::Visual,
                                     width: c_int,
                                     height: c_int)
                                     -> *mut cairo_surface_t;
    #[cfg(any(feature = "xlib", feature = "dox"))]
    pub fn cairo_xlib_surface_create_for_bitmap(dpy: *mut xlib::Display,
                                                bitmap: xlib::Pixmap,
                                                screen: *mut xlib::Screen,
                                                width: c_int,
                                                height: c_int)
                                                -> *mut cairo_surface_t;
    #[cfg(any(feature = "xlib", feature = "dox"))]
    pub fn cairo_xlib_surface_set_size(surface: *mut cairo_surface_t,
                                       width: c_int,
                                       height: c_int);
    #[cfg(any(feature = "xlib", feature = "dox"))]
    pub fn cairo_xlib_surface_set_drawable(surface: *mut cairo_surface_t,
                                           drawable: xlib::Drawable,
                                           width: c_int,
                                           height: c_int);
    #[cfg(any(feature = "xlib", feature = "dox"))]
    pub fn cairo_xlib_surface_get_display(surface: *mut cairo_surface_t)
                                          -> *mut xlib::Display;
    #[cfg(any(feature = "xlib", feature = "dox"))]
    pub fn cairo_xlib_surface_get_drawable(surface: *mut cairo_surface_t)
                                           -> xlib::Drawable;
    #[cfg(any(feature = "xlib", feature = "dox"))]
    pub fn cairo_xlib_surface_get_screen(surface: *mut cairo_surface_t)
                                         -> *mut xlib::Screen;
    #[cfg(any(feature = "xlib", feature = "dox"))]
    pub fn cairo_xlib_surface_get_visual(surface: *mut cairo_surface_t)
                                         -> *mut xlib::Visual;
    #[cfg(any(feature = "xlib", feature = "dox"))]
    pub fn cairo_xlib_surface_get_depth(surface: *mut cairo_surface_t)
                                        -> c_int;
    #[cfg(any(feature = "xlib", feature = "dox"))]
    pub fn cairo_xlib_surface_get_width(surface: *mut cairo_surface_t)
                                        -> c_int;
    #[cfg(any(feature = "xlib", feature = "dox"))]
    pub fn cairo_xlib_surface_get_height(surface: *mut cairo_surface_t)
                                         -> c_int;

    // CAIRO WINDOWS SURFACE
    #[cfg(any(windows, feature = "dox"))]
    pub fn cairo_win32_surface_create(hdc: winapi::HDC) -> *mut cairo_surface_t;
    #[cfg(any(windows, feature = "dox"))]
    pub fn cairo_win32_surface_create_with_dib(format: cairo_format_t,
                                               width: c_int,
                                               height: c_int)
                                               -> *mut cairo_surface_t;
    #[cfg(any(windows, feature = "dox"))]
    pub fn cairo_win32_surface_create_with_ddb(hdc: winapi::HDC,
                                               format: cairo_format_t,
                                               width: c_int,
                                               height: c_int)
                                               -> *mut cairo_surface_t;
    #[cfg(any(windows, feature = "dox"))]
    pub fn cairo_win32_printing_surface_create(hdc: winapi::HDC) -> *mut cairo_surface_t;
    #[cfg(any(windows, feature = "dox"))]
    pub fn cairo_win32_surface_get_dc(surface: *mut cairo_surface_t) -> winapi::HDC;
    #[cfg(any(windows, feature = "dox"))]
    pub fn cairo_win32_surface_get_image(surface: *mut cairo_surface_t) -> *mut cairo_surface_t;

    #[cfg(any(target_os = "macos", target_os = "ios", feature = "dox"))]
    pub fn cairo_quartz_surface_create(format: cairo_format_t,
                                       width: c_uint,
                                       height: c_uint)
                                       -> *mut cairo_surface_t;
    #[cfg(any(target_os = "macos", target_os = "ios", feature = "dox"))]
    pub fn cairo_quartz_surface_create_for_cg_context(cg_context: CGContextRef,
                                                      width: c_uint,
                                                      height: c_uint)
                                                      -> *mut cairo_surface_t;
    #[cfg(any(target_os = "macos", target_os = "ios", feature = "dox"))]
    pub fn cairo_quartz_surface_get_cg_context(surface: *mut cairo_surface_t) -> CGContextRef;
}

#[cfg(feature = "use_glib")]
pub mod gobject;

pub const STATUS_SUCCESS: i32 = 0;
pub const STATUS_NO_MEMORY: i32 = 1;
pub const STATUS_INVALID_RESTORE: i32 = 2;
pub const STATUS_INVALID_POP_GROUP: i32 = 3;
pub const STATUS_NO_CURRENT_POINT: i32 = 4;
pub const STATUS_INVALID_MATRIX: i32 = 5;
pub const STATUS_INVALID_STATUS: i32 = 6;
pub const STATUS_NULL_POINTER: i32 = 7;
pub const STATUS_INVALID_STRING: i32 = 8;
pub const STATUS_INVALID_PATH_DATA: i32 = 9;
pub const STATUS_READ_ERROR: i32 = 10;
pub const STATUS_WRITE_ERROR: i32 = 11;
pub const STATUS_SURFACE_FINISHED: i32 = 12;
pub const STATUS_SURFACE_TYPE_MISMATCH: i32 = 13;
pub const STATUS_PATTERN_TYPE_MISMATCH: i32 = 14;
pub const STATUS_INVALID_CONTENT: i32 = 15;
pub const STATUS_INVALID_FORMAT: i32 = 16;
pub const STATUS_INVALID_VISUAL: i32 = 17;
pub const STATUS_FILE_NOT_FOUND: i32 = 18;
pub const STATUS_INVALID_DASH: i32 = 19;
pub const STATUS_INVALID_DSC_COMMENT: i32 = 20;
pub const STATUS_INVALID_INDEX: i32 = 21;
pub const STATUS_CLIP_NOT_REPRESENTABLE: i32 = 22;
pub const STATUS_TEMP_FILE_ERROR: i32 = 23;
pub const STATUS_INVALID_STRIDE: i32 = 24;
pub const STATUS_FONT_TYPE_MISMATCH: i32 = 25;
pub const STATUS_USER_FONT_IMMUTABLE: i32 = 26;
pub const STATUS_USER_FONT_ERROR: i32 = 27;
pub const STATUS_NEGATIVE_COUNT: i32 = 28;
pub const STATUS_INVALID_CLUSTERS: i32 = 29;
pub const STATUS_INVALID_SLANT: i32 = 30;
pub const STATUS_INVALID_WEIGHT: i32 = 31;
pub const STATUS_INVALID_SIZE: i32 = 32;
pub const STATUS_USER_FONT_NOT_IMPLEMENTED: i32 = 33;
pub const STATUS_DEVICE_TYPE_MISMATCH: i32 = 34;
pub const STATUS_DEVICE_ERROR: i32 = 35;
pub const STATUS_INVALID_MESH_CONSTRUCTION: i32 = 36;
pub const STATUS_DEVICE_FINISHED: i32 = 37;
pub const STATUS_J_BIG2_GLOBAL_MISSING: i32 = 38;
pub const STATUS_PNG_ERROR: i32 = 39;
pub const STATUS_FREETYPE_ERROR: i32 = 40;
pub const STATUS_WIN32_GDI_ERROR: i32 = 41;
pub const STATUS_LAST_STATUS: i32 = 42;
pub const ANTIALIAS_DEFAULT: i32 = 0;
pub const ANTIALIAS_NONE: i32 = 1;
pub const ANTIALIAS_GRAY: i32 = 2;
pub const ANTIALIAS_SUBPIXEL: i32 = 3;
pub const ANTIALIAS_FAST: i32 = 4;
pub const ANTIALIAS_GOOD: i32 = 5;
pub const ANTIALIAS_BEST: i32 = 6;
pub const FILL_RULE_WINDING: i32 = 0;
pub const FILL_RULE_EVEN_ODD: i32 = 1;
pub const LINE_CAP_BUTT: i32 = 0;
pub const LINE_CAP_ROUND: i32 = 1;
pub const LINE_CAP_SQUARE: i32 = 2;
pub const LINE_JOIN_MITER: i32 = 0;
pub const LINE_JOIN_ROUND: i32 = 1;
pub const LINE_JOIN_BEVEL: i32 = 2;
pub const OPERATOR_CLEAR: i32 = 0;
pub const OPERATOR_SOURCE: i32 = 1;
pub const OPERATOR_OVER: i32 = 2;
pub const OPERATOR_IN: i32 = 3;
pub const OPERATOR_OUT: i32 = 4;
pub const OPERATOR_ATOP: i32 = 5;
pub const OPERATOR_DEST: i32 = 6;
pub const OPERATOR_DEST_OVER: i32 = 7;
pub const OPERATOR_DEST_IN: i32 = 8;
pub const OPERATOR_DEST_OUT: i32 = 9;
pub const OPERATOR_DEST_ATOP: i32 = 10;
pub const OPERATOR_XOR: i32 = 11;
pub const OPERATOR_ADD: i32 = 12;
pub const OPERATOR_SATURATE: i32 = 13;
pub const OPERATOR_MULTIPLY: i32 = 14;
pub const OPERATOR_SCREEN: i32 = 15;
pub const OPERATOR_OVERLAY: i32 = 16;
pub const OPERATOR_DARKEN: i32 = 17;
pub const OPERATOR_LIGHTEN: i32 = 18;
pub const OPERATOR_COLOR_DODGE: i32 = 19;
pub const OPERATOR_COLOR_BURN: i32 = 20;
pub const OPERATOR_HARD_LIGHT: i32 = 21;
pub const OPERATOR_SOFT_LIGHT: i32 = 22;
pub const OPERATOR_DIFFERENCE: i32 = 23;
pub const OPERATOR_EXCLUSION: i32 = 24;
pub const OPERATOR_HSL_HUE: i32 = 25;
pub const OPERATOR_HSL_SATURATION: i32 = 26;
pub const OPERATOR_HSL_COLOR: i32 = 27;
pub const OPERATOR_HSL_LUMINOSITY: i32 = 28;
pub const PATH_DATA_TYPE_MOVE_TO: i32 = 0;
pub const PATH_DATA_TYPE_LINE_TO: i32 = 1;
pub const PATH_DATA_TYPE_CURVE_TO: i32 = 2;
pub const PATH_DATA_TYPE_CLOSE_PATH: i32 = 3;
pub const CONTENT_COLOR     : i32 = 0x1000;
pub const CONTENT_ALPHA     : i32 = 0x2000;
pub const CONTENT_COLOR_ALPHA: i32 = 0x3000;
pub const EXTEND_NONE: i32 = 0;
pub const EXTEND_REPEAT: i32 = 1;
pub const EXTEND_REFLECT: i32 = 2;
pub const EXTEND_PAD: i32 = 3;
pub const FILTER_FAST: i32 = 0;
pub const FILTER_GOOD: i32 = 1;
pub const FILTER_BEST: i32 = 2;
pub const FILTER_NEAREST: i32 = 3;
pub const FILTER_BILINEAR: i32 = 4;
pub const FILTER_GAUSSIAN: i32 = 5;
pub const PATTERN_TYPE_SOLID: i32 = 0;
pub const PATTERN_TYPE_SURFACE: i32 = 1;
pub const PATTERN_TYPE_LINEAR_GRADIENT: i32 = 2;
pub const PATTERN_TYPE_RADIAL_GRADIENT: i32 = 3;
pub const PATTERN_TYPE_MESH: i32 = 4;
pub const PATTERN_TYPE_RASTER_SOURCE: i32 = 5;
pub const FONT_SLANT_NORMAL: i32 = 0;
pub const FONT_SLANT_ITALIC: i32 = 1;
pub const FONT_SLANT_OBLIQUE: i32 = 2;
pub const FONT_WEIGHT_NORMAL: i32 = 0;
pub const FONT_WEIGHT_BOLD: i32 = 1;
pub const TEXT_CLUSTER_FLAGS_NONE    : i32 = 0x00000000;
pub const TEXT_CLUSTER_FLAGS_BACKWARD: i32 = 0x00000001;
pub const FONT_TYPE_FONT_TYPE_TOY: i32 = 0;
pub const FONT_TYPE_FONT_TYPE_FT: i32 = 1;
pub const FONT_TYPE_FONT_TYPE_WIN32: i32 = 2;
pub const FONT_TYPE_FONT_TYPE_QUARTZ: i32 = 3;
pub const FONT_TYPE_FONT_TYPE_USER: i32 = 4;
pub const SUBPIXEL_ORDER_DEFAULT: i32 = 0;
pub const SUBPIXEL_ORDER_RGB: i32 = 1;
pub const SUBPIXEL_ORDER_BGR: i32 = 2;
pub const SUBPIXEL_ORDER_VRGB: i32 = 3;
pub const SUBPIXEL_ORDER_VBGR: i32 = 4;
pub const HINT_STYLE_DEFAULT: i32 = 0;
pub const HINT_STYLE_NONE: i32 = 1;
pub const HINT_STYLE_SLIGHT: i32 = 2;
pub const HINT_STYLE_MEDIUM: i32 = 3;
pub const HINT_STYLE_FULL: i32 = 4;
pub const HINT_METRICS_DEFAULT: i32 = 0;
pub const HINT_METRICS_OFF: i32 = 1;
pub const HINT_METRICS_ON: i32 = 2;
pub const SURFACE_TYPE_IMAGE: i32 = 0;
pub const SURFACE_TYPE_PDF: i32 = 1;
pub const SURFACE_TYPE_PS: i32 = 2;
pub const SURFACE_TYPE_XLIB: i32 = 3;
pub const SURFACE_TYPE_XCB: i32 = 4;
pub const SURFACE_TYPE_GLITZ: i32 = 5;
pub const SURFACE_TYPE_QUARTZ: i32 = 6;
pub const SURFACE_TYPE_WIN32: i32 = 7;
pub const SURFACE_TYPE_BE_OS: i32 = 8;
pub const SURFACE_TYPE_DIRECT_FB: i32 = 9;
pub const SURFACE_TYPE_SVG: i32 = 10;
pub const SURFACE_TYPE_OS2: i32 = 11;
pub const SURFACE_TYPE_WIN32_PRINTING: i32 = 12;
pub const SURFACE_TYPE_QUARTZ_IMAGE: i32 = 13;
pub const SURFACE_TYPE_SCRIPT: i32 = 14;
pub const SURFACE_TYPE_QT: i32 = 15;
pub const SURFACE_TYPE_RECORDING: i32 = 16;
pub const SURFACE_TYPE_VG: i32 = 17;
pub const SURFACE_TYPE_GL: i32 = 18;
pub const SURFACE_TYPE_DRM: i32 = 19;
pub const SURFACE_TYPE_TEE: i32 = 20;
pub const SURFACE_TYPE_XML: i32 = 21;
pub const SURFACE_TYPE_SKIA: i32 = 22;
pub const SURFACE_TYPE_SUBSURFACE: i32 = 23;
pub const SURFACE_TYPE_COGL: i32 = 24;
pub const SVG_UNIT_USER: i32 = 0;
pub const SVG_UNIT_EM: i32 = 1;
pub const SVG_UNIT_EX: i32 = 2;
pub const SVG_UNIT_PX: i32 = 3;
pub const SVG_UNIT_IN: i32 = 4;
pub const SVG_UNIT_CM: i32 = 5;
pub const SVG_UNIT_MM: i32 = 6;
pub const SVG_UNIT_PT: i32 = 7;
pub const SVG_UNIT_PC: i32 = 8;
pub const SVG_UNIT_PERCENT: i32 = 9;
pub const FORMAT_INVALID: i32 = -1;
pub const FORMAT_A_RGB32: i32 = 0;
pub const FORMAT_RGB24: i32 = 1;
pub const FORMAT_A8: i32 = 2;
pub const FORMAT_A1: i32 = 3;
pub const FORMAT_RGB16_565: i32 = 4;
pub const FORMAT_RGB30: i32 = 5;
pub const REGION_OVERLAP_IN: i32 = 0;
pub const REGION_OVERLAP_OUT: i32 = 1;
pub const REGION_OVERLAP_PART: i32 = 2;
pub const PDF_VERSION__1_4: i32 = 0;
pub const PDF_VERSION__1_5: i32 = 1;
pub const SVG_VERSION__1_1: i32 = 0;
pub const SVG_VERSION__1_2: i32 = 1;
pub const PS_LEVEL__2: i32 = 0;
pub const PS_LEVEL__3: i32 = 1;
pub const MESH_CORNER_MESH_CORNER0: u32 = 0;
pub const MESH_CORNER_MESH_CORNER1: u32 = 1;
pub const MESH_CORNER_MESH_CORNER2: u32 = 2;
pub const MESH_CORNER_MESH_CORNER3: u32 = 3;
