//! Business-logic-like structures for processing parsed messages

pub mod request;
pub mod response;
pub mod encryption;

pub use request::RequestProcesor;
pub use response::ResponseProcesor;
