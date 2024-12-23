use std::io;
use std::cmp::Ordering;
use rand::Rng;

struct Card {
    value: String,
    suit: Suit,
    hidden: bool
}

impl Card {
    fn reveal_card(&mut self) {
        self.hidden = false;
    }
}

enum Suit {
    Clubs,
    Spades,
    Diamonds,
    Hearts
}

struct Player {
    name: String,
    cards: Vec<Card>
}

impl Player {
    fn print_cards(&self) {
        // Prints out all of a player's currently-held cards

        let held_cards = &self.cards[..];

        for card in held_cards {

            if card.hidden {
                print!("  |XXXXXXXXX|");
                continue;
            }

            print!("  | ");

            match card.suit{
                Suit::Spades => print!("{}", '♠'),
                Suit::Clubs => print!("{}", '♣'),
                Suit::Diamonds => print!("{}", '♦'),
                Suit::Hearts => print!("{}", '♥')
            };

            print!("    {} |", card.value.as_str());
        }

        print!("\n");
    }

    fn draw_starting_hand(&mut self) {
        self.cards.push(draw_card(false));

        if self.name == "Dealer".to_string() {
            self.cards.push(draw_card(true));
        } else {
            self.cards.push(draw_card(false));
        }
    }

    fn get_total_value(&self) -> i32 {
        // Returns the total numerical value of the player's hand

        let held_cards = &self.cards[..];

        let mut total = 0;

        for card in held_cards {

            // TODO: improve ace value checks
            
            match card.value.as_str() {
                "A" => if total + 11 > 21 { total += 1} else {total += 11},
                "J" | "Q" | "K" => total += 10,
                _ => total += card.value.parse::<i32>().expect("Error parsing card value!")
            }
        }

        return total;
    }
}

fn yes_no() -> bool {
    let mut ans = String::new();

    loop {
        io::stdin()
            .read_line(&mut ans)
            .expect("Failed to read line");

        let ans = ans.trim();

        match ans {
            "y" | "Y" => return true,
            "n" | "N" => return false,
            _ => println!("Please enter y or n"),
        }
    }
}

fn draw_card(hidden : bool) -> Card{
    // Draw and return one card out of a standard 52 card deck
    let card = rand::thread_rng().gen_range(1..52);

    // 52 % 13 gets a value between 0-12, which will represent a card's value
    let val : String = match card % 13 {
        1 => "A".to_string(),
        11 => "J".to_string(),
        12 => "Q".to_string(),
        0 => "K".to_string(),
        _ => (card % 13).to_string(),
    };

    // 52 / 13 gets a value between 0 and 4, which will represent a card's suit
    let suit = match card / 13 {
        0 => Suit::Spades,
        1 => Suit::Clubs,
        2 => Suit::Diamonds,
        _ => Suit::Hearts
    };

    return Card {
        value: val,
        suit,
        hidden
    }
}

fn play_game() {
    
    // Initialize human player and computer dealer
    let mut human = Player {
        name: "Human".to_string(),
        cards: Vec::new()
    };

    let mut dealer = Player {
        name: "Dealer".to_string(),
        cards: Vec::new()
    };

    // Draw player's starting hands
    human.draw_starting_hand();
    dealer.draw_starting_hand();

    dealer.print_cards();
    human.print_cards();

    let mut player_total = human.get_total_value();

    loop {

        // Begin player's turn
        println!("Draw a card? Y/N");

        match yes_no() {
            true => human.cards.push(draw_card(false)),
            false => break
        }

        player_total = human.get_total_value();

        human.print_cards();

        // Check if player has busted or reached 21
        match player_total {
            21 => {
                println!("You reached 21!");
                break;
            },
            21.. => {
                println!("Player busts!");
                break;
            },
            _ => println!("Your total is now {}", player_total),
        }
    }

    // Begin dealer's turn
    let mut dealer_total = 0;
    
    while dealer_total < 15 {

        dealer.cards[1].reveal_card();

        dealer.cards.push(draw_card(false));

        dealer_total = dealer.get_total_value();

        dealer.print_cards();

        match dealer_total {
            21 => {
                println!("Dealer reached 21!");
                break;
            },
            21.. => {
                println!("Dealer busts!");
                break;
            },
            _ => println!("Total for {} is {}", dealer.name, dealer.get_total_value())
        }
    }

    // Check which player one
    if player_total > 21 && dealer_total > 21 {
        println!("Both players busted. It's a tie!");
    } else if player_total > 21 {
        println!("Player busted. You lost!");
    } else if dealer_total > 21 {
        println!("Dealer busted. You win!");
    } else {
        println!("Dealer has {dealer_total}, Player has {player_total}");

        match player_total.cmp(&dealer_total) {
            Ordering::Less => println!("Dealer wins!"),
            Ordering::Greater => println!("Player wins!"),
            Ordering::Equal => println!("It's a tie!")
        };
    }
}

fn main() {
    println!("Would you like to play a game of blackjack? Y/N");

    loop {
        match yes_no() {
            true => play_game(),
            false => {
                println!("Alright, see you later!");
                break;
            }
        }

        println!("Would you like to play another game? Y/N");
    }
}
