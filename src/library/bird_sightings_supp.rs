/*
        This is a support module for all Bird sightings

        2022.02.21   Sven Ponelat

*/


use crate::library::my_file_funcs::*;
use crate::library::bird_species::*;
use termion::{color, style };
use super::settings::SettingsText;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fmt::{Debug};
use std::cmp::Ordering;
use crate::library::bird_sightings::*;





#[derive(Clone, Debug )]
pub struct WhatNumber{
    pub is_number: bool,
    pub sighting: Option<Sightings>,
    pub number: Option<usize>,
    pub species: Option<Species>
}

impl WhatNumber {
    pub fn new() -> WhatNumber {
        WhatNumber {
            is_number: false,
            sighting: None,
            number: None,
            species: None,
        }
    }
}


    
#[derive(Clone, Debug )]
pub struct SearchFields{
    ch: char,
    value: String,
    d1: i64,
    d2: i64
}

impl SearchFields{
    pub fn new(ch: char, value: String) -> SearchFields {
        SearchFields {
            ch: ch,
            value: value,
            d1: 0,
            d2: 0,
        }
    }
}


#[derive(Clone, Debug, PartialOrd, Eq )]
pub struct Location {
    location: String,
    town: String,
    province: String,
    // date: String
}

impl Location{
    pub fn new(sighting: &Sightings) -> Location {
        // let display = sighting.display_date();
        let s = sighting.clone();
        Location { location: s.location, 
                    town: s.town, 
                    province: s.province, 
                    // date: display 
        }
    }
}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        if self.location == other.location &&
            self.town == other.town &&
            self.province == other.province {
            return true
        }
        else {
            return false
        }
    }
}

impl Ord for Location {
    fn cmp(&self, other: &Self) -> Ordering {
        self.location.cmp(&other.location)
        // if self.date == other.date {
        //     return self.location.cmp(&other.location)
        // }
        // return self.date.cmp(&other.date)
    }
}



// Error check the number from arguments
pub fn what_number(sub: &str, sbirds: &BTreeMap<String,Species> , sightings: &Vec<Sightings>) -> WhatNumber {
    let mut answer = WhatNumber::new();
    let lower = sub.to_lowercase();
    let trim = lower.trim();
 
    let res2 = trim.clone().parse::<i64>();
    if res2.is_ok(){
        let given = res2.clone().unwrap();
        let len: i64 = sightings.len() as i64;
        
        // something wrong with the number
        if given < 1 || given > len {
            answer.is_number = false;
            answer.sighting = None;
            answer.number = None;
            answer.species = None;
            
            return answer;
        }

        // the number will not be zero-based but make it so.
        let index: usize = given as usize - 1; 
        
        // Is a number
        let r = sightings.get(index);
        if r.is_some(){
            let sight = r.unwrap().to_owned();
            let species = sbirds.get(&sight.sname).unwrap().clone();
            answer.number = Some(index);
            answer.sighting = Some(sight);
            answer.is_number = true;
            answer.species = Some(species);
        } 
        
        // unlikely but something went wrong
        else {
            answer.is_number = false;
            answer.sighting = None;
            answer.number = None;
            answer.species = None;
            
            return answer;
        }
    }
    
    // It is not a number 
    if res2.is_err(){
        answer.is_number = false;
        answer.sighting = None;
        answer.number = None;
        answer.species = None;
    }

    return answer;
} // end of what_number



