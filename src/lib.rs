#![allow(non_snake_case, non_camel_case_types)]
#![feature(stdarch)]

mod matrix;
mod vec2;
mod vec3;
mod vec4;

use minifb::{Window, WindowOptions};

use matrix::*;
use vec2::*;
use vec3::*;
use vec4::*;

use std::cell::RefCell;
use std::ffi::{CStr, c_void};
use std::ptr;
use std::rc::Rc;
use std::slice;

// TODO: Can we query this from the dc or something?
const WIDTH: usize = 640;
const HEIGHT: usize = 480;

type LPVOID = *mut c_void;

type BOOL = i32;
const FALSE: BOOL = 0;
const TRUE: BOOL = 1;

type DWORD = i32;
type PDWORD = *mut DWORD;
type LONG = i32;

type HANDLE = LPVOID;
type HWND = HANDLE;
type HDC = HANDLE;
type HGLRC = HANDLE;

type LPCSTR = LPVOID;
type PROC = LPVOID;
type SIZE_T = u32;

const PAGE_READWRITE: DWORD = 0x04;

#[link(name = "kernel32")]
extern "stdcall" {
    fn VirtualProtect(lpAddress: LPVOID, dwSize: SIZE_T, flNewProtect: DWORD, lpflOldProtect: PDWORD) -> BOOL;
}

#[link(name = "gdi32")]
extern "stdcall" {
    fn SwapBuffers(hdc: HDC) -> BOOL;
}

#[link(name = "user32")]
extern "stdcall" {
    fn ChangeDisplaySettingsExA(lpszDeviceName: LPCSTR, lpDevMode: *const c_void, hwnd: HWND, dwflags: DWORD, lParam: LPVOID) -> LONG;
}

type GLboolean = i32;
type GLubyte = u8;
type GLshort = i16;
type GLenum = u32;
type GLint = i32;
type GLuint = u32;
type GLsizei = u32;
type GLfloat = f32;
type GLbitfield = u32;
type GLdouble = f64;
type GLvoid = c_void;

const GL_NO_ERROR: GLenum = 0;

const GL_ZERO: GLenum = 0;
const GL_ONE: GLenum = 1;

const GL_FALSE: GLboolean = 0;
const GL_TRUE: GLboolean = 1;

const GL_TRIANGLES: GLenum = 0x0004;
const GL_QUADS: GLenum = 0x0007;

const GL_DEPTH_BUFFER_BIT: GLbitfield = 0x00000100;
const GL_COLOR_BUFFER_BIT: GLbitfield = 0x00004000;

const GL_SRC_COLOR: GLenum = 0x0300;
const GL_SRC_ALPHA: GLenum = 0x0302;
const GL_ONE_MINUS_SRC_ALPHA: GLenum = 0x0303;

const GL_DEPTH_TEST: GLenum = 0x0b71;
const GL_BLEND: GLenum = 0x0be2;

const GL_UNPACK_SWAP_BYTES: GLenum = 0x0cf0;
const GL_UNPACK_LSB_FIRST: GLenum = 0x0cf1;
const GL_UNPACK_ROW_LENGTH: GLenum = 0x0cf2;
const GL_UNPACK_SKIP_ROWS: GLenum = 0x0cf3;
const GL_UNPACK_SKIP_PIXELS: GLenum = 0x0cf4;
const GL_UNPACK_ALIGNMENT: GLenum = 0x0cf5;
const GL_PACK_SWAP_BYTES: GLenum = 0x0d00;
const GL_PACK_LSB_FIRST: GLenum = 0x0d01;
const GL_PACK_ROW_LENGTH: GLenum = 0x0d02;
const GL_PACK_SKIP_ROWS: GLenum = 0x0d03;
const GL_PACK_SKIP_PIXELS: GLenum = 0x0d04;
const GL_PACK_ALIGNMENT: GLenum = 0x0d05;

const GL_MAX_TEXTURE_SIZE: GLenum = 0xd33;

const GL_TEXTURE_2D: GLenum = 0x0de1;

const GL_COMPILE: GLenum = 0x1300;
const GL_COMPILE_AND_EXECUTE: GLenum = 0x1301;

const GL_UNSIGNED_BYTE: GLenum = 0x1401;
const GL_UNSIGNED_SHORT: GLenum = 0x1403;
const GL_FLOAT: GLenum = 0x1406;

const GL_MODELVIEW: GLenum = 0x1700;
const GL_PROJECTION: GLenum = 0x1701;

const GL_RGB: GLenum = 0x1907;
const GL_RGBA: GLenum = 0x1908;

const GL_NEAREST: GLint = 0x2600;
const GL_LINEAR: GLint = 0x2601;
const GL_LINEAR_MIPMAP_NEAREST: GLint = 0x2701;
const GL_LINEAR_MIPMAP_LINEAR: GLint = 0x2703;

const GL_TEXTURE_MAG_FILTER: GLenum = 0x2800;
const GL_TEXTURE_MIN_FILTER: GLenum = 0x2801;
const GL_TEXTURE_WRAP_S: GLenum = 0x2802;
const GL_TEXTURE_WRAP_T: GLenum = 0x2803;

const GL_CLAMP: GLint = 0x2900;

const GL_VERTEX_ARRAY: GLenum = 0x8074;
const GL_NORMAL_ARRAY: GLenum = 0x8075;

enum BlendSrcFactor {
    Zero,
    SrcColor,
    SrcAlpha,
}

enum BlendDstFactor {
    One,
    SrcAlpha,
    OneMinusSrcAlpha,
}

struct DisplayList {
    commands: Vec<Command>,
}

impl DisplayList {
    fn new() -> DisplayList {
        DisplayList {
            commands: Vec::new(),
        }
    }
}

enum MatrixMode {
    ModelView,
    Projection,
}

enum TextureFilter {
    Nearest,
    Linear,
    LinearMipmapNearest,
    LinearMipmapLinear,
}

struct Texture {
    mag_filter: TextureFilter,
    min_filter: TextureFilter,
    wrap_s: WrapParameter,
    wrap_t: WrapParameter,
    width: usize,
    height: usize,
    data: Vec<u32>,
}

impl Texture {
    fn new() -> Texture {
        Texture {
            mag_filter: TextureFilter::Linear,
            min_filter: TextureFilter::Linear,
            wrap_s: WrapParameter::Repeat,
            wrap_t: WrapParameter::Repeat,
            width: 0,
            height: 0,
            data: Vec::new(),
        }
    }
}

enum WrapParameter {
    Clamp,
    Repeat,
}

#[derive(Clone, Copy)]
enum PrimitiveMode {
    Triangles,
    Quads,
}

#[derive(Clone, Copy)]
struct Vertex {
    position: Vec4,
    normal: Vec3,
    color: Vec4,
    tex_coord: Vec4,
}

enum Command {
    ActiveTextureARB { texture: GLenum },
    Begin { mode: GLenum },
    BindTexture { target: GLenum, texture: GLuint },
    BlendFunc { sfactor: GLenum, dfactor: GLenum },
    CallList { list: GLuint },
    Clear { mask: GLbitfield },
    ClearColor { red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat },
    Color3f { red: GLfloat, green: GLfloat, blue: GLfloat },
    Color4f { red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat },
    CullFace { mode: GLenum },
    DepthMask { flag: GLboolean },
    Disable { cap: GLenum },
    Enable { cap: GLenum },
    End,
    Lightf { light: GLenum, pname: GLenum, param: GLfloat },
    LoadIdentity,
    MatrixMode { mode: GLenum },
    MultiTexCoord2fARB { target: GLenum, s: GLfloat, t: GLfloat },
    MultMatrixd { m: [GLdouble; 16] },
    MultMatrixf { m: [GLfloat; 16] },
    Normal3f { nx: GLfloat, ny: GLfloat, nz: GLfloat },
    Normal3fv { v: [GLfloat; 3] },
    Ortho { left: GLdouble, right: GLdouble, bottom: GLdouble, top: GLdouble, zNear: GLdouble, zFar: GLdouble },
    PolygonMode { face: GLenum, mode: GLenum },
    PopMatrix,
    PushMatrix,
    Rotatef { angle: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat },
    ShadeModel { mode: GLenum },
    TexCoord2f { s: GLfloat, t: GLfloat },
    TexGenf { coord: GLenum, pname: GLenum, param: GLfloat },
    TexGeni { coord: GLenum, pname: GLenum, param: GLint },
    TexParameteri { target: GLenum, pname: GLenum, param: GLint },
    Translated { x: GLdouble, y: GLdouble, z: GLdouble },
    Translatef { x: GLfloat, y: GLfloat, z: GLfloat },
    Vertex3f { x: GLfloat, y: GLfloat, z: GLfloat },
    Viewport { x: GLint, y: GLint, width: GLsizei, height: GLsizei },
}

