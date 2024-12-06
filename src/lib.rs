pub mod template;

// Use this file to add helper functions and additional modules.

pub type BytesResult<'a, T, I = &'a [u8]> = nom::IResult<I, T, ()>;

pub type Span<'a, X = ()> = nom_locate::LocatedSpan<&'a [u8], X>;
pub type SpanResult<'a, T> = BytesResult<'a, T, Span<'a>>;

pub mod bitset;
