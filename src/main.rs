#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Rank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum CardValue {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}
fn map_card_value(value: char, suit: char) -> Option<Card> {
    let card_value = match value {
        '2' => CardValue::Two,
        '3' => CardValue::Three,
        '4' => CardValue::Four,
        '5' => CardValue::Five,
        '6' => CardValue::Six,
        '7' => CardValue::Seven,
        '8' => CardValue::Eight,
        '9' => CardValue::Nine,
        'T' => CardValue::Ten,
        'J' => CardValue::Jack,
        'Q' => CardValue::Queen,
        'K' => CardValue::King,
        'A' => CardValue::Ace,
        _ => return None,
    };

    let suit_enum = match suit {
        'H' => Suits::Hearts,
        'D' => Suits::Diamonds,
        'C' => Suits::Clubs,
        'S' => Suits::Spades,
        _ => return None,
    };

    Some(Card {
        value: card_value,
        suit: suit_enum,
    })
}


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Suits {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Card {
    value: CardValue,
    suit: Suits,
}



//fn rank_hand(cards: &[Card]) -> Rank {
//    // Implement the logic to determine the hand rank based on the cards
//    // Return the appropriate Rank enum variant
//    // You can use pattern matching, if-else statements, or other logic based on poker rules
//    // ... implementation omitted for brevity
//}

fn main() {
    let hand_str_1 = "8C TS KC 9H 4S";
    let hand_str_2 = "5C AD 5D AC 9C";

    let mut cards_1: Vec<Card> = hand_str_1
    .split(' ')
    .map(|card| {
        let value = card.chars().next().unwrap();
        let suit = card.chars().nth(1).unwrap();
        map_card_value(value, suit).unwrap()
    })
    .collect();

let mut cards_2: Vec<Card> = hand_str_2
    .split(' ')
    .map(|card| {
        let value = card.chars().next().unwrap();
        let suit = card.chars().nth(1).unwrap();
        map_card_value(value, suit).unwrap()
    })
    .collect();



    //let ranked_hand_1 = rank_hand(&cards_1);
    //let ranked_hand_2 = rank_hand(&cards_2);

    //println!("Hand 1 rank: {:?}", ranked_hand_1);
    //println!("Hand 2 rank: {:?}", ranked_hand_2);
//
    //// Compare the rankings to determine the winner
    //if ranked_hand_1 > ranked_hand_2 {
    //    println!("Hand 1 wins!");
    //} else if ranked_hand_1 < ranked_hand_2 {
    //    println!("Hand 2 wins!");
    //} else {
    //    println!("It's a tie!");
    //}

    cards_1.sort();
    cards_2.sort();
    println!("Hand 1 is: {:?}", cards_1);
    println!("Hand 2 is: {:?}", cards_2);
}


    // Read the file and create a vector of strings
    //let file_contents = std::fs::read_to_string("./src/poker.txt").unwrap();
    //let hands: Vec<Vec<Vec<&str>>> = file_contents
    //    .lines()
    //    .map(|line| {
    //        line.split(' ')
    //            .map(|card| card.trim_matches('"'))
    //            .collect::<Vec<&str>>()
    //            .chunks(5)
    //            .map(|chunk| chunk.to_vec())
    //            .collect()
    //    })
    //    .collect();
    //println!("The vector of hands is: {:?}", hands);



    
//fn rank_hand(hand: &[CardValue]) -> Rank {
//    let has_flush = check_flush(hand);
//    let has_straight = check_straight(hand);
//
//    match (has_flush, has_straight) {
//        (true, true) => Rank::StraightFlush,
//        (true, false) => Rank::Flush,
//        (false, true) => Rank::Straight,
//        _ => {
//            if let Some(kind) = check_kind(hand, 4) {
//                Rank::FourOfAKind
//            } else if let Some(kind) = check_kind(hand, 3) {
//                if let Some(_) = check_kind(hand, 2) {
//                    Rank::FullHouse
//                } else {
//                    Rank::ThreeOfAKind
//                }
//            } else if let Some(_) = check_kind(hand, 2) {
//                if let Some(_) = check_kind(hand, 2) {
//                    Rank::TwoPair
//                } else {
//                    Rank::OnePair
//                }
//            } else {
//                Rank::HighCard
//            }
//        }
//    }
//}
//
//fn check_flush(hand: &[CardValue]) -> bool {
//    let first_suit = hand[0] as u8 % 4;
//    hand.iter().all(|&card| card as u8 % 4 == first_suit)
//}
//
//fn check_straight(hand: &[CardValue]) -> bool {
//    let mut values: Vec<u8> = hand.iter().map(|&card| card as u8).collect();
//    values.sort();
//    values.windows(2).all(|window| window[1] == window[0] + 1)
//}
//
//fn check_kind(hand: &[CardValue], num: usize) -> Option<CardValue> {
//    let mut value_counts: std::collections::HashMap<CardValue, usize> = std::collections::HashMap::new();
//    for &card in hand {
//        *value_counts.entry(card).or_insert(0) += 1;
//    }
//    value_counts.iter().find(|(_, &count)| count == num).map(|(&value, _)| value)
//}
//
//