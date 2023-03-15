#[macro_export]
macro_rules! get_signer {
    ($seed:expr, $nonce:expr) => {{
        &[&[$seed.as_bytes(), &[$nonce]]]
    }};
}

#[macro_export]
macro_rules! size {
    ($name: ident) => {
        impl $name {
            pub const LEN: usize = std::mem::size_of::<$name>() + 8;
        }
    };
}

#[macro_export]
macro_rules! array_ref {
    ($arr:expr, $offset:expr, $len:expr) => {{
        {
            #[inline]
            unsafe fn as_array<T>(slice: &[T]) -> &[T; $len] {
                &*(slice.as_ptr() as *const [_; $len])
            }
            let offset = $offset;
            let slice = &$arr[offset..offset + $len];
            #[allow(unused_unsafe)]
            unsafe {
                as_array(slice)
            }
        }
    }};
}
