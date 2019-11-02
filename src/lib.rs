use std::ffi::c_void;

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
type GLenum = u32;
type GLint = i32;
type GLuint = u32;
type GLsizei = u32;
type GLfloat = f32;
type GLbitfield = u32;
type GLdouble = f64;
type GLvoid = c_void;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn DllMain(_hinstDLL: HINSTANCE, fdwReason: DWORD, _lpvReserved: LPVOID) -> BOOL {
    println!("henlo from gloat! Reason: {}", fdwReason);

    TRUE
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
pub extern "stdcall" fn glLoadIdentity() {
    unimplemented!()
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
pub extern "stdcall" fn glMatrixMode(_mode: GLenum) {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn glMultMatrixd(_m: *const GLdouble) {
    unimplemented!()
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
pub extern "stdcall" fn glViewport(_x: GLint, _y: GLint, _width: GLsizei, _height: GLsizei) {
    unimplemented!()
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
pub extern "stdcall" fn wglGetProcAddress(_name: LPCSTR) -> PROC {
    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn wglMakeCurrent(_dc: HDC, _rc: HGLRC) -> BOOL {
    unimplemented!()
}
