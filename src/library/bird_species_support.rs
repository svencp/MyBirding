/*
    My module for support structures etc for the bird species module

    2022.02.11  Sven Ponelat

*/


use termion::{color, style };
// use termion::style;
use crate::library::bird_species::*;
use crate::library::my_file_funcs::*;
use crate::library::settings::*;
use std::{collections::BTreeMap};
use super::my_file_funcs::Feedback;





#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Code{
    Neither,
    Code,
    Number,
}

#[derive(Clone, Debug )]
pub struct CodeOrNumber{
    pub what: Code,
    pub code: Option<String>,
    pub species: Option<Species>,
    pub number: Option<usize>
}


#[allow(dead_code)]
impl CodeOrNumber{

    pub fn new() -> CodeOrNumber {
        CodeOrNumber {
            what: Code::Neither,
            code: None,
            species: None,
            number: None
        }
    }
    
    pub fn make(what: Code, code: String, species: Species, number: usize) -> CodeOrNumber {
        CodeOrNumber {
            what:  what,
            code:  Some(code),
            species: Some(species),
            number: Some(number)
        }
    }



} // end of impl









// Tests and receives whether the argument given is a legit code or an integer or neither 
// but have decided to combine both, so that both number and code are there
pub fn what_code(sub: &str, birds: &mut BTreeMap<String,Species>) -> CodeOrNumber {
    let mut answer = CodeOrNumber::new();
    let lower = sub.to_lowercase();
    let trim = lower.trim();

    // Lets test the code first
    let res1 = birds.get_mut(trim.clone());
    if res1.is_some(){
        let value = res1.unwrap().to_owned();
        answer.what = Code::Code;
        answer.species = Some(value.clone()); 
        answer.code = Some(trim.to_string());

        // Get the index
        let r = get_index_from_sname(&value.sname, birds);
        if r.is_ok(){
            answer.number = Some(r.unwrap());
        } 
        // unlikely but something went wrong
        else {
            answer.number = None;
        }
        
        return answer;
    }
    
    let res2 = trim.clone().parse::<i64>();
    if res2.is_ok(){
        let given = res2.clone().unwrap();
        let len: i64 = birds.len() as i64;
        
        // something wrong with the number
        if given < 1 || given > len {
            answer.what = Code::Neither;
            answer.species = None; 
            answer.code = None;
            answer.number = None;
            
            return answer;
        }

        // the number will not be zero-based
        let index: usize = given as usize - 1; 
        
        // Is a number
        let r = get_species_from_index(index, birds);
        if r.is_ok(){
            let code = r.clone().unwrap().0;
            let species = r.unwrap().1;
            answer.what = Code::Number;
            answer.number = Some(index);
            answer.species = Some(species);
            answer.code = Some(code);
        } 

        // unlikely but something went wrong
        else {
            answer.what = Code::Neither;
            answer.number = None;
            answer.species = None;
            answer.code = None;

            return answer;
        }
    }
    
    // It is not a number either
    if res2.is_err(){
        answer.what = Code::Neither;
        answer.number = None;
        answer.species = None;
        answer.code = None;
    }

    return answer;
} // end of what_code


// Find the index of the sname in the birds database
pub fn get_index_from_sname(sname: &str, birds: &mut BTreeMap<String,Species>)  -> Result<usize, String> {
    let mut index: usize = 0;
    let mut found: bool = false;

    for (_k,v) in birds {
        if sname == v.sname {
            found = true;
            break;
        }
        index += 1;
    }
    if found {
        return Ok(index)
    }
    else {
        return Err("Sname was not found in birds database.".to_string())
    }
}

// Get the bird position (index 0-based) in the BTreeMap
pub fn get_index_from_code(code: &str, birds: &BTreeMap<String,Species>) -> Result<usize, String> {
    let mut counter = 0;
    let mut found: bool = false; 
    for (_k, v) in birds {
        if code == v.code {
            found = true;
            break;
        }
        counter += 1;
    }
    
    if !found {
        return Err("The species code was not found in the birds map".to_string())
    }
    
    Ok(counter as usize)
}




// Find the species from the birds database using the index
pub fn get_species_from_index<'a>(z_index: usize, birds: &'a mut BTreeMap<String,Species>)  -> Result<(String, Species), String> {
    let mut counter: usize = 0;
    let mut found: bool = false;
    let mut key = String::new();
    let mut species = Species::new(); 

    for (k,v) in birds {
        if counter == z_index {
            found = true;
            key = k.to_string();
            species = v.clone();
            break;
        }
        counter += 1;
    }
    if found {
        let ret = (key, species);
        return Ok(ret)
    }
    else {
        return Err("Sname was not found in birds database.".to_string())
    }
}


// Get the last species viewed from the birds database
pub fn get_last_species_viewed<'a>(options: &SettingsText, birds: &'a mut BTreeMap<String,Species>) -> CodeOrNumber {
    let mut answer = CodeOrNumber::new();
    let species;

    let index = options.get_number("lastSpeciesViewed");
    let r_species = get_species_from_index(index, birds);
    if r_species.is_ok() {

        species = r_species.unwrap().1;

        answer.what = Code::Number;
        answer.code = Some(species.clone().code);
        answer.number = Some(index);
        answer.species = Some(species);

        return answer;
    } 
    
    // Revert to first position
    species = get_species_from_index(0, birds).unwrap().1;

    answer.what = Code::Number;
    answer.code = Some(species.clone().code);
    answer.number = Some(0);
    answer.species = Some(species);

    return answer;
}


