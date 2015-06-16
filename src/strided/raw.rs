pub struct Slice<T> {
    pub data: *mut T,
    pub len: u32,
    pub stride: u32,
}
