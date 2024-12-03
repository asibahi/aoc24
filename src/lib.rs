pub mod template;

// Use this file to add helper functions and additional modules.

pub type BytesResult<'a, T> = nom::IResult<&'a [u8], T, ()>;
