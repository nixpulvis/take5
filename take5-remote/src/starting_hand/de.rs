use super::StartingHand;
use serde::de::Error;
use serde::{Deserialize, Deserializer};

impl<'de> Deserialize<'de> for StartingHand {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // let cards = try!(deserializer.visit_seq(VecVisitor::new()));
        let cards = Vec::deserialize(deserializer)?;
        let len = cards.len();
        match StartingHand::new(cards) {
            Ok(hand) => Ok(hand),
            // Err(_) => Err(Error::length_mismatch(len)),
            Err(_) => Err(Error::custom(len)),
        }
    }
}
