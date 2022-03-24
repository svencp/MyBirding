/*  A file to keep functions that I use on files here
And I had to add sentence case here too

2022.01.06   Sven Ponelat

*/

use std::fs::metadata;
use std::fs::set_permissions;
use inflections::Inflect;
use substring::Substring;
use termion::{color, style};
use std::path::Path;
use std::ffi::OsStr;
use std::time::{SystemTime};
use chrono::Datelike;
use chrono::prelude::*;
// use chrono::Utc;


#[allow(dead_code)]
pub struct Timer{
    start: SystemTime,
}

#[allow(dead_code)]
impl Timer{
    pub fn new() -> Timer {
        Timer {
            start:  SystemTime::now(),
        }
    }

    pub fn stop(&self) {
        let duration = self.start.elapsed().unwrap().as_millis();
        let message = format!("Time elapsed is: {:?}ms", duration);
        println!("{}",message);
    }
}


#[allow(dead_code)]
pub enum Justify {
    Left,
    Center,
    Right
}


#[allow(dead_code)]
pub enum Feedback{
    Info,
    Warning,
    Error
}


#[allow(dead_code)]
// A function that takes a line and splits it into an array of words
pub fn line_to_words(line: &str) -> Vec<String> {
    // line.split_whitespace().map(str::to_string).collect()
    // line.split(" ").map(str::to_string).collect();
    let temp = line.split(" ");
    let vec: Vec<&str> = temp.collect();
    
    // get rid of empties
    let mut ret: Vec<String> = Vec::new();
    for i in vec {
        if i.len() != 0  {
            ret.push(i.to_string());
        }
    }
    ret
}


#[allow(dead_code)]
// Set permissions to read-only
pub fn set_to_readonly(path: &str)-> std::io::Result<()> {
    let mut perms = metadata(path)?.permissions();
    perms.set_readonly(true);
    set_permissions(path, perms)?;
    Ok(())
}


#[allow(dead_code)]
/*
    A function that converts a string into sentence case
    I had to drop the to_ as it sounded too close to the others
*/
pub fn sentence_case(str: &str) -> String {
    let lower = str.to_lower_case();
    let temp = lower.split(" ");
    let mut array: Vec<&str> = temp.collect();
    array = remove_empties(array);
    if array.len() == 0 {
        return "".to_string();
    }

    let mut res: String = "".to_string();
    let len: usize = array.len();
    for i in 0..len {
        match i {
            0 =>    {
                res = uppercase_first_letter(array[0]);
            }
            _other => {
                let r = array[i].to_lower_case();
                res = format!("{} {}",&res, r);
            }
        }
    }
    res
}


#[allow(dead_code)]
/*
    A function that converts a string into sentence case
    I had to drop the to_ as it sounded too close to the others
*/
pub fn title_case(str: &str) -> String {
    let array = line_to_words(str);
    if array.len() == 0 {
        return "".to_string();
    }

    let mut res: String = "".to_string();
    let len: usize = array.len();
    for i in 0..len {
        let mut temp = array[i].to_lower_case();
        temp = uppercase_first_letter(&temp);
        temp = format!("{} ",temp);
        res.push_str(&temp);
    }
    res.trim().to_string()
}


// Function where only the first char is converted to upper case
pub fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}


#[allow(dead_code)]
// Function to remove empty strings in a vector
pub fn remove_empties(array: Vec<&str>) -> Vec<&str> {
    let mut res: Vec<&str> = Vec::new();
    for i in array {
        if i.len() != 0 {
            res.push(i);
        }
    } 
    res
}


#[allow(dead_code)]
/*
    A function that justifies a phrase in a given number of characters

*/
// pub fn justify(phrase: String, num: i32, which: Justify) -> String {
pub fn justify(phrase: String, num: usize, which: Justify) -> String {
    let ret: String = phrase.trim().to_string();
    let p_len = ret.len() as usize;

    if p_len >= num {
        return ret
    }

    let spare = num - p_len;
    let padding = repeat_char(" ".to_string(), spare);

    match which {
        Justify::Left   => { return format!("{}{}",ret, padding) }
        Justify::Right  => { return format!("{}{}",padding, ret) }
        Justify::Center => {
                        if let 0 = spare % 2 {        // if spare is even
                            let front_len = spare / 2;
                            let front = repeat_char(" ".to_string(), front_len);
                            let back = front.clone();
                            return format!("{}{}{}", front, ret, back) 

                        }                             // else spare is odd
                        // let front_len = spare / 2;
                        // let front = repeat_char(" ".to_string(), front_len);
                        // let back_len = front_len + 1;
                        // let back = repeat_char(" ".to_string(), back_len);
                        // return format!("{}{}{}", front, ret, back) 
                        let back_len = spare / 2 ;
                        let back  = repeat_char(" ".to_string(), back_len);
                        let front_len = back_len + 1;
                        let front = repeat_char(" ".to_string(), front_len);
                        return format!("{}{}{}", front, ret, back) 
        }
    }
}