struct PatchedFunction {
    original_addr: *mut u8,
    restore_data: [u8; 5],
}

impl PatchedFunction {
    fn new(original_addr: *mut u8, patch_addr: *const u8) -> PatchedFunction {
        unsafe {
            let mut old_protection = 0;
            if VirtualProtect(original_addr as _, 5, PAGE_READWRITE, &mut old_protection as *mut _) == FALSE {
                panic!("Couldn't make memory region readable/writable");
            }

            let patch_region = slice::from_raw_parts_mut(original_addr, 5);

            let mut restore_data = [0; 5];
            restore_data.copy_from_slice(patch_region);

            let rel_addr = (patch_addr as usize).wrapping_sub((original_addr as usize).wrapping_add(5));
            patch_region[0] = 0xe9; // JMP rel
            patch_region[1..].copy_from_slice(&rel_addr.to_le_bytes());

            if VirtualProtect(original_addr as _, 5, old_protection, &mut old_protection as *mut _) == FALSE {
                panic!("Couldn't restore memory region protection");
            }

            PatchedFunction {
                original_addr,
                restore_data,
            }
        }
    }
}

impl Drop for PatchedFunction {
    fn drop(&mut self) {
        unsafe {
            let mut old_protection = 0;
            if VirtualProtect(self.original_addr as _, 5, PAGE_READWRITE, &mut old_protection as *mut _) == FALSE {
                panic!("Couldn't make memory region readable/writable");
            }

            let patch_region = slice::from_raw_parts_mut(self.original_addr, 5);
            patch_region.copy_from_slice(&self.restore_data);

            if VirtualProtect(self.original_addr as _, 5, old_protection, &mut old_protection as *mut _) == FALSE {
                panic!("Couldn't restore memory region protection");
            }
        }
    }
}

struct Context {
    window: Window,
    back_buffer: Vec<u32>,
    depth_buffer: Vec<u16>,

    _swap_buffers: PatchedFunction,
    _change_display_settings: PatchedFunction,

    clear_color_red: GLfloat,
    clear_color_green: GLfloat,
    clear_color_blue: GLfloat,
    clear_color_alpha: GLfloat,

    blend_enable: bool,
    blend_src_factor: BlendSrcFactor,
    blend_dst_factor: BlendDstFactor,

    depth_test: bool,
    depth_mask: bool,

    display_lists: Vec<Rc<RefCell<DisplayList>>>,
    new_list: Option<GLuint>,
    new_list_mode: GLenum,

    modelview: Matrix,
    projection: Matrix,
    matrix_mode: MatrixMode,
    matrix_stack: Vec<Matrix>,

    textures: Vec<Texture>,
    texture_2d: GLuint,
    texture_2d_enable: bool,

    primitive_mode: Option<PrimitiveMode>,

    current_normal: Vec3,
    current_color: Vec4,
    current_tex_coord: Vec4,

    verts: Vec<Vertex>,

    unpack_swap_bytes: GLint,
    unpack_lsb_first: GLint,
    unpack_row_length: GLint,
    unpack_skip_rows: GLint,
    unpack_skip_pixels: GLint,
    unpack_alignment: GLint,
    pack_swap_bytes: GLint,
    pack_lsb_first: GLint,
    pack_row_length: GLint,
    pack_skip_rows: GLint,
    pack_skip_pixels: GLint,
    pack_alignment: GLint,

    vertex_array_enabled: bool,
    vertex_pointer: *const GLvoid,
    vertex_size: GLint,
    vertex_type: GLenum,
    vertex_stride: GLsizei,

    normal_array_enabled: bool,
    normal_pointer: *const GLvoid,
    normal_type: GLenum,
    normal_stride: GLsizei,

    viewport_x: GLint,
    viewport_y: GLint,
    viewport_width: GLsizei,
    viewport_height: GLsizei,
}

impl Context {
    fn new() -> Context {
        Context {
            window: Window::new("gloat waddup", WIDTH, HEIGHT, WindowOptions::default()).expect("Could not create output window"),
            back_buffer: vec![0; WIDTH * HEIGHT],
            depth_buffer: vec![65535; WIDTH * HEIGHT],

            _swap_buffers: PatchedFunction::new(SwapBuffers as _, swap_buffers as _),
            _change_display_settings: PatchedFunction::new(ChangeDisplaySettingsExA as _, change_display_settings_ex_a as _),

            clear_color_red: 0.0,
            clear_color_green: 0.0,
            clear_color_blue: 0.0,
            clear_color_alpha: 0.0,

            blend_enable: false,
            // TODO: I wasn't able to find defaults for these factors, so they may be incorrect
            blend_src_factor: BlendSrcFactor::SrcAlpha,
            blend_dst_factor: BlendDstFactor::OneMinusSrcAlpha,

            depth_test: false,
            depth_mask: true,

            display_lists: Vec::new(),
            new_list: None,
            new_list_mode: 0,

            modelview: Matrix::identity(),
            projection: Matrix::identity(),
            matrix_mode: MatrixMode::ModelView,
            matrix_stack: Vec::new(),

            textures: Vec::new(),
            texture_2d: 0,
            texture_2d_enable: false,

            primitive_mode: None,

            current_normal: Vec3::zero(),
            current_color: Vec4::new(0.0, 0.0, 0.0, 1.0),
            current_tex_coord: Vec4::new(0.0, 0.0, 0.0, 1.0),

            verts: Vec::new(),

            unpack_swap_bytes: 0,
            unpack_lsb_first: 0,
            unpack_row_length: 0,
            unpack_skip_rows: 0,
            unpack_skip_pixels: 0,
            unpack_alignment: 4,
            pack_swap_bytes: 0,
            pack_lsb_first: 0,
            pack_row_length: 0,
            pack_skip_rows: 0,
            pack_skip_pixels: 0,
            pack_alignment: 4,

            normal_array_enabled: false,
            normal_pointer: ptr::null(),
            normal_type: 0,
            normal_stride: 0,

            vertex_array_enabled: false,
            vertex_pointer: ptr::null(),
            vertex_size: 0,
            vertex_type: 0,
            vertex_stride: 0,

            viewport_x: 0,
            viewport_y: 0,
            viewport_width: WIDTH as _,
            viewport_height: HEIGHT as _,
        }
    }

    fn current_matrix(&self) -> Matrix {
        match self.matrix_mode {
            MatrixMode::ModelView => self.modelview,
            MatrixMode::Projection => self.projection,
        }
    }

    fn set_current_matrix(&mut self, m: Matrix) {
        match self.matrix_mode {
            MatrixMode::ModelView => {
                self.modelview = m;
            }
            MatrixMode::Projection => {
                self.projection = m;
            }
        }
    }

    fn multiply_current_matrix(&mut self, m: Matrix) {
        self.set_current_matrix(self.current_matrix() * m);
    }

    fn array_element(&mut self, index: GLint) {
        if self.normal_array_enabled {
            // TODO: Properly handle type, stride
            let normal_buffer = self.normal_pointer as *const GLfloat;
            unsafe {
                let normal = normal_buffer.add((index * 3) as usize);
                let nx = *normal.add(0);
                let ny = *normal.add(1);
                let nz = *normal.add(2);
                self.issue(Command::Normal3f { nx, ny, nz });
            }
        }
        if self.vertex_array_enabled {
            // TODO: Properly handle size, type, stride
            let vertex_buffer = self.vertex_pointer as *const GLfloat;
            unsafe {
                let vertex = vertex_buffer.add((index * self.vertex_size) as usize);
                let x = *vertex.add(0);
                let y = *vertex.add(1);
                let z = *vertex.add(2);
                self.issue(Command::Vertex3f { x, y, z });
            }
        }
    }

