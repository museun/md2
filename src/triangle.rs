use {header::*, vertex::*, *};

#[derive(Debug)]
pub struct Triangle {
    vertex: [Vertex; 3],
    st: [short_t; 3],
}

#[derive(Debug)]
pub enum TriangleError {
    MissingIndex,
    VertexError(VertexError),
}

impl Triangle {
    pub fn load<R>(rd: &mut R, hd: &Header) -> Result<Self, TriangleError>
    where
        R: io::Read,
    {
        macro_rules! read {
            ($err:expr) => {
                read_i16(rd, $err)?
            };
        }

        let st = [
            read!(TriangleError::MissingIndex),
            read!(TriangleError::MissingIndex),
            read!(TriangleError::MissingIndex),
        ];

        let vertex = [
            vertex::Vertex::load(rd, hd).map_err(|e| TriangleError::VertexError(e))?,
            vertex::Vertex::load(rd, hd).map_err(|e| TriangleError::VertexError(e))?,
            vertex::Vertex::load(rd, hd).map_err(|e| TriangleError::VertexError(e))?,
        ];

        Ok(Self { vertex, st })
    }
}
