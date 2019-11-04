#![allow(non_snake_case, non_camel_case_types)]

use std::cell::RefCell;
use std::ffi::{CStr, c_void};
use std::rc::Rc;
use std::slice;

type LPVOID = *mut c_void;

type BOOL = i32;
const FALSE: BOOL = 0;
const TRUE: BOOL = 1;

type DWORD = i32;
type PDWORD = *mut DWORD;

type HANDLE = LPVOID;
type HINSTANCE = HANDLE;
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

const GL_MODELVIEW: GLenum = 0x1700;
const GL_PROJECTION: GLenum = 0x1701;

const GL_LINEAR: GLint = 0x2601;
const GL_LINEAR_MIPMAP_NEAREST: GLint = 0x2701;

const GL_TEXTURE_MAG_FILTER: GLenum = 0x2800;
const GL_TEXTURE_MIN_FILTER: GLenum = 0x2801;

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
    Linear,
    LinearMipmapNearest,
}

struct Texture {
    mag_filter: TextureFilter,
    min_filter: TextureFilter,
}

impl Texture {
    fn new() -> Texture {
        Texture {
            mag_filter: TextureFilter::Linear,
            min_filter: TextureFilter::Linear,
        }
    }
}

enum Command {
    ActiveTextureARB { texture: GLenum },
    ArrayElement { index: GLint },
    Begin { mode: GLenum },
    BindTexture { target: GLenum, texture: GLuint },
    BlendFunc { sfactor: GLenum, dfactor: GLenum },
    CallList { list: GLuint },
    Clear { mask: GLbitfield },
    ClearColor { red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat },
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
    MultMatrixd { _m: [GLdouble; 16] },
    MultMatrixf { _m: [GLfloat; 16] },
    Normal3fv { v: [GLfloat; 3] },
    Ortho { left: GLdouble, right: GLdouble, bottom: GLdouble, top: GLdouble, zNear: GLdouble, zFar: GLdouble },
    PolygonMode { face: GLenum, mode: GLenum },
    PopMatrix,
    PushMatrix,
    ShadeModel { mode: GLenum },
    TexCoord2f { s: GLfloat, t: GLfloat },
    TexGenf { coord: GLenum, pname: GLenum, param: GLfloat },
    TexParameteri { target: GLenum, pname: GLenum, param: GLint },
    Translated { x: GLdouble, y: GLdouble, z: GLdouble },
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
    _swap_buffers: PatchedFunction,

    clear_color_red: GLfloat,
    clear_color_green: GLfloat,
    clear_color_blue: GLfloat,
    clear_color_alpha: GLfloat,

    display_lists: Vec<Rc<RefCell<DisplayList>>>,
    new_list: Option<GLuint>,
    new_list_mode: GLenum,

    matrix_mode: MatrixMode,

    textures: Vec<Texture>,
    texture_2d: GLuint,

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
}

impl Context {
    fn new() -> Context {
        Context {
            _swap_buffers: PatchedFunction::new(SwapBuffers as _, swap_buffers as _),

            clear_color_red: 0.0,
            clear_color_green: 0.0,
            clear_color_blue: 0.0,
            clear_color_alpha: 0.0,

            display_lists: Vec::new(),
            new_list: None,
            new_list_mode: 0,

            matrix_mode: MatrixMode::ModelView,

            textures: Vec::new(),
            texture_2d: 0,

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
        }
    }

    fn disable_client_state(&mut self, array: GLenum) {
        // TODO
        println!("DisableClientState: array: 0x{:08x}", array);
    }