// Functions to get results of search terms from sightings database
pub fn get_searched_slice_of_sightings<'a>(arg: &str, sbirds: &'a BTreeMap<String,Species>, 
                    sightings: &'a Vec<Sightings> ) -> Result<(Vec<usize>,Vec<Sightings>), String> { 
    
    let capacity: usize = 500;

    // let timer = Timer::new();
    let mut terms_str = arg.to_lowercase().trim().to_string();
    let mut ret: Vec<Sightings> = Vec::with_capacity(capacity);
    let mut positions: Vec<usize> = Vec::with_capacity(capacity);
    
    let search = make_search_vector(&mut terms_str);
    if search.is_err(){
        return Err(search.err().unwrap())
    }
    let found = search.clone().unwrap().len();
    let mut index: usize = 1;
    
    // Lets loop on sightings
    for sight in sightings{
        let r_species = get_sname_as_key_and_return_value(&sight.sname, &sbirds);
        if r_species.is_err(){
            return Err(r_species.err().unwrap())
        } 
        let species = r_species.unwrap();
        let mut and_counter = 0;
        let s_array = search.clone().unwrap();
        
        for st in s_array{
            match st.ch {
                'a' => {
                    if sight.location.to_lowercase().contains(&st.value){
                        and_counter += 1; 
                    }
                    else {
                        break;
                    }
                }
                'c' => {
                    if species.code == st.value {
                        and_counter += 1; 
                    }
                    else {
                        break;
                    }
                }
                'd' => {
                    if st.d2 == 0 {
                        if sight.date == st.d1 {
                            and_counter += 1; 
                        }
                        else {
                            break;
                        }
                    }
                    else {
                        if sight.date >= st.d1 && sight.date <= st.d2 {
                            and_counter += 1; 
                        }
                        else {
                            break;
                        }
                    }
                }
                'e' => {
                    if species.aname.to_lowercase().contains(&st.value) {
                        and_counter += 1; 
                    }
                    else {
                        break;
                    }
                }
                'l' => {
                    if species.list.to_lowercase().contains(&st.value) {
                        and_counter += 1; 
                    }
                    else {
                        break;
                    }
                }
                'm' => {
                    if species.family.to_lowercase().contains(&st.value) {
                        and_counter += 1; 
                    }
                    else {
                        break;
                    }
                }
                'n' => {
                    if species.name.to_lowercase().contains(&st.value) {
                        and_counter += 1; 
                    }
                    else {
                        break;
                    }
                }
                'o' => {
                    if sight.comments.to_lowercase().contains(&st.value) {
                        and_counter += 1; 
                    }
                    else {
                        break;
                    }
                }
                'p' => {
                    if sight.province.to_lowercase().contains(&st.value) {
                        and_counter += 1; 
                    }
                    else {
                        break;
                    }
                }
                'r' => {
                    if species.order.to_lowercase().contains(&st.value) {
                        and_counter += 1; 
                    }
                    else {
                        break;
                    }
                }
                's' => {
                    if sight.sname.to_lowercase().contains(&st.value) {
                        and_counter += 1; 
                    }
                    else {
                        break;
                    }
                }
                'w' => {
                    if sight.town.to_lowercase().contains(&st.value) {
                        and_counter += 1; 
                    }
                    else {
                        break;
                    }
                }
                'A' => {
                    if sight.adult {
                        and_counter += 1; 
                    }
                    else {
                        break;
                    }
                }
                'B' => {
                    if sight.breeding {
                        and_counter += 1; 
                    }
                    else {
                        break;
                    }
                }
                'C' => {
                    if sight.chicks {
                        and_counter += 1; 
                    }
                    else {
                        break;
                    }
                }
                'E' => {
                    if sight.dead {
                        and_counter += 1; 
                    }
                    else {
                        break;
                    }
                }
                'F' => {
                    if sight.female {
                        and_counter += 1; 
                    }
                    else {
                        break;
                    }
                }
                'G' => {
                    if sight.eggs {
                        and_counter += 1; 
                    }
                    else {
                        break;
                    }
                }
                'H' => {
                    if sight.heard {
                        and_counter += 1; 
                    }
                    else {
                        break;
                    }
                }
                'I' => {
                    if sight.immature {
                        and_counter += 1; 
                    }
                    else {
                        break;
                    }
                }
                'M' => {
                    if sight.male {
                        and_counter += 1; 
                    }
                    else {
                        break;
                    }
                }
                'N' => {
                    if sight.nonbreeding {
                        and_counter += 1; 
                    }
                    else {
                        break;
                    }
                }
                'P' => {
                    if sight.photo {
                        and_counter += 1; 
                    }
                    else {
                        break;
                    }
                }
                'R' => {
                    if sight.ringed {
                        and_counter += 1; 
                    }
                    else {
                        break;
                    }
                }
                'S' => {
                    if sight.seen {
                        and_counter += 1; 
                    }
                    else {
                        break;
                    }
                }
                'T' => {
                    if sight.nest {
                        and_counter += 1; 
                    }
                    else {
                        break;
                    }
                }
                
                
                
                _ => {
                    return Err("Query contains a wrong seatch character.".to_string())
                }
            }
        }
        
        if and_counter == found {
            ret.push(sight.clone());
            positions.push(index);
        }
        index += 1;
    }
    
    // timer.stop();
    let tuple = (positions, ret);
    Ok(tuple)
}