    fn assemble_triangle(&mut self, verts: [Vertex; 3]) {
        // TODO: Clipping, culling, ...
        for vert in verts.iter() {
            if vert.position.z() < -vert.position.w() || vert.position.z() > vert.position.w() {
                return;
            }
        }

        // Viewport transform
        let mut window_verts = [Vec3::zero(); 3];
        for i in 0..3 {
            let clip = verts[i].position;
            let ndc = Vec3::new(clip.x(), clip.y(), clip.z()) / clip.w();
            let viewport_near = 0.0;
            let viewport_far = 1.0;
            let viewport_scale = Vec3::new(self.viewport_width as f32 / 2.0, self.viewport_height as f32 / 2.0, (viewport_far - viewport_near) / 2.0);
            let viewport_bias = Vec3::new(self.viewport_x as f32 + self.viewport_width as f32 / 2.0, self.viewport_y as f32 + self.viewport_height as f32 / 2.0, (viewport_far + viewport_near) / 2.0);
            window_verts[i] = ndc * viewport_scale + viewport_bias;
        }

        // TODO: Use properly interpolated color
        let src_color = verts[0].color;

        let mut bb_min = Vec2::new(window_verts[0].x(), window_verts[0].y());
        let mut bb_max = bb_min;
        for i in 1..verts.len() {
            bb_min = bb_min.min(Vec2::new(window_verts[i].x(), window_verts[i].y()));
            bb_max = bb_max.max(Vec2::new(window_verts[i].x(), window_verts[i].y()));
        }
        bb_min = bb_min.max(Vec2::new(self.viewport_x as f32, self.viewport_y as f32));
        bb_max = bb_max.min(Vec2::new((self.viewport_x + self.viewport_width as i32 - 1) as f32, (self.viewport_y + self.viewport_height as i32 - 1) as f32));
        bb_min = bb_min.max(Vec2::zero());
        bb_max = bb_max.min(Vec2::new((WIDTH - 1) as f32, (HEIGHT - 1) as f32));
        let bb_min_x = bb_min.x().floor() as i32;
        let bb_min_y = bb_min.y().floor() as i32;
        let bb_max_x = bb_max.x().ceil() as i32;
        let bb_max_y = bb_max.y().ceil() as i32;

        fn orient2d(a: Vec2, b: Vec2, c: Vec2) -> f32 {
            (b.x() - a.x()) * (c.y() - a.y()) - (b.y() - a.y()) * (c.x() - a.x())
        }

        let scaled_area = orient2d(
            Vec2::new(window_verts[0].x(), window_verts[0].y()),
            Vec2::new(window_verts[1].x(), window_verts[1].y()),
            Vec2::new(window_verts[2].x(), window_verts[2].y()));

        let p = Vec2::new(bb_min_x as f32, bb_min_y as f32) + 0.5; // Offset to sample pixel centers

        // TODO: Proper top/left fill rule, perspective correction
        let w0_min = orient2d(Vec2::new(window_verts[1].x(), window_verts[1].y()), Vec2::new(window_verts[2].x(), window_verts[2].y()), p) / scaled_area;
        let w1_min = orient2d(Vec2::new(window_verts[2].x(), window_verts[2].y()), Vec2::new(window_verts[0].x(), window_verts[0].y()), p) / scaled_area;
        let w2_min = orient2d(Vec2::new(window_verts[0].x(), window_verts[0].y()), Vec2::new(window_verts[1].x(), window_verts[1].y()), p) / scaled_area;
        let w0_dx = (window_verts[1].y() - window_verts[2].y()) / scaled_area;
        let w1_dx = (window_verts[2].y() - window_verts[0].y()) / scaled_area;
        let w2_dx = (window_verts[0].y() - window_verts[1].y()) / scaled_area;
        let w0_dy = (window_verts[2].x() - window_verts[1].x()) / scaled_area;
        let w1_dy = (window_verts[0].x() - window_verts[2].x()) / scaled_area;
        let w2_dy = (window_verts[1].x() - window_verts[0].x()) / scaled_area;
        let z_min = window_verts[0].z() * w0_min + window_verts[1].z() * w1_min + window_verts[2].z() * w2_min;
        let z_dx = window_verts[0].z() * w0_dx + window_verts[1].z() * w1_dx + window_verts[2].z() * w2_dx;
        let z_dy = window_verts[0].z() * w0_dy + window_verts[1].z() * w1_dy + window_verts[2].z() * w2_dy;
        let s_min = verts[0].tex_coord.x() * w0_min + verts[1].tex_coord.x() * w1_min + verts[2].tex_coord.x() * w2_min;
        let t_min = verts[0].tex_coord.y() * w0_min + verts[1].tex_coord.y() * w1_min + verts[2].tex_coord.y() * w2_min;
        let s_dx = verts[0].tex_coord.x() * w0_dx + verts[1].tex_coord.x() * w1_dx + verts[2].tex_coord.x() * w2_dx;
        let t_dx = verts[0].tex_coord.y() * w0_dx + verts[1].tex_coord.y() * w1_dx + verts[2].tex_coord.y() * w2_dx;
        let s_dy = verts[0].tex_coord.x() * w0_dy + verts[1].tex_coord.x() * w1_dy + verts[2].tex_coord.x() * w2_dy;
        let t_dy = verts[0].tex_coord.y() * w0_dy + verts[1].tex_coord.y() * w1_dy + verts[2].tex_coord.y() * w2_dy;

        let mut w0_row = w0_min;
        let mut w1_row = w1_min;
        let mut w2_row = w2_min;
        let mut z_row = z_min;
        let mut s_row = s_min;
        let mut t_row = t_min;

        for y in bb_min_y..bb_max_y + 1 {
            let mut w0 = w0_row;
            let mut w1 = w1_row;
            let mut w2 = w2_row;
            let mut z = z_row;
            let mut s = s_row;
            let mut t = t_row;

            for x in bb_min_x..bb_max_x + 1 {
                if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                    let z = (z * 65535.0).floor() as u16;
                    let buffer_index = (HEIGHT - 1 - y as usize) * WIDTH + x as usize;
                    if !self.depth_test || z < self.depth_buffer[buffer_index] {
                        let src_color = if self.texture_2d_enable && (self.texture_2d as usize) < self.textures.len() {
                            let fragment_tex_coord = Vec2::new(s, t);
                            let texture = &self.textures[self.texture_2d as usize];
                            // Offset to sample texel centers
                            let u = fragment_tex_coord.x() * texture.width as f32 - 0.5;
                            let v = fragment_tex_coord.y() * texture.height as f32 - 0.5;
                            let u_floor = u.floor() as usize;
                            let v_floor = v.floor() as usize;
                            let u_fract = u - u.floor();
                            let v_fract = v - v.floor();
                            fn fetch_texel(texture: &Texture, u: usize, v: usize) -> Vec4 {
                                let u = u & (texture.width - 1);
                                let v = v & (texture.height - 1);
                                let texel = texture.data[v * texture.width + u];
                                let texel_red = (texel >> 16) & 0xff;
                                let texel_green = (texel >> 8) & 0xff;
                                let texel_blue = (texel >> 0) & 0xff;
                                let texel_alpha = (texel >> 24) & 0xff;
                                Vec4::new(texel_red as f32, texel_green as f32, texel_blue as f32, texel_alpha as f32) / 255.0
                            }
                            let texel_color0 = fetch_texel(texture, u_floor + 0, v_floor + 0);
                            let texel_color1 = fetch_texel(texture, u_floor + 1, v_floor + 0);
                            let texel_color2 = fetch_texel(texture, u_floor + 0, v_floor + 1);
                            let texel_color3 = fetch_texel(texture, u_floor + 1, v_floor + 1);
                            let a = texel_color0 * (1.0 - u_fract) + texel_color1 * u_fract;
                            let b = texel_color2 * (1.0 - u_fract) + texel_color3 * u_fract;
                            let filtered_texel = a * (1.0 - v_fract) + b * v_fract;
                            src_color * filtered_texel
                        } else {
                            src_color
                        };

                        let color = if self.blend_enable {
                            let src_scale_factors = match self.blend_src_factor {
                                BlendSrcFactor::Zero => Vec4::zero(),
                                BlendSrcFactor::SrcColor => src_color,
                                BlendSrcFactor::SrcAlpha => Vec4::splat(src_color.w()),
                            };

                            let dst_color = self.back_buffer[buffer_index];
                            let dst_red = (dst_color >> 16) & 0xff;
                            let dst_green = (dst_color >> 8) & 0xff;
                            let dst_blue = (dst_color >> 0) & 0xff;
                            let dst_alpha = (dst_color >> 24) & 0xff;
                            let dst_color = Vec4::new(dst_red as f32, dst_green as f32, dst_blue as f32, dst_alpha as f32) / 255.0;
                            let dst_scale_factors = match self.blend_dst_factor {
                                BlendDstFactor::One => Vec4::splat(1.0),
                                BlendDstFactor::SrcAlpha => Vec4::splat(src_color.w()),
                                BlendDstFactor::OneMinusSrcAlpha => Vec4::splat(1.0 - src_color.w()),
                            };

                            src_color * src_scale_factors + dst_color * dst_scale_factors
                        } else {
                            src_color
                        };

                        let color = color.max(Vec4::zero()).min(Vec4::splat(1.0)) * 255.0;
                        let color_red = color.x().floor() as u32;
                        let color_green = color.y().floor() as u32;
                        let color_blue = color.z().floor() as u32;
                        let color_alpha = color.w().floor() as u32;
                        self.back_buffer[buffer_index] = (color_alpha << 24) | (color_red << 16) | (color_green << 8) | (color_blue << 0);

                        if self.depth_mask {
                            self.depth_buffer[buffer_index] = z;
                        }
                    }
                }

                w0 += w0_dx;
                w1 += w1_dx;
                w2 += w2_dx;
                z += z_dx;
                s += s_dx;
                t += t_dx;
            }

            w0_row += w0_dy;
            w1_row += w1_dy;
            w2_row += w2_dy;
            z_row += z_dy;
            s_row += s_dy;
            t_row += t_dy;
        }
    }

    fn disable_client_state(&mut self, array: GLenum) {
        match array {
            GL_VERTEX_ARRAY => {
                self.vertex_array_enabled = false;
            }
            GL_NORMAL_ARRAY => {
                self.normal_array_enabled = false;
            }
            _ => panic!("DisableClientState called with invalid array: 0x{:08x}", array)
        }
    }

    fn enable_client_state(&mut self, array: GLenum) {
        match array {
            GL_VERTEX_ARRAY => {
                self.vertex_array_enabled = true;
            }
            GL_NORMAL_ARRAY => {
                self.normal_array_enabled = true;
            }
            _ => panic!("EnableClientState called with invalid array: 0x{:08x}", array)
        }
    }

    fn end_list(&mut self) {
        self.new_list = None;
    }

    fn execute(&mut self, command: &Command) {
        match *command {
            Command::ActiveTextureARB { texture } => {
                // TODO
                println!("ActiveTextureARB: texture: 0x{:08x}", texture);
            }
            Command::Begin { mode } => {
                if self.primitive_mode.is_some() {
                    panic!("glBegin called twice with no glEnd call");
                }
                self.primitive_mode = Some(match mode {
                    GL_TRIANGLES => PrimitiveMode::Triangles,
                    GL_QUADS => PrimitiveMode::Quads,
                    _ => panic!("glBegin called with invalid mode: 0x{:08x}", mode)
                });
            }
            Command::BindTexture { target, texture } => {
                match target {
                    GL_TEXTURE_2D => {
                        self.texture_2d = texture;
                    }
                    _ => panic!("glBindTexture called with invalid target: 0x{:08x}", target)
                }
            }
            Command::BlendFunc { sfactor, dfactor } => {
                self.blend_src_factor = match sfactor {
                    GL_ZERO => BlendSrcFactor::Zero,
                    GL_SRC_COLOR => BlendSrcFactor::SrcColor,
                    GL_SRC_ALPHA => BlendSrcFactor::SrcAlpha,
                    _ => panic!("glBlendFunc called with invalid sfactor: 0x{:08x}", sfactor)
                };
                self.blend_dst_factor = match dfactor {
                    GL_ONE => BlendDstFactor::One,
                    GL_SRC_ALPHA => BlendDstFactor::SrcAlpha,
                    GL_ONE_MINUS_SRC_ALPHA => BlendDstFactor::OneMinusSrcAlpha,
                    _ => panic!("glBlendFunc called with invalid dfactor: 0x{:08x}", dfactor)
                };
            }
            Command::CallList { list } => {
                if (list as usize) < self.display_lists.len() {
                    for command in self.display_lists[list as usize].clone().borrow().commands.iter() {
                        self.execute(command);
                    }
                } else {
                    println!("glCallList called with invalid list: {}", list);
                }
            }
            Command::Clear { mask } => {
                // TODO: Only clear within viewport
                if (mask & GL_DEPTH_BUFFER_BIT) != 0 {
                    for depth in self.depth_buffer.iter_mut() {
                        *depth = 65535;
                    }
                }
                if (mask & GL_COLOR_BUFFER_BIT) != 0 {
                    let clear_value =
                        (((self.clear_color_alpha * 255.0) as u32) << 24) |
                        (((self.clear_color_red * 255.0) as u32) << 16) |
                        (((self.clear_color_green * 255.0) as u32) << 8) |
                        (((self.clear_color_blue * 255.0) as u32) << 0);
                    for pixel in self.back_buffer.iter_mut() {
                        *pixel = clear_value;
                    }
                }
                println!("Clear: mask: 0x{:08x}", mask);
            }
            Command::ClearColor { red, green, blue, alpha } => {
                self.clear_color_red = red;
                self.clear_color_green = green;
                self.clear_color_blue = blue;
                self.clear_color_alpha = alpha;
            }
            Command::Color3f { red, green, blue } => {
                self.current_color = Vec4::new(red, green, blue, 1.0);
            }
            Command::Color4f { red, green, blue, alpha } => {
                self.current_color = Vec4::new(red, green, blue, alpha);
            }
            Command::CullFace { mode } => {
                // TODO
                println!("CullFace: mode: {}", mode);
            }
            Command::DepthMask { flag } => {
                self.depth_mask = match flag {
                    GL_FALSE => false,
                    GL_TRUE => true,
                    _ => panic!("glDepthMask called with invalid flag: 0x{:08x}", flag)
                };
            }
            Command::Disable { cap } => {
                match cap {
                    GL_DEPTH_TEST => {
                        self.depth_test = false;
                    }
                    GL_BLEND => {
                        self.blend_enable = false;
                    }
                    GL_TEXTURE_2D => {
                        self.texture_2d_enable = false;
                    }
                    _ => println!("Disable: cap: 0x{:08x}", cap)
                }
            }
            Command::Enable { cap } => {
                match cap {
                    GL_DEPTH_TEST => {
                        self.depth_test = true;
                    }
                    GL_BLEND => {
                        self.blend_enable = true;
                    }
                    GL_TEXTURE_2D => {
                        self.texture_2d_enable = true;
                    }
                    _ => println!("Enable: cap: 0x{:08x}", cap)
                }
            }
            Command::End => {
                if let Some(primitive_mode) = self.primitive_mode {
                    let verts_per_primitive = match primitive_mode {
                        PrimitiveMode::Triangles => 3,
                        PrimitiveMode::Quads => 4,
                    };
                    if self.verts.len() % verts_per_primitive != 0 {
                        panic!("Incorrect number of vertices specified for primitive type");
                    }
                    for vert in self.verts.iter_mut() {
                        let object = vert.position;
                        let eye = self.modelview * object;
                        let clip = self.projection * eye;
                        vert.position = clip;
                    }
                    for i in (0..self.verts.len()).step_by(verts_per_primitive) {
                        match primitive_mode {
                            PrimitiveMode::Triangles => {
                                self.assemble_triangle([self.verts[i + 0], self.verts[i + 1], self.verts[i + 2]]);
                            }
                            PrimitiveMode::Quads => {
                                self.assemble_triangle([self.verts[i + 0], self.verts[i + 1], self.verts[i + 2]]);
                                self.assemble_triangle([self.verts[i + 2], self.verts[i + 3], self.verts[i + 0]]);
                            }
                        }
                    }
                    self.verts.clear();
                    self.primitive_mode = None;
                } else {
                    panic!("glEnd called with no matching glBegin call");
                }
            }
            Command::Lightf { light, pname, param } => {
                // TODO
                println!("Lightf: light: 0x{:08x}, pname: 0x{:08x}, param: {}", light, pname, param);
            }
            Command::LoadIdentity => {
                self.set_current_matrix(Matrix::identity());
            }
            Command::MatrixMode { mode } => {
                self.matrix_mode = match mode {
                    GL_MODELVIEW => MatrixMode::ModelView,
                    GL_PROJECTION => MatrixMode::Projection,
                    _ => panic!("glMatrixMode called with invalid mode: 0x{:08x}", mode),
                };
            }
            Command::MultiTexCoord2fARB { target, s, t } => {
                // TODO
                //println!("MultiTexCoord2fARB: target: 0x{:08x}, s: {}, t: {}", target, s, t);
                self.current_tex_coord = Vec4::new(s, t, 0.0, 1.0);
            }
            Command::MultMatrixd { m } => {
                self.multiply_current_matrix(Matrix::from_doubles(&m));
            }
            Command::MultMatrixf { m } => {
                self.multiply_current_matrix(Matrix::from_floats(&m));
            }
            Command::Normal3f { nx, ny, nz } => {
                self.current_normal = Vec3::new(nx, ny, nz);
            }
            Command::Normal3fv { v } => {
                self.current_normal = Vec3::new(v[0], v[1], v[2]);
            }
            Command::Ortho { left, right, bottom, top, zNear, zFar } => {
                self.multiply_current_matrix(Matrix::ortho(left as f32, right as f32, bottom as f32, top as f32, zNear as f32, zFar as f32));
            }
            Command::PolygonMode { face, mode } => {
                // TODO
                println!("PolygonMode: face: 0x{:08x}, mode: 0x{:08x}", face, mode);
            }
            Command::PopMatrix => {
                let m = self.matrix_stack.pop().expect("Matrix stack underflow");
                self.set_current_matrix(m);
            }
            Command::PushMatrix => {
                self.matrix_stack.push(self.current_matrix());
            }
            Command::Rotatef { angle, x, y, z } => {
                // TODO
                println!("Rotatef: angle: {}, x: {}, y: {}, z: {}", angle, x, y, z);
            }
            Command::ShadeModel { mode } => {
                // TODO
                println!("ShadeModel: mode: 0x{:08x}", mode);
            }
            Command::TexCoord2f { s, t } => {
                self.current_tex_coord = Vec4::new(s, t, 0.0, 1.0);
            }
            Command::TexGenf { coord, pname, param } => {
                // TODO
                println!("TexGenf: coord: 0x{:08x}, pname: 0x{:08x}, param: {}", coord, pname, param);
            }
            Command::TexGeni { coord, pname, param } => {
                // TODO
                println!("TexGeni: coord: 0x{:08x}, pname: 0x{:08x}, param: {}", coord, pname, param);
            }
            Command::TexParameteri { target, pname, param } => {
                match target {
                    GL_TEXTURE_2D => {
                        let texture = &mut self.textures[self.texture_2d as usize];
                        match pname {
                            // TODO: De-dupe filter param decoding
                            GL_TEXTURE_MAG_FILTER => match param {
                                GL_LINEAR => {
                                    texture.mag_filter = TextureFilter::Linear;
                                }
                                _ => panic!("glTexParameteri called with invalid param for GL_TEXTURE_MAG_FILTER: 0x{:08x}", param)
                            }
                            GL_TEXTURE_MIN_FILTER => match param {
                                GL_NEAREST => {
                                    texture.min_filter = TextureFilter::Nearest;
                                }
                                GL_LINEAR => {
                                    texture.min_filter = TextureFilter::Linear;
                                }
                                GL_LINEAR_MIPMAP_NEAREST => {
                                    texture.min_filter = TextureFilter::LinearMipmapNearest;
                                }
                                GL_LINEAR_MIPMAP_LINEAR => {
                                    texture.min_filter = TextureFilter::LinearMipmapLinear;
                                }
                                _ => panic!("glTexParameteri called with invalid param for GL_TEXTURE_MIN_FILTER: 0x{:08x}", param)
                            }
                            GL_TEXTURE_WRAP_S => match param {
                                GL_CLAMP => {
                                    texture.wrap_s = WrapParameter::Clamp;
                                }
                                _ => panic!("glTexParameteri called with invalid param for GL_TEXTURE_WRAP_S: 0x{:08x}", param)
                            }
                            GL_TEXTURE_WRAP_T => match param {
                                GL_CLAMP => {
                                    texture.wrap_t = WrapParameter::Clamp;
                                }
                                _ => panic!("glTexParameteri called with invalid param for GL_TEXTURE_WRAP_S: 0x{:08x}", param)
                            }
                            _ => panic!("glTexParameteri called with invalid pname: 0x{:08x}", pname)
                        }
                    }
                    _ => panic!("glTexParameteri called with invalid target: 0x{:08x}", target)
                }
            }
            Command::Translated { x, y, z } => {
                self.multiply_current_matrix(Matrix::translation(x as f32, y as f32, z as f32));
            }
            Command::Translatef { x, y, z } => {
                self.multiply_current_matrix(Matrix::translation(x, y, z));
            }
            Command::Vertex3f { x, y, z } => {
                self.verts.push(Vertex {
                    position: Vec4::new(x, y, z, 1.0),
                    normal: self.current_normal,
                    color: self.current_color,
                    tex_coord: self.current_tex_coord,
                });
            }
            Command::Viewport { x, y, width, height } => {
                self.viewport_x = x;
                self.viewport_y = y;
                self.viewport_width = width;
                self.viewport_height = height;
            }
        }
    }

    fn gen_lists(&mut self, range: GLsizei) -> GLuint {
        let ret = self.display_lists.len() as _;
        for _ in 0..range {
            self.display_lists.push(Rc::new(RefCell::new(DisplayList::new())));
        }
        ret
    }

    unsafe fn gen_textures(&mut self, n: GLsizei, textures: *mut GLuint) {
        for i in 0..n {
            *textures.add(i as _) = self.textures.len() as _;
            self.textures.push(Texture::new());
        }
    }

    fn get_integerv(&self, pname: GLenum, params: *mut GLint) {
        match pname {
            GL_UNPACK_SWAP_BYTES => unsafe {
                *params = self.unpack_swap_bytes;
            }
            GL_UNPACK_LSB_FIRST => unsafe {
                *params = self.unpack_lsb_first;
            }
            GL_UNPACK_ROW_LENGTH => unsafe {
                *params = self.unpack_row_length;
            }
            GL_UNPACK_SKIP_ROWS => unsafe {
                *params = self.unpack_skip_rows;
            }
            GL_UNPACK_SKIP_PIXELS => unsafe {
                *params = self.unpack_skip_pixels;
            }
            GL_UNPACK_ALIGNMENT => unsafe {
                *params = self.unpack_alignment;
            }
            GL_PACK_SWAP_BYTES => unsafe {
                *params = self.pack_swap_bytes;
            }
            GL_PACK_LSB_FIRST => unsafe {
                *params = self.pack_lsb_first;
            }
            GL_PACK_ROW_LENGTH => unsafe {
                *params = self.pack_row_length;
            }
            GL_PACK_SKIP_ROWS => unsafe {
                *params = self.pack_skip_rows;
            }
            GL_PACK_SKIP_PIXELS => unsafe {
                *params = self.pack_skip_pixels;
            }
            GL_PACK_ALIGNMENT => unsafe {
                *params = self.pack_alignment;
            }
            GL_MAX_TEXTURE_SIZE => unsafe {
                *params = 4096; // TODO: Is this big enough? :)
            }
            _ => panic!("glGetIntegerv called with invalid pname: 0x{:08x}", pname)
        }
    }

    fn issue(&mut self, command: Command) {
        if let Some(list) = self.new_list {
            if self.new_list_mode == GL_COMPILE_AND_EXECUTE {
                self.execute(&command);
            }
            self.display_lists[list as usize].borrow_mut().commands.push(command);
        } else {
            self.execute(&command);
        }
    }

    fn new_list(&mut self, list: GLuint, mode: GLenum) {
        self.new_list = Some(list);
        self.new_list_mode = match mode {
            GL_COMPILE | GL_COMPILE_AND_EXECUTE => mode,
            _ => panic!("glNewList called with invalid mode: 0x{:08x}", mode)
        };
        self.display_lists[list as usize].borrow_mut().commands.clear();
    }

    fn normal_pointer(&mut self, type_: GLenum, stride: GLsizei, pointer: *const GLvoid) {
        match type_ {
            GL_FLOAT => {
                self.normal_type = type_;
            }
            _ => panic!("NormalPointer called with invalid type: 0x{:08x}", type_)
        }
        match stride {
            0 => {
                self.normal_stride = stride;
            }
            _ => panic!("NormalPointer called with invalid stride: {}", stride)
        }
        self.normal_pointer = pointer;
    }

    fn pixel_storei(&mut self, pname: GLenum, param: GLint) {
        match pname {
            GL_UNPACK_SWAP_BYTES => match param {
                0 | 1 => {
                    self.unpack_swap_bytes = param;
                }
                _ => panic!("glPixelStorei called with invalid param for GL_UNPACK_SWAP_BYTES: 0x{:08x}", param)
            }
            GL_UNPACK_LSB_FIRST => match param {
                0 | 1 => {
                    self.unpack_lsb_first = param;
                }
                _ => panic!("glPixelStorei called with invalid param for GL_UNPACK_LSB_FIRST: 0x{:08x}", param)
            }
            GL_UNPACK_ROW_LENGTH => {
                if param >= 0 {
                    self.unpack_row_length = param;
                } else {
                    panic!("glPixelStorei called with invalid param for GL_UNPACK_ROW_LENGTH: 0x{:08x}", param);
                }
            }
            GL_UNPACK_SKIP_ROWS => {
                if param >= 0 {
                    self.unpack_skip_rows = param;
                } else {
                    panic!("glPixelStorei called with invalid param for GL_UNPACK_SKIP_ROWS: 0x{:08x}", param);
                }
            }
            GL_UNPACK_SKIP_PIXELS => {
                if param >= 0 {
                    self.unpack_skip_pixels = param;
                } else {
                    panic!("glPixelStorei called with invalid param for GL_UNPACK_SKIP_PIXELS: 0x{:08x}", param);
                }
            }
            GL_UNPACK_ALIGNMENT => match param {
                1 | 2 | 4 | 8 => {
                    self.unpack_alignment = param;
                }
                _ => panic!("glPixelStorei called with invalid param for GL_UNPACK_ALIGNMENT: 0x{:08x}", param)
            }
            _ => panic!("glPixelStorei called with invalid pname: 0x{:08x}", pname)
        }
    }

    fn swap_buffers(&mut self, dc: HDC) -> BOOL {
        println!("swap_buffers: dc: 0x{:08x}", dc as u32);
        self.window.update_with_buffer(&self.back_buffer).expect("Couldn't swap buffers");
        TRUE
    }

    fn tex_image_2d(&mut self, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, type_: GLenum, data: *const GLvoid) {
        if target != GL_TEXTURE_2D {
            panic!("glTexImage2D called with invalid target: 0x{:08x}", target);
        }

        if level > 0 {
            println!("Skipping glTexImage2D call with level > 0, level: 0x{:08x}", level);
            return;
        }

        if border != 0 {
            panic!("glTexImage2D called with border != 0, border: 0x{:08x}", border);
        }

        let num_components = match format {
            GL_RGB => 3,
            GL_RGBA => 4,
            _ => panic!("glTexImage2D called with invalid format: 0x{:08x}", format)
        };

        println!("TexImage2D: internalformat: 0x{:08x}, width: 0x{:08x}, height: 0x{:08x}, data: 0x{:08x}", internalformat, width, height, data as u32);

        let texture = &mut self.textures[self.texture_2d as usize];
        texture.width = width as usize;
        texture.height = height as usize;
        texture.data = vec![0; (width * height) as usize];

        for y in 0..height as usize {
            for x in 0..width as usize {
                let buffer_index = y * width as usize + x;
                let color = match type_ {
                    GL_UNSIGNED_BYTE => {
                        let data = unsafe { (data as *const u8).add(buffer_index * num_components) };
                        let red = unsafe { *data.add(0) } as u32;
                        let green = unsafe { *data.add(1) } as u32;
                        let blue = unsafe { *data.add(2) } as u32;
                        let alpha = if num_components == 4 { unsafe { *data.add(3) } } else { 255 } as u32;
                        (alpha << 24) | (red << 16) | (green << 8) | (blue << 0)
                    }
                    GL_UNSIGNED_SHORT => {
                        let data = unsafe { (data as *const u16).add(buffer_index * num_components) };
                        let red = unsafe { *data.add(0) } as u32 >> 8;
                        let green = unsafe { *data.add(1) } as u32 >> 8;
                        let blue = unsafe { *data.add(2) } as u32 >> 8;
                        let alpha = if num_components == 4 { unsafe { *data.add(3) } } else { 65535 } as u32 >> 8;
                        (alpha << 24) | (red << 16) | (green << 8) | (blue << 0)
                    }
                    _ => panic!("glTexImage2D called with invalid type: 0x{:08x}", type_)
                };
                texture.data[buffer_index] = color;
            }
        }
    }

    fn vertex_pointer(&mut self, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const GLvoid) {
        match size {
            3 => {
                self.vertex_size = size;
            }
            _ => panic!("VertexPointer called with invalid size: {}", size)
        }
        match type_ {
            GL_FLOAT => {
                self.vertex_type = type_;
            }
            _ => panic!("VertexPointer called with invalid type: 0x{:08x}", type_)
        }
        match stride {
            0 => {
                self.vertex_stride = stride;
            }
            _ => panic!("VertexPointer called with invalid stride: {}", stride)
        }
        self.vertex_pointer = pointer;
    }
}

