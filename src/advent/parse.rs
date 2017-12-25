use ::nom::{ alpha, digit };

#[macro_export]
macro_rules! to_str(
    ($i:expr, $submac:ident!( $($args:tt)* )) => (
        map_res!($i, $submac!($($args)*), ::std::str::from_utf8);
    );

    ($i:expr, $f:expr) => (
        to_str!($i, call!($f));
    );
);

#[macro_export]
macro_rules! from_str(
    ($i:expr, $submac:ident!( $($args:tt)* )) => (
        map_res!($i, $submac!($($args)*), ::std::str::FromStr::from_str);
    );

    ($i:expr, $f:expr) => (
        from_str!($i, call!($f));
    );
);

#[macro_export]
macro_rules! from_str_bytes(
    ($i:expr, $submac:ident!( $($args:tt)* )) => (
        from_str!($i, to_str!($submac!($($args)*)));
    );

    ($i:expr, $f:expr) => (
        from_str_bytes!($i, call!($f));
    );
);

#[macro_export]
macro_rules! lines(
    ($i:expr, $submac:ident!( $($args:tt)* )) => (
        separated_list_complete!($i, ::nom::newline, $submac!($($args)*));
    );

    ($i:expr, $f:expr) => (
        lines!($i, call!($f));
    );
);

named!{ pub name(&[u8]) -> &str,
    to_str!(alpha)
}

named!{ pub unsigned_number(&[u8]) -> usize,
    from_str_bytes!(digit)
}

named!{ pub signed_number (&[u8]) -> isize,
    do_parse!(
        negative: opt!(char!('-')) >>
        digits: unsigned_number >>

        (digits as isize * if negative.is_some() { -1 } else { 1 })
    )
}
