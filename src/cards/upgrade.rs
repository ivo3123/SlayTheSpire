use crate::core::card::Card;

pub fn upgrade_card(card: Card) -> Card {
    (card.upgrade_fn())(card.instance_id())
}