static mut CONTEXT: Option<Context> = None;

fn context() -> &'static mut Context {
    unsafe {
        if CONTEXT.is_none() {
            CONTEXT = Some(Context::new());
        }
        CONTEXT.as_mut().unwrap()
    }
}

#[no_mangle]
pub extern "stdcall" fn glActiveTextureARB(texture: GLenum) {
    context().issue(Command::ActiveTextureARB { texture });
}

#[no_mangle]
pub extern "stdcall" fn glArrayElement(index: GLint) {
    context().array_element(index);
}

#[no_mangle]
pub extern "stdcall" fn glBegin(mode: GLenum) {
    context().issue(Command::Begin { mode });
}

#[no_mangle]
pub extern "stdcall" fn glBindTexture(target: GLenum, texture: GLuint) {
    context().issue(Command::BindTexture { target, texture });
}

#[no_mangle]
pub extern "stdcall" fn glBitmap(_width: GLsizei, _height: GLsizei, _xorig: GLfloat, _yorig: GLfloat, _xmove: GLfloat, _ymove: GLfloat, _bitmap: *const GLubyte) {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glBlendFunc(sfactor: GLenum, dfactor: GLenum) {
    context().issue(Command::BlendFunc { sfactor, dfactor });
}

#[no_mangle]
pub extern "stdcall" fn glCallList(list: GLuint) {
    context().issue(Command::CallList { list });
}

#[no_mangle]
pub extern "stdcall" fn glClear(mask: GLbitfield) {
    context().issue(Command::Clear { mask });
}

#[no_mangle]
pub extern "stdcall" fn glClearColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) {
    context().issue(Command::ClearColor { red, green, blue, alpha });
}

