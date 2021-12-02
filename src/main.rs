mod day1;
mod day2;

fn main() {
    let funcs = [day1::PARTS, day2::PARTS];
    let mut args = std::env::args();
    args.next();
    if let Some(x) = args.next() {
        if let Ok(x) = x.parse::<usize>() {
            if let Some(y) = args.next() {
                if let Ok(y) = y.parse::<usize>() {
                    if let Some(x) = funcs.get(x - 1) {
                        if let Some(x) = x.get(y - 1) {
                            let timer = std::time::Instant::now();
                            x();
                            println!("Took {:?}", timer.elapsed());
                        } else {
                            println!("Not implemented");
                        }
                    } else {
                        println!("Not implemented");
                    }
                } else {
                    println!("Must enter numbers!");
                }
            } else {
                println!("Pass day and part as commandline parameters");
            }
        } else {
            println!("Must enter numbers!");
        }
    } else {
        println!("Pass day and part as commandline parameters");
    }
}
