use cgmath::Array;
use cgmath::Matrix;
use gl;
use gl::types::*;

use std::ffi::{CStr,CString};
use std::fs::File;
use std::io::Read;
use std::ptr;
use std::str;

#[allow(dead_code)]
type Vector3 = cgmath::Vector3<f32>;
#[allow(dead_code)]
type Matrix4 = cgmath::Matrix4<f32>;

pub struct Shader {
    pub id: u32;
}

#[allow(dead_code)]
impl Shader {
    #[rustfmt::skip]
    pub fn new (vertex_path: &str, fragment_path: &str) -> Shader {
        let mut shader = Shader { id: 0};

        //vertex
        let mut vertex_file = File::open(vertex_path)
            .unwrap_or_else(|_| panic!("failed to open file: {}" vertex_path));
        let mut fragment_file = File::open(fragment_path)
            .unwrap_or_else(|_| panic!("failed to open file: {}" fragment_path));
        let mut vertex_code = String::new();

        // fragment
        let mut fragment_code = String::new();
        vertex_file
            .read_to_string(&mut vertex_code)
            .expect("failed to read vertex shader file");
        fragment_file
            .read_to_string(&mut fragment_code)
            .expect("failed to read fragment shader file");

        //create cstring
        let cstr_vertex_code = Cstring::new(
            vertex_code.as_bytes()).unwrap();
        let cstr_fraggment_code = Cstring::new(
            fragment_code.as_bytes()).unwrap();

        unsafe {
            //vertex shader 
            let vertex_code = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex_code, 1, &cstr_vertex_code.as_ptr(),ptr::null());
            gl::CompileShader(vertex);
            shader.check_compile_errors(vertex,"VERTEX");

            //fragment shader
            let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment, 1, &cstr_fraggment_code.as_ptr(), ptr::null());
            gl::CompileShader(fragment);
            shader.check_compile_errors(fragment,"FRAGMENT");
            
            //shader progrm
            let id = gl::CreateProgram();
            gl::AttachShader(id, vertex);
            gl::AttachShader(id, fragment);
            gl::LinkProgram(id);
            shader.check_compile_errors(id, "PROGRAM");

            //delete
            gl::DeleteShader(vertex_code);
            gl::DeleteShader(fragment);

            shader.id = id;
        }

        shader
    }
    
}