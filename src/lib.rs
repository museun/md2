#![feature(tool_attributes)]
#![feature(concat_idents)]
#![allow(dead_code, non_camel_case_types)]

#[macro_use]
extern crate log;
extern crate byteorder;
extern crate env_logger;

pub(crate) type char_t = i8;
pub(crate) type short_t = i16;
pub(crate) type int_t = i32;
pub(crate) type float_t = i32;

pub(crate) type uchar_t = u8;
pub(crate) type ushort_t = u16;
pub(crate) type uint_t = u32;
pub(crate) type ufloat_t = u32;

pub(crate) type vec3_t = [float_t; 3];

pub mod frame;
pub mod header;
pub mod model;
pub mod skin;
pub mod texcoords;
pub mod triangle;
pub mod vertex;

use byteorder::{LittleEndian, ReadBytesExt};
use std::io;

pub(crate) fn read_many<R, F, T, E>(
    mut rd: R,
    read: F,
    header: &header::Header,
    offset: i32,
    count: i32,
) -> Result<Vec<T>, E>
where
    R: io::Read + io::Seek,
    F: Fn(&mut R, &header::Header) -> Result<T, E>,
{
    skip_to(&mut rd, offset);
    let mut vec = Vec::with_capacity(count as usize);
    for _ in 0..count {
        vec.push(read(&mut rd, header)?)
    }
    Ok(vec)
}

pub(crate) fn skip_to<R>(rd: &mut R, offset: i32)
where
    R: io::Read + io::Seek,
{
    let _pos = rd.seek(io::SeekFrom::Start(offset as u64));
}

#[derive(Debug)]
pub enum StringError {
    Read,
    InvalidUtf8,
}

/// max will read that many bytes
pub(crate) fn read_string<R>(rd: &mut R, max: usize) -> Result<String, StringError>
where
    R: io::Read,
{
    let mut buf = Vec::with_capacity(max);
    for n in 0..max {
        buf.push(rd.read_u8().map_err(|e| {
            warn!("cannot read string at {}: {}", n, e);
            StringError::Read
        })?)
    }

    // this'll truncate it down to the NUL byte
    String::from_utf8(buf.into_iter().take_while(|x| *x != '\0' as u8).collect()).map_err(|e| {
        warn!("invalid utf8 when reading string: {}", e);
        StringError::InvalidUtf8
    })
}

// the num crate would be nice here
// do I want to error function, or just discard the reads?
fn read_i32<R, E>(rd: &mut R, err: E) -> Result<i32, E>
where
    R: io::Read,
{
    rd.read_i32::<LittleEndian>().map_err(|_e| err)
}

fn read_i16<R, E>(rd: &mut R, err: E) -> Result<i16, E>
where
    R: io::Read,
{
    rd.read_i16::<LittleEndian>().map_err(|_e| err)
}

fn read_u8<R, E>(rd: &mut R, err: E) -> Result<u8, E>
where
    R: io::Read,
{
    rd.read_u8().map_err(|_e| err)
}

#[cfg(test)]
mod tests {
    use super::*;
    pub fn init_logger() {
        let _ = env_logger::Builder::from_default_env()
            .default_format_timestamp(false)
            .try_init();
    }

    #[test]
    fn do_stuff() {
        init_logger();

        let mut f = ::std::fs::File::open("./etc/ogro/ogro.md2").expect("to open file");
        let model = model::Model::load(&mut f).expect("no errors");
        println!("{}", model);
    }
}
