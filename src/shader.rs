#![allow(dead_code)] // XXX Gotta get rid of this someday

extern crate gl33;
use gl33::global_loader::*;
use gl33::*;

use std::fs::read;

pub struct Shader {
    pub id: u32,
}

impl Shader {

    pub fn new(vertex_path: &str, fragment_path: &str) -> Self {

        //
        // ─── READ SHADER FILES ───────────────────────────────────────────
        //
            
        let vertex_bytes = read(vertex_path).expect("Could not open vertex shader!");
        let fragment_bytes = read(fragment_path).expect("Could not open fragment shader!");

        
        //
        // ─── VERTEX SHADER ───────────────────────────────────────────────
        //  

        let vertex_shader = glCreateShader(GL_VERTEX_SHADER);   // Create shader

        unsafe {
            glShaderSource(                     // Load shade from source
                vertex_shader,                  // Shader object
                1,                              // How many string objects
                &vertex_bytes.as_ptr().cast(),  // Source code of vertex shader
                &(vertex_bytes.len() as i32)    // Shader source code size
            );
        }

        glCompileShader(vertex_shader); // Compile shader

        // Check for errors
        check_shader_errors(vertex_shader, "Vertex shader error");

        //
        // ─── FRAGMENT SHADER ─────────────────────────────────────────────
        //

        let fragment_shader = glCreateShader(GL_FRAGMENT_SHADER);   // Create shader

        unsafe {
            glShaderSource(                       // Load shade from source
                fragment_shader,                  // Shader object
                1,                                // How many string objects
                &fragment_bytes.as_ptr().cast(),  // Source code of vertex shader
                &(fragment_bytes.len() as i32)    // Shader source code size
            );
        }

        glCompileShader(fragment_shader); // Compile shader

        // Check for errors
        check_shader_errors(fragment_shader, "Fragment shader error");
        

        //
        // ─── SHADER PROGRAM ──────────────────────────────────────────────
        //  

        let id = glCreateProgram();

        glAttachShader(id, vertex_shader);   // Attach vertex shader to program
        glAttachShader(id, fragment_shader); // Attach fragment shader to program
        glLinkProgram(id);                   // Link program

        let mut success = 0;
        unsafe {
            glGetProgramiv(id, GL_LINK_STATUS, &mut success); // Get link status

            if success == 0 {
                let mut log: Vec<u8> = Vec::with_capacity(1024); // Buffer for possible error message
                let mut len = 0;
                glGetProgramInfoLog(         // Get possible error message from program
                    id,
                    log.capacity() as i32,
                    &mut len,
                    log.as_mut_ptr().cast()
                );

                log.set_len(len as usize); // Trims the log of unused bytes
                panic!(                                // In case of shader error
                    "Shader program error: {}",        // Error message
                    std::str::from_utf8(&log).unwrap()
                )
            }
        }

        // Delete shaders, they aren't needed anymore. So long, partners.
        glDeleteShader(vertex_shader);
        glDeleteShader(fragment_shader);

        Shader {
            id: id
        }
    }

    pub fn use_shader(&self) {
        glUseProgram(self.id);
    }

    pub fn set_bool(&self, name: &str, value: bool) {
        let name = String::from(name) + "\0";
        unsafe {
            glUniform1i(
                glGetUniformLocation(self.id, name.as_str().as_ptr()),
                value as i32
            );
        }
    }

    pub fn set_int(&self, name: &str, value: i32) {
        let name = String::from(name) + "\0";
        unsafe {
            glUniform1i(
                glGetUniformLocation(self.id, name.as_str().as_ptr()),
                value
            );
        }
    }

    pub fn set_float(&self, name: &str, value: f32) {
        let name = String::from(name) + "\0";
        unsafe {
            glUniform1f(
                glGetUniformLocation(self.id, name.as_str().as_ptr()),
                value
            );
        }
    }

}

// Gets an error from OpenGL and panics with given error message.
pub fn check_shader_errors(shader: u32, error_msg: &str) {
    let mut success = 0;
    unsafe {
        glGetShaderiv(shader, GL_COMPILE_STATUS, &mut success); // Get compile status

        // Check for errors
        if success == 0 {
            let mut log: Vec<u8> = Vec::with_capacity(1024); // Buffer for error message
            let mut len = 0;
            glGetShaderInfoLog(         // Get possible error message from shader
                shader,
                log.capacity() as i32,
                &mut len,
                log.as_mut_ptr().cast()
            );

            log.set_len(len as usize); // Trims the log of unused bytes
            panic!(                                // In case of shader error
                "{0}: {1}",       // Error message
                error_msg,
                std::str::from_utf8(&log).unwrap()
            )
        }
    }
}