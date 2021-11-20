use std::io::{Cursor, Read};

use smallbufreader::SmallBufReader;

#[test]
fn test_initialization() {
    let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let data_clone = data.clone();
    let data_cursor = Cursor::new(data);
    let mut reader = SmallBufReader::<Cursor<Vec<u8>>, [u8; 32]>::new(data_cursor);

    let mut read_buf: Vec<u8> = Vec::new();
    let read_len = reader
        .read_to_end(&mut read_buf)
        .expect("Failed to read to end");

    assert_eq!(data_clone.len(), read_len);
    assert_eq!(data_clone, read_buf);
}
