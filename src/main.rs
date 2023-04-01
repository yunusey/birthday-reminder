#![allow(unused)]

use std::{fs::File, io::{ErrorKind, Write, Read}, collections::HashMap, process::Command};

#[derive(Debug)]
struct Date {
    day: u32,
    month: u32,
    year: u32,
}

impl Date {
    fn new(_day: u32, _month: u32, _year: u32) -> Self {
        Self {
            day: _day,
            month: _month,
            year: _year,
        }
    }

    fn convert_to_date(date: String) -> Self {
        let as_arr: Vec<&str> = date.split("/").collect();
        let _day:   u32 = as_arr[1].parse::<u32>().unwrap();
        let _month: u32 = as_arr[0].parse::<u32>().unwrap();
        let _year:  u32 = as_arr[2].parse::<u32>().unwrap();
        Self {
            day: _day,
            month: _month,
            year: _year,
        }
    }

    fn equals(&self, other: &Date) -> bool {
        self.day == other.day && self.month == other.month && self.year == other.year
    }

    fn format(&self) -> String {
        format!("{}/{}/{}", self.month, self.day, self.year)
    }

    fn as_line(&self, name: &String) -> String {
        format!("{} <--> {}", name, self.format())
    }
}

fn read_content(file_name: &str) -> String {
    let mut file: File = File::open(file_name)
        .expect("Problem occured while creating the File object...");
    let mut content: String = String::new();
    file.read_to_string(&mut content);
    return content;
}

fn open_file(file_name: &str) -> File {
    let file_res = File::options().write(true).open(file_name);
    let file = match file_res {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(file_name){
                Ok(fc) => fc,
                Err(error) => panic!("Error found! {:?}", error),
            },
            _ => panic!("Error found! {:?}", error),
        }
    };
    return file;
}

fn get_dates(content: String) -> HashMap<String, Date> {
    let mut map: HashMap<String, Date> = HashMap::new();

    for i in content.lines() {
        let line: String = i.trim_end().to_string();
        let name_and_date: Vec<&str> = line.split("->").collect();
        let name: String = name_and_date[0].trim_end().trim_start().to_string();
        let date_as_string: String = name_and_date[1].trim_end().trim_start().to_string();
        let date: Date = Date::convert_to_date(date_as_string);
        map.insert(name, date);
    }

    return map;
}

fn get_input() -> String {
    let mut input: String = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error on reading the line into string");
    return input.trim_end().to_string();
}

fn write_to_file(map: &HashMap<String, Date>, file: &mut File) {
    let mut lines: Vec<String> = Vec::new();
    for (name, date) in map {
        let date_as_string: String = date.format();
        let line: String = format!("{} -> {}\n", name, date_as_string);
        file.write(line.as_bytes());
    }
}

fn color_string(s: &str, r: u32, g: u32, b: u32) -> String {
    return format!("\x1b[0;38;2;{};{};{}m{}\x1b[0m", r, g, b, s)
}

fn get_choice() -> String {
    println!("\x1bc{}\n{}\n{}\n{}\n{}\n{}\x1b[0m",
             color_string("Please choose what you want to do!", 255, 255, 0),
             color_string("1: Enter a new birthday date", 0, 255, 255),
             color_string("2: Check someone's birthday", 250, 0, 0),
             color_string("3: Learn whose birthday it's for a certain day", 150, 150, 150),
             color_string("4: Debug the dates!", 0, 255, 0),
             color_string("5: Quit", 0, 150, 255),
             );
    let line: String = get_input();
    return line;
}

fn next_choice(last_choice: &String) -> String {
    println!("{}\n{}\n{}\n{}\x1b[0m",
             color_string("Please choose what you want to do next!", 255, 255, 0),
             color_string("1: Redo last action", 0, 255, 255),
             color_string("2: Open menu", 250, 0, 0),
             color_string("3: Quit", 0, 150, 255),
             );
    let line: String = get_input();
    match line.as_str() {
        "1" => last_choice.clone(),
        "2" => {
            get_choice()
        },
        "3" => String::from("5"),
        _ => {
            println!("Unknown choice");
            next_choice(last_choice)
        }
    }
}

fn main() {
    let file_name = "dates.txt";
    let mut file: File = open_file(file_name);
    let content: String = read_content(file_name);
    let mut dates: HashMap<String, Date> = get_dates(content);
    let mut choice: String = get_choice();
    loop {
        match choice.as_str() {
            "1" => {
                println!("{}", 
                    color_string(
                        "Please enter the name of the person you want to enter:",
                        255, 0, 255
                    ),
                );
                let name: String = get_input();
                println!("{}", 
                    color_string(
                        format!("Please enter the birthdate of {name}:").as_str(),
                        255, 0, 255
                    ),
                );
                let date: Date = Date::convert_to_date(get_input());
                dates.insert(name, date);
                println!("{}", 
                    color_string(
                        format!("Done!🎉").as_str(),
                        0, 255, 255
                    ),
                );
            },
            "2" => {
                println!("{}", 
                    color_string(
                        "Please enter the name of the person you want to check (\"print\" to print the names):",
                        255, 0, 255
                    ),
                );
                let name: String = get_input();
                if name == "print" {
                    println!("\x1bc{}", color_string("Names & Dates", 0, 150, 255)); // Clear the terminal
                    for (name, date) in &dates {
                        let line: String = date.as_line(&name);
                        println!("{}", color_string(line.as_str(), 255, 255, 0));
                    }
                    continue;
                }
                else {
                    match dates.get(&name) {
                        Some(date) => {
                            println!("{}",
                                     color_string(
                                         format!("{name}'s birthday is \"{}\"🎉", date.format()).as_str(), 
                                         0, 255, 255)
                                     );
                        },
                        None => {
                            println!("The name is not in the map, restarting...");
                            // Then, we'll ask it again... Since the choice variable is not
                            // changed, we can just restart the loop...
                            continue;
                        }
                    }
                }
            },
            "3" => {
                println!("{}", 
                    color_string(
                        "Please enter the date you want to check:",
                        255, 0, 255
                    ),
                );
                let date: Date = Date::convert_to_date(get_input());
                let mut name: String = String::new();
                for (k, v) in &dates {
                    if v.equals(&date) {
                        name = k.clone();
                    }
                }
                if name == "" {
                    println!("{}", color_string("Date not found!", 255, 0, 0));
                }
                else {
                    println!("{}", 
                             color_string(format!("It's {name}'s birthday!🎉").as_str(), 0, 255, 255)
                            );
                }
            },
            "4" => {
                dbg!(&dates);
            }
            "5" => break,
            _ => {
                println!("Unknown choice");
                choice = get_choice();
            }
        }
        choice = next_choice(&choice);
    }
    write_to_file(&dates, &mut file);
}
