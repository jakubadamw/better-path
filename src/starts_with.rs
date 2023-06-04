use crate::{flavours::*, Path, PathFlavour, StartsWith};

impl StartsWith<Absolute> for Path<Absolute> {}

impl StartsWith<Unknown> for Path<Absolute> {}

impl StartsWith<Relative> for Path<Relative> {}

impl StartsWith<Unknown> for Path<Relative> {}

impl StartsWith<Absolute> for Path<Unknown> {}

impl StartsWith<Relative> for Path<Unknown> {}

impl StartsWith<Unknown> for Path<Unknown> {}

impl<BF: PathFlavour> Path<BF> {
    pub fn starts_with<OF: PathFlavour>(&self, other: &Path<OF>) -> bool
    where
        Path<BF>: StartsWith<OF>,
    {
        self.starts_with_internal(other)
    }
}
