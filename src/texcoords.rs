use {header::*, *};

#[derive(Debug)]
pub struct TexCoords {
    pub s: short_t,
    pub t: short_t,
}

#[derive(Debug)]
pub enum TexCoordsError {
    InvalidWidth,
    InvalidHeight,
}

impl TexCoords {
    pub fn load<R>(rd: &mut R, _: &Header) -> Result<Self, TexCoordsError>
    where
        R: io::Read,
    {
        macro_rules! read {
            ($err:expr) => {
                read_i16(rd, $err)?
            };
        }

        Ok(TexCoords {
            s: read!(TexCoordsError::InvalidWidth),
            t: read!(TexCoordsError::InvalidHeight),
        })
    }
}
