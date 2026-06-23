use std::marker::PhantomData;

use gl::types::{GLenum, GLint, GLuint, GLvoid};

pub trait Attrib {
    fn get_kind(&self) -> GLenum;
    fn get_size(&self) -> GLint;
    fn get_mem_size(&self) -> GLint;
}

pub struct VertexAttrib<T> {
    kind: GLenum,
    size: GLint,
    phantom: PhantomData<T>,
}

impl<T> VertexAttrib<T> {
    pub fn new(kind: GLenum, size: GLint) -> Self {
        VertexAttrib {
            kind,
            size,
            phantom: PhantomData,
        }
    }
}

impl<T> Attrib for VertexAttrib<T> {
    fn get_kind(&self) -> GLenum {
        self.kind
    }

    fn get_size(&self) -> GLint {
        self.size
    }

    fn get_mem_size(&self) -> GLint {
        (self.size as usize * std::mem::size_of::<T>()) as GLint
    }
}

type VertexAttribArray = Vec<Box<dyn Attrib>>;

trait HasStride {
    fn get_stride(&self) -> GLint;
}

impl<'a> HasStride for VertexAttribArray {
    fn get_stride(&self) -> GLint {
        self.iter().fold(0, |a, b| a + b.get_mem_size())
    }
}

pub struct ArrayObject {
    pub id: GLuint,
    pub attribs: Vec<Box<dyn Attrib>>,
}

impl ArrayObject {
    pub fn new() -> Self {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        ArrayObject {
            id,
            attribs: Vec::new(),
        }
    }

    pub fn add_attrib<T: 'static>(&mut self, attrib: VertexAttrib<T>) {
        self.attribs.push(Box::new(attrib));
    }

    pub fn set(&self) {
        self.bind();
        self.setup();
    }

    fn setup(&self) {
        let stride = self.attribs.get_stride();
        let mut offset = 0;
        for (idx, arr) in self.attribs.iter().enumerate() {
            unsafe {
                gl::EnableVertexAttribArray(idx as u32);
                gl::VertexAttribPointer(
                    idx as u32,
                    arr.get_size(),
                    arr.get_kind(),
                    gl::FALSE,
                    stride,
                    offset as *const GLvoid,
                );
            }
            offset += arr.get_mem_size();
        }
    }

    pub fn get_stride(&self) -> GLint {
        self.attribs.get_stride()
    }

    fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    fn delete(&self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}

impl Drop for ArrayObject {
    fn drop(&mut self) {
        self.unbind();
        self.delete();
    }
}

impl Default for ArrayObject {
    fn default() -> Self {
        ArrayObject::new()
    }
}
