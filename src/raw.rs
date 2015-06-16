pub struct Mat<T> {
    pub data: *mut T,
    pub ncols: u32,
    pub nrows: u32,
}

pub struct Slice<T> {
    pub data: *mut T,
    pub len: u32,
}
