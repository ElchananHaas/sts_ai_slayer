use crate::card::Card;

pub fn insert_sorted(card: Card, vec: &mut Vec<Card>) {
    let pos = vec.binary_search(&card).unwrap_or_else(|e| e);
    vec.insert(pos, card);
}
