pub enum PlayerAction {
    //The index of the card to play.
    PlayCard(usize),
    //The enemy to target.
    ChooseEnemy(usize),
}

pub enum AvailableAction {
    //Play the i'th card in hand
    PlayCard(u8),
    //Choose an enemy for the card being played.
    ChooseEnemy(u8),
    //End the turn
    EndTurn,
}