/*
    A function that returns a string with repeated char (although
    in this function it is a string).
*/
pub fn repeat_char(ch: String, num: usize) -> String {
    let mut ret = String::new();
    for i in 0..num {
        match i {
            _ => { ret.push_str(&ch) }
        }
    }
    ret
}


#[allow(dead_code)]
/*
A function that prints to the screen with a given letter (char) 
that will be underlined. Put here so that I can use it in sightings as well.
*/
pub fn underline(letter: char, text: &str){
    let mut one_and_done = false;
    for ch in text.chars(){
        if ch == letter && !one_and_done {
            print!("{}{}{}",style::Underline, ch, style::NoUnderline   );
            one_and_done = true;
        } else {
            print!("{}", ch );
        }
    }
}



// A function to give coomand line feedback to situations such as errors or warnings
#[allow(dead_code)]
pub fn feedback(status: Feedback, message: String){
    
    match status {
        Feedback::Info    => { print!("{}{}{}",color::Fg(termion::color::LightYellow),"Info:",style::Reset);}
        Feedback::Warning => { print!("{}{}{}",color::Fg(termion::color::Yellow),"Warning:",style::Reset);}
        Feedback::Error   => { print!("{}{}{}",color::Fg(termion::color::Red),"Error:",style::Reset);}
    }
    print!("  {}\n",message);
}


#[allow(dead_code)]
// A function to count the newlines in a string
pub fn count_newlines(s: &str) -> usize {
    s.as_bytes().iter().filter(|&&c| c == b'\n').count()
}


// This function moves a char alphabetically
pub fn move_shift(data: &String, shift: usize) -> String {
    data.chars().map(|c| (c as u8 + shift as u8) as char).collect::<String>()
}


// This function returns a string length equal to or less than the required number
pub fn limit_length(string: String, limit: usize) -> String {
    if string.len() > limit {
        let ret = string.substring(0, limit).to_string();
        return ret
    }
    return string;
}


// Function to get filename extension
pub fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
}


// Function to return the plural of the word if not equal to 1
pub fn plural(word: &str, number: usize) -> String {
    let mut ret = word.to_string();
    if number != 1 {
        ret.push('s');
        return ret
    }
    return  ret;
}


// Function to convert a string (assumed date) into a unix timestamp
pub fn convert_assumed_date(str: &str) -> Result<i64, String> {
    
    let current_date = chrono::Utc::now();
    let first = str.split(".");
    let vec: Vec<&str> = first.collect();
    let year: i32;
    let month: u32;
    let day: u32;

    match vec.len() {
        1 => {    // Assume day
            let d_value = vec[0].parse::<u32>();
            if d_value.is_err(){
                return Err("Cannot convert date from string given".to_string())
            }
            day = d_value.unwrap();
            month = current_date.month();
            year = current_date.year() as i32;
        }
        2 => {    // Assume month.day
            let m_value = vec[0].parse::<u32>();
            if m_value.is_err(){
                return Err("Cannot convert date from string given".to_string())
            }
            month = m_value.unwrap();
            year = current_date.year() as i32;
            let d_value = vec[1].parse::<u32>();
            if d_value.is_err(){
                return Err("Cannot convert date from string given".to_string())
            }
            day = d_value.unwrap();
        }
        3 => {    // Assume year.month.day
            match vec[0].len() {
                2 => {
                    let value = vec[0].parse::<i32>();
                    if value.is_err(){
                        return Err("Cannot convert date from string given".to_string())
                    }
                    year = 2000 + value.unwrap();
                }
                4 => {
                    let value = vec[0].parse::<i32>();
                    if value.is_err(){
                        return Err("Cannot convert date from string given".to_string())
                    }
                    year = value.unwrap();
                }
                _ => {
                    return Err("Cannot convert date from string given".to_string())
                }
            }
            // Lets do the month
            let value = vec[1].parse::<u32>();
            if value.is_err(){
                return Err("Cannot convert date from string given".to_string())
            }
            month = value.unwrap();

            // and the day
            let d_val = vec[2].parse::<u32>();
            if d_val.is_err(){
                return Err("Cannot convert date from string given".to_string())
            }
            day = d_val.unwrap();            
        }
        _ => {
            return Err("Incorrect number of terms for the date string".to_string())
        }
    }

    let ret = convert_date_to_timestamp(year,month,day);
    return Ok(ret)
}


// Function to convert a date(year,month,day) in digits to a unix timestamp
pub fn convert_date_to_timestamp(year: i32, month: u32, day: u32) -> i64 {
                                         
    let p_date = NaiveDate::from_ymd(year, month, day);
    let p_time = NaiveTime::from_hms(0, 0, 0); 
    let date_time = p_date.and_time(p_time);

    let ret = date_time.timestamp();
    return ret;
}



















/*
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
                                    ALL TESTS ARE RUN:  ONE AT A TIME   
                                    
    Running concurrent tests in the same directory with reading and writing has unpredictable results                                    
*/
#[warn(unused_assignments)]
#[cfg(test)]
mod tests {                   //     DONT RUN THE TESTS ABOVE THIS LINE
    use super::*;
    

