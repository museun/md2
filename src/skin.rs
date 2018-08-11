use {header::*, *};

#[derive(Debug)]
pub struct Skin {
    pub name: String, // max 64
}

#[derive(Debug)]
pub enum SkinError {
    NameError(StringError),
}

impl From<StringError> for SkinError {
    fn from(e: StringError) -> Self {
        SkinError::NameError(e)
    }
}

impl Skin {
    pub fn load<R>(rd: &mut R, _: &Header) -> Result<Self, SkinError>
    where
        R: io::Read,
    {
        Ok(Skin {
            name: read_string(rd, 64)?,
        })
    }
}