    fn enable_client_state(&mut self, array: GLenum) {
        // TODO
        println!("EnableClientState: array: 0x{:08x}", array);
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
            Command::ArrayElement { index } => {
                // TODO
                println!("ArrayElement: index: 0x{:08x}", index);
            }
            Command::Begin { mode } => {
                // TODO
                println!("Begin: mode: 0x{:08x}", mode);
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
                // TODO
                println!("BlendFunc: sfactor: 0x{:08x}, dfactor: 0x{:08x}", sfactor, dfactor);
            }
            Command::CallList { list } => {
                for command in self.display_lists[list as usize].clone().borrow().commands.iter() {
                    self.execute(command);
                }
            }
            Command::Clear { mask } => {
                // TODO
                println!("Clear: mask: 0x{:08x}", mask);
            }
            Command::ClearColor { red, green, blue, alpha } => {
                self.clear_color_red = red;
                self.clear_color_green = green;
                self.clear_color_blue = blue;
                self.clear_color_alpha = alpha;
            }
            Command::Color4f { red, green, blue, alpha } => {
                // TODO
                println!("Color4f: red: {}, green: {}, blue: {}, alpha: {}", red, green, blue, alpha);
            }
            Command::CullFace { mode } => {
                // TODO
                println!("CullFace: mode: {}", mode);
            }
            Command::DepthMask { flag } => {
                // TODO
                println!("DepthMask: flag: {}", flag);
            }
            Command::Disable { cap } => {
                // TODO
                println!("Disable: cap: 0x{:08x}", cap);
            }
            Command::Enable { cap } => {
                // TODO
                println!("Enable: cap: 0x{:08x}", cap);
            }
            Command::End => {
                // TODO
                println!("End");
            }
            Command::Lightf { light, pname, param } => {
                // TODO
                println!("Lightf: light: 0x{:08x}, pname: 0x{:08x}, param: {}", light, pname, param);
            }
            Command::LoadIdentity => {
                // TODO
                println!("LoadIdentity")
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
                println!("Ortho: target: 0x{:08x}, s: {}, t: {}", target, s, t);
            }
            Command::MultMatrixd { _m } => {
                // TODO
                println!("MultMatrixd");
            }
            Command::MultMatrixf { _m } => {
                // TODO
                println!("MultMatrixf");
            }
            Command::Normal3fv { v } => {
                // TODO
                println!("Normal3fv: v: {}, {}, {}", v[0], v[1], v[2]);
            }
            Command::Ortho { left, right, bottom, top, zNear, zFar } => {
                // TODO
                println!("Ortho: left: {}, right: {}, bottom: {}, top: {}, zNear: {}, zFar: {}", left, right, bottom, top, zNear, zFar);
            }
            Command::PolygonMode { face, mode } => {
                // TODO
                println!("Enable: face: 0x{:08x}, mode: 0x{:08x}", face, mode);
            }
            Command::PopMatrix => {
                // TODO
                println!("PopMatrix");
            }
            Command::PushMatrix => {
                // TODO
                println!("PushMatrix");
            }
            Command::ShadeModel { mode } => {
                // TODO
                println!("ShadeModel: mode: 0x{:08x}", mode);
            }
            Command::TexCoord2f { s, t } => {
                // TODO
                println!("TexCoord2f: s: {}, t: {}", s, t);
            }
            Command::TexGenf { coord, pname, param } => {
                // TODO
                println!("TexGenf: coord: 0x{:08x}, pname: 0x{:08x}, param: {}", coord, pname, param);
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
                                GL_LINEAR => {
                                    texture.min_filter = TextureFilter::Linear;
                                }
                                GL_LINEAR_MIPMAP_NEAREST => {
                                    texture.min_filter = TextureFilter::LinearMipmapNearest;
                                }
                                _ => panic!("glTexParameteri called with invalid param for GL_TEXTURE_MIN_FILTER: 0x{:08x}", param)
                            }
                            _ => panic!("glTexParameteri called with invalid pname: 0x{:08x}", pname)
                        }
                    }
                    _ => panic!("glTexParameteri called with invalid target: 0x{:08x}", target)
                }
            }
            Command::Translated { x, y, z } => {
                // TODO
                println!("Translated: {}, {}, {}", x, y, z);
            }
            Command::Vertex3f { x, y, z } => {
                // TODO
                println!("Vertex3f: {}, {}, {}", x, y, z);
            }
            Command::Viewport { x, y, width, height } => {
                // TODO
                println!("Viewport: {}, {}, {}, {}", x, y, width, height);
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
        // TODO
        println!("NormalPointer: type: 0x{:08x}, stride: 0x{:08x}, pointer: 0x{:08x}", type_, stride, pointer as u32);
    }

    fn pixel_storei(&mut self, pname: GLenum, param: GLint) {
        match pname {
            GL_UNPACK_SWAP_BYTES => match param {
                0 | 1 => {
                    self.unpack_swap_bytes = param;
                }
                _ => panic!("glPixelStorei called with invalid param for GL_UNPACK_SWAP_BYTES: 0x{:08x}", param)
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

    fn vertex_pointer(&mut self, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const GLvoid) {
        // TODO
        println!("NormalPointer: size: 0x{:08x}, type: 0x{:08x}, stride: 0x{:08x}, pointer: 0x{:08x}", size, type_, stride, pointer as u32);
    }
}

static mut CONTEXT: Option<Context> = None;

fn context() -> &'static mut Context {
    unsafe { CONTEXT.as_mut().expect("Attempted to get context reference when no context was initialized") }
}

const DLL_PROCESS_DETACH: DWORD = 0;
const DLL_PROCESS_ATTACH: DWORD = 1;
const DLL_THREAD_ATTACH: DWORD = 2;
const DLL_THREAD_DETACH: DWORD = 3;

#[no_mangle]
pub unsafe extern "system" fn DllMain(_hinstDLL: HINSTANCE, fdwReason: DWORD, _lpvReserved: LPVOID) -> BOOL {
    match fdwReason {
        DLL_PROCESS_ATTACH => {
            println!("DllMain: process attach");
            CONTEXT = Some(Context::new());
        }
        DLL_PROCESS_DETACH => {
            println!("DllMain: process detach");
            CONTEXT = None;
        }
        DLL_THREAD_ATTACH => {
            println!("DllMain: thread attach");
        }
        DLL_THREAD_DETACH => {
            println!("DllMain: thread detach");
        }
        _ => panic!("DllMain called with invalid fdwReason value: {}", fdwReason)
    }

    TRUE
}

#[no_mangle]
pub extern "stdcall" fn glActiveTextureARB(texture: GLenum) {
    context().issue(Command::ActiveTextureARB { texture });
}

#[no_mangle]
pub extern "stdcall" fn glArrayElement(index: GLint) {
    context().issue(Command::ArrayElement { index });
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
pub extern "stdcall" fn glColor3f(_red: GLfloat, _green: GLfloat, _blue: GLfloat) {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glColor4f(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) {
    context().issue(Command::Color4f { red, green, blue, alpha });
}

#[no_mangle]
pub extern "stdcall" fn glCullFace(mode: GLenum) {
    context().issue(Command::CullFace { mode });
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
pub extern "stdcall" fn glLoadIdentity() {
    context().issue(Command::LoadIdentity);
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
    context().issue(Command::MultMatrixd { _m: m_copy });
}

#[no_mangle]
pub extern "stdcall" fn glMultMatrixf(m: *const GLfloat) {
    let mut m_copy = [0.0; 16];
    m_copy.copy_from_slice(unsafe { slice::from_raw_parts(m, 16) });
    context().issue(Command::MultMatrixf { _m: m_copy });
}

#[no_mangle]
pub extern "stdcall" fn glNewList(list: GLuint, mode: GLenum) {
    context().new_list(list, mode);
}

#[no_mangle]
pub extern "stdcall" fn glNormal3f(_nx: GLfloat, _ny: GLfloat, _nz: GLfloat) {
    unimplemented!()
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
pub extern "stdcall" fn glTexGenf(coord: GLenum, pname: GLenum, param: GLfloat) {
    context().issue(Command::TexGenf { coord, pname, param });
}

#[no_mangle]
pub extern "stdcall" fn glTexGeni(_coord: GLenum, _pname: GLenum, _param: GLint) {
    unimplemented!()
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
    // TODO!
    println!("TexImage2D: target: 0x{:08x}, level: 0x{:08x}, internalformat: 0x{:08x}, width: 0x{:08x}, height: 0x{:08x}, border: 0x{:08x}, format: 0x{:08x}, type: 0x{:08x}, data: 0x{:08x}", target, level, internalformat, width, height, border, format, type_, data as u32);
}

#[no_mangle]
pub extern "stdcall" fn glTranslated(x: GLdouble, y: GLdouble, z: GLdouble) {
    context().issue(Command::Translated { x, y, z });
}

#[no_mangle]
pub extern "stdcall" fn glTranslatef(_x: GLfloat, _y: GLfloat, _z: GLfloat) {
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn glVertex3f(x: GLfloat, y: GLfloat, z: GLfloat) {
    context().issue(Command::Vertex3f { x, y, z });
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
    unimplemented!()
}

#[no_mangle]
pub extern "stdcall" fn wglDeleteContext(_rc: HGLRC) -> BOOL {
    println!("wglDeleteContext called, ignoring");
    TRUE
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
    panic!("WE AIN'T SWAPPIN' SHIT!!! dc: 0x{:08x}", dc as u32);
}
