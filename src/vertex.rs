use {header::*, *};

#[derive(Debug)]
pub struct Vertex {
    v: [uchar_t; 3],
    normal_index: uchar_t,
}

#[derive(Debug)]
pub enum VertexError {
    MissingVertex,
    MissingNormalIndex,
}

impl Vertex {
    pub fn load<R>(rd: &mut R, _: &Header) -> Result<Self, VertexError>
    where
        R: io::Read,
    {
        macro_rules! read {
            ($err:expr) => {
                read_u8(rd, $err)?
            };
        }

        let v = [
            read!(VertexError::MissingVertex),
            read!(VertexError::MissingVertex),
            read!(VertexError::MissingVertex),
        ];

        let normal_index = read!(VertexError::MissingNormalIndex);
        Ok(Self { v, normal_index })
    }
}
