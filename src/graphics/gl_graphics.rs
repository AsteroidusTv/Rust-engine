use std::mem;
use std::os::raw::*;
use gl::types::{GLuint, GLenum, GLboolean, GLsizei, self};

pub struct Vao {
    id: gl::types::GLuint,
}

impl Vao {
    pub fn new() -> Vao {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }

        Vao { id }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

}

pub struct BufferObject {
    id: gl::types::GLuint,
    r#types: gl::types::GLenum,
    usages: gl::types::GLenum
}

impl  BufferObject {
    pub fn new(types: gl::types::GLenum, usages: gl::types::GLenum) -> BufferObject {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        BufferObject { id, types, usages }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.types, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(self.types, 0);
        }
    }

    pub fn store_f32_data(&self, data: &[f32]) {
        unsafe {
            gl::BufferData(
                self.types,
                (data.len() * mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
                &data[0] as *const f32 as *const c_void,
                self.usages,
            );
        }
    }

    pub fn store_i32_data(&self, data: &[i32]) {
        unsafe {
            gl::BufferData(
                self.types,
                (data.len() * mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
                &data[0] as *const i32 as *const c_void,
                self.usages,
            );
        }
    }
}

pub struct VertexAttribute {
    index: GLuint,
}

impl VertexAttribute {
    pub fn new(
        index: u32,
        size: i32,
        r#type: GLenum,
        normalized: GLboolean,    
        stride: GLsizei,
        pointer: *const c_void,
    ) -> VertexAttribute {
        unsafe {
            gl::VertexAttribPointer(index, size, r#type, normalized, stride, pointer)
        }
        VertexAttribute { index }
    }

    pub fn enable(&self)  {
        unsafe {
            gl::EnableVertexAttribArray(self.index)
        }
    }
    pub fn disable(&self)  {
        unsafe {
            gl::DisableVertexAttribArray(self.index)
        }
    }


}
