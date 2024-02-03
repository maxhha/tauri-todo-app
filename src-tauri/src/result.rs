use core::result;
use std::fmt::{self, Debug, Display};

use serde::ser::SerializeMap;

pub struct Error {
    inner: anyhow::Error,
}

impl From<anyhow::Error> for Error {
    #[cold]
    fn from(error: anyhow::Error) -> Self {
        Self { inner: error }
    }
}

impl From<validator::ValidationErrors> for Error {
    #[cold]
    fn from(error: validator::ValidationErrors) -> Self {
        Self {
            inner: error.into(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        <anyhow::Error as Display>::fmt(&self.inner, formatter)
    }
}

impl Debug for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        <anyhow::Error as Debug>::fmt(&self.inner, formatter)
    }
}

impl std::error::Error for Error {}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;

        if self.inner.is::<validator::ValidationErrors>() {
            let errors = self
                .inner
                .downcast_ref::<validator::ValidationErrors>()
                .expect("downcast error");

            map.serialize_entry("validation", errors)?;
        } else {
            let error = serde_error::Error::new(&*self.inner);

            map.serialize_entry("unknown", &error)?;
        }
        map.end()
    }
}

pub type Result<T> = result::Result<T, Error>;
