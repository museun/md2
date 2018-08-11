use {header::*, vertex::*, *};

#[derive(Debug)]
pub struct Frame {
    scale: vec3_t,
    translate: vec3_t,
    name: String, // max 16
    verts: Vec<Vertex>,
}

#[derive(Debug)]
pub enum FrameError {
    MissingScale,
    MissingTranslate,
    MissingName,
    MissingVertices(VertexError),
    NameError(StringError),
}

impl From<StringError> for FrameError {
    fn from(e: StringError) -> Self {
        FrameError::NameError(e)
    }
}
impl From<VertexError> for FrameError {
    fn from(e: VertexError) -> Self {
        FrameError::MissingVertices(e)
    }
}

impl Frame {
    pub fn load<R>(rd: &mut R, header: &Header) -> Result<Self, FrameError>
    where
        R: io::Read + io::Seek,
    {
        macro_rules! read_vec3 {
            ($err:expr) => {
                [
                    read_i32(rd, $err)?,
                    read_i32(rd, $err)?,
                    read_i32(rd, $err)?,
                ]
            };
        }

        let scale = read_vec3!(FrameError::MissingScale);
        let translate = read_vec3!(FrameError::MissingTranslate);
        let name = read_string(rd, 16)?;

        let mut verts = Vec::with_capacity(header.num_vertices as usize);
        for _ in 0..header.num_vertices {
            verts.push(Vertex::load(rd, header)?);
        }

        Ok(Self {
            scale,
            translate,
            name,
            verts,
        })
    }
}
