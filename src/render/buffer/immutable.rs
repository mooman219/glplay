use gl;
use render::buffer::*;
use std::marker::PhantomData;
use std::mem;

pub struct ImmutableBuffer<T> {
    vbo: u32,
    length: usize,
    buffer_type: u32,
    phantom: PhantomData<T>,
}

impl<T> ImmutableBuffer<T> {
    pub fn new(buffer_type: u32, items: Vec<T>) -> ImmutableBuffer<T> {
        let mut vbo = 0u32;
        let size = (mem::size_of::<T>() * items.len()) as isize;
        let data = items.as_ptr() as *const _;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(buffer_type, vbo);
            gl::BufferData(
                buffer_type,     // Buffer type
                size,            // Size
                data,            // Initial data
                gl::STATIC_DRAW, // Usage
            );
            // Testing using BufferStorage vs BufferData
            // gl::BufferStorage(
            //     buffer_type, // Buffer type
            //     size,        // Buffer size
            //     data,        // Initial data
            //     0,           // Flags
            // );
        }
        ImmutableBuffer {
            vbo: vbo,
            length: items.len(),
            buffer_type: buffer_type,
            phantom: PhantomData,
        }
    }
}

impl<T> RawBuffer<T> for ImmutableBuffer<T> {
    fn add(&mut self, _: T) -> usize {
        panic!("Cannot add immutable buffers.");
    }

    fn remove(&mut self, _: usize) {
        panic!("Cannot remove immutable buffers.");
    }

    fn update(&mut self, _: usize, _: T) {
        panic!("Cannot update immutable buffers.");
    }

    fn offset_index(&self) -> usize {
        0
    }

    fn offset_size(&self) -> usize {
        0
    }

    fn len(&self) -> usize {
        self.length
    }

    fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.buffer_type, self.vbo);
        }
    }

    fn sync(&mut self) {
        // We're always in sync.
    }
}

impl<T> Drop for ImmutableBuffer<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vbo as *const _);
        }
    }
}