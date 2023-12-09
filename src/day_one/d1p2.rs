use crate::day_one::d1p1::get_first_and_last_digits;
use crate::utils::file_reader::read_lines;
use super::d1_structs::{ValueNIndex, TextValue};


const TEXT_VALUES: [TextValue; 9] = [ TextValue{text: "one", value: 1 },
TextValue{text: "two", value: 2 },
TextValue{text: "three", value: 3 },
TextValue{text: "four", value: 4 },
TextValue{text: "five", value: 5 },
TextValue{text: "six", value: 6 },
TextValue{text: "seven", value: 7 },
TextValue{text: "eight", value: 8 },
TextValue{text: "nine", value: 9 },
];

pub fn d1p2() {
    let mut result: u32 = 0;
    if let Ok(lines) = read_lines("./resources/d1p1.txt") {
        for line in lines {
            if let Ok(entry) = line {
                let (first_digit, last_digit ) = get_first_and_last_digits(entry.clone());
                let (first_text, last_text) = get_first_and_last_str_digits(entry.clone());
                let aboslute_first = get_absolute_first(first_digit.unwrap(), first_text);
                let aboslute_last = get_absolute_last(last_digit.unwrap(), last_text);
                result += aboslute_first.value*10+aboslute_last.value;
            }
        }
    }

    println!("D1P2 result: {}", result);
}

pub fn get_first_and_last_str_digits(entry: String)-> (Option<ValueNIndex>, Option<ValueNIndex>)  {
    let mut first_digit: Option<ValueNIndex> = None;
    let mut last_digit: Option<ValueNIndex> = None;

    for text_value in TEXT_VALUES {
         match entry.find(&text_value.text){
            Some(index) => {first_digit = get_first_digit( first_digit, index, text_value.value); }, 
            None => {},
        }
    }
    for text_value in TEXT_VALUES {
         match entry.rfind(&text_value.text){
            Some(index) => { last_digit = get_last_digit(last_digit, index, text_value.value)}, 
            None => {},
        }
    }

    (first_digit, last_digit)
}
    

fn get_first_digit(current_first: Option<ValueNIndex>, index: usize, value:u32) -> Option<ValueNIndex>{
    match current_first{
        Some(ref value_n_index) => {
            if value_n_index.index < index{
                return current_first;
            }else {
                Some(ValueNIndex { index, value })
            }
        },
        None =>  Some(ValueNIndex{ index, value })
    }
}


fn get_last_digit(current_first: Option<ValueNIndex>, index: usize, value:u32) -> Option<ValueNIndex>{
    match current_first{
        Some(ref value_n_index) => {
            if value_n_index.index > index{
                return current_first;
            }else {
                Some(ValueNIndex { index, value })
            }
        },
        None =>  Some(ValueNIndex{ index, value })
    }
}


fn get_absolute_first(first_digit: ValueNIndex, first_text: Option<ValueNIndex>) -> ValueNIndex{
    match first_text{
        Some(ref first_text_exists)=> {
            if first_digit.index < first_text_exists.index{
                first_digit
            }else {
                first_text.unwrap()
            }
        },
        None => first_digit,
    }
}

fn get_absolute_last(first_digit: ValueNIndex, first_text: Option<ValueNIndex>) -> ValueNIndex{
    match first_text{
        Some(ref first_text_exists)=> {
            if first_digit.index > first_text_exists.index{
                first_digit
            }else {
                first_text.unwrap()
            }
        },
        None => first_digit,
    }
}
