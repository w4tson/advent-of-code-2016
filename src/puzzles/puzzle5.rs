
use crypto::md5::Md5;
use crypto::digest::Digest;
use std::u64::MAX;

pub fn puzzle5() {
    let input = "ffykfhsq".to_string();

    let mut hasher = Md5::new();
    for i in 0..MAX {

        hasher.input_str(&input);
        hasher.input_str(&i.to_string());
        let hash = hasher.result_str();
        hasher.reset();
        let result = match &hash[0..5] {
            "00000" => println!("{} ", hash.chars().nth(5).unwrap()),
            _ => ()
        };

    }
}

