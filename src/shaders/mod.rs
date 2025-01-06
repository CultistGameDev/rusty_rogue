use std::{ffi::CString, ops::Deref};

use gl::types;

#[derive(Default)]
pub struct Program(u32);

impl Drop for Program {
    fn drop(&mut self) {
        println!("Dropped Program");
        unsafe {
            gl::DeleteProgram(self.0);
        }
    }
}

impl Deref for Program {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Default)]
struct Shader(u32);

impl Program {
    pub fn builder() -> ProgramBuilder {
        ProgramBuilder::default()
    }
}

#[derive(Default)]
pub struct ProgramBuilder {
    vertex: Shader,
    fragment: Shader,
}

unsafe fn shader(source: String, t: types::GLenum) -> u32 {
    let shader = gl::CreateShader(t);
    let source = CString::new(source).unwrap();
    gl::ShaderSource(
        shader,
        1,
        &source.as_ptr() as *const *const i8,
        std::ptr::null(),
    );
    gl::CompileShader(shader);
    shader
}

impl ProgramBuilder {
    pub fn build(self) -> Program {
        Program(unsafe {
            let program = gl::CreateProgram();
            gl::AttachShader(program, self.vertex.0);
            gl::AttachShader(program, self.fragment.0);
            gl::LinkProgram(program);
            gl::DeleteShader(self.vertex.0);
            gl::DeleteShader(self.fragment.0);

            program
        })
    }
    pub fn vertex(mut self, source: String) -> Self {
        self.vertex = Shader(unsafe { shader(source, gl::VERTEX_SHADER) });
        self
    }

    pub fn fragment(mut self, source: String) -> Self {
        self.fragment = Shader(unsafe { shader(source, gl::FRAGMENT_SHADER) });
        self
    }
}
