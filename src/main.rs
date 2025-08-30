fn main() {
    // Collect command-line arguments, skipping the first (program name)
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() != 6 {
        eprintln!("Please provide the winning numbers as exactly 6 integers.");
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

    println!("You entered: {:?}", winners);

    // Validate the winning numbers
    if !validate_ticket(&winners) {
        std::process::exit(1);
    }

    // Purchased Lottery Tickets are an array of 10 arrays (or tickets), each containing 6 integers
    // The last is the PowerBall (for that ticket)
    let tickets: [[i32; 6]; 10] = [
        [1,2,3,4,5,6],
        [1,2,3,4,5,6],
        [1,2,3,4,5,6],
        [1,2,3,4,5,6],
        [1,2,3,4,5,6],
        [1,2,3,4,5,6],
        [1,2,3,4,5,6],
        [1,2,3,4,5,6],
        [1,2,3,4,5,6],
        [1,2,3,4,5,6],
        ];

    // Example: print the array
    println!("Your tickets: {:?}", tickets);

    // Validate each ticket
    tickets.iter().enumerate().for_each(|(n, ticket)| {
        if !validate_ticket(ticket) {
            eprintln!("Ticket {} is invalid. It is {:?}", n, ticket);
            std::process::exit(1);
        }
    });


    // Check each ticket against the winning numbers
    // Print a the non-zero count of the numbers in each ticket
    // that are also found in the winning ticket
    // Print out the ticket, too.
    tickets.iter().enumerate().for_each(|(n, ticket)| {
        let count = ticket.iter().filter(|&&n| winners.contains(&n)).count();
        if count > 0 {
            println!("Ticket {} has {} matching numbers. It is {:?}", n, count, ticket);
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
