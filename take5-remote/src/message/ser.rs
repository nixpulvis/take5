use super::{RequestMessage, ResponseMessage};
use serde::{Serialize, Serializer};

impl Serialize for ResponseMessage {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: Serializer,
    {
        match *self {
            ResponseMessage::StartRound => serializer.serialize_bool(true),
            ResponseMessage::TakeTurn(ref card) => serializer.serialize_some(card),
            ResponseMessage::Choose(ref stack) => serializer.serialize_some(stack),
        }
    }
}

impl Serialize for RequestMessage {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: Serializer,
    {
        // serializer.serialize_seq(RequestMessageSeqVisitor::new(self))
        serializer.serializer_seq(vec![]);
    }
}

// struct RequestMessageSerializer<'a> {
//     value: &'a RequestMessage,
//     state: u8,
// }

// impl<'a> RequestMessageSerializer<'a> {
//     fn new(value: &'a RequestMessage) -> RequestMessageSerializer{
//         RequestMessageSerializer {
//             value: value,
//             state: 0,
//         }
//     }
// }

// impl<'a> SerializerSeq for RequestMessageSerializer<'a> {
//     fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
//        where T: Serialize + ?Sized
//     {
//     }

//     fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
//     where
//         S: Serializer,
//     {
//         match self.state {
//             0 => {
//                 self.state += 1;
//                 let message_type = match *self.value {
//                     RequestMessage::StartRound(_) => "start-round",
//                     RequestMessage::TakeTurn(_) => "take-turn",
//                     RequestMessage::Choose(_) => "choose",
//                 };
//                 Ok(Some(try!(serializer.visit_seq_elt(message_type))))
//             }
//             1 => {
//                 self.state += 1;
//                 match *self.value {
//                     RequestMessage::StartRound(ref cards) => {
//                         Ok(Some(try!(serializer.visit_seq_elt(cards))))
//                     }
//                     RequestMessage::TakeTurn(ref board) => {
//                         Ok(Some(try!(serializer.visit_seq_elt(board))))
//                     }
//                     RequestMessage::Choose(ref board) => {
//                         Ok(Some(try!(serializer.visit_seq_elt(board))))
//                     }
//                 }
//             }
//             _ => Ok(None),
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use message::{RequestMessage, ResponseMessage};
    use serde_json as json;
    use starting_hand::StartingHand;
    use wrapper::{Card, Stack};

    #[test]
    fn test_request_encode_start_round() {
        let mut cards = Card::deck(|_| 2);
        cards.split_off(10);
        let msg = RequestMessage::StartRound(StartingHand::new(cards).unwrap());
        let json = json::to_string(&msg).unwrap();
        assert_eq!(
            "[\"start-round\",[[1,2],[2,2],[3,2],[4,2],[5,2],[6,2],[7,2],[8,2],[9,2],[10,2]]]",
            json
        );
    }

    #[test]
    fn test_response_encode_start_round() {
        let msg = ResponseMessage::StartRound;
        assert_eq!("true", json::to_string(&msg).unwrap());
    }

    #[test]
    fn test_response_encode_take_turn() {
        let card = Card::deck(|_| 2).remove(0);
        let msg = ResponseMessage::TakeTurn(card);
        assert_eq!("[1,2]", json::to_string(&msg).unwrap());
    }

    #[test]
    fn test_response_encode_choose() {
        let card = Card::deck(|_| 2).remove(0);
        let stack = Stack::new(vec![card]);
        let msg = ResponseMessage::Choose(stack);
        assert_eq!("[[1,2]]", json::to_string(&msg).unwrap());
    }
}
