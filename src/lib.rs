use std::ffi::{CStr, c_void};

type LPVOID = *mut c_void;

type BOOL = i32;
const FALSE: BOOL = 0;
const TRUE: BOOL = 1;

type DWORD = u32;

type HANDLE = LPVOID;
type HINSTANCE = HANDLE;
type HDC = HANDLE;
type HGLRC = HANDLE;

type LPCSTR = LPVOID;
type PROC = LPVOID;

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

const GL_MODELVIEW: GLenum = 0x1700;
const GL_PROJECTION: GLenum = 0x1701;

enum MatrixMode {
    ModelView,
    Projection,
}

struct Context {
    matrix_mode: MatrixMode,
}

impl Context {
    fn new() -> Context {
        Context {
            matrix_mode: MatrixMode::ModelView,
        }
    }

    fn load_identity(&mut self) {
        // TODO
    }

    fn matrix_mode(&mut self, mode: MatrixMode) {
        self.matrix_mode = mode;
    }

    fn mult_matrix_d(&mut self, _m: *const GLdouble) {
        // TODO
    }

    fn viewport(&mut self, x: GLint, y: GLint, width: GLsizei, height: GLsizei)
    {
        // TODO
        println!("viewport: {}, {}, {}, {}", x, y, width, height);
    }
}

static mut CONTEXT: Option<Context> = None;

