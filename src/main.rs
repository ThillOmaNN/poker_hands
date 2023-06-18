use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Card {
    value: CardValue,
    suit: Suits,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Suits {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
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
enum Rank {
    HighCard,
    OnePair, 
    TwoPairs,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

fn get_card_value_histogram(cards: &[Card]) -> Vec<(CardValue, usize)> {
    let mut histogram: HashMap<CardValue, usize> = HashMap::new();

    for card in cards {
        *histogram.entry(card.value.clone()).or_insert(0) += 1;
    }

    let mut sorted_histogram: Vec<(CardValue, usize)> = histogram.into_iter().collect();
    sorted_histogram.sort_by(|a, b| {
        let count_comparison = b.1.cmp(&a.1);
        if count_comparison == std::cmp::Ordering::Equal {
            a.0.cmp(&b.0)
        } else {
            count_comparison
        }
    });
    sorted_histogram
}

fn check_flush(hand: &[Card]) -> bool {
    let first_suit = &hand[0].suit;
    hand.iter().all(|card| &card.suit == first_suit)
}

fn check_straight(hand: &[Card]) -> bool {
    let mut values: Vec<u8> = hand.iter().map(|card| card.value.clone() as u8 ).collect();
    values.sort();
    values.windows(2).all(|window| window[1] == window[0] + 1)
}

fn rank_hand(histogram: &[(CardValue, usize)], hand: &[Card]) -> Rank {
    let has_flush = check_flush(hand);
    let has_straight = check_straight(hand);

    match (has_flush, has_straight) {
        (true, true) => Rank::StraightFlush,
        (true, false) => Rank::Flush,
        (false, true) => Rank::Straight,
        _ => {
            // If the hand doesn't match any of the above cases, check other ranks
            match histogram {
                [(_value, 4), ..] => Rank::FourOfAKind,
                [(_value1, 3), (_value2, 2), ..] => Rank::FullHouse,
                [(_value, 3), ..] => Rank::ThreeOfAKind,
                [(_value1, 2), (_value2, 2), ..] => Rank::TwoPairs,
                [(_value, 2), ..] => Rank::OnePair,
                _ => Rank::HighCard, // Default case if no other rank matches
            }
        }
    }
}

#[derive(Debug)]
enum ComparisonResult {
    Hand1Wins,
    Hand2Wins,
    Tie,
}

fn compare_hands(rank_1: &Rank, cards_1: &[Card], rank_2: &Rank, cards_2: &[Card]) -> ComparisonResult {
    match rank_1.cmp(&rank_2) {
        std::cmp::Ordering::Greater => return ComparisonResult::Hand1Wins,
        std::cmp::Ordering::Less => return ComparisonResult::Hand2Wins,
        std::cmp::Ordering::Equal => {
            match rank_1 {
                Rank::StraightFlush | Rank::Straight => {
                    let hand_1_highest_card = &cards_1.first().unwrap().value;
                    let hand_2_highest_card = &cards_2.first().unwrap().value;

                    match hand_1_highest_card.cmp(&hand_2_highest_card) {
                        std::cmp::Ordering::Greater => return ComparisonResult::Hand1Wins,
                        std::cmp::Ordering::Less => return ComparisonResult::Hand2Wins,
                        std::cmp::Ordering::Equal => return ComparisonResult::Tie,
                    }
                }
                Rank::Flush => {
                    let hand_1_sorted_values: Vec<u8> = cards_1
                        .iter()
                        .map(|card| card.value.clone() as u8)
                        .collect();
                    let hand_2_sorted_values: Vec<u8> = cards_2
                        .iter()
                        .map(|card| card.value.clone() as u8)
                        .collect();

                    // Compare each card value from highest to lowest
                    for (value_1, value_2) in hand_1_sorted_values
                        .iter()
                        .zip(hand_2_sorted_values.iter()) 
                    {
                        match value_1.cmp(&value_2) {
                            std::cmp::Ordering::Greater => return ComparisonResult::Hand1Wins,
                            std::cmp::Ordering::Less => return ComparisonResult::Hand2Wins,
                            std::cmp::Ordering::Equal => continue,
                        }
                    }

                    // If all card values are equal, it's a tie
                    return ComparisonResult::Tie;
                }
                Rank::FourOfAKind => {
                    let hand_1_matching_card_value = &cards_1[0].value;
                    let hand_2_matching_card_value = &cards_2[0].value;
        
                    match hand_1_matching_card_value.cmp(&hand_2_matching_card_value) {
                        std::cmp::Ordering::Greater => return ComparisonResult::Hand1Wins,
                        std::cmp::Ordering::Less => return ComparisonResult::Hand2Wins,
                        std::cmp::Ordering::Equal => return ComparisonResult::Tie,
                    }
                }
                Rank::FullHouse => {
                    let hand_1_matching_card_values = (&cards_1[0].value, &cards_1[3].value);
                    let hand_2_matching_card_values = (&cards_2[0].value, &cards_2[3].value);
                                    match hand_1_matching_card_values.cmp(&hand_2_matching_card_values) {
                                        std::cmp::Ordering::Greater => return ComparisonResult::Hand1Wins,
                                        std::cmp::Ordering::Less => return ComparisonResult::Hand2Wins,
                                        std::cmp::Ordering::Equal => return ComparisonResult::Tie,
                                    }
                }
                Rank::ThreeOfAKind => {
                    let hand_1_matching_card_value = &cards_1[0].value;
                    let hand_2_matching_card_value = &cards_2[0].value;
        
                    match hand_1_matching_card_value.cmp(&hand_2_matching_card_value) {
                        std::cmp::Ordering::Greater => return ComparisonResult::Hand1Wins,
                        std::cmp::Ordering::Less => return ComparisonResult::Hand2Wins,
                        std::cmp::Ordering::Equal => return ComparisonResult::Tie,
                    }
                }
                Rank::TwoPairs => {
                    let hand_1_matching_card_values = (&cards_1[0].value, &cards_1[2].value);
                    let hand_2_matching_card_values = (&cards_2[0].value, &cards_2[2].value);
                
                    match hand_1_matching_card_values.cmp(&hand_2_matching_card_values) {
                        std::cmp::Ordering::Greater => return ComparisonResult::Hand1Wins,
                        std::cmp::Ordering::Less => return ComparisonResult::Hand2Wins,
                        std::cmp::Ordering::Equal => {
                            // Compare the remaining 5th card value if the two pairs are equal
                            let hand_1_remaining_card_value = &cards_1[4].value;
                            let hand_2_remaining_card_value = &cards_2[4].value;
                
                            match hand_1_remaining_card_value.cmp(&hand_2_remaining_card_value) {
                                std::cmp::Ordering::Greater => return ComparisonResult::Hand1Wins,
                                std::cmp::Ordering::Less => return ComparisonResult::Hand2Wins,
                                std::cmp::Ordering::Equal => return ComparisonResult::Tie,
                            }
                        }
                    }
                }                
                Rank::OnePair => {
                    let hand_1_matching_card_value = &cards_1[0].value;
                    let hand_2_matching_card_value = &cards_2[0].value;
        
                    match hand_1_matching_card_value.cmp(&hand_2_matching_card_value) {
                        std::cmp::Ordering::Greater => return ComparisonResult::Hand1Wins,
                        std::cmp::Ordering::Less => return ComparisonResult::Hand2Wins,
                        std::cmp::Ordering::Equal => return ComparisonResult::Tie,
                    }
                }
                Rank::HighCard => {
                    let hand_1_sorted_values: Vec<u8> = cards_1
                        .iter()
                        .map(|card| card.value.clone() as u8)
                        .collect();
                    let hand_2_sorted_values: Vec<u8> = cards_2
                        .iter()
                        .map(|card| card.value.clone() as u8)
                        .collect();

                    // Compare each card value from highest to lowest
                    for (value_1, value_2) in hand_1_sorted_values
                        .iter()
                        .zip(hand_2_sorted_values.iter())
                    {
                        match value_1.cmp(&value_2) {
                            std::cmp::Ordering::Greater => return ComparisonResult::Hand1Wins,
                            std::cmp::Ordering::Less => return ComparisonResult::Hand2Wins,
                            std::cmp::Ordering::Equal => return ComparisonResult::Tie,
                        }
                    }

                    // If all card values are equal, it's a tie
                    return ComparisonResult::Tie;
                }
            }
        }
    }
}

fn process_hand(hand_str: &str) -> (Rank, Vec<Card>) {
    let mut cards: Vec<Card> = hand_str
        .split(' ')
        .map(|card| {
            let value = card.chars().next().unwrap();
            let suit = card.chars().nth(1).unwrap();
            map_card_value(value, suit).unwrap()
        })
        .collect();

    let histogram = get_card_value_histogram(&cards);

    // Sort the cards based on the histogram and then by value in descending order
    cards.sort_by(|a, b| {
        let count_a = histogram.iter().find(|(value, _)| *value == a.value).map(|(_, count)| count).unwrap_or(&0);
        let count_b = histogram.iter().find(|(value, _)| *value == b.value).map(|(_, count)| count).unwrap_or(&0);

        if count_a != count_b {
            count_b.cmp(count_a) // Sort by count in descending order
        } else {
            b.value.cmp(&a.value) // Sort by value in descending order
        }
    });

    let rank = rank_hand(&histogram, &cards);

    (rank, cards)
}


fn main() {
    // Read the file and create a vector of strings
    let file_contents = std::fs::read_to_string("./poker.txt").unwrap();
    let hands: Vec<Vec<Vec<&str>>> = file_contents
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|card| card.trim_matches('"'))
                .collect::<Vec<&str>>()
                .chunks(5)
                .map(|chunk| chunk.to_vec())
                .collect()
        })
        .collect();
    
    let mut hand1_wins = 0;
    let mut hand2_wins = 0;
    let mut tied = 0;
    
    for pair in hands {
        let hand1 = &pair[0];
        let hand2 = &pair[1];
    
        let hand1_str = hand1.join(" ");
        let hand2_str = hand2.join(" ");
            
        let (rank1, cards1) = process_hand(&hand1_str);
        let (rank2, cards2) = process_hand(&hand2_str);
    
        let result = compare_hands(&rank1, &cards1, &rank2, &cards2);

        match result {
            ComparisonResult::Hand1Wins => hand1_wins += 1,
            ComparisonResult::Hand2Wins => hand2_wins += 1,
            ComparisonResult::Tie => tied += 1,
        }
    }
    
    println!("Hand 1 wins: {}", hand1_wins);
    println!("Hand 2 wins: {}", hand2_wins);
    println!("Tied: {}", tied);
}