#[no_mangle]
pub extern "stdcall" fn glClearDepth(_depth: GLdouble) {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glClientActiveTextureARB(_texture: GLenum) {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glColor3f(red: GLfloat, green: GLfloat, blue: GLfloat) {
    context().issue(Command::Color3f { red, green, blue });
}

#[no_mangle]
pub extern "stdcall" fn glColor4f(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) {
    context().issue(Command::Color4f { red, green, blue, alpha });
}

#[no_mangle]
pub extern "stdcall" fn glColor4fv(_v: *const GLfloat) {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glCopyTexSubImage2D(_target: GLenum, _level: GLint, _xoffset: GLint, _yoffset: GLint, _x: GLint, _y: GLint, _width: GLsizei, _height: GLsizei) {
    // TODO!
    //unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glCullFace(mode: GLenum) {
    context().issue(Command::CullFace { mode });
}

#[no_mangle]
pub extern "stdcall" fn glDeleteLists(_list: GLuint, _range: GLsizei) {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glDeleteTextures(_n: GLsizei, _textures: *const GLuint) {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glDepthFunc(_func: GLenum) {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glDepthMask(flag: GLboolean) {
    context().issue(Command::DepthMask { flag });
}

#[no_mangle]
pub extern "stdcall" fn glDisable(cap: GLenum) {
    context().issue(Command::Disable { cap });
}

#[no_mangle]
pub extern "stdcall" fn glDisableClientState(array: GLenum) {
    context().disable_client_state(array);
}

#[no_mangle]
pub extern "stdcall" fn glDrawElements(_mode: GLenum, _count: GLsizei, _type_: GLenum, _indices: *const GLvoid) {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glDrawBuffer(_mode: GLenum) {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glEnable(cap: GLenum) {
    context().issue(Command::Enable { cap });
}

#[no_mangle]
pub extern "stdcall" fn glEnableClientState(array: GLenum) {
    context().enable_client_state(array);
}

#[no_mangle]
pub extern "stdcall" fn glEnd() {
    context().issue(Command::End);
}

#[no_mangle]
pub extern "stdcall" fn glEndList() {
    context().end_list();
}

#[no_mangle]
pub extern "stdcall" fn glEvalCoord1f(_u: GLfloat) {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glEvalCoord2f(_u: GLfloat, _v: GLfloat) {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glEvalMesh1(_mode: GLenum, _i1: GLint, _i2: GLint) {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glEvalMesh2(_mode: GLenum, _i1: GLint, _i2: GLint, _j1: GLint, _j2: GLint) {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glEvalPoint2(_i: GLint, _j: GLint) {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glFinish() {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glFogf(_pname: GLenum, _param: GLfloat) {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glFogfv(_pname: GLenum, _params: *const GLfloat) {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glFogi(_pname: GLenum, _param: GLint) {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glFrontFace(_mode: GLenum) {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glFrustum(_left: GLdouble, _right: GLdouble, _bottom: GLdouble, _top: GLdouble, _zNear: GLdouble, _zFar: GLdouble) {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glGenLists(range: GLsizei) -> GLuint {
    context().gen_lists(range)
}

#[no_mangle]
pub unsafe extern "stdcall" fn glGenTextures(n: GLsizei, textures: *mut GLuint) {
    context().gen_textures(n, textures);
}

#[no_mangle]
pub extern "stdcall" fn glGetError() -> GLenum {
    // Since we always just panic on error, we'll never have anything to report
    GL_NO_ERROR
}

#[no_mangle]
pub extern "stdcall" fn glGetFloatv(_pname: GLenum, _params: *mut GLfloat) {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glGetIntegerv(pname: GLenum, params: *mut GLint) {
    context().get_integerv(pname, params);
}

#[no_mangle]
pub extern "stdcall" fn glGetString(_name: GLenum) -> *const GLubyte {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glLightf(light: GLenum, pname: GLenum, param: GLfloat) {
    context().issue(Command::Lightf { light, pname, param });
}

#[no_mangle]
pub extern "stdcall" fn glLightfv(_light: GLenum, _pname: GLenum, _params: *const GLfloat) {
    // TODO: Parameter unpacking needs to happen here
    println!("Lightfv");
}

#[no_mangle]
pub extern "stdcall" fn glLightModelfv(_pname: GLenum, _params: *const GLfloat) {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glLightModeli(_pname: GLenum, _param: GLint) {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glLineWidth(_width: GLfloat) {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glLoadIdentity() {
    context().issue(Command::LoadIdentity);
}

#[no_mangle]
pub extern "stdcall" fn glLoadMatrixf(_m: *const GLfloat) {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glMap1f(_target: GLenum, _u1: GLfloat, _u2: GLfloat, _stride: GLint, _order: GLint, _points: *const GLfloat) {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glMap2f(_target: GLenum, _u1: GLfloat, _u2: GLfloat, _ustride: GLint, _uorder: GLint, _v1: GLfloat, _v2: GLfloat, _vstride: GLint, _vorder: GLint, _points: *const GLfloat) {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glMapGrid1f(_un: GLint, _u1: GLfloat, _u2: GLfloat) {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glMapGrid2d(_un: GLint, _u1: GLdouble, _u2: GLdouble, _vn: GLint, _v1: GLdouble, _v2: GLdouble) {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glMapGrid2f(_un: GLint, _u1: GLfloat, _u2: GLfloat, _vn: GLint, _v1: GLfloat, _v2: GLfloat) {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glMaterialf(_face: GLenum, _pname: GLenum, _param: GLfloat) {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glMaterialfv(_face: GLenum, _pname: GLenum, _params: *const GLfloat) {
    // TODO: Parameter unpacking needs to happen here
    println!("Materialfv");
}

#[no_mangle]
pub extern "stdcall" fn glMatrixMode(mode: GLenum) {
    context().issue(Command::MatrixMode { mode });
}

extern "stdcall" fn glMultiTexCoord1dEXT(_target: GLenum, _s: GLdouble) {
    unimplemented!()
}

extern "stdcall" fn glMultiTexCoord1dvARB(_target: GLenum, _v: *const GLfloat) {
    unimplemented!()
}

extern "stdcall" fn glMultiTexCoord1fARB(_target: GLenum, _s: GLfloat) {
    unimplemented!()
}

extern "stdcall" fn glMultiTexCoord1fvARB(_target: GLenum, _v: *const GLfloat) {
    unimplemented!()
}

extern "stdcall" fn glMultiTexCoord1iARB(_target: GLenum, _s: GLint) {
    unimplemented!()
}

extern "stdcall" fn glMultiTexCoord1ivARB(_target: GLenum, _v: *const GLint) {
    unimplemented!()
}

extern "stdcall" fn glMultiTexCoord1sARB(_target: GLenum, _s: GLshort) {
    unimplemented!()
}

extern "stdcall" fn glMultiTexCoord1svARB(_target: GLenum, _v: *const GLshort) {
    unimplemented!()
}

extern "stdcall" fn glMultiTexCoord2dARB(_target: GLenum, _s: GLdouble, _t: GLdouble) {
    unimplemented!()
}

extern "stdcall" fn glMultiTexCoord2dvARB(_target: GLenum, _v: *const GLdouble) {
    unimplemented!()
}

extern "stdcall" fn glMultiTexCoord2fARB(target: GLenum, s: GLfloat, t: GLfloat) {
    context().issue(Command::MultiTexCoord2fARB { target, s, t });
}

extern "stdcall" fn glMultiTexCoord2fvARB(_target: GLenum, _v: *const GLfloat) {
    unimplemented!()
}

extern "stdcall" fn glMultiTexCoord2iARB(_target: GLenum, _s: GLint, _t: GLint) {
    unimplemented!()
}

extern "stdcall" fn glMultiTexCoord2ivARB(_target: GLenum, _v: *const GLint) {
    unimplemented!()
}

extern "stdcall" fn glMultiTexCoord2sARB(_target: GLenum, _s: GLshort, _t: GLshort) {
    unimplemented!()
}

extern "stdcall" fn glMultiTexCoord2svARB(_target: GLenum, _v: *const GLshort) {
    unimplemented!()
}

extern "stdcall" fn glMultiTexCoord3dARB(_target: GLenum, _s: GLdouble, _t: GLdouble, _r: GLdouble) {
    unimplemented!()
}

extern "stdcall" fn glMultiTexCoord3dvARB(_target: GLenum, _v: *const GLdouble) {
    unimplemented!()
}

extern "stdcall" fn glMultiTexCoord3fARB(_target: GLenum, _s: GLfloat, _t: GLfloat, _r: GLfloat) {
    unimplemented!()
}

extern "stdcall" fn glMultiTexCoord3fvARB(_target: GLenum, _v: *const GLfloat) {
    unimplemented!()
}

extern "stdcall" fn glMultiTexCoord3iARB(_target: GLenum, _s: GLint, _t: GLint, _r: GLint) {
    unimplemented!()
}

extern "stdcall" fn glMultiTexCoord3ivARB(_target: GLenum, _v: *const GLint) {
    unimplemented!()
}

extern "stdcall" fn glMultiTexCoord3sARB(_target: GLenum, _s: GLshort, _t: GLshort, _r: GLshort) {
    unimplemented!()
}

extern "stdcall" fn glMultiTexCoord3svARB(_target: GLenum, _v: *const GLshort) {
    unimplemented!()
}

extern "stdcall" fn glMultiTexCoord4dARB(_target: GLenum, _s: GLdouble, _t: GLdouble, _r: GLdouble, _q: GLdouble) {
    unimplemented!()
}

extern "stdcall" fn glMultiTexCoord4dvARB(_target: GLenum, _v: *const GLdouble) {
    unimplemented!()
}

extern "stdcall" fn glMultiTexCoord4fARB(_target: GLenum, _s: GLfloat, _t: GLfloat, _r: GLfloat, _q: GLfloat) {
    unimplemented!()
}

extern "stdcall" fn glMultiTexCoord4fvARB(_target: GLenum, _v: *const GLfloat) {
    unimplemented!()
}

extern "stdcall" fn glMultiTexCoord4iARB(_target: GLenum, _s: GLint, _t: GLint, _r: GLint, _q: GLint) {
    unimplemented!()
}

extern "stdcall" fn glMultiTexCoord4ivARB(_target: GLenum, _v: *const GLint) {
    unimplemented!()
}

extern "stdcall" fn glMultiTexCoord4sARB(_target: GLenum, _s: GLshort, _t: GLshort, _r: GLshort, _q: GLshort) {
    unimplemented!()
}

extern "stdcall" fn glMultiTexCoord4svARB(_target: GLenum, _v: *const GLshort) {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glMultMatrixd(m: *const GLdouble) {
    let mut m_copy = [0.0; 16];
    m_copy.copy_from_slice(unsafe { slice::from_raw_parts(m, 16) });
    context().issue(Command::MultMatrixd { m: m_copy });
}

#[no_mangle]
pub extern "stdcall" fn glMultMatrixf(m: *const GLfloat) {
    let mut m_copy = [0.0; 16];
    m_copy.copy_from_slice(unsafe { slice::from_raw_parts(m, 16) });
    context().issue(Command::MultMatrixf { m: m_copy });
}

#[no_mangle]
pub extern "stdcall" fn glNewList(list: GLuint, mode: GLenum) {
    context().new_list(list, mode);
}

#[no_mangle]
pub extern "stdcall" fn glNormal3f(nx: GLfloat, ny: GLfloat, nz: GLfloat) {
    context().issue(Command::Normal3f { nx, ny, nz });
}

#[no_mangle]
pub extern "stdcall" fn glNormal3fv(v: *const GLfloat) {
    let mut v_copy = [0.0; 3];
    v_copy.copy_from_slice(unsafe { slice::from_raw_parts(v, 3) });
    context().issue(Command::Normal3fv { v: v_copy });
}

#[no_mangle]
pub extern "stdcall" fn glNormalPointer(type_: GLenum, stride: GLsizei, pointer: *const GLvoid) {
    context().normal_pointer(type_, stride, pointer);
}

#[no_mangle]
pub extern "stdcall" fn glOrtho(left: GLdouble, right: GLdouble, bottom: GLdouble, top: GLdouble, zNear: GLdouble, zFar: GLdouble) {
    context().issue(Command::Ortho { left, right, bottom, top, zNear, zFar });
}

#[no_mangle]
pub extern "stdcall" fn glPixelStorei(pname: GLenum, param: GLint) {
    context().pixel_storei(pname, param);
}

#[no_mangle]
pub extern "stdcall" fn glPolygonMode(face: GLenum, mode: GLenum) {
    context().issue(Command::PolygonMode { face, mode });
}

#[no_mangle]
pub extern "stdcall" fn glPopAttrib() {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glPopMatrix() {
    context().issue(Command::PopMatrix);
}

#[no_mangle]
pub extern "stdcall" fn glPushAttrib(_mask: GLbitfield) {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glPushMatrix() {
    context().issue(Command::PushMatrix);
}

#[no_mangle]
pub extern "stdcall" fn glReadBuffer(_mode: GLenum) {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glRotatef(angle: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat) {
    context().issue(Command::Rotatef { angle, x, y, z });
}

#[no_mangle]
pub extern "stdcall" fn glScalef(_x: GLfloat, _y: GLfloat, _z: GLfloat) {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glShadeModel(mode: GLenum) {
    context().issue(Command::ShadeModel { mode });
}

#[no_mangle]
pub extern "stdcall" fn glTexCoord2f(s: GLfloat, t: GLfloat) {
    context().issue(Command::TexCoord2f { s, t });
}

#[no_mangle]
pub extern "stdcall" fn glTexCoord2i(_s: GLint, _t: GLint) {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glTexCoordPointer(_size: GLint, _type_: GLenum, _stride: GLsizei, _pointer: *const GLvoid) {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glTexEnvf(_target: GLenum, _pname: GLenum, _param: GLfloat) {
    // TODO!
    //unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glTexEnvi(_target: GLenum, _pname: GLenum, _param: GLint) {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glTexGenf(coord: GLenum, pname: GLenum, param: GLfloat) {
    context().issue(Command::TexGenf { coord, pname, param });
}

#[no_mangle]
pub extern "stdcall" fn glTexGeni(coord: GLenum, pname: GLenum, param: GLint) {
    context().issue(Command::TexGeni { coord, pname, param });
}

#[no_mangle]
pub extern "stdcall" fn glTexParameteri(target: GLenum, pname: GLenum, param: GLint) {
    context().issue(Command::TexParameteri { target, pname, param });
}

#[no_mangle]
pub extern "stdcall" fn glTexImage1D(_target: GLenum, _level: GLint, _internalformat: GLint, _width: GLsizei, _border: GLint, _format: GLenum, _type: GLenum, _data: *const GLvoid) {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glTexImage2D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, type_: GLenum, data: *const GLvoid) {
    context().tex_image_2d(target, level, internalformat, width, height, border, format, type_, data);
}

#[no_mangle]
pub extern "stdcall" fn glTranslated(x: GLdouble, y: GLdouble, z: GLdouble) {
    context().issue(Command::Translated { x, y, z });
}

#[no_mangle]
pub extern "stdcall" fn glTranslatef(x: GLfloat, y: GLfloat, z: GLfloat) {
    context().issue(Command::Translatef { x, y, z });
}

#[no_mangle]
pub extern "stdcall" fn glVertex2f(_x: GLfloat, _y: GLfloat) {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glVertex2i(_x: GLint, _y: GLint) {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glVertex3f(x: GLfloat, y: GLfloat, z: GLfloat) {
    context().issue(Command::Vertex3f { x, y, z });
}

#[no_mangle]
pub extern "stdcall" fn glVertex3fv(_v: *const GLfloat) {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glVertexPointer(size: GLint, type_: GLenum, stride: GLsizei, pointer: *const GLvoid) {
    context().vertex_pointer(size, type_, stride, pointer);
}

#[no_mangle]
pub extern "stdcall" fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    context().issue(Command::Viewport { x, y, width, height });
}

#[no_mangle]
pub extern "stdcall" fn wglCreateContext(_dc: HDC) -> HGLRC {
    // TODO
    0 as _
}

#[no_mangle]
pub extern "stdcall" fn wglDeleteContext(_rc: HGLRC) -> BOOL {
    println!("wglDeleteContext called, ignoring");
    TRUE
}

#[no_mangle]
pub extern "stdcall" fn wglGetCurrentContext() -> HGLRC {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn wglGetCurrentDC() -> HDC {
    unimplemented!()
}

#[no_mangle]
pub unsafe extern "stdcall" fn wglGetProcAddress(name: LPCSTR) -> PROC {
    match CStr::from_ptr(name as *const i8).to_string_lossy().as_ref() {
        "glMultiTexCoord1dEXT" => glMultiTexCoord1dEXT as _,
        "glMultiTexCoord1dvARB" => glMultiTexCoord1dvARB as _,
        "glMultiTexCoord1fARB" => glMultiTexCoord1fARB as _,
        "glMultiTexCoord1fvARB" => glMultiTexCoord1fvARB as _,
        "glMultiTexCoord1iARB" => glMultiTexCoord1iARB as _,
        "glMultiTexCoord1ivARB" => glMultiTexCoord1ivARB as _,
        "glMultiTexCoord1sARB" => glMultiTexCoord1sARB as _,
        "glMultiTexCoord1svARB" => glMultiTexCoord1svARB as _,
        "glMultiTexCoord2dARB" => glMultiTexCoord2dARB as _,
        "glMultiTexCoord2dvARB" => glMultiTexCoord2dvARB as _,
        "glMultiTexCoord2fARB" => glMultiTexCoord2fARB as _,
        "glMultiTexCoord2fvARB" => glMultiTexCoord2fvARB as _,
        "glMultiTexCoord2iARB" => glMultiTexCoord2iARB as _,
        "glMultiTexCoord2ivARB" => glMultiTexCoord2ivARB as _,
        "glMultiTexCoord2sARB" => glMultiTexCoord2sARB as _,
        "glMultiTexCoord2svARB" => glMultiTexCoord2svARB as _,
        "glMultiTexCoord3dARB" => glMultiTexCoord3dARB as _,
        "glMultiTexCoord3dvARB" => glMultiTexCoord3dvARB as _,
        "glMultiTexCoord3fARB" => glMultiTexCoord3fARB as _,
        "glMultiTexCoord3fvARB" => glMultiTexCoord3fvARB as _,
        "glMultiTexCoord3iARB" => glMultiTexCoord3iARB as _,
        "glMultiTexCoord3ivARB" => glMultiTexCoord3ivARB as _,
        "glMultiTexCoord3sARB" => glMultiTexCoord3sARB as _,
        "glMultiTexCoord3svARB" => glMultiTexCoord3svARB as _,
        "glMultiTexCoord4dARB" => glMultiTexCoord4dARB as _,
        "glMultiTexCoord4dvARB" => glMultiTexCoord4dvARB as _,
        "glMultiTexCoord4fARB" => glMultiTexCoord4fARB as _,
        "glMultiTexCoord4fvARB" => glMultiTexCoord4fvARB as _,
        "glMultiTexCoord4iARB" => glMultiTexCoord4iARB as _,
        "glMultiTexCoord4ivARB" => glMultiTexCoord4ivARB as _,
        "glMultiTexCoord4sARB" => glMultiTexCoord4sARB as _,
        "glMultiTexCoord4sdARB" => {
            println!("haujobb hack lol");
            glMultiTexCoord4sARB as _
        }
        "glMultiTexCoord4svARB" => glMultiTexCoord4svARB as _,
        "glActiveTextureARB" => glActiveTextureARB as _,
        "glClientActiveTextureARB" => glClientActiveTextureARB as _,
        name => panic!("wglGetProcAddress called with invalid name: {}", name)
    }
}

#[no_mangle]
pub extern "stdcall" fn wglMakeCurrent(_dc: HDC, _rc: HGLRC) -> BOOL {
    println!("wglMakeCurrent called, ignoring");
    TRUE
}

extern "stdcall" fn swap_buffers(dc: HDC) -> BOOL {
    context().swap_buffers(dc)
}

extern "stdcall" fn change_display_settings_ex_a(_lpszDeviceName: LPCSTR, _lpDevMode: *const c_void, _hwnd: HWND, _dwflags: DWORD, _lParam: LPVOID) -> LONG {
    println!("ChangeDisplaySettingsExA called, ignoring");
    0
}