const DLL_PROCESS_DETACH: DWORD = 0;
const DLL_PROCESS_ATTACH: DWORD = 1;
const DLL_THREAD_ATTACH: DWORD = 2;
const DLL_THREAD_DETACH: DWORD = 3;

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn DllMain(_hinstDLL: HINSTANCE, fdwReason: DWORD, _lpvReserved: LPVOID) -> BOOL {
    match fdwReason {
        DLL_PROCESS_ATTACH => {
            println!("DllMain: process attach");
            CONTEXT = Some(Context::new());
        }
        DLL_PROCESS_DETACH => {
            println!("DllMain: process detach");
        }
        DLL_THREAD_ATTACH => {
            println!("DllMain: thread attach");
        }
        DLL_THREAD_DETACH => {
            println!("DllMain: thread detach");
            CONTEXT = None;
        }
        _ => panic!("DllMain called with invalid fdwReason value: {}", fdwReason)
    }

    TRUE
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glActiveTextureARB(_texture: GLenum) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glArrayElement(_index: GLint) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glBegin(_mode: GLenum) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glBindTexture(_target: GLenum, _texture: GLuint) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glBlendFunc(_sfactor: GLenum, _dfactor: GLenum) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glCallList(_list: GLuint) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glClear(_mask: GLbitfield) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glClearColor(_red: GLfloat, _green: GLfloat, _blue: GLfloat, _alpha: GLfloat) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glClearDepth(_depth: GLdouble) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glClientActiveTextureARB(_texture: GLenum) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glColor3f(_red: GLfloat, _green: GLfloat, _blue: GLfloat) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glColor4f(_red: GLfloat, _green: GLfloat, _blue: GLfloat, _alpha: GLfloat) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glCullFace(_mode: GLenum) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glDepthFunc(_func: GLenum) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glDepthMask(_flag: GLboolean) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glDisable(_cap: GLenum) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glDisableClientState(_array: GLenum) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glEnable(_cap: GLenum) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glEnableClientState(_array: GLenum) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glEnd() {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glEndList() {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glEvalCoord1f(_u: GLfloat) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glEvalCoord2f(_u: GLfloat, _v: GLfloat) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glEvalMesh1(_mode: GLenum, _i1: GLint, _i2: GLint) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glEvalMesh2(_mode: GLenum, _i1: GLint, _i2: GLint, _j1: GLint, _j2: GLint) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glEvalPoint2(_i: GLint, _j: GLint) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glGenLists(_range: GLsizei) -> GLuint {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glGenTextures(_n: GLsizei, _textures: *mut GLuint) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glGetError() -> GLenum {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glGetFloatv(_pname: GLenum, _params: *mut GLfloat) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glGetIntegerv(_pname: GLenum, _params: *mut GLint) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glGetString(_name: GLenum) -> *const GLubyte {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glLightf(_light: GLenum, _pname: GLenum, _param: GLfloat) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glLightfv(_light: GLenum, _pname: GLenum, _params: *const GLfloat) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn glLoadIdentity() {
    let context = CONTEXT.as_mut().unwrap();
    context.load_identity();
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glMap1f(_target: GLenum, _u1: GLfloat, _u2: GLfloat, _stride: GLint, _order: GLint, _points: *const GLfloat) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glMap2f(_target: GLenum, _u1: GLfloat, _u2: GLfloat, _ustride: GLint, _uorder: GLint, _v1: GLfloat, _v2: GLfloat, _vstride: GLint, _vorder: GLint, _points: *const GLfloat) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glMapGrid1f(_un: GLint, _u1: GLfloat, _u2: GLfloat) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glMapGrid2d(_un: GLint, _u1: GLdouble, _u2: GLdouble, _vn: GLint, _v1: GLdouble, _v2: GLdouble) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glMaterialfv(_face: GLenum, _pname: GLenum, _params: *const GLfloat) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn glMatrixMode(mode: GLenum) {
    let context = CONTEXT.as_mut().unwrap();
    let mode = match mode {
        GL_MODELVIEW => MatrixMode::ModelView,
        GL_PROJECTION => MatrixMode::Projection,
        _ => panic!("glMatrixMode called with invalid mode: 0x{:08x}", mode),
    };
    context.matrix_mode(mode);
}

#[allow(non_snake_case)]
extern "stdcall" fn glMultiTexCoord1dEXT(_target: GLenum, _s: GLdouble) {
    unimplemented!()
}

#[allow(non_snake_case)]
extern "stdcall" fn glMultiTexCoord1dvARB(_target: GLenum, _v: *const GLfloat) {
    unimplemented!()
}

#[allow(non_snake_case)]
extern "stdcall" fn glMultiTexCoord1fARB(_target: GLenum, _s: GLfloat) {
    unimplemented!()
}

#[allow(non_snake_case)]
extern "stdcall" fn glMultiTexCoord1fvARB(_target: GLenum, _v: *const GLfloat) {
    unimplemented!()
}

#[allow(non_snake_case)]
extern "stdcall" fn glMultiTexCoord1iARB(_target: GLenum, _s: GLint) {
    unimplemented!()
}

#[allow(non_snake_case)]
extern "stdcall" fn glMultiTexCoord1ivARB(_target: GLenum, _v: *const GLint) {
    unimplemented!()
}

#[allow(non_snake_case)]
extern "stdcall" fn glMultiTexCoord1sARB(_target: GLenum, _s: GLshort) {
    unimplemented!()
}

#[allow(non_snake_case)]
extern "stdcall" fn glMultiTexCoord1svARB(_target: GLenum, _v: *const GLshort) {
    unimplemented!()
}

#[allow(non_snake_case)]
extern "stdcall" fn glMultiTexCoord2dARB(_target: GLenum, _s: GLdouble, _t: GLdouble) {
    unimplemented!()
}

#[allow(non_snake_case)]
extern "stdcall" fn glMultiTexCoord2dvARB(_target: GLenum, _v: *const GLdouble) {
    unimplemented!()
}

#[allow(non_snake_case)]
extern "stdcall" fn glMultiTexCoord2fARB(_target: GLenum, _s: GLfloat, _t: GLfloat) {
    unimplemented!()
}

#[allow(non_snake_case)]
extern "stdcall" fn glMultiTexCoord2fvARB(_target: GLenum, _v: *const GLfloat) {
    unimplemented!()
}

#[allow(non_snake_case)]
extern "stdcall" fn glMultiTexCoord2iARB(_target: GLenum, _s: GLint, _t: GLint) {
    unimplemented!()
}

#[allow(non_snake_case)]
extern "stdcall" fn glMultiTexCoord2ivARB(_target: GLenum, _v: *const GLint) {
    unimplemented!()
}

#[allow(non_snake_case)]
extern "stdcall" fn glMultiTexCoord2sARB(_target: GLenum, _s: GLshort, _t: GLshort) {
    unimplemented!()
}

#[allow(non_snake_case)]
extern "stdcall" fn glMultiTexCoord2svARB(_target: GLenum, _v: *const GLshort) {
    unimplemented!()
}

#[allow(non_snake_case)]
extern "stdcall" fn glMultiTexCoord3dARB(_target: GLenum, _s: GLdouble, _t: GLdouble, _r: GLdouble) {
    unimplemented!()
}

#[allow(non_snake_case)]
extern "stdcall" fn glMultiTexCoord3dvARB(_target: GLenum, _v: *const GLdouble) {
    unimplemented!()
}

#[allow(non_snake_case)]
extern "stdcall" fn glMultiTexCoord3fARB(_target: GLenum, _s: GLfloat, _t: GLfloat, _r: GLfloat) {
    unimplemented!()
}

#[allow(non_snake_case)]
extern "stdcall" fn glMultiTexCoord3fvARB(_target: GLenum, _v: *const GLfloat) {
    unimplemented!()
}

#[allow(non_snake_case)]
extern "stdcall" fn glMultiTexCoord3iARB(_target: GLenum, _s: GLint, _t: GLint, _r: GLint) {
    unimplemented!()
}

#[allow(non_snake_case)]
extern "stdcall" fn glMultiTexCoord3ivARB(_target: GLenum, _v: *const GLint) {
    unimplemented!()
}

#[allow(non_snake_case)]
extern "stdcall" fn glMultiTexCoord3sARB(_target: GLenum, _s: GLshort, _t: GLshort, _r: GLshort) {
    unimplemented!()
}

#[allow(non_snake_case)]
extern "stdcall" fn glMultiTexCoord3svARB(_target: GLenum, _v: *const GLshort) {
    unimplemented!()
}

#[allow(non_snake_case)]
extern "stdcall" fn glMultiTexCoord4dARB(_target: GLenum, _s: GLdouble, _t: GLdouble, _r: GLdouble, _q: GLdouble) {
    unimplemented!()
}

#[allow(non_snake_case)]
extern "stdcall" fn glMultiTexCoord4dvARB(_target: GLenum, _v: *const GLdouble) {
    unimplemented!()
}

#[allow(non_snake_case)]
extern "stdcall" fn glMultiTexCoord4fARB(_target: GLenum, _s: GLfloat, _t: GLfloat, _r: GLfloat, _q: GLfloat) {
    unimplemented!()
}

#[allow(non_snake_case)]
extern "stdcall" fn glMultiTexCoord4fvARB(_target: GLenum, _v: *const GLfloat) {
    unimplemented!()
}

#[allow(non_snake_case)]
extern "stdcall" fn glMultiTexCoord4iARB(_target: GLenum, _s: GLint, _t: GLint, _r: GLint, _q: GLint) {
    unimplemented!()
}

#[allow(non_snake_case)]
extern "stdcall" fn glMultiTexCoord4ivARB(_target: GLenum, _v: *const GLint) {
    unimplemented!()
}

#[allow(non_snake_case)]
extern "stdcall" fn glMultiTexCoord4sARB(_target: GLenum, _s: GLshort, _t: GLshort, _r: GLshort, _q: GLshort) {
    unimplemented!()
}

#[allow(non_snake_case)]
extern "stdcall" fn glMultiTexCoord4svARB(_target: GLenum, _v: *const GLshort) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn glMultMatrixd(m: *const GLdouble) {
    let context = CONTEXT.as_mut().unwrap();
    context.mult_matrix_d(m);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glMultMatrixf(_m: *const GLfloat) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glNewList(_list: GLuint, _mode: GLenum) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glNormal3f(_nx: GLfloat, _ny: GLfloat, _nz: GLfloat) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glNormal3fv(_v: *const GLfloat) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glNormalPointer(_type: GLenum, _stride: GLsizei, _pointer: *const GLvoid) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glOrtho(_left: GLdouble, _right: GLdouble, _bottom: GLdouble, _top: GLdouble, _zNear: GLdouble, _zFar: GLdouble) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glPixelStorei(_pname: GLenum, _param: GLint) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glPolygonMode(_face: GLenum, _mode: GLenum) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glPopAttrib() {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glPopMatrix() {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glPushAttrib(_mask: GLbitfield) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glPushMatrix() {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glScalef(_x: GLfloat, _y: GLfloat, _z: GLfloat) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glShadeModel(_mode: GLenum) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glTexCoord2f(_s: GLfloat, _t: GLfloat) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glTexGenf(_coord: GLenum, _pname: GLenum, _param: GLfloat) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glTexGeni(_coord: GLenum, _pname: GLenum, _param: GLint) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glTexParameteri(_target: GLenum, _pname: GLenum, _param: GLint) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glTexImage1D(_target: GLenum, _level: GLint, _internalformat: GLint, _width: GLsizei, _border: GLint, _format: GLenum, _type: GLenum, _data: *const GLvoid) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glTexImage2D(_target: GLenum, _level: GLint, _internalformat: GLint, _width: GLsizei, _height: GLsizei, _border: GLint, _format: GLenum, _type: GLenum, _data: *const GLvoid) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glTranslated(_x: GLdouble, _y: GLdouble, _z: GLdouble) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glTranslatef(_x: GLfloat, _y: GLfloat, _z: GLfloat) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glVertex3f(_x: GLfloat, _y: GLfloat, _z: GLfloat) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glVertexPointer(_size: GLint, _type: GLenum, _stride: GLsizei, _pointer: *const GLvoid) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    let context = CONTEXT.as_mut().unwrap();
    context.viewport(x, y, width, height);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn wglCreateContext(_dc: HDC) -> HGLRC {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn wglDeleteContext(_rc: HGLRC) -> BOOL {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
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
#[allow(non_snake_case)]
pub extern "stdcall" fn wglMakeCurrent(_dc: HDC, _rc: HGLRC) -> BOOL {
    println!("wglMakeCurrent called, ignoring");
    TRUE
}
