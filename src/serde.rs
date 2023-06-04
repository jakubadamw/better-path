impl<'de, PF: crate::PathFlavour> serde::Deserialize<'de> for crate::PathBuf<PF> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        use serde::de::Error;

        let string = String::deserialize(deserializer)?;
        Ok(crate::Path::<PF>::new(&string)
            .map_err(|err| D::Error::custom(err.to_string()))?
            .to_owned())
    }
}
