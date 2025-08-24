//import libraries
use rand::Rng;
use std::io;
use std::io::Write;


//create card struct
pub struct Card 
{
    pub suit: u8,
    pub rank: u8,
    pub taken: bool
}

//Display trait
pub trait Display {
    fn print_card(&self);
}

//implement functions for display trait for card struct
impl Display for Card 
{
    
    fn print_card(&self)
    {
        match self.suit {
            0 => print!("S"),
            1 => print!("H"),
            2 => print!("D"),
            3 => print!("C"),
            _ => print!("?"),
        }
        
        match self.rank {
            0 => print!("A"),
            10 => print!("J"),
            11 => print!("Q"),
            12 => print!("K"),
            _ => print!("{}", self.rank+1),
        }
    }
}

//create deck struct to hold cards
pub struct Deck
{
    //vector to hold vector of cards
    pub cards:Vec<Card>,
}

//various functions to manage game
pub trait DeckStuff 
{
    fn new() -> Deck;
    fn shuffle(cards: Vec<Card>) -> Vec<Card>;
    fn draw(&mut self) -> Card;
    fn is_empty(&self) -> bool;
    
}

//implement deck functions
impl DeckStuff for Deck
{
    
    //shuffle deck
    fn shuffle(incards: Vec<Card>) -> Vec<Card>
    {
        //set random position
        let mut pos = rand::thread_rng().gen_range(0..51);

        let mut cards = incards;

        //iterate through and randomize cards
        for num in 0..52
        {
            
            let temp1 = Card{suit: cards[num].suit, rank: cards[num].rank, taken: cards[num].taken};
            let temp2 = Card{suit: cards[pos].suit, rank: cards[pos].rank, taken: cards[pos].taken};

            copy_card(&mut cards[pos], &temp1);
            copy_card(&mut cards[num], &temp2);
        

            pos = rand::thread_rng().gen_range(0..51);
        }
        cards
    }
    
    //create new deck
    fn new() -> Deck
    {
        let mut new_cards:Vec<Card> = Vec::new();

        //for each suit and rank create new card and add to vector
        for cur_suit in 0..4
        {
            for num in 0..13
            {
                new_cards.push(Card {suit: cur_suit, rank: num, taken: false});
            }
        } 
        
        //create new Deck
        new_cards = Deck::shuffle(new_cards);
        
        Deck {cards: new_cards}
    }

    // Draw the top card from the deck
    fn draw(&mut self) -> Card
    {
        if self.cards.len() == 0
        {
            return Card {suit: 100, rank: 100, taken: false};
        }
        return self.cards.pop().unwrap();
    }


    // check if deck is empty
    fn is_empty(&self) -> bool
    {
        if self.cards.len() == 0
        {
            return false;
        }
        return true;
    }
}

// Create player struct
pub struct Player
{
    pub hand:Vec<Card>
}

// create trait for plater
pub trait PlayerStuff 
{
    fn new() -> Player;
    fn add_cards(&mut self, incards: Vec<Card>);
    fn has_rank(&self, rank: u8) -> bool;
    fn remove_cards(&mut self, rank: u8) -> Vec<Card>;
    fn check_set(&mut self) -> u8;
}

//implement player functions
impl PlayerStuff for Player
{
    // Create a new Player
    fn new() -> Player
    {
        let new_cards:Vec<Card> = Vec::new();
        Player {hand:new_cards}
    }
    // Add card to player hand
    fn add_cards(&mut self, incards: Vec<Card>)
    {
        for card in 0..incards.len()
        {
            self.hand.push(Card {suit: incards[card].suit, rank: incards[card].rank, taken: incards[card].taken});
        }
    }

    // check if player has rank
    fn has_rank(&self, rank: u8) -> bool
    {
        for card in 0..self.hand.len()
        {
            if self.hand[card].rank == rank
            {
                return true;
            }
        }
        return false;
    }

    // Remove cards of a certain rank from the player and returns vector of cards removed
    fn remove_cards(&mut self, rank: u8) -> Vec<Card>
    {
        let mut removed_cards:Vec<Card> = Vec::new();
        let mut card = 0;


        
        while card < self.hand.len()
        {
            if self.hand[card].rank == rank
            {
                //add removed card to vector
                removed_cards.push(Card {
                    suit: self.hand[card].suit, rank: self.hand[card].rank, taken: self.hand[card].taken
                });
                //remove that card from player hand
                self.hand.remove(card);
                card = 0;
            }
            card = card + 1;
        }

        removed_cards
    }

    // Check if player has a set
    fn check_set(&mut self) -> u8
    {
        let mut count = 0;
        for cur_rank in 0..13
        {
            for card in 0..self.hand.len()
            {
                if self.hand[card].rank == cur_rank
                {
                    count = count + 1;
                }
                if count >= 4
                {
                    self.remove_cards(cur_rank);
                    println!("Set has been found of rank {}", cur_rank+1);
                    return 1;
                }
            }
            count = 0;
        } 
        return 0;       
    }

}

// Create game struct
pub struct Game
{
    pub deck:Deck,
    pub human:Player,
    pub computer:Player
    
}