    #[ignore]
    #[test]
    fn t001_sentence_case() {
        let sentence = "who knows, if this-will work.";
        let res = sentence_case(sentence);
        let ans = "Who knows, if this-will work.".to_string();

        assert_eq!(res, ans);
    }


    #[ignore]
    #[test]
    fn t002_sentence_case() {
        let sentence = "  YOu   did   it   svenny  ";
        let res = sentence_case(sentence);
        let ans = "You did it svenny".to_string();

        assert_eq!(res, ans);
    }


    #[ignore]
    #[test]
    fn t003_justify_left1() {
        let sentence = "  Y ".to_string();
        let ans =  justify(sentence,9,Justify::Left);

        assert_eq!(ans, "Y        ");
    }


    #[ignore]
    #[test]
    fn t004_justify_left2() {
        let sentence = "Yy ".to_string();
        let ans =  justify(sentence,9,Justify::Left);

        assert_eq!(ans, "Yy       ");
    }


    #[ignore]
    #[test]
    fn t005_justify_left3() {
        let sentence = "Yy had Enough ".to_string();
        let ans =  justify(sentence,5,Justify::Left);

        assert_eq!(ans, "Yy had Enough");
    }


    #[ignore]
    #[test]
    fn t006_justify_left4() {
        let sentence = "    hello   all ".to_string();
        let ans =  justify(sentence,15,Justify::Left);

        assert_eq!(ans, "hello   all    ");
    }


    #[ignore]
    #[test]
    fn t007_justify_right1() {
        let sentence = "    q ".to_string();
        let ans =  justify(sentence,3,Justify::Right);

        assert_eq!(ans, "  q");
    }


    #[ignore]
    #[test]
    fn t008_justify_right2() {
        let sentence = "Yy ".to_string();
        let ans =  justify(sentence,9,Justify::Right);

        assert_eq!(ans, "       Yy");
    }


    #[ignore]
    #[test]
    fn t009_justify_right3() {
        let sentence = "Yy  had  Enough ".to_string();
        let ans =  justify(sentence,5,Justify::Right);

        assert_eq!(ans, "Yy  had  Enough");
    }


    #[ignore]
    #[test]
    fn t010_justify_right4() {
        let sentence = "    hello   all ".to_string();
        let ans =  justify(sentence,15,Justify::Right);

        assert_eq!(ans, "    hello   all");
    }


    #[ignore]
    #[test]
    fn t011_justify_center1() {
        let sentence = "    q ".to_string();
        let ans =  justify(sentence,3,Justify::Center);

        assert_eq!(ans, " q ");
    }


    #[ignore]
    #[test]
    fn t012_justify_center2() {
        let sentence = "Yy ".to_string();
        let ans =  justify(sentence,9,Justify::Center);

        assert_eq!(ans, "    Yy   ");
    }


    #[ignore]
    #[test]
    fn t013_justify_center3() {
        let sentence = "Yy  had  Enough ".to_string();
        let ans =  justify(sentence,5,Justify::Center);

        assert_eq!(ans, "Yy  had  Enough");
    }


    #[ignore]
    #[test]
    fn t014_justify_center4() {
        let sentence = "    hello   all ".to_string();
        let ans =  justify(sentence,15,Justify::Center);

        assert_eq!(ans, "  hello   all  ");
    }


    #[ignore]
    #[test]
    fn t015_title_case1() {
        let message = "YOu cAn bOOgie.";
        let ans =  title_case(message);

        assert_eq!(ans,"You Can Boogie.".to_string());
    }


    #[ignore]
    #[test]
    fn t016_title_case2() {
        let message = "YOu cAn-bOOgie.";
        let ans =  title_case(message);

        assert_eq!(ans,"You Can-boogie.".to_string());
    }


    #[ignore]
    #[test]
    fn t017_uppercase_first_letter() {
        let message = "yOu cAn-bOOgie.";
        let ans = uppercase_first_letter(message);
        let first = ans.chars().nth(0).unwrap();

        assert_eq!(first,'Y');
    }


    #[ignore]
    #[test]
    fn t018_sentence_case3() {
        let message = " sure   yOu     cAn-bOOgie   ";
        let ans = sentence_case(message);

        assert_eq!(ans,"Sure you can-boogie");
    }


    #[ignore]
    #[test]
    fn t019_plural() {
        let mut word = plural("record", 7);
        assert_eq!(word,"records");
        
        word = plural("number", 1);
        assert_eq!(word,"number");

    }
    
    #[ignore]
    #[test]
    fn t020_assume_date() {

        let str = "2022.09.26";
        let ans  = convert_assumed_date(str);
        if ans.is_ok(){
            assert_eq!(ans.unwrap(),1664150400)
        }

        let str = "22.09.26";
        let ans  = convert_assumed_date(str);
        if ans.is_ok(){
            assert_eq!(ans.unwrap(),1664150400)
        }
    }






































    
} // End of tests











