//! Helper functions for dealing with shaders.

// External crates.
use gl;
use gl::types::{
    GLchar,
    GLenum,
    GLint,
    GLuint,
};

use std::ptr;

/// Compiles a shader.
///
/// Returns a shader or a message with the error.
pub fn compile_shader(
    shader_type: GLenum,
    source: &str
) -> Result<GLuint, String> {
    let shader = gl::CreateShader(shader_type);
    unsafe {
        source.with_c_str(
            |ptr| gl::ShaderSource(shader, 1, &ptr, ptr::null())
        );
        gl::CompileShader(shader);
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
        if status == (gl::TRUE as GLint) {
            Ok(shader)
        } else {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);

            if len == 0 {
                Err("Compilation failed with no log. The OpenGL context might have been created on another thread, or not have been created.".to_string())
            }
            else {
                // Subtract 1 to skip the trailing null character.
                let mut buf = Vec::from_elem(len as uint - 1, 0u8);
                gl::GetShaderInfoLog(
                    shader, 
                    len, 
                    ptr::mut_null(), 
                    buf.as_mut_ptr() as *mut GLchar
                );
                
                gl::DeleteShader(shader);
                
                Err(String::from_utf8(buf).ok().expect(
                    "ShaderInfoLog not valid utf8"
                ))
            }
        }
    }
}

/// Gets the attribute location from a program.
///
/// Returns `None` if there is no attribute with such name.
pub fn attribute_location(program: GLuint, name: &'static str) -> Option<GLuint> {
    unsafe {
        name.with_c_str(|ptr| {
            let id = gl::GetAttribLocation(program, ptr);
            if gl::GetError() != 0 { None } else { Some(id as GLuint) }
        })
    }
}

