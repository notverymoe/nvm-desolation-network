use tinyvec::SliceVec;

pub struct Buffer<T: Default + Copy>(Box<[T]>);

impl<T: Default + Copy> Buffer<T> {

    pub fn new(size: usize) -> Self {
        let mut buffer = Vec::with_capacity(size);
        buffer.resize(size, Default::default());
        Self(buffer.into_boxed_slice())
    }

    pub fn get(&mut self) -> SliceVec<T> {
        SliceVec::from_slice_len(&mut self.0, 0)
    }

}