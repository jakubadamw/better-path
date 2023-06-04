use crate::{flavours::*, Path, PathInternals};

impl Path<Relative> {
    pub fn parent(&self) -> Option<&Path<Relative>> {
        self.inner().parent().map(Self::wrap)
    }
}
