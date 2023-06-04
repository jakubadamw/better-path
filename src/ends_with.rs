use crate::{flavours::*, EndsWith, Path, PathFlavour};

impl EndsWith<Absolute> for Path<Absolute> {}

impl EndsWith<Relative> for Path<Absolute> {}

impl EndsWith<Unknown> for Path<Absolute> {}

impl EndsWith<Relative> for Path<Relative> {}

impl EndsWith<Unknown> for Path<Relative> {}

impl EndsWith<Absolute> for Path<Unknown> {}

impl EndsWith<Relative> for Path<Unknown> {}

impl EndsWith<Unknown> for Path<Unknown> {}

impl<BF: PathFlavour> Path<BF> {
    pub fn ends_with<OF: PathFlavour>(&self, other: &Path<OF>) -> bool
    where
        Path<BF>: EndsWith<OF>,
    {
        self.ends_with_internal(other)
    }
}
