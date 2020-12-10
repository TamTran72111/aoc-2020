use crate::utilities::read_block;
use std::collections::HashMap;

struct Passport {
    byr: i32,
    iyr: i32,
    eyr: i32,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
}

impl Passport {
    fn new(passport_info: String) -> Result<Passport, ()> {
        let passport_info = passport_info.replace('\n', " ");
        let infos: Vec<&str> = passport_info.split_ascii_whitespace().collect();
        let mut map = HashMap::new();
        for info in infos {
            let mut pair = info.split(':');
            let key = pair.next().unwrap();
            let value = pair.next().unwrap();
            map.insert(key.to_string(), value.to_string());
        }
        Ok(Passport {
            byr: map.get("byr").ok_or(())?.parse().unwrap(),
            iyr: map.get("iyr").ok_or(())?.parse().unwrap(),
            eyr: map.get("eyr").ok_or(())?.parse().unwrap(),
            hgt: map.get("hgt").ok_or(())?.to_string(),
            hcl: map.get("hcl").ok_or(())?.to_string(),
            ecl: map.get("ecl").ok_or(())?.to_string(),
            pid: map.get("pid").ok_or(())?.to_string(),
        })
    }

    fn validate(&self) -> bool {
        self.validate_birthdate()
            && self.validate_issue_date()
            && self.validate_expire_date()
            && self.validate_height()
            && self.validate_hair_color()
            && self.validate_eye_color()
            && self.validate_passport_id()
    }

    fn validate_birthdate(&self) -> bool {
        self.byr >= 1920 && self.byr <= 2002
    }

    fn validate_issue_date(&self) -> bool {
        self.iyr >= 2010 && self.iyr <= 2020
    }

    fn validate_expire_date(&self) -> bool {
        self.eyr >= 2020 && self.eyr <= 2030
    }

    fn validate_height(&self) -> bool {
        let height = self.hgt.as_str();
        let len = height.len();
        if len > 3 {
            let unit = &height[len - 2..];
            let height: i32 = height[..len - 2].parse().unwrap();
            if unit == "cm" {
                if height >= 150 && height <= 193 {
                    return true;
                }
            } else if unit == "in" {
                if height >= 59 && height <= 76 {
                    return true;
                }
            }
        }

        false
    }

    fn validate_hair_color(&self) -> bool {
        if self.hcl.len() == 7 {
            let mut hcl = self.hcl.chars();
            if hcl.next().unwrap() == '#' {
                return hcl.all(|c| {
                    c.is_ascii_digit() || (c as usize >= 'a' as usize && c as usize <= 'f' as usize)
                });
            }
        }

        false
    }

    fn validate_eye_color(&self) -> bool {
        // amb blu brn gry grn hzl oth
        let eye_colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        for i in 0..eye_colors.len() {
            if self.ecl == eye_colors[i] {
                return true;
            }
        }
        false
    }

    fn validate_passport_id(&self) -> bool {
        self.pid.len() == 9 && self.pid.chars().all(|c| c.is_ascii_digit())
    }
}

fn part_1(passports: &Vec<Passport>) {
    println!("\tPart 1: {}", passports.len());
}

fn part_2(passports: &Vec<Passport>) {
    println!(
        "\tPart 2: {}",
        passports
            .iter()
            .filter(|passport| passport.validate())
            .count()
    );
}

pub fn main() {
    println!("Day 4");
    let passports = read_block("../inputs/day4.txt")
        .into_iter()
        .map(Passport::new)
        .filter(|passport| passport.is_ok())
        .map(|passport| passport.unwrap())
        .collect();
    part_1(&passports);
    part_2(&passports);
}
