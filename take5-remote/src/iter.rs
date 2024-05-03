use serde::Deserialize;
use serde_json as json;
use serde_json::error::Error as JsonError;
use std::io::{Bytes, Read};
use std::iter::Peekable;
use std::marker::PhantomData;

/// Iterator over JSON deserialized data.
pub struct DeserializeJsonIter<'a, 'de, T: Deserialize<'de>, R: 'a + Read> {
    bytes: Peekable<Bytes<&'a mut R>>,
    _phantom: PhantomData<T>,
    _phantom2: PhantomData<&'de ()>,
}

impl<'a, 'de, T: Deserialize<'de>, R: Read> DeserializeJsonIter<'a, 'de, T, R> {
    /// Returns an iterator given some reader, for example a `TcpStream`.
    pub fn new(reader: &mut R) -> DeserializeJsonIter<'a, 'de, T, R> {
        DeserializeJsonIter {
            bytes: reader.bytes().peekable(),
            _phantom: PhantomData,
            _phantom2: PhantomData,
        }
    }
}

impl<'a, 'de, T: Deserialize<'de>, R: Read> Iterator for DeserializeJsonIter<'a, 'de, T, R> {
    type Item = Result<T, JsonError>;

    fn next(&mut self) -> Option<Result<T, JsonError>> {
        if self.bytes.peek().is_none() {
            None
        } else {
            let mut deserializer = json::Deserializer::new(self.bytes);
            Some(Deserialize::deserialize(&mut deserializer))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use message::RequestMessage;
    use std::io::Cursor;
    use take5::StackId;

    #[test]
    fn test_iterate_message() {
        let json = "[\"take-turn\",[[[1,2]],[[2,2]],[[3,2]],[[4,2]]]][\"take-turn\",[[[1,2]],[[2,2]],[[3,2]],[[4,2]]]]";
        let mut stream = Cursor::new(json);
        for result in DeserializeJsonIter::new(&mut stream) {
            match result {
                Ok(RequestMessage::TakeTurn(board)) => {
                    assert_eq!(1, board[StackId::A].len());
                }
                _ => assert!(false),
            }
        }
    }
}
