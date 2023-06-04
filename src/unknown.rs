use crate::{flavours::*, Path, PathInternals};

impl Path<Unknown> {
    pub fn parent(&self) -> Option<&Path<Unknown>> {
        self.inner().parent().map(Self::wrap)
    }

    pub fn is_relative(&self) -> bool {
        self.inner().is_relative()
    }

    pub fn is_absolute(&self) -> bool {
        self.inner().is_absolute()
    }
}
