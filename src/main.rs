mod aa_str;

use aa_str::EXCLAMATION;
use aa_str::PERIOD;
use aa_str::SPACE;
use aa_str::AA;

use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    let (r, e) = create_aa(args);
    println!("{}\n{}", r, e);
    if r != "" {
        // copy to clipboard
    }
}

fn create_aa(args: Vec<String>) -> (String, String){
    let mut r = String::from("");
    let mut e = args.clone();
    e.remove(0);
    for i in 1..args.len() {
        let result = get_aa(&args[i], 0);
        if result == "error" {
            let result = get_aa(&args[i], 1);
            if result == "error" {
                e[i - 1] += "<- too long"
            } else {
                r += &result;
            }
        } else {
            r += &result;
        }
    }
    if r.match_indices("\n").count() > 0 {
        r = SPACE.repeat(26) + &("\n".to_string() + &r);
    }
    (r, e.join("\n"))
}

fn get_aa(i: &str, mode: usize) -> String{
    // Result for each row
    let mut o: [String; 3] = [SPACE.to_string(), SPACE.to_string(), SPACE.to_string()];
    // Join array "o"
    let mut result: String = String::from("");
    let aa: [[&str; 3]; 26] = AA[mode];
    let upper_i: String = i.to_uppercase();
    for j in upper_i.chars() {
        if 65 <= j as usize && j as usize <= 90 {
            let str: [&str; 3] = aa[j as usize - 65];
            o[0] += &(str[0].to_owned() + SPACE);
            o[1] += &(str[1].to_owned() + SPACE);
            o[2] += &(str[2].to_owned() + SPACE);
        } else if j as usize == 33 {
            o[0] += &(EXCLAMATION[0].to_owned() + SPACE);
            o[1] += &(EXCLAMATION[1].to_owned() + SPACE);
            o[2] += &(EXCLAMATION[2].to_owned() + SPACE);
        } else if j as usize == 46 {
            o[0] += &(PERIOD[0].to_owned() + SPACE);
            o[1] += &(PERIOD[1].to_owned() + SPACE);
            o[2] += &(PERIOD[2].to_owned() + SPACE);
        } else if j as usize == 32 {
            o[0] += &(SPACE.to_owned() + SPACE);
            o[1] += &(SPACE.to_owned() + SPACE);
            o[2] += &(SPACE.to_owned() + SPACE);
        }
    }
    // if row length is too long -> error
    if o[0].chars().count() > 26 {
        result += "error";
    } else {
        for mut j in o {
            while j.chars().count() < 26 {
                j = SPACE.to_string() + &(j.to_string() + SPACE);
            }
            if j.chars().count()  != 26 {
                j.pop();
            }
            result += &(j.to_string() + "\n");
        }
        result += &(SPACE.repeat(26) + "\n");
    }
    result
}