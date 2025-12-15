fn main() {
    // Collect command-line arguments, skipping the first (program name)
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() != 6 {
        eprintln!("Please provide exactly 6 integers: the 5 regular numbers followed by 1 PowerBall number.");
        std::process::exit(1);
    }

    // Parse the arguments into integers
    let winners: [i32; 6] = [
        args[0].parse().expect("Invalid integer"),
        args[1].parse().expect("Invalid integer"),
        args[2].parse().expect("Invalid integer"),
        args[3].parse().expect("Invalid integer"),
        args[4].parse().expect("Invalid integer"),
        args[5].parse().expect("Invalid integer"),
    ];

    println!("\nThe winning ticket numbers are: {:?}", winners);

    // Validate the winning numbers
    if !validate_ticket(&winners) {
        eprintln!("The winning ticket numbers are invalid. They must be 5 unique numbers between 1 and 69, and a PowerBall number between 1 and 26.");
        std::process::exit(1);
    }

    // Purchased PowerBall Tickets are an array of 10 arrays (or tickets), each containing 6 integers
    // The last is the PowerBall (for that ticket)
    let tickets: Vec<[i32; 6]> = vec![
        [8,17,21,31,68,3],
        [37,40,45,47,52,26],
        [7,18,30,45,64,3],
        [2,20,48,50,52,17],
        [35,51,60,61,68,22],
        [2,8,12,57,62,22],
        [11,22,39,59,68,8],
        [12,29,40,41,53,18],
        [36,42,56,68,69,25],
        [12,36,42,45,58,3],

        [7,19,39,41,49,19],
        [5,14,42,54,59,18],
        [18,29,37,43,53,4],
        [6,23,31,47,52,17],
        [9,19,31,44,64,5],
        [6,16,18,33,51,2],
        [13,20,25,44,46,21],
        [1,20,43,46,57,3],
        [15,19,21,30,60,21],
        [1,21,36,39,59,7],
        ];

    // Print each ticket on a separate line
    println!("\nYour tickets:");
    tickets.iter().enumerate().for_each(|(n, ticket)| {
        println!("Ticket {}: {:?}", n+1, ticket);
    });

    // Validate each ticket
    tickets.iter().enumerate().for_each(|(n, ticket)| {
        if !validate_ticket(ticket) {
            eprintln!("Ticket {} is invalid. It is {:?}", n+1, ticket);
            std::process::exit(1);
        }
    });


    // Check each ticket against the winning numbers
    // Print a the non-zero count of the numbers in each ticket
    // that are also found in the winning ticket
    // Print out the ticket, too.
    println!("\nResults:");
    tickets.iter().enumerate().for_each(|(n, ticket)| {
        let count = ticket.iter().filter(|&&n| winners.contains(&n)).count();
        let powerball_match = ticket[5] == winners[5];
        if count > 0 {
            println!("Ticket {} has {} matching number{} {}", n+1, count, if count > 1 { "s" } else { "" }, if powerball_match {"including the *PowerBall*"} else {""});
        }
    });
}

fn validate_ticket(ticket: &[i32; 6]) -> bool {
    // Validate that the regular numbers are >= 1 and <= 69
    if !ticket.iter().take(5).all(|&n| n >= 1 && n <= 69) {
        return false;
    }

    // Validate that the PowerBall number is >= 1 and <= 26
    if !ticket.iter().skip(5).all(|&n| n >= 1 && n <= 26) {
        return false;
    }

    // Validate that the regular numbers are unique
    let mut seen = std::collections::HashSet::new();
    for &n in ticket.iter().take(5) {
        if !seen.insert(n) {
            return false;
        }
    }

    true        
}
