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

    println!("\nThe winning ticket numbers are: {:?}", winners);

    // Validate the winning numbers
    if !validate_ticket(&winners) {
        std::process::exit(1);
    }

    // Purchased PowerBall Tickets are an array of 10 arrays (or tickets), each containing 6 integers
    // The last is the PowerBall (for that ticket)
    let tickets: [[i32; 6]; 10] = [
        [12,24,43,59,62,18],
        [12,31,44,51,57,24],
        [24,37,41,57,67,24],
        [04,22,23,63,67,21],
        [04,30,34,48,63,01],
        [17,23,38,58,69,09],
        [09,20,34,40,57,09],
        [08,12,17,41,62,19],
        [06,08,50,51,64,06],
        [02,28,36,47,48,02],
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
        if count > 0 {
            println!("Ticket {} has {} matching numbers.", n+1, count);
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