// create game trait
pub trait GameStuff 
{
    fn new() -> Game;
    fn human_turn(&mut self) -> u8;
    fn computer_turn(&mut self) -> u8;
    fn play(&mut self) -> (u8, u8);
    fn print_scores(&self, h_score: u8, c_score: u8);
    fn winner(&self, h_score: u8, c_score: u8);

}


impl GameStuff for Game
{
    //create new game
    fn new() -> Game
    {
        //initialize deck and players
        let mut deck = Deck::new();
        let mut hum = Player::new();
        let mut com = Player::new();

        let mut draw_cards:Vec<Card> = Vec::new();
        let mut draw_cards2:Vec<Card> = Vec::new();

        //draw cards
        for _card in 0..7
        {      
            draw_cards.push(deck.draw());
            draw_cards2.push(deck.draw());
        }

        //add cards to players
        hum.add_cards(draw_cards);
        com.add_cards(draw_cards2);

        Game {deck: deck, human: hum, computer: com}
    }

    //ask user to enter rank
    fn human_turn(&mut self) -> u8
    {
        //print player hand
        println!("\nHuman turn\nYour hand");
        for card in 0..self.human.hand.len()
        {
            self.human.hand[card].print_card();
            print!(" ");
        }
        println!("");

        print!("Enter rank (A for ace, 11 for J, 12 for Q, 13 for K): ");
        io::stdout().flush().unwrap();

        println!("");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        //process input
        let chars: Vec<char> = input.trim().chars().collect();
        if chars.len() != 1  && chars.len() != 2{
            println!("Invalid input. Please try again.");
        }
        else
        {
            //determine rank
            let rank;
            if chars.len() == 1
            {

                rank = match chars[0].to_ascii_uppercase() {
                    'A' => 0,
                    '2' => 1,
                    '3' => 2,
                    '4' => 3,
                    '5' => 4,
                    '6' => 5,
                    '7' => 6,
                    '8' => 7,
                    '9' => 8,
                    _ => 15
    
                };
            }
            else
            {
                rank = match chars[1].to_ascii_uppercase() {
                    '0' => 9,
                    '1' => 10,
                    '2' => 11,
                    '3' => 12,
                    _ => 15

                };
            }

            // Check if computer has the requested rank
            if self.computer.has_rank(rank)
            {
                let new_cards = self.computer.remove_cards(rank);
    
                println!("Cards found");
    
                for card in 0..new_cards.len()
                {
                    new_cards[card].print_card();
                    println!("");
                }
    
                self.human.add_cards(new_cards);
            }
            //if not have user go fish
            else
            {
                println!("Go fish");
                let mut draw_cards:Vec<Card> = Vec::new();
                
                draw_cards.push(self.deck.draw());
                if draw_cards[0].suit != 100
                {
                    self.human.add_cards(draw_cards);
                }
                else
                {
                    println!("No more cards in deck");
                }
            }

            return self.human.check_set();
        }
        return 0;
    }

    fn computer_turn(&mut self) -> u8
    {
        // generate random rank
        let rank = rand::thread_rng().gen_range(0..13);

        println!("\nCpu Turn");

        //determine if human has rank
        if self.human.has_rank(rank)
        {
            let new_cards = self.human.remove_cards(rank);

            println!("CPU took your cards!");

            self.computer.add_cards(new_cards);
        }
        // if not cpu goes fish
        else
        {
            println!("CPU went fish");
            let mut draw_cards:Vec<Card> = Vec::new();
            
            draw_cards.push(self.deck.draw());
            if draw_cards[0].suit != 100
            {
                self.computer.add_cards(draw_cards);
            }
            else
            {
                println!("No more cards in deck");
            }
        }
        return self.computer.check_set();
    }
    
    //play function runs the game
    fn play(&mut self) -> (u8, u8)
    {
        let mut h_score = 0;
        let mut c_score = 0;

        while h_score + c_score < 13
        {
            h_score = h_score + self.human_turn();
            c_score = c_score + self.computer_turn();
        }

        return (h_score, c_score);
    }

    //print scores
    fn print_scores(&self, h_score: u8, c_score: u8)
    {
        println!("human score {}, cpu score {}", h_score, c_score);
    }

    //determine winner
    fn winner(&self, h_score: u8, c_score: u8)
    {
        if h_score > c_score
        {
            println!("The winner is: Human!");
        }
        else
        {
            println!("The winner is: CPU!");
        }
    }

}
    
fn main() 
{
    // Initialize game
    println!("Welcome to Go Fish!");
    
    //run the game
    let mut game = Game::new();
    let (h_score, c_score) = game.play();
    
    //print out results
    game.print_scores(h_score, c_score);
    game.winner(h_score, c_score);
    
}

//copies c2 into c1 and returns previous c1
fn copy_card(c1: &mut Card, c2: &Card) -> Card
{
    let temp = Card{suit: c1.suit, rank: c1.rank, taken: c1.taken};
    c1.suit = c2.suit;
    c1.rank = c2.rank;
    c1.taken = c2.taken;

    temp
}
