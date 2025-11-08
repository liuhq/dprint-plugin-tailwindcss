pub trait IntoUsize {
    fn into_usize(self) -> usize;
}

impl IntoUsize for u32 {
    fn into_usize(self) -> usize {
        self.try_into().unwrap_or(0)
    }
}

pub trait IntoU32 {
    fn into_u32(self) -> u32;
}

impl IntoU32 for usize {
    fn into_u32(self) -> u32 {
        self.try_into().unwrap_or(0)
    }
}

impl IntoU32 for u8 {
    fn into_u32(self) -> u32 {
        self.try_into().unwrap_or(0)
    }
}
