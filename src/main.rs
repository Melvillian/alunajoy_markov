use markov::Chain;
use std::path::Path;

fn main() {
    let path = Path::new("aluna_ravings.txt");

    let mut chain = Chain::new();
    let chain = chain.feed_file(path).unwrap();
    //println!("{:?}", chain.generate_str());

    let mut raving = String::new();
    let mut is_first = true;
    for line in chain.iter_for(5) {
        if is_first {
            is_first = false;
            raving.push_str(&line.join(" "));
        } else {
            raving.push_str(" ");
            raving.push_str(&line.join(" "));
        }
    }

    println!("{:?}", raving);
}