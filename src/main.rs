use std::env;
use rand::Rng;
use colored::*;

const NUMBER_DOORS: usize = 3;


#[derive(Debug)]
struct Quiz {
  doors: Vec<Prize>,
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
enum Prize {
  Goat, 
  Car,
}

impl Quiz {
    fn new_round() -> Quiz {
        let prize_door = rand::thread_rng().gen_range(0..NUMBER_DOORS);

        let mut quiz = Quiz {
            //doors: [Prize::Goat; NUMBER_DOORS],
            doors: vec![Prize::Goat; NUMBER_DOORS],
        };

        quiz.doors[prize_door] =  Prize::Car;

        quiz
    }

    // Reveals another door with a goat behind it after the selection
    // has been made by the contestant. 
    fn reveal(&self, candidate_selection: usize) -> usize {

        let mut reveal_options: Vec<usize> = Vec::new();
    
        for (index, value) in self.doors.iter().enumerate() {
            if *value == Prize::Goat && index != candidate_selection {
                reveal_options.push(index);
            }
        }

        
        // Randomly chose one of the doors which have a goat behind them
        let reveal_selection= rand::thread_rng()
                                .gen_range(0..reveal_options.len());

                               
        
        assert_eq!(self.doors[reveal_options[reveal_selection]], Prize::Goat);  
        assert_ne!(reveal_options[reveal_selection], candidate_selection);

        reveal_options[reveal_selection]

    }
    
    
}


fn main() {
    let mut stick_wins: u16 = 0;
    let mut swap_wins: u16 = 0;
    let mut number_iterations: u16 = 10000;

    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        number_iterations = args[1].parse().expect("Invalid parameter");
    }

    for i in 1..=number_iterations {
        let quiz = Quiz::new_round();
        
        let candidate_selection = rand::thread_rng().gen_range(0..NUMBER_DOORS);
        
        let reveal_selection = quiz.reveal(candidate_selection);
        
        //Candidate swaps
        let swapped_selection =  {
            loop {
                let selection = rand::thread_rng().gen_range(0..NUMBER_DOORS);
                if selection != candidate_selection  && selection != reveal_selection {
                    break selection;
                    }
                }
            };

        assert!(swapped_selection != candidate_selection && swapped_selection != reveal_selection);

        // Candidate sticks
        if quiz.doors[candidate_selection] == Prize::Car {
            stick_wins = stick_wins + 1 ;
        }
        // Candidate swaps 
        if quiz.doors[swapped_selection] == Prize::Car {
            swap_wins = swap_wins + 1 ;
        } 
        let stick_msg = format!("Stick has won {} times / {:.2}%, ", stick_wins, percentage(&i, &stick_wins));
        let swap_msg =  format!("Swap has won  {} times / {:.2}%", swap_wins, percentage(&i, &swap_wins));
                
        println!("{}{} out of {} games", stick_msg.yellow(), swap_msg.green(), i);
    }

}

fn percentage(total: &u16, number: &u16) -> f32 {
    let total_f = f32::from(*total); 
    let number_f = f32::from(*number);
    (number_f/total_f)*100.0

}


