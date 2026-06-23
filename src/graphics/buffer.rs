use gl::types::{GLenum, GLsizeiptr, GLuint, GLvoid};

#[derive(Clone)]
pub struct BufferObject {
    pub id: GLuint,
    kind: GLenum,
}

impl BufferObject {
    pub fn new(kind: GLenum) -> Self {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }

        Self { id, kind }
    }

    pub fn data<T>(&self, data: &[T]) {
        self.bind();
        unsafe {
            let (_, data_bytes, _) = data.align_to::<u8>();
            gl::BufferData(
                self.kind,
                data_bytes.len() as GLsizeiptr,
                data_bytes.as_ptr() as *const GLvoid,
                gl::DYNAMIC_DRAW,
            )
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.kind, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(self.kind, 0);
        }
    }

    fn delete(&self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}

impl Drop for BufferObject {
    fn drop(&mut self) {
        self.unbind();
        self.delete();
    }
}
