use std::io::{Read, Cursor, Result, Seek, SeekFrom};
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};

pub fn write_u24_be(cursor: &mut Cursor<Vec<u8>>, value: u32) -> Result<()> {
    debug_assert!(value <= 16777215, "Value is greater than what can fit in 3 bytes");

    try!(cursor.write_u32::<BigEndian>(value));
    
    {
        let mut inner = cursor.get_mut();
        let index_to_remove = inner.len() - 1 - 3;
        inner.remove(index_to_remove);
    }

    try!(cursor.seek(SeekFrom::End(0)));
    Ok(())
}

pub fn read_u24_be<R: Read>(cursor: &mut R) -> Result<u32> {
    let first_byte = try!(cursor.read_u8()) as u32;
    let second_byte = try!(cursor.read_u8()) as u32;
    let third_byte = try!(cursor.read_u8()) as u32;

    let result = (first_byte * 65536) + (second_byte * 256) + third_byte;
    Ok(result)
}

#[cfg(test)]
mod test {
    use super::{write_u24_be, read_u24_be};
    use std::io::Cursor;
    use byteorder::{WriteBytesExt, ReadBytesExt};

    #[test]
    fn can_write_u24() {
        let mut cursor = Cursor::new(Vec::new());
        write_u24_be(&mut cursor, 16777215).unwrap();

        // Make sure next writes are at the 4th byte
        cursor.write_u8(8).unwrap();
        
        assert_eq!(cursor.into_inner(), vec![255, 255, 255, 8]);
    }

    #[test]
    fn can_read_u24() {
        let mut cursor = Cursor::new(vec![100, 200, 255, 8]);
        let first_result = read_u24_be(&mut cursor).unwrap();
        let second_result = cursor.read_u8().unwrap(); // Make sure cursor advances properly

        assert_eq!(first_result, 6605055);
        assert_eq!(second_result, 8);
    }
}