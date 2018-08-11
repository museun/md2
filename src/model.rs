use {frame::*, header::*, skin::*, std::fmt, texcoords::*, triangle::*, *};

#[derive(Debug)]
pub struct Model {
    pub header: Header,
    pub skins: Vec<Skin>,
    pub texcoords: Vec<TexCoords>,
    pub triangles: Vec<Triangle>,
    pub frames: Vec<Frame>,
    // gl stuff
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "skins: {}", self.skins.len())?;
        writeln!(f, "texcoords: {}", self.texcoords.len())?;
        writeln!(f, "triangles: {}", self.triangles.len())?;
        writeln!(f, "frames: {}", self.frames.len())
    }
}

#[derive(Debug)]
pub enum ModelError {
    HeaderError(HeaderError),
    SkinError(SkinError),
    TexCoordsError(TexCoordsError),
    TriangleError(TriangleError),
    FrameError(FrameError),
}

impl Model {
    pub fn load<R>(mut rd: R) -> Result<Self, ModelError>
    where
        R: io::Read + io::Seek,
    {
        let header = Header::load(&mut rd).map_err(|e| ModelError::HeaderError(e))?;
        let skins = read_many(
            &mut rd,             // reader
            Skin::load,          // load function
            &header,             // header
            header.offset_skins, // offset
            header.num_skins,    // number of items
        ).map_err(|e| ModelError::SkinError(e))?;

        let texcoords = read_many(
            &mut rd,          // reader
            TexCoords::load,  // load function
            &header,          // header
            header.offset_st, // offset
            header.num_st,    // number of items
        ).map_err(|e| ModelError::TexCoordsError(e))?;

        let triangles = read_many(
            &mut rd,            // reader
            Triangle::load,     // load function
            &header,            // header
            header.offset_tris, // offset
            header.num_tris,    // number of items
        ).map_err(|e| ModelError::TriangleError(e))?;

        let frames = read_many(
            &mut rd,              // reader
            Frame::load,          // load function
            &header,              // header
            header.offset_frames, // offset
            header.num_frames,    // number of items
        ).map_err(|e| ModelError::FrameError(e))?;

        Ok(Model {
            header,
            skins,
            texcoords,
            triangles,
            frames,
        })
    }
}
