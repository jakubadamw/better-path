use crate::{flavours::*, Join, Path, PathBuf, PathFlavour};

impl Join<Absolute> for Path<Absolute> {
    type ResultFlavour = Absolute;
}

impl Join<Relative> for Path<Absolute> {
    type ResultFlavour = Absolute;
}

impl Join<Unknown> for Path<Absolute> {
    type ResultFlavour = Absolute;
}

impl Join<Absolute> for Path<Relative> {
    type ResultFlavour = Absolute;
}

impl Join<Relative> for Path<Relative> {
    type ResultFlavour = Relative;
}

impl Join<Unknown> for Path<Relative> {
    type ResultFlavour = Unknown;
}

impl Join<Absolute> for Path<Unknown> {
    type ResultFlavour = Absolute;
}

impl Join<Relative> for Path<Unknown> {
    type ResultFlavour = Unknown;
}

impl Join<Unknown> for Path<Unknown> {
    type ResultFlavour = Unknown;
}

impl<BF: PathFlavour> Path<BF> {
    pub fn join<OF: PathFlavour>(
        &self,
        other: &Path<OF>,
    ) -> PathBuf<<Self as Join<OF>>::ResultFlavour>
    where
        Path<BF>: Join<OF>,
    {
        self.join_internal(other)
    }
}
