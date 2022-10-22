use paste::paste;

macro_rules! rand_type_float {
    ( $( $t:ty ),* ) => {
        paste! {
            $(
                #[allow(dead_code)]
                pub fn [<rand_ $t>]() -> $t {
                   ::rand::random::<$t>()
                }

                #[allow(dead_code)]
                pub fn [<rand_max_ $t>](max: $t) -> $t {
                   ::rand::random::<$t>() * max
                }

                #[allow(dead_code)]
                pub fn [<rand_range_ $t>](min: $t, max: $t) -> $t {
                   ::rand::random::<$t>() * (max - min) + min
                }
            )*
        }
    }
}

macro_rules! rand_type_int {
    ( $( $t:ty ),* ) => {
        paste! {
            $(
                #[allow(dead_code)]
                pub fn [<rand_ $t>]() -> $t {
                   ::rand::random::<$t>()
                }

                #[allow(dead_code)]
                pub fn [<rand_max_ $t>](max: $t) -> $t {
                   ::rand::random::<$t>() % max
                }

                #[allow(dead_code)]
                pub fn [<rand_range_ $t>](min: $t, max: $t) -> $t {
                   ::rand::random::<$t>() % (max - min) + min
                }
            )*
        }
    }
}

rand_type_float!(f32, f64);

rand_type_int!(u8, i8, u16, i16, u32, i32, u64, i64, usize, isize);