// Function to return an array of search terms
pub fn make_search_vector(terms_str: &mut String) -> Result<Vec<SearchFields>, String> {
    let mut ret: Vec<SearchFields> = Vec::new();
    
    terms_str.retain(|c| !r#"""#.contains(c));
    let lower = terms_str.to_lowercase();
    let temp = lower.split("#");
    let vec: Vec<&str> = temp.collect();
    
    for term in vec {
        let i_temp = term.split("=");
        let indi: Vec<&str> = i_temp.collect();
        match indi.len(){
            1 => {
                let alpha = term.replace(|c: char| !c.is_alphabetic(), "");
                let up_a = alpha.to_uppercase();
                for ch in up_a.chars(){
                    match ch {
                        'S'|'H'|'R'|'B'|'N'|'T'|'G'|'C'|'I'|'E'|'M'|'F'|'A'|'P' => {
                            // let arr = SearchFields::seen {ch: ch, value: true };
                            let arr = SearchFields::new(ch, "".to_string());
                            ret.push(arr);
                            continue;
                        }
                        
                        _ => {
                            let message = format!("Wrong characteristic for search term was included i.e. {}",ch);
                            return Err(message);
                        }
                    }
                }
            }
            2 => {
                match indi[0]{
                    "a" => { //location
                        let arr = SearchFields::new('a', indi[1].to_string());
                        ret.push(arr);
                        continue;
                    }
                    "c" => { //code
                        let arr = SearchFields::new ('c', indi[1].to_string());
                        ret.push(arr);
                        continue;
                    }
                    "d" => { //date
                        let range = indi[1].split("-");
                        let dd: Vec<&str> = range.collect(); 
                        if dd.len() == 2 {
                            let d1 = convert_date_text(dd[0]);
                            if d1.is_err(){
                                return Err("Problem parsing date text".to_string())
                            }
                            let d2  = convert_date_text(dd[1]);
                            if d2.is_err(){
                                return Err("Problem parsing date text".to_string())
                            }
                            // let arr = SearchFields::date {ch: 'd', start: d1.unwrap(), end: d2.unwrap()};
                            let arr = SearchFields {ch: 'd', value: "".to_string(), d1: d1.unwrap(), d2: d2.unwrap() };
                            ret.push(arr);
                            continue;
                        }
                        else if dd.len() == 1 {
                            let d1 = convert_date_text(dd[0]);
                            if d1.is_err(){
                                return Err("Problem parsing date text".to_string())
                            }
                            // let arr = SearchFields::date {ch: 'd', start: d1.unwrap(), end: 0};
                            let arr = SearchFields {ch: 'd', value: "".to_string(), d1: d1.unwrap(), d2: 0 };
                            ret.push(arr);
                            continue;
                        }
                        else {
                            return Err("Wrong number of search terms in date string".to_string())
                        }
                    }
                    "e" => { //aname
                        let arr = SearchFields::new('e',indi[1].to_string());
                        ret.push(arr);
                        continue;
                    }
                    "l" => { //list
                        let arr = SearchFields::new('l',indi[1].to_string());
                        ret.push(arr);
                        continue;
                    }
                    "m" => { //family
                        let arr = SearchFields::new('m',indi[1].to_string());
                        ret.push(arr);
                        continue;
                    }
                    "n" => { //name
                        let arr = SearchFields::new('n',indi[1].to_string());
                        ret.push(arr);
                        continue;
                    }
                    "o" => { //comments
                        let arr = SearchFields::new('o',indi[1].to_string());
                        ret.push(arr);
                        continue;
                    }
                    "p" => { //province
                        let arr = SearchFields::new('p',indi[1].to_string());
                        ret.push(arr);
                        continue;
                    }
                    "r" => { //order
                        let arr = SearchFields::new('r',indi[1].to_string());
                        ret.push(arr);
                        continue;
                    }
                    "s" => { //sname
                        let arr = SearchFields::new('s',indi[1].to_string());
                        ret.push(arr);
                        continue;
                    }
                    "t" => {  //country
                        let arr = SearchFields::new('t',indi[1].to_string());
                        ret.push(arr);
                        continue;
                    }
                    "u" => { //status
                        let arr = SearchFields::new('u',indi[1].to_string());
                        ret.push(arr);
                        continue;
                    }
                    "w" => { //town
                        let arr = SearchFields::new('w',indi[1].to_string());
                        ret.push(arr);
                        continue;
                    }


                    _ => {
                        let message = format!("Wrong char for search term was included i.e. {}",indi[0]);
                        return Err(message);
                    }
                }
            }
            _ => {
                let message = format!("Problem in search term parsing");
                return Err(message);
            }
        }
    }

    Ok(ret)
}


// Function to display the search results
pub fn display_search_results( options: &SettingsText, sbirds: &BTreeMap<String,Species>, 
            positions: Vec<usize>, results: Vec<Sightings>){

    let my_normal_gray: color::Rgb = options.clone().get_color("myNormalGray");

    for i in 0..results.len() {
        // let species = get_sname_value(&results[i].sname, &birds).unwrap();
        let species = sbirds.get(&results[i].sname).unwrap().clone();
        let pos = positions[i];      

        let pos_text = justify(pos.to_string(), P_LEN, Justify::Right); 
        let name_text = justify(species.name, NAME_39, Justify::Left); 
        let date_text = justify(results[i].display_date(), DATE_LEN, Justify::Center); 
        let location_text = justify(results[i].location.to_string(), NAME_39, Justify::Left); 
        let comment_text = justify(results[i].comments.to_string(), FAMILY_59, Justify::Left);  

        print!("{}{}  ", color::Fg(my_normal_gray), pos_text);  
        print!("{}", name_text);  
        print!("{} {} {}\n", date_text, location_text, comment_text); 
    }

    println!();
    let message = format!("The search results ended with {} {}.",results.len(),plural("record", results.len()));
    feedback(Feedback::Info, message);
}


// Function to get the last 10 locations with dates to use as a shortcut
pub fn get_last_10(sightings: &Vec<Sightings>) -> Vec<String> {
    let len = sightings.len();
    let mut ret: Vec<String> = Vec::with_capacity(10);
    let mut num_locs: u8 = 0;
    let mut loop_counter = 0;
    let mut set: BTreeSet<Location> = BTreeSet::new();

    while loop_counter < len && num_locs < 10 {
        let zero = len - loop_counter ;
        let sight = sightings.get(zero - 1).unwrap();
        let loc = Location::new(sight);
        if set.insert(loc) {
            let last = make_last_line(zero,sight.clone(),num_locs);
            ret.push(last);
            num_locs += 1;
        }

        loop_counter += 1;
    }

    return ret
}


// Function to make a line that will be used to display befor adding a sighting.
pub fn make_last_line(zero: usize, sight: Sightings, num_locs: u8)  -> String {
    let mut ret = String::with_capacity(260);
    let date = sight.display_date();

    let str = format!("  {}  {}  {}  {}  {}  # {}",num_locs.to_string(), 
                            sight.location, sight.town, sight.province, date, zero.to_string());
    ret.push_str(&str);

    return ret
}


// Function to display the last 10
pub fn display_last_10(options: &SettingsText, last_10: Vec<String>) {

    let my_normal_gray: color::Rgb = options.clone().get_color("myNormalGray");

    for line_number in 0..last_10.len() {
        print!("{}{}  \n", color::Fg(my_normal_gray), last_10[line_number]);  
    }   
}

// Function to message the addition of a sighting
pub fn oa_display_addition(wn: WhatNumber, sbirds: BTreeMap<String, Species>, sightings: Vec<Sightings> ) {
    
    let sight = sightings.get(wn.number.unwrap()).unwrap().clone();
    let species = sbirds.get(&sight.clone().sname).unwrap().clone();
    let mut assembled = String::new();

    let part1 = format!("The sighting of the {}{}{} {}",
            color::Fg(color::Yellow), style::Italic, species.name, style::Reset);

    let part2 = format!(" has been added to sightings file at position {}{}{} {} ",
            color::Fg(color::Yellow), style::Italic, (wn.number.unwrap()+1), style::Reset);

    assembled.push_str(&part1);
    assembled.push_str(&part2);

    feedback(Feedback::Info, assembled);
}

// Function to show the success of editing the sighting
pub fn show_edit(old: WhatNumber, new: WhatNumber) {
    
    let mut assembled = String::with_capacity(300);
    let part1: String;
    let part2: String;

    let o2 = old.number.unwrap() + 1;
    let n2 = new.number.unwrap() + 1;

    // Is the old position different to the new position
    let shuffled: bool = old.number.unwrap() != new.number.unwrap();

    if shuffled {
        part1 = format!("The sighting which was in position {}{}{} {}",
        color::Fg(color::Yellow), style::Italic, o2, style::Reset);

        part2 = format!(" has been moved to position {}{}{} {}.",
        color::Fg(color::Yellow), style::Italic, n2, style::Reset);
    }
    // not shuffled
    else {
        part1 = format!("The editing of the {}{}{} {}",
        color::Fg(color::Yellow), style::Italic, old.species.unwrap().name, style::Reset);
        
        part2 = format!("was successfully done at position {}{}{} {}.",
        color::Fg(color::Yellow), style::Italic, o2, style::Reset);
    }
    
    assembled.push_str(&part1);
    assembled.push_str(&part2);

    feedback(Feedback::Info, assembled);
}

// Function to get shortcut, if one was given
pub fn get_shortcut(last: Vec<String>, arg: &str, sightings: &Vec<Sightings>) -> Result<Option<Sightings>, String> {
    
    // let mut ret: Option<Sightings> = None;
    let mut to_be_added: Sightings = Sightings::new();
    let mut only_one_shortcut = true;
    let mut has_values = false;
    let mut terms_str = arg.to_lowercase().to_string();
    terms_str.retain(|c| !r#"""#.contains(c));
    let temp = terms_str.split("#");
    let vec: Vec<&str> = temp.collect();

    for string in vec {
        let split_again = string.split("=");
        let query: Vec<&str> = split_again.collect();
        match query.len(){
            1 => {
                if !only_one_shortcut {
                    return Err("Shortcut should be in string with no '=' sign in it".to_string())
                    // return None;
                }
                let digit: String = query[0].chars().filter(|c| c.is_digit(10)).collect();
                if digit.len() > 1 {
                    return Err("Too many digits in the shortcut string".to_string())
                    // return None;
                } 
                if digit.len() == 1 {
                    let index: usize = digit.parse::<usize>().unwrap();
                    let scut = last[index].split("#");
                    let split: Vec<&str> = scut.collect();
                    let rec_num = split[1].trim().parse::<usize>().unwrap();
                    let tsight = sightings.get(rec_num - 1).unwrap().clone();
                    to_be_added.location = tsight.location;
                    to_be_added.town = tsight.town;
                    to_be_added.province = tsight.province;
                    to_be_added.country = tsight.country;
                    to_be_added.date = tsight.date;

                    only_one_shortcut = false;
                    has_values = true;
                }
            }

            _ => {
                // leave it for another function
            }
        }
    }
    if !has_values{
        return Ok(None); 
    }
    
    Ok(Some(to_be_added))
}






















// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@  Tests  @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@         @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@




#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::copy};
    use std::fs::remove_file;

    
    // #[ignore]
    // #[test]
    // fn t001_deconstruct_sighting_str() {
    //     let arg = "a=delta park#d=2001#s=anas".to_string();
    //     let field = "d".to_string();
    //     let result = deconstruct_sighting_str(field, arg);

    //     assert_eq!(result.unwrap(),"2001".to_string());
    // }
    
    
    #[ignore]
    #[test]
    fn t002_get_searched_slice_of_sightings() {
        let source = "./test/store/sightings/to_use.json";
        let destination = "./test/sights.json";
        copy(source,destination).expect("Failed to copy");
        let sightings = Sightings::import(destination).unwrap();
        remove_file(destination).expect("Cleanup test failed");

        let source = "./test/store/species/birds.bin";
        let destination = "./test/birds.bin";
        copy(source,destination).expect("Failed to copy");
        let birds = Species::load(destination).unwrap();
        let sbirds = crate::library::bird_species_support::make_sname_btree(&birds);
        remove_file(destination).expect("Cleanup test failed");

        let arg = "a=eagle".to_string();
        let res = get_searched_slice_of_sightings(&arg, &sbirds, &sightings);

        assert_eq!(res.unwrap().1.len(),392);
    }
    
    
    #[ignore]
    #[test]
    fn t003_equality1() {
        let source = "./test/store/sightings/to_use.json";
        let destination = "./test/sights.json";
        copy(source,destination).expect("Failed to copy");
        let sightings = Sightings::import(destination).unwrap();
        remove_file(destination).expect("Cleanup test failed");


        let mut s2: Vec<Sightings> = Vec::new();
        let b1 = sightings.get(0).unwrap().to_owned();
        let b2 = sightings.get(1).unwrap().to_owned();
        let mut b3 = sightings.get(0).unwrap().to_owned();

        s2.push(b1);
        s2.push(b2);
        s2.push(b3);

        let is_equal = s2[0] == s2[1];
        assert_eq!(is_equal,false);
        
        b3 = s2.pop().unwrap();
        b3.location = "delt park ".to_string();
        s2.push(b3);

        let is_equal = s2[0] == s2[2];
        assert_eq!(is_equal,false);
        
        b3 = s2.pop().unwrap();
        b3.location = "Delta Park".to_string();
        s2.push(b3);
        
        let is_equal = s2[0] == s2[2];
        assert_eq!(is_equal,true);

        assert_eq!(s2.len(),3);
    }


    #[ignore]
    #[test]
    fn t004_deconstruct_sighting_str2() {
        let mut arg = "a=delta park#d=2001.01.01-2020.03.31#s=anas#S. A.".to_string();
        let mut result = make_search_vector(&mut arg);
        if result.is_ok(){
            assert_eq!(result.unwrap().len(),5);
        }
        
        arg = "c=jabl#d=2001.09.01#n=blu#..P,,I,,CAG#e=lemon  ".to_string();
        result = make_search_vector(&mut arg);
        if result.is_ok(){
            assert_eq!(result.unwrap().len(),9);
        }

        arg = "c=jabl#d=2001.09.01#n=blu#..P,X,I,,CAG#e=lemon  ".to_string();
        result = make_search_vector(&mut arg);
        assert_eq!(result.is_err(),true);
        
        arg = "c=jabl#d=2001.09.01#n=blu#..P,X,I,,CAG#e=lemon  ".to_string();
        result = make_search_vector(&mut arg);
        assert_eq!(result.is_err(),true);
        
        arg = "u=NOT#w=ran#n=blu#..P,I,,CAG#e=lemon#o=ife#t=sout  ".to_string();
        result = make_search_vector(&mut arg);
        if result.is_ok(){
            assert_eq!(result.unwrap().len(),11);
        }
    }
    
    
    #[ignore]
    #[test]
    fn t005_search_slice1() {
        let source = "./test/store/sightings/to_use.json";
        let destination = "./test/sights.json";
        copy(source,destination).expect("Failed to copy");
        let sightings = Sightings::import(destination).unwrap();
        remove_file(destination).expect("Cleanup test failed");
        
        let source = "./test/store/species/birds.bin";
        let destination = "./test/birds.bin";
        copy(source,destination).expect("Failed to copy");
        let birds = Species::load(destination).unwrap();
        let sbirds = crate::library::bird_species_support::make_sname_btree(&birds);
        remove_file(destination).expect("Cleanup test failed");
        
        let mut arg = "d=2021.06.26#o=lifer".to_string();
        let mut slice = get_searched_slice_of_sightings(&arg, &sbirds, &sightings);
        if slice.is_ok(){
            assert_eq!(slice.unwrap().1.len(),1);
        }
        
        arg = "H#d=2010.06.01".to_string();
        slice = get_searched_slice_of_sightings(&arg, &sbirds, &sightings);
        if slice.is_ok(){
            assert_eq!(slice.unwrap().1.len(),3);
        }
        
        arg = "MFA#d=2014.02.16".to_string();
        slice = get_searched_slice_of_sightings(&arg, &sbirds, &sightings);
        if slice.is_ok(){
            assert_eq!(slice.unwrap().1.len(),2);
        }
        
        arg = "E".to_string();
        slice = get_searched_slice_of_sightings(&arg, &sbirds, &sightings);
        if slice.is_ok(){
            assert_eq!(slice.unwrap().1.len(),71);
        }
        
        arg = "a=delta park#d=2001.09.01".to_string();
        slice = get_searched_slice_of_sightings(&arg, &sbirds, &sightings);
        if slice.is_ok(){
            assert_eq!(slice.unwrap().1.len(),7);
        }
        
        arg = "w=kgomo".to_string();
        slice = get_searched_slice_of_sightings(&arg, &sbirds, &sightings);
        if slice.is_ok(){
            assert_eq!(slice.unwrap().1.len(),94);
        }

    }

    #[ignore]
    #[test]
    fn t006_get_last_10() {
        let source = "./test/store/sightings/sightings.bin";
        let destination = "./test/sights.bin";
        copy(source,destination).expect("Failed to copy");
        let sightings = Sightings::load(destination).unwrap();
        remove_file(destination).expect("Cleanup test failed");
        
        let ten = get_last_10(&sightings);
        assert_eq!(ten.len(),10);
    }

    #[ignore]
    #[test]
    fn t006_get_shortcut1() {
        let source = "./test/store/sightings/sightings.bin";
        let destination = "./test/sights.bin";
        copy(source,destination).expect("Failed to copy");
        let sightings = Sightings::load(destination).unwrap();
        remove_file(destination).expect("Cleanup test failed");
        
        let ten = get_last_10(&sightings);
        let arg = "fart1";
        let res = get_shortcut(ten, arg, &sightings);
        if res.is_ok(){
            assert_eq!(res.unwrap().unwrap().date,1638057600);
        }
    }












} // end of test









