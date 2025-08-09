#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! regex = "1"
//! ```

// △ Unrecommended
#[allow(dead_code)]
#[derive(Debug)]
enum WrappedError {
    Wrapped(Box<dyn std::error::Error>),
    General(String),
}

impl std::fmt::Display for WrappedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WrappedError::Wrapped(e) => write!(f, "Wrapped error: {}", e),
            WrappedError::General(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for WrappedError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            WrappedError::Wrapped(e) => Some(e.as_ref()),
            WrappedError::General(_) => None,
        }
    }
}

// ✖ クレートcoreのimpl<T>From<T> for T実装を衝突し、エラーとなる
// impl<E: 'static + std::error::Error> From<E> for WrappedError {
//     fn from(err: E) -> Self {
//         Self::Wrapped(Box::new(err))
//     }
// }

fn main() {}
