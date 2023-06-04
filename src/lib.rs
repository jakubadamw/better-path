pub trait PathFlavour: Sized {
    type ParseError: std::fmt::Display;

    fn new_path<S: AsRef<std::ffi::OsStr> + ?Sized>(
        string: &S,
    ) -> Result<&Path<Self>, Self::ParseError>;
}

pub trait PathInternals {
    fn inner(&self) -> &std::path::Path;
}

pub trait Join<OperandFlavour: crate::PathFlavour>: PathInternals {
    type ResultFlavour: crate::PathFlavour;
    fn join_internal(&self, operand: &Path<OperandFlavour>) -> PathBuf<Self::ResultFlavour> {
        PathBuf::wrap(self.inner().join(operand.inner()))
    }
}

pub trait StartsWith<OperandFlavour: crate::PathFlavour>: PathInternals {
    fn starts_with_internal(&self, operand: &Path<OperandFlavour>) -> bool {
        self.inner().starts_with(operand.inner())
    }
}

pub trait EndsWith<OperandFlavour: crate::PathFlavour>: PathInternals {
    fn ends_with_internal(&self, operand: &Path<OperandFlavour>) -> bool {
        self.inner().ends_with(operand.inner())
    }
}

mod flavours {
    use std::ffi::OsStr;

    pub struct Absolute;
    #[derive(Debug, thiserror::Error)]
    #[error("path is relative")]
    pub struct PathIsRelative;
    impl super::PathFlavour for Absolute {
        type ParseError = PathIsRelative;

        fn new_path<S: AsRef<OsStr> + ?Sized>(
            string: &S,
        ) -> Result<&super::Path<Self>, Self::ParseError> {
            let path = std::path::Path::new(string);
            if path.is_absolute() {
                Ok(super::Path::wrap(path))
            } else {
                Err(PathIsRelative)
            }
        }
    }

    pub struct Relative;
    #[derive(Debug, thiserror::Error)]
    #[error("path is absolute")]
    pub struct PathIsAbsolute;
    impl super::PathFlavour for Relative {
        type ParseError = PathIsAbsolute;

        fn new_path<S: AsRef<OsStr> + ?Sized>(
            string: &S,
        ) -> Result<&super::Path<Self>, Self::ParseError> {
            let path = std::path::Path::new(string);
            if path.is_relative() {
                Ok(super::Path::wrap(path))
            } else {
                Err(PathIsAbsolute)
            }
        }
    }

    pub struct Unknown;
    #[derive(Debug, thiserror::Error)]
    #[error("impossible")]
    pub enum Impossible {}
    impl super::PathFlavour for Unknown {
        type ParseError = Impossible;

        fn new_path<S: AsRef<OsStr> + ?Sized>(
            string: &S,
        ) -> Result<&super::Path<Self>, Impossible> {
            Ok(super::Path::wrap(std::path::Path::new(string)))
        }
    }
}

use std::ffi::OsStr;

pub use flavours::*;

mod absolute;
mod ends_with;
mod join;
mod relative;
mod starts_with;
mod unknown;

#[cfg(feature = "serde")]
mod serde;

#[cfg(test)]
mod tests;

#[derive(ref_cast::RefCast)]
#[repr(transparent)]
pub struct Path<PF: PathFlavour = Unknown>(std::marker::PhantomData<PF>, std::path::Path);

impl<PF: PathFlavour> PathInternals for Path<PF> {
    fn inner<'a>(&'a self) -> &'a std::path::Path {
        &self.1
    }
}

#[derive(Copy, Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Ancestors<'a, PF: PathFlavour> {
    next: Option<&'a std::path::Path>,
    data: std::marker::PhantomData<PF>,
}

impl<'a, PF: PathFlavour + 'a> Iterator for Ancestors<'a, PF> {
    type Item = &'a Path<PF>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next;
        self.next = next.and_then(|path| path.parent());
        next.map(Path::<PF>::wrap)
    }
}

impl<PF: PathFlavour> Path<PF> {
    fn wrap<'a>(path: &'a std::path::Path) -> &'a Path<PF> {
        use ref_cast::RefCast;
        Self::ref_cast(path)
    }

    pub fn new<S: AsRef<std::ffi::OsStr> + ?Sized>(string: &S) -> Result<&Self, PF::ParseError> {
        PF::new_path(string)
    }

    pub fn ancestors(&self) -> Ancestors<'_, PF> {
        Ancestors {
            next: Some(self.inner()),
            data: Default::default(),
        }
    }

    pub fn components(&self) -> std::path::Components<'_> {
        self.1.components()
    }

    pub fn display(&self) -> std::path::Display<'_> {
        self.1.display()
    }

    pub fn extension(&self) -> Option<&OsStr> {
        self.1.extension()
    }

    pub fn file_name(&self) -> Option<&OsStr> {
        self.1.file_name()
    }

    pub fn file_stem(&self) -> Option<&OsStr> {
        self.1.file_stem()
    }

    pub fn with_extension<S: AsRef<OsStr>>(&self, extension: S) -> PathBuf<PF> {
        let mut buf = self.1.to_path_buf();
        buf.set_extension(extension);
        PathBuf::wrap(buf)
    }

    pub fn with_file_name<S: AsRef<OsStr>>(&self, file_name: S) -> PathBuf<PF> {
        let mut buf = self.1.to_path_buf();
        buf.set_file_name(file_name);
        PathBuf::wrap(buf)
    }

    pub fn as_std(&self) -> &std::path::Path {
        &self.1
    }
}

impl<PF: PathFlavour> std::borrow::Borrow<Path<PF>> for PathBuf<PF> {
    fn borrow(&self) -> &Path<PF> {
        Path::wrap(self.path_buf.as_path())
    }
}

impl<PF: PathFlavour> std::borrow::ToOwned for Path<PF> {
    type Owned = PathBuf<PF>;

    fn to_owned(&self) -> Self::Owned {
        PathBuf::wrap(self.1.to_owned())
    }
}

pub struct PathBuf<PF: PathFlavour> {
    path_buf: std::path::PathBuf,
    state: std::marker::PhantomData<PF>,
}

impl<PF: PathFlavour> PathBuf<PF> {
    fn wrap(path_buf: std::path::PathBuf) -> PathBuf<PF> {
        Self {
            path_buf,
            state: std::marker::PhantomData,
        }
    }
}
