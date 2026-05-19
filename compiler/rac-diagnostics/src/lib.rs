#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize
}

#[macro_export]
macro_rules! select {
    ($src:expr, $span:expr) => {
        $src[$span.start .. $span.end]
    };
}

pub enum Stage {
    Parsing,
    Resolving,
    Typechecking
}

pub struct Report {
    pub stage: Stage,
    pub span: Span,
    pub msg: String
}

// pub enum Result<T> {
//     Value(T),
//     Error(
//         Span, // span of the erroneous code
//         String // the error message
//     )
// }
//
// impl<T> Result<T> {
//     pub fn bind<V> (self, f: impl FnOnce(T) -> Result<V>) -> Result<V> {
//         match self {
//             Result::Value(t) => f(t),
//             Result::Error(sp, str) => Result::Error(sp, str)
//         }
//     }
// }
