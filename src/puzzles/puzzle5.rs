
use crypto::md5::Md5;
use crypto::digest::Digest;
use std::u64::MAX;
use std::collections::BTreeMap;

pub fn puzzle5() {
    let input = "ffykfhsq".to_string();

    let mut hasher = Md5::new();
    let mut pwd = BTreeMap::new();

    for i in 0..MAX {

        hasher.input_str(&input);
        hasher.input_str(&i.to_string());
        let hash = hasher.result_str();
        hasher.reset();
        match &hash[0..5] {
            "00000" => update_pwd_map(&mut pwd, hash.chars().nth(5).unwrap(), hash.chars().nth(6).unwrap()),
            _ => ()
        };

        //ouput cool animation :D
        if i % 75 == 0 {
            let output : String = (0..8)
            .map(|index| *(pwd.get(&(index as u8)).unwrap_or(&(hash.chars().nth(index).unwrap()))))
            .collect::<String>();
            print!("\r{}", output);
            if pwd.keys().len() == 8 {
              break;
            }
        }

    }
}

fn update_pwd_map( pwd : &mut BTreeMap<u8,char>, index : char, ch : char)  {
    let i : u8 = index.to_digit(36).unwrap() as u8;
    if !pwd.contains_key(&i) {
        pwd.insert(i, ch);
    }
}

