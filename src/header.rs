use *;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Header {
    pub ident: int_t,         /* magic number: "IDP2" */
    pub version: int_t,       /* version: must be 8 */
    pub skinwidth: int_t,     /* texture width */
    pub skinheight: int_t,    /* texture height */
    pub framesize: int_t,     /* size in bytes of a frame */
    pub num_skins: int_t,     /* number of skins */
    pub num_vertices: int_t,  /* number of vertices per frame */
    pub num_st: int_t,        /* number of texture coordinates */
    pub num_tris: int_t,      /* number of triangles */
    pub num_glcmds: int_t,    /* number of opengl commands */
    pub num_frames: int_t,    /* number of frames */
    pub offset_skins: int_t,  /* offset skin data */
    pub offset_st: int_t,     /* offset texture coordinate data */
    pub offset_tris: int_t,   /* offset triangle data */
    pub offset_frames: int_t, /* offset frame data */
    pub offset_glcmds: int_t, /* offset OpenGL command data */
    pub offset_end: int_t,    /* offset end of file */
}

#[derive(Debug)]
pub enum HeaderError {
    ReadIdent,
    InvalidIdent,
    ReadVersion,
    InvalidVersion,

    Version,
    Skinwidth,
    Skinheight,
    Framesize,
    NumSkins,
    NumVertices,
    NumSt,
    NumTris,
    NumGlcmds,
    NumFrames,
    OffsetSkins,
    OffsetSt,
    OffsetTris,
    OffsetFrames,
    OffsetGlcmds,
    OffsetEnd,
}

impl Header {
    pub fn load<R>(rd: &mut R) -> Result<Self, HeaderError>
    where
        R: io::Read,
    {
        const IDENT: int_t = ((('2' as int_t) << 24)
            + (('P' as int_t) << 16)
            + (('D' as int_t) << 8)
            + 'I' as int_t);

        macro_rules! read {
            ($err:expr) => {
                read_i32(rd, $err)?
            };
        }

        let ident = read!(HeaderError::ReadIdent);
        trace!("0x{:04X}", ident);

        if ident != IDENT {
            Err(HeaderError::InvalidIdent)?
        }

        let version = read!(HeaderError::ReadVersion);
        if version != 8 {
            Err(HeaderError::InvalidVersion)?
        }

        Ok(Header {
            ident,
            version,
            skinwidth: read!(HeaderError::Skinwidth),
            skinheight: read!(HeaderError::Skinheight),
            framesize: read!(HeaderError::Framesize),
            num_skins: read!(HeaderError::NumSkins),
            num_vertices: read!(HeaderError::NumVertices),
            num_st: read!(HeaderError::NumSt),
            num_tris: read!(HeaderError::NumTris),
            num_glcmds: read!(HeaderError::NumGlcmds),
            num_frames: read!(HeaderError::NumFrames),
            offset_skins: read!(HeaderError::OffsetSkins),
            offset_st: read!(HeaderError::OffsetSt),
            offset_tris: read!(HeaderError::OffsetTris),
            offset_frames: read!(HeaderError::OffsetFrames),
            offset_glcmds: read!(HeaderError::OffsetGlcmds),
            offset_end: read!(HeaderError::OffsetEnd),
        })
    }
}