// This function display a string with two variables in it
pub fn be_display_change(what: &str, var1: &str, var2: &str) {
    let mut assembled = String::new();

    let part1 = format!("The bird species {}{}{}{}",
            color::Fg(color::Yellow), style::Italic, what, style::Reset);

    let part2 = format!(" has changed from {}{}{}{}",
            color::Fg(color::Yellow), style::Italic, var1, style::Reset);

    let part3 = format!(" to {}{}{}{}.",
            color::Fg(color::Yellow), style::Italic, var2, style::Reset);

    assembled.push_str(&part1);
    assembled.push_str(&part2);
    assembled.push_str(&part3);

    feedback(Feedback::Info, assembled);
}


// Get a slice of the birds database        
// pub fn get_code_slice_of_birds(start: String, end: String, birds: BTreeMap<String,Species>) 
pub fn get_code_slice_of_birds<'a>(start: &str, end: &str, birds: &'a mut BTreeMap<String,Species>) 
                        -> Result<BTreeMap<String,Species>, String> {

    let mut ret: BTreeMap<String,Species> = BTreeMap::new();

    for (k,v) in birds {
        if is_code_in_range(&start,&end,&k) {
            let key = k.clone();
            let value = v.clone();
            ret.insert(key, value); 
        }
    }

    if ret.clone().len() == 0 {
        return Err("There is no bird species that contains a substring of the given code".to_string())
    }

    return Ok(ret)
}  


// Function to get a birds btreemap wjere sname is the key
pub fn make_sname_btree<'a>(birds: &'a BTreeMap<String,Species>) -> BTreeMap<String,Species> {
    let mut ret: BTreeMap<String,Species> = BTreeMap::new();

    for (_k,v) in birds {
        let key = v.sname.to_string();
        let value = v;
        ret.insert(key, value.clone());
    }

    return ret
}





























#[cfg(test)]
mod tests {
    // use crate::library::bird_species::{find_index_from_sname};

    use super::*;
    use std::fs::copy;
    use std::fs::remove_file;


    
    #[ignore]
    #[test]
    fn t001_is_code() {
        let source = "./test/store/species/species5_import.json";
        let destination = "./test/birds_import.json";
        copy(source,destination).expect("Failed to copy");
        let mut birds = Species::import(destination).unwrap();
        remove_file(destination).expect("Cleanup test failed");

        let sub = "jabl";
        let ans = what_code(sub, &mut birds);
        assert_eq!(ans.what, Code::Code);
        assert_eq!(ans.code.unwrap(), "jabl");

        let sub = "3";
        let ans = what_code(sub, &mut birds);
        assert_eq!(ans.what, Code::Number);
        assert_eq!(ans.number.unwrap(), 2);
        assert_eq!(ans.species.unwrap().fname, "Ruff");
        assert_eq!(ans.code.unwrap(), "susu");

        let sub = "23sp";
        let ans = what_code(sub, &mut birds);
        assert_eq!(ans.what, Code::Neither);
        assert_eq!(ans.species.is_none(), true);
        assert_eq!(ans.code, None);
        assert_eq!(ans.number, None);
    }


    #[ignore]
    #[test]
    fn t002_find() {
        let source = "./test/store/species/species5_import.json";
        let destination = "./test/birds_import.json";
        copy(source,destination).expect("Failed to copy");
        let mut birds = Species::import(destination).unwrap();
        remove_file(destination).expect("Cleanup test failed");

        let res = get_species_from_index(4, &mut birds);
        assert_eq!(res.unwrap().1.code,"yuwh2");
    }


    #[ignore]
    #[test]
    fn t003_find2() {
        let source = "./test/store/species/species5_import.json";
        let destination = "./test/birds_import.json";
        copy(source,destination).expect("Failed to copy");
        let mut birds = Species::import(destination).unwrap();
        remove_file(destination).expect("Cleanup test failed");

        let res = get_index_from_sname("Calidris pugnax", &mut birds);
        assert_eq!(res.unwrap(),2);
    }


    #[ignore]
    #[test]
    fn t004_what_code_() {
        let source = "./test/store/species/species5_import.json";
        let destination = "./test/birds_import.json";
        copy(source,destination).expect("Failed to copy");
        let mut birds = Species::import(destination).unwrap();
        remove_file(destination).expect("Cleanup test failed");

        let res = what_code("101", &mut birds);
        assert_eq!(res.what, Code::Neither);
        
        let res = what_code("susu1", &mut birds);
        assert_eq!(res.number.unwrap(), 3);


    }


    // #[ignore]
    // #[test]
    // fn t004_make1() {
    //     let source = "./test/store/species/species5_import.json";
    //     let destination = "./test/birds_import.json";
    //     copy(source,destination).expect("Failed to copy");
    //     let mut birds = Species::import(destination).unwrap();
    //     remove_file(destination).expect("Cleanup test failed");

    //     let species = *birds.get("susu").unwrap();
    //     let code = birds.get("susu").unwrap().code;
    //     let index = get_index_from_code(&code, &birds).unwrap();

    //     let yes = CodeOrNumber::make(Code::Code, code, species, index);
    //     let res = what_code("101", &mut birds);
        
    //     assert_eq!(yes.number.unwrap(), 3);
    // }













































} // end of tests