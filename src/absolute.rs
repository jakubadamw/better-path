use crate::{flavours::*, Path, PathInternals};

impl Path<Absolute> {
    pub fn parent(&self) -> Option<&Path<Absolute>> {
        self.inner().parent().map(Self::wrap)
    }
}
