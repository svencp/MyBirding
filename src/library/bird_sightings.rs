/*
        This is the class for all Bird sightings

        2022.01.06   Sven Ponelat

*/


use crate::library::my_file_funcs::*;
use crate::library::bird_species::*;
use crate::library::bird_species_support::*;
use crate::library::bird_sightings_box::{self};
use super::bird_sightings_supp::*;
// use super::bird_sightings_box::show_sighting;
// use super::bird_species;
use super::settings::SettingsText;
use std::io::prelude::*;
use std::path::Path;
use std::fs::{ OpenOptions };
use std::collections::BTreeMap;
use std::io::Write;
use std::process::exit;
use std::fmt::{Debug};
// use std::slice::SliceIndex;
use serde::{Serialize, Deserialize};
use substring::Substring;
// use inflections::Inflect;
use termion::{color, style};
use chrono::prelude::*;
use chrono::Utc;
use std::time::{UNIX_EPOCH, Duration};
// use thousands::Separable;




#[allow(dead_code)]
pub const SIGHTINGS_BIN_FILENAME:  &str = "./sightings.bin";
#[allow(dead_code)]
pub const SIGHTINGS_JSON_FILENAME: &str = "./sightings.json";

pub const N_LEN:     usize =  7;
pub const P_LEN:     usize =  9;
// pub const CODE_LEN:  usize = 10;
pub const DATE_LEN:  usize = 12;
pub const NAME_39:   usize = 39;
pub const FAMILY_59: usize = 59;


#[allow(non_snake_case)]
#[derive(Clone, Debug, Serialize, Deserialize, derivative::Derivative)]
#[derivative(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Sightings {
    pub date: i64,
    pub sname: String,
    pub location: String,
    pub town: String,
    pub province: String,
    pub country: String,
    pub seen: bool,
    pub heard: bool,
    pub ringed: bool,
    pub dead: bool,
    pub photo: bool,
    pub male: bool,
    pub female: bool,
    pub adult: bool,
    pub immature: bool,
    pub breeding: bool,
    pub eggs: bool,
    pub nonbreeding: bool,
    pub nest: bool,
    pub chicks: bool,
    pub comments: String,
}



#[allow(non_snake_case)]
#[allow(dead_code)]
impl Sightings {
    
    
    pub fn build_sighting(
        birds: &BTreeMap<String,Species>,
        code: &str,
        date: &str,
        // sname: String,
        location: &str,
        town: &str,
        province: &str,
        country: &str,
        seen: bool,
        heard: bool,
        ringed: bool,
        dead: bool,
        photo: bool,
        male: bool,
        female: bool,
        adult: bool,
        immature: bool,
        breeding: bool,
        eggs: bool,
        nonbreeding: bool,
        nest: bool,
        chicks: bool,
        comments: String
                                )    -> Result<Sightings, String> {            
            
        // Is the code right                                                                    -- sname
        let species = birds.get(code.to_lowercase().trim());   
        if species.is_none(){
            return Err("Code does not exist in species database".to_string());
        }  
        let i_sname = species.unwrap().clone().sname;

        // Lets do the date                                                                     -- date
        let t_date = date.trim();  
        let mut fmt: &str = ".";
        if t_date.substring(4, 5) == fmt {
            fmt = "%Y.%m.%d"
        } else {
            fmt = "%Y-%m-%d"
        }                                            
        let p_date = NaiveDate::parse_from_str(date, fmt); 
        let time_only = NaiveTime::from_hms(0, 0, 0); 
        let date_time: NaiveDateTime;
        let i_date: i64;
        if p_date.is_ok(){
            date_time = p_date.unwrap().and_time(time_only);
            i_date = date_time.timestamp();
        } else {
            return Err("Error in parsing the date string, use something like 2022.02.13".to_string());
        }

        // Lets do the location                                                                 -- location
        let mut i_location = title_case(location.trim());
        if i_location.len() == 0 {
            return Err("Error in giving no location".to_string());
        }
        if i_location.len() > 39  {
            i_location = i_location[0..40].to_string();
        }
        
        // Lets do the Town                                                                     -- Town
        let mut i_town = title_case(town.trim());
        if i_town.len() == 0 {
            return Err("Error in giving no town".to_string());
        }
        if i_town.len() > 39  {
            i_town = i_town[0..40].to_string();
        }
        
        // Lets do the Province                                                                 -- Province
        let mut i_province = title_case(province.trim());
        if i_province.len() == 0 {
            return Err("Error in giving no province/state".to_string());
        }
        if i_province.len() > 39  {
            i_province = i_province[0..40].to_string();
        }
        
        // Lets do the Country                                                                  -- Country
        let mut i_country = title_case(country.trim());
        if i_country.len() == 0 {
            return Err("Error in giving no country".to_string());
        }
        if i_country.len() > 39  {
            i_country = i_country[0..40].to_string();
        }

        // Lets do the Comments                                                                 -- Country
        let mut i_comments = comments.trim();
        if i_comments.len() > 120  {
            i_comments = &i_comments[0..120];
        }

        // Lets do the seen                                                                     -- seen
        let i_seen = seen;
        let i_heard = heard;
        let i_ringed = ringed;
        let i_dead = dead;
        let i_photo = photo;
        let i_male = male;
        let i_female = female;
        let i_adult = adult;
        let i_immature = immature;
        let i_breeding = breeding;
        let i_eggs = eggs;
        let i_nonbreeding = nonbreeding;
        let i_chicks = chicks;
        let i_nest = nest;
        // let i_comments = i_comments;
   

        // Has anything been observed
        let mut bool_counter = 0;
        if seen {bool_counter += 1;}
        if heard {bool_counter += 1;}
        if ringed {bool_counter += 1;}
        if dead {bool_counter += 1;}
        if photo {bool_counter += 1;}
        if male {bool_counter += 1;}
        if female {bool_counter += 1;}
        if adult {bool_counter += 1;}
        if immature {bool_counter += 1;}
        if breeding {bool_counter += 1;}
        if eggs {bool_counter += 1;}
        if nonbreeding {bool_counter += 1;}
        if chicks {bool_counter += 1;}
        if nest {bool_counter += 1;}
        
        if bool_counter == 0 {
            return Err("Nothing has beed observed.".to_string())
        }
        
        Ok(Sightings {
            sname:  i_sname,
            date: i_date,
            location: i_location,
            town: i_town,
            province: i_province,
            country: i_country,
            seen: i_seen,
            heard: i_heard,
            ringed: i_ringed,
            dead: i_dead,
            photo: i_photo,
            male: i_male,
            female: i_female,
            adult: i_adult,
            immature: i_immature,
            breeding: i_breeding,
            eggs: i_eggs,
            nonbreeding: i_nonbreeding,
            nest: i_nest,
            chicks: i_chicks,
            comments: i_comments.to_string()
        })
            
    }   // end of build_species


    pub fn export(json_file: &str, obs: Vec<Sightings>) -> Result<(), String> {
        let path = Path::new(json_file);
        
        let serialized = serde_json::to_string_pretty(&obs);
        let mut file = match OpenOptions::new()
                                .read(false)
                                .write(true)
                                .create(true)
                                .truncate(true)
                                .open(path)  {
            
            Err(_) => { return Err("Problem exporting species json file".to_string()); }
            Ok(file)   => { file }
        };
        
        match file.write_all(serialized.unwrap().as_bytes()) {
            Err(_) => { return Err("Problem writing species json file".to_string()); } 
            // Ok(file)   => { file } 
            Ok(_)   => { Ok(()) } 
        }
    }


    pub fn import(json_file: &str) -> Result<Vec<Sightings>, String> {
        let str_file  = std::fs::read_to_string(json_file );

        let content = match str_file {
            Ok(content) => { content },
            Err(_) => { return Err("Problem importing species json file".to_string()); }
        };
        
        let map: Vec<Sightings> = match serde_json::from_str(&content){
            Ok(map) => map,
            Err(_) => { return Err("Problem converting species json file".to_string()); }
        };
        Ok(map)
    }


    pub fn save(bin_file: &str, sightings: &Vec<Sightings>) -> Result<(), String> {
        
        // let encoded:  BTreeMap<String,Species>;
        let encoded: Vec<u8> = bincode::serialize(sightings).unwrap();
                
        // Lets open the bin file
        let mut file = match OpenOptions::new()
            .read(false)
            .write(true)
            .create(true)
            .truncate(true)
            .open(bin_file){
                Ok(content) => content,
                Err(_) => { return Err("Problem saving sightings bin file".to_string()); }
        };
        
        match file.write_all(&encoded) {
            Ok(file) => file,
            Err(_) => { return Err("Problem writing to sightings bin file".to_string()); }
        }
        Ok(())
    }


    // Load all the bird species from binary file into map
    pub fn load(bin_file: &str) -> Result<Vec<Sightings>, String> {
        
        let decoded:  Vec<Sightings>;
                
        // Lets open the bin file
        let mut file = match OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .open(bin_file){
                Ok(content) => content,
                Err(_) => { return Err("Problem opening species bin file".to_string()); }
        };
        
        let mut buffer = Vec::<u8>::new();
        match file.read_to_end(&mut buffer) {
            Ok(content)  => { content }
            Err(_) => { return Err("Problem reading species bin file".to_string()); }
        };
        
        decoded = bincode::deserialize(&buffer[..]).unwrap();
        
        Ok(decoded)
    }

    // Display the date like 2020.02.15
    pub fn display_date(&self) -> String {
        
        let d = UNIX_EPOCH + Duration::from_secs(self.date as u64);
        let datetime = DateTime::<Utc>::from(d);

        let timestamp_str = datetime.format("%Y.%m.%d" ).to_string();
        // if sep == "."{
            // timestamp_str = datetime.format("%Y.%m.%d" ).to_string();
        // } else {
        //     timestamp_str = datetime.format("%Y-%m-%d" ).to_string();
        // }

        return timestamp_str
    }    
        
    // Make an empty sighting to build on    
    pub fn new() -> Sightings {
        let ret = Sightings {
            sname:  "".to_string(),
            date: 0,
            location: "".to_string(),
            town: "".to_string(),
            province: "".to_string(),
            country: "".to_string(),
            seen: false,
            heard: false,
            ringed: false,
            dead: false,
            photo: false,
            male: false,
            female: false,
            adult: false,
            immature: false,
            breeding: false,
            eggs: false,
            nonbreeding: false,
            nest: false,
            chicks: false,
            comments: "".to_string(),
        };

        return ret
    }    
        
    // Clear all the attributes of the sighting
    pub fn clear_attributes(mut self) -> Sightings {
        self.seen = false;
        self.heard = false;
        self.ringed = false;
        self.dead = false;
        self.photo = false; 
        self.male = false; 
        self.female = false; 
        self.adult = false; 
        self.immature = false; 
        self.breeding = false; 
        self.eggs = false; 
        self.nonbreeding = false; 
        self.nest = false; 
        self.chicks = false; 

        return self
    }
        
    // Function to allocate booleans but needs a result
    // For adding  : sightings is valid if at least one boolean is present  -> allow_no_booleans = false
    // For editing : sighting must cahnge booleans only if there are any mentioned in arg string -> allow_no_booleans = true
    pub fn do_booleans(mut self, arg: &str, allow_no_booleans: bool) -> Result<Sightings, String> {
        let mut vec_r: Vec<&str> = Vec::with_capacity(2);

        let mut terms_str = arg.to_string();
        terms_str.retain(|c| !r#"""#.contains(c));
        let temp = terms_str.split("#");
        let vec: Vec<&str> = temp.collect();
        
        for line in vec {
            let l1 = line.split("=");
            let vec1: Vec<&str> = l1.collect();

            match vec1.len() {
                1 => {
                    vec_r.push(vec1[0]);
                }
                _ => {
                    // Do nothing
                }
            }
        }

        match vec_r.len() {
            // Do not change any booleans if none were found in the given string
            0 => {
                if allow_no_booleans{
                    return Ok(self)
                }
                return Err("No boolean attrributes found in given string".to_string())
            }
            1 => {
                // This is good, leave alone
            }
            _ => {
                return Err("Too many boolean attribute strings given".to_string())
            }
        }

        self  = self.clear_attributes();

        // We now have what we want, lets deal with it
        let upper = vec_r[0].trim().to_uppercase();

        if upper.contains('S'){
            self.seen = true;
        }
        if upper.contains('H'){
            self.heard = true;
        }
        if upper.contains('R'){
            self.ringed = true;
        }
        if upper.contains('B'){
            self.breeding = true;
        }
        if upper.contains('N'){
            self.nonbreeding = true;
        }
        if upper.contains('T'){
            self.nest = true;
        }
        if upper.contains('G'){
            self.eggs = true;
        }
        if upper.contains('C'){
            self.chicks = true;
        }
        if upper.contains('I'){
            self.immature = true;
        }
        if upper.contains('E'){
            self.dead = true;
        }
        if upper.contains('M'){
            self.male = true;
        }
        if upper.contains('F'){
            self.female = true;
        }
        if upper.contains('A'){
            self.adult = true;
        }
        if upper.contains('P'){
            self.photo = true;
        }


        Ok(self)
    }
        
        
        
    // Function to allocate bird, date and places
    pub fn do_places(mut self, arg: &str, birds: &BTreeMap<String, Species>) -> Result<Sightings, String> {

        let mut terms_str = arg.to_string();
        terms_str.retain(|c| !r#"""#.contains(c));
        let temp = terms_str.split("#");
        let vec: Vec<&str> = temp.collect();
        
        for line in vec {
            let l1 = line.split("=");
            let vec1: Vec<&str> = l1.collect();

            match vec1.len() {
                2 => {
                    let first = vec1[0].to_lowercase().chars().nth(0).unwrap();
                    match first {
                        'c' => {
                            let code = vec1[1].to_lowercase();
                            let r_sname = birds.get(&code);
                            if r_sname.is_none(){
                                return Err("Code in string does not exist".to_string())
                            }
                            self.sname = r_sname.unwrap().clone().sname
                        }
                        'd' => {
                            let date = convert_assumed_date(vec1[1]);
                            if date.is_err(){
                                let message = format!("Something wrong in the date string");
                                return Err(message);   
                            }
                            self.date = date.unwrap();
                        }
                        'a' => {
                            self.location = vec1[1].trim().to_string();
                        }
                        'w' => {
                            self.town = vec1[1].trim().to_string();
                        }
                        'p' => {
                            self.province = vec1[1].trim().to_string();
                        }
                        't' => {
                            self.country = vec1[1].trim().to_string();
                        }
                        'o' => {
                            self.comments = vec1[1].trim().to_string();
                        }

                        _ => {
                            return Err("Wrong char given.".to_string())
                        }
                    }
                }
                _ => {
                    // Do nothing
                }
            }
        }

        Ok(self)
    }
        
    // Function to do booleans places and validation
    pub fn do_bool_places_val(mut self, arg: &str, allow_no_booleans: bool,
            birds: &BTreeMap<String, Species>, sbirds: &BTreeMap<String, Species>) -> Result<Sightings, String> {
        
        let res1 = self.do_booleans(arg, allow_no_booleans);
        if res1.is_err(){
            return Err(res1.err().unwrap())
        }

        let res2 = res1.unwrap().do_places(arg, birds);
        if res2.is_err(){
            return Err(res2.err().unwrap())
        }

        let ret = res2.unwrap().validate(sbirds);
        if ret.is_err(){
            return Err(ret.err().unwrap())
        }
        self = ret.unwrap();

        // return Err("".to_string())
        Ok(self)
    }
        
        

    // Function to do validation on self
    pub fn validate(mut self, sbirds: &BTreeMap<String, Species>) -> Result<Sightings, String> {
        
        //Lets start with sname                                                                 -- sname
        let r_sname = sbirds.get(&self.sname);
        if r_sname.is_none(){
            return Err("Validation error: sname".to_string())
        }

        if self.date == 0 {
            return Err("Validation error: no date given".to_string())
        }

        // Lets do the Town                                                                     -- Town
        let mut i_town = title_case(self.town.trim());
        if i_town.len() == 0 {
            return Err("Error in giving no town".to_string());
        }
        if i_town.len() > 39 {
            i_town = i_town[0..40].to_string();
        }
        self.town = i_town;


        // Lets do the Province                                                                 -- Province
        let mut i_province = title_case(self.province.trim());
        if i_province.len() == 0 {
            return Err("Error in giving no province/state".to_string());
        }
        if i_province.len() > 39 {
            i_province = i_province[0..40].to_string();
        }
        self.province = i_province; 


        // Lets do the Country                                                                  -- Country
        let mut i_country = self.country.trim().to_string();
        if i_country.len() == 0 {
            return Err("Error in giving no country".to_string());
        }
        if i_country.len() > 39 {
            i_country = i_country[0..40].to_string();
        }
        self.country = i_country; 


        // Lets do the Comments                                                                  -- Comments
        let mut i_comments = self.comments.trim();
        if i_comments.len() > 120 {
            i_comments = &i_comments[0..120];
        }
        self.comments = i_comments.to_string(); 
    

        // Has anything been observed
        let mut bool_counter = 0;
        if self.seen {bool_counter += 1;}
        if self.heard {bool_counter += 1;}
        if self.ringed {bool_counter += 1;}
        if self.dead {bool_counter += 1;}
        if self.photo {bool_counter += 1;}
        if self.male {bool_counter += 1;}
        if self.female {bool_counter += 1;}
        if self.adult {bool_counter += 1;}
        if self.immature {bool_counter += 1;}
        if self.breeding {bool_counter += 1;}
        if self.eggs {bool_counter += 1;}
        if self.nonbreeding {bool_counter += 1;}
        if self.chicks {bool_counter += 1;}
        if self.nest {bool_counter += 1;}
        
        if bool_counter == 0 {
            return Err("Nothing has beed observed.".to_string())
        }

        Ok(self)
    }
















        
        
        
        
        
    } //end of impl Sightings
    
    


// Function to convert a date text string into unix time
pub fn convert_date_text(text: &str) -> Result<i64, String> {
                                                                      
    let t_date = text.trim();  
    let mut fmt: &str = ".";
    if t_date.substring(4, 5) == fmt {
        fmt = "%Y.%m.%d"
    } else {
        fmt = "%Y-%m-%d"
    }                                            
    let p_date = NaiveDate::parse_from_str(t_date, fmt); 
    if p_date.is_err(){
        return Err("Problem parsing date text".to_string())
    }
    let time_only = NaiveTime::from_hms(0, 0, 0); 
    let date_time: NaiveDateTime;
    // let i_date: i64;
    
    date_time = p_date.unwrap().and_time(time_only);
    let i_date = date_time.timestamp();

    Ok(i_date)
}



// Function to show the sighting
pub fn show_sightings_number<'a>(wn: WhatNumber, options: &'a mut SettingsText,
                sbirds: &BTreeMap<String, Species>,sightings: &'a Vec<Sightings> ) ->  &'a mut SettingsText {
  
    let total_records = sightings.len() as usize;
    let z_index = wn.number.unwrap();
    if total_records  < z_index {
        let message = "Bird observation number is out of range from the sightings database".to_string();
        feedback(Feedback::Error, message);
        exit(17);
    }
    
    let mut rec_num: usize = 0;
    let mut o_sight: Option<Sightings> = None; 
    //lets find that record
    for v in sightings { 
        if rec_num == z_index {
            o_sight = Some(v.clone());
            break;
        }  
        rec_num += 1;
    }
    
    // We have to find that record since   total_records  <   z_index 
    let sight = o_sight.unwrap();

    // get the species
    let res =  sbirds.get(&sight.sname);
    if res.is_none(){
        let message = format!("Bird Scientific name associated with record number {} does not exist in species database", rec_num);
        feedback(Feedback::Error, message);
        exit(17);
    }

    // lets show the species
    let options = bird_sightings_box::show_sighting( 
                                            options, 
                                    z_index,
                                    res.unwrap().clone(),
                                    sight
                                    );                                

    options.set_value_for_key("lastSightingViewed", z_index.to_string()).expect("Option File Problems");

    options
}


// Function to return the number of times the species has been observed in an array of indices
// The order is latest first.
pub fn get_array_of_sname_indices_from_records(sname: &str, sightings: &Vec<Sightings>) -> Vec<usize> {
    let mut count = 0;
    let mut array: Vec<usize> = Vec::new();
    
    for bird in sightings {
        if sname == bird.sname {
            array.insert(0, count);
        }
        count += 1
    }
    
    return array
}


// Function to display all records that have had their sname changed
pub fn display_sname_changes(z_index: usize, con: CodeOrNumber, options: &mut SettingsText,  sight: Sightings) {
    
    
    // let sep = &options.get_date_separator();
    let my_normal_gray: color::Rgb = options.get_color("myNormalGray");
    
    let date  = sight.display_date();
    let num: i32 = z_index as i32 + 1;
    // let rec_num = justify(num.separate_with_spaces(), N_LEN, Justify::Right); 
    let rec_num = justify(num.to_string(), N_LEN, Justify::Right); 
    
    let old_name = con.clone().species.unwrap().name;
    let old_name_text = justify(old_name, NAME_39, Justify::Left); 
    
    let old_sname = con.species.unwrap().sname;
    let old_sname_text = justify(old_sname, NAME_39, Justify::Left); 
    
    print!("{}{}",color::Fg(my_normal_gray),rec_num);
    print!("   {} {}",old_name_text, date);
    print!("   {}{}",old_sname_text, "\n");
}

// Function to add a sighting (if sucessful)
pub fn add_sighting<'a>(last: Vec<String>, arg: &str, birds: &BTreeMap<String, Species>, sbirds: &BTreeMap<String, Species>, 
                        sightings: &'a mut Vec<Sightings>) -> Result<usize, String> {

    let scut_sight = get_shortcut(last.clone(), arg, &sightings);
    if scut_sight.is_err(){
        return Err(scut_sight.clone().err().unwrap())
    }
    let mut to_be_added = Sightings::new();
    if scut_sight.clone().unwrap().is_some(){
        to_be_added = scut_sight.unwrap().unwrap();
    }


    let result1 = to_be_added.do_bool_places_val(arg, false, birds, sbirds);
    if result1.is_err(){
        return Err(result1.err().unwrap())
    }
    to_be_added = result1.unwrap();


    let to_find = to_be_added.clone();
    sightings.push(to_be_added);
    sightings.sort();
    

    let r_pos = find_index_of_sighting(to_find, sightings.to_vec());
    if r_pos.is_err(){
        return Err(r_pos.err().unwrap());
    }

    Ok(r_pos.unwrap())

}


// // Function to validate a sighting
// pub fn validate_sighting<'a>(sbirds: &BTreeMap<String, Species>, sighting: &'a mut Sightings) -> Result<&'a mut Sightings, String> {

//     //Lets start with sname
//     let r_sname = sbirds.get(&sighting.sname);
//     if r_sname.is_none(){
//         return Err("Validation error: sname".to_string())
//     }

//     if sighting.date == 0 {
//         return Err("Validation error: no date given".to_string())
//     }

//     // Lets do the Town                                                                     -- Town
//     let mut i_town = title_case(sighting.town.trim());
//     if i_town.len() == 0 {
//         return Err("Error in giving no town".to_string());
//     }
//     if i_town.len() > 39 {
//         i_town = i_town[0..40].to_string();
//     }
//     sighting.town = i_town;


//     // Lets do the Province                                                                 -- Province
//     let mut i_province = title_case(sighting.province.trim());
//     if i_province.len() == 0 {
//         return Err("Error in giving no province/state".to_string());
//     }
//     if i_province.len() > 39 {
//         i_province = i_province[0..40].to_string();
//     }
//     sighting.province = i_province; 


//     // Lets do the Country                                                                  -- Country
//     let mut i_country = title_case(sighting.country.trim());
//     if i_country.len() == 0 {
//         return Err("Error in giving no country".to_string());
//     }
//     if i_country.len() > 39 {
//         i_country = i_country[0..40].to_string();
//     }
//     sighting.country = i_country; 


//     // Lets do the Comments                                                                  -- Country
//     let mut i_comments = sighting.comments.trim();
//     if i_comments.len() > 120 {
//         i_comments = &i_comments[0..120];
//     }
//     sighting.comments = i_comments.to_string(); 


//     // Has anything been observed
//     let mut bool_counter = 0;
//     if sighting.seen {bool_counter += 1;}
//     if sighting.heard {bool_counter += 1;}
//     if sighting.ringed {bool_counter += 1;}
//     if sighting.dead {bool_counter += 1;}
//     if sighting.photo {bool_counter += 1;}
//     if sighting.male {bool_counter += 1;}
//     if sighting.female {bool_counter += 1;}
//     if sighting.adult {bool_counter += 1;}
//     if sighting.immature {bool_counter += 1;}
//     if sighting.breeding {bool_counter += 1;}
//     if sighting.eggs {bool_counter += 1;}
//     if sighting.nonbreeding {bool_counter += 1;}
//     if sighting.chicks {bool_counter += 1;}
//     if sighting.nest {bool_counter += 1;}
    
//     if bool_counter == 0 {
//         return Err("Nothing has beed observed.".to_string())
//     }

//     Ok(sighting)
// }



// Function to find position (index) of sighting in vector
pub fn find_index_of_sighting(sight: Sightings, sightings: Vec<Sightings>) -> Result<usize, String> {
    
    let mut counter: usize = 0;
    for ss in sightings {
        if ss == sight {
            return Ok(counter)
        }
        counter += 1;
    }
    
    return Err("Added sighting not found in database".to_string())
}


// Function to edit a sighting -- shortcuts should not be used
pub fn edit_sighting(arg: &str, wn: WhatNumber, birds: &BTreeMap<String,Species> , 
                    sbirds: &BTreeMap<String,Species> , sightings: &mut Vec<Sightings> ) -> Result<usize, String> {

    let non_zero = wn.number.unwrap();
    let mut to_edit = sightings.get(non_zero).unwrap().clone();

    // // Do booleans ONLY if some booleans were mentioned
    // let r_bool = to_edit.do_booleans(arg, true);
    // if r_bool.is_err(){
    //     return Err(r_bool.err().unwrap())
    // }
    // to_edit = r_bool.unwrap();
    
    // // do places
    // let r_places = to_edit.do_places(arg, &birds);
    // if r_places.is_err(){
    //     return Err(r_places.err().unwrap())
    // }
    // to_edit = r_places.unwrap();

    // // validate
    // let result= validate_sighting( sbirds, &mut to_edit);
    // if result.is_err(){
    //     return Err(result.err().unwrap());
    // }


    let result1 = to_edit.do_bool_places_val(arg, true, birds, sbirds);
    if result1.is_err(){
        return Err(result1.err().unwrap())
    }
    to_edit = result1.unwrap();


    // sighting is ok , delete original
    sightings.remove(wn.number.unwrap());
    let to_find = to_edit.clone();
    sightings.push(to_edit);
    sightings.sort();
    
    let r_pos = find_index_of_sighting(to_find, sightings.to_vec());
    if r_pos.is_err(){
        return Err(r_pos.err().unwrap());
    }
    
    Ok(r_pos.unwrap())
}


// Function to delete a sighting
pub fn delete_sighting(wn: WhatNumber, options: &mut SettingsText , 
                       sbirds: &BTreeMap<String,Species> , sightings: &mut Vec<Sightings> ) {

    //Show the sighting first
    show_sightings_number(wn.clone(), options, &sbirds, &sightings);

    sightings.remove(wn.number.unwrap().clone());
    let sight_number = wn.number.unwrap() + 1;
    let message = format!("The bird sighting:  {}{}{} {} with the code:  {}{}{} {} in position  {}{}{} {} was deleted.\n",
                    color::Fg(color::Yellow), style::Italic ,&wn.clone().species.unwrap().name, style::Reset,
                    color::Fg(color::Yellow), style::Italic ,&wn.species.unwrap().code, style::Reset,
                    color::Fg(color::Yellow), style::Italic ,&sight_number, style::Reset);
    feedback(Feedback::Info, message);

}









//@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ Errors @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
//@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@        @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

// This was coded to reduce the number of lines in the main.rs
// This should make readability a bit better
pub fn if_sightings_length_is_zero(sightings: &Vec<Sightings>){
    
    if sightings.len() == 0 {
        let message = format!("You have no records in the sightings database.");
        feedback(Feedback::Warning, message);
        exit(17);
    }
}

// Error message for invalid argument (number)
pub fn error_no_legit_number(){
    let mut assembled = String::new();
    
    let message = format!("No legitimate {}{}{} was given. (Anything from 1 to number of sightings is valid.) ",
    color::Fg(color::Yellow), "number", style::Reset);       
    assembled.push_str(&message);
    
    feedback(Feedback::Warning, assembled);
}

// Function - error message for giving the last argument
pub fn error_last_sight_arg_edit(name: String) {
    let mut assembled = String::new();
    
    let message = format!("The last argument editing record number {}{}{} was missing.",
    color::Fg(color::Yellow), name, style::Reset);       
    assembled.push_str(&message);
    
    feedback(Feedback::Warning, assembled);
}





















#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::copy;
    // use substring::Substring;
    use std::fs::remove_file;

    
    #[ignore]
    #[test]
    fn t001_convert_date() {
        let date_string = "2017-04-09";
        let date_only = NaiveDate::parse_from_str(date_string, "%Y-%m-%d");
        let time_only = NaiveTime::from_hms(0, 0, 0); 
        let date_time: NaiveDateTime;
        let mut secs: i64 = 0;
        if date_only.is_ok(){
            date_time = date_only.unwrap().and_time(time_only);
            secs = date_time.timestamp();
        }

        assert_eq!(secs,1491696000);
    }


    #[ignore]
    #[test]
    fn t002_convert_date2() {
        let d = UNIX_EPOCH + Duration::from_secs(1491696000);
        let datetime = DateTime::<Utc>::from(d);
        let timestamp_str = datetime.format("%Y-%m-%d" ).to_string();
        let date_string = "2017-04-09";

        assert_eq!(date_string,timestamp_str);
    }


    #[ignore]
    #[test]
    fn t003_build_sighting() {
        let source = "./test/store/species/birds.bin";
        let destination = "./test/birds.bin";
        copy(source,destination).expect("Failed to copy");
        let birds = Species::load(destination).unwrap();
        remove_file(destination).expect("Cleanup test failed");

        let location = "Gomers backYard";
        let town = "bedFord";
        let province = "nOva sCotia";
        let code = "swbl";
        let country = "cAnAdA";
        let date = "2022-01-01";
        let seen = true;
        let heard = false;
        let ringed = false;
        let dead = false;
        let photo = false;
        let male = true;
        let female = true;
        let adult = true;
        let immature = false;
        let eggs = false;
        let breeding = false;
        let nonbreeding = false;
        let nest = false;
        let chicks = false;
        let comments = "This is the first comment, and I'm hoping to get more maybe. We shall see what happens.".to_string();

        let sighting = Sightings::build_sighting(&birds, 
            code, date, location, town, province, country, 
            seen, heard, ringed, dead, photo, male, female, adult, immature, breeding, eggs, nonbreeding, nest, chicks, comments);

        assert_eq!(sighting.unwrap().sname,"Cygnus atratus");
    }
    
    
    #[ignore]
    #[test]
    fn t004_build_sighting() {
        let source = "./test/store/sightings/sights2.json";
        let destination = "./test/sights.json";
        copy(source,destination).expect("Failed to copy");
        let mut sights = Sightings::import(destination).unwrap();
        remove_file(destination).expect("Cleanup test failed");
        
        let source = "./test/store/species/birds.bin";
        let destination = "./test/birds.bin";
        copy(source,destination).expect("Failed to copy");
        let birds = Species::load(destination).unwrap();
        remove_file(destination).expect("Cleanup test failed");
        
        let location = "Fluffsters backYard";
        let town = "bedFord";
        let province = "nOva sCotia";
        let code = "jabl1";
        let country = "cAnAdA";
        let date = "2022-01-03";
        let seen = false;
        let heard = true;
        let ringed = false;
        let dead = false;
        let photo = false;
        let male = false;
        let female = false;
        let adult = false;
        let immature = true;
        let eggs = false;
        let breeding = false;
        let nonbreeding = false;
        let nest = true;
        let chicks = false;
        let comments = "This is the second comment, and I'm hoping to get more maybe. We shall see what happens.".to_string();
        
        let sighting = Sightings::build_sighting(&birds, 
            code, date, location, town, province, country, 
            seen, heard, ringed, dead, photo, male, female, adult, immature, breeding, eggs, nonbreeding, nest, chicks, comments);
            
        sights.push(sighting.unwrap());
        let _rrr = Sightings::export("./test/sights.json", sights);
        let fff = Sightings::import("./test/sights.json");
        remove_file("./test/sights.json").expect("Cleanup test failed");
        
        assert_eq!(fff.unwrap().len(),3);
    }
        
        

    #[ignore]
    #[test]
    fn t005_save_and_load() {
        let source = "./test/store/sightings/sights2.json";
        let destination = "./test/sights.json";
        copy(source,destination).expect("Failed to copy");
        let sights = Sightings::import(destination).unwrap();
        remove_file(destination).expect("Cleanup test failed");
        
        let destination = "./test/sights.bin";
        let res = Sightings::save(destination, &sights);
        let mut s2: Vec<Sightings> = Vec::new();
        if res.is_ok(){
            s2 = Sightings::load(destination).unwrap();
            remove_file(destination).expect("Cleanup test failed");
        }

        assert_eq!(s2[1].nest,true);
    }


    #[ignore]
    #[test]
    fn t006_build_sighting() {
        let source = "./test/store/species/birds.bin";
        let destination = "./test/birds.bin";
        copy(source,destination).expect("Failed to copy");
        let birds = Species::load(destination).unwrap();
        remove_file(destination).expect("Cleanup test failed");

        let location = "Gomers backYard";
        let town = "bedFord";
        let province = "nOva sCotia";
        let code = "swbl";
        let country = "cAnAdA";
        let date = "2022-01-01";
        let seen = false;
        let heard = false;
        let ringed = false;
        let dead = false;
        let photo = false;
        let male = false;
        let female = false;
        let adult = false;
        let immature = false;
        let eggs = false;
        let breeding = false;
        let nonbreeding = false;
        let nest = false;
        let chicks = false;
        let comments = "This is the first comment, and I'm hoping to get more maybe. We shall see what happens.".to_string();

        let sighting = Sightings::build_sighting(&birds, 
            code, date, location, town, province, country, 
            seen, heard, ringed, dead, photo, male, female, adult, immature, breeding, eggs, nonbreeding, nest, chicks, comments);

        assert_eq!(sighting.is_err(),true);
    }


    #[ignore]
    #[test]
    fn t007_save_and_load2() {
        let source = "./test/store/sightings/to_use.json";
        let destination = "./test/sights.json";
        copy(source,destination).expect("Failed to copy");
        let sights = Sightings::import(destination).unwrap();
        remove_file(destination).expect("Cleanup test failed");
        
        let destination = "./test/sightings.bin";
        let res = Sightings::save(destination, &sights);
        let mut s2: Vec<Sightings> = Vec::new();
        if res.is_ok(){
            s2 = Sightings::load(destination).unwrap();
            remove_file(destination).expect("Cleanup test failed");
        }

        assert_eq!(s2[1].eggs,true);
    }


    #[ignore]
    #[test]
    fn t008_get_number_observations1() {
        let source = "./test/store/sightings/sightsSome.json";
        let destination = "./test/sights.json";
        copy(source,destination).expect("Failed to copy");
        let mut sightings = Sightings::import(destination).unwrap();
        remove_file(destination).expect("Cleanup test failed");
        
        let obs = get_array_of_sname_indices_from_records("Anas undulata", &mut sightings);

        assert_eq!(obs.len(),1);
    }

    

    #[ignore]
    #[test]
    fn t009_display_sname_changes1() {
        let source = "./test/store/sightings/sightsSome.json";
        let destination = "./test/sights.json";
        copy(source,destination).expect("Failed to copy");
        let mut sightings = Sightings::import(destination).unwrap();
        remove_file(destination).expect("Cleanup test failed");
        
        let obs = get_array_of_sname_indices_from_records("Cyanocitta cristata", &mut sightings);

        assert_eq!(obs.len(),5);
    }


    #[ignore]
    #[test]
    fn t010_display_date() {
        let source = "./test/store/sightings/sightsSome.json";
        let destination = "./test/sights.json";
        copy(source,destination).expect("Failed to copy");
        let sightings = Sightings::import(destination).unwrap();
        remove_file(destination).expect("Cleanup test failed");
        
        let obs = sightings.get(0).unwrap();
        let date = obs.display_date();

        assert_eq!(date,"2001.09.01");
    }
    
    
    #[ignore]
    #[test]
    fn t011_convert_date_text1() {
        let text = "2001.09.01";        
        let ans = convert_date_text(text);

        assert_eq!(ans.unwrap(),999302400);
    }


    #[ignore]
    #[test]
    fn t012_add_a_sighting() {
        // let mut options = SettingsText::new("");

        let source = "./test/store/sightings/sightings.bin";
        let destination = "./test/sights.bin";
        copy(source,destination).expect("Failed to copy");
        let mut sightings = Sightings::load(destination).unwrap();
        remove_file(destination).expect("Cleanup test failed");

        let source = "./test/store/species/birds.bin";
        let destination = "./test/birds.bin";
        copy(source,destination).expect("Failed to copy");
        let birds = Species::load(destination).unwrap();
        let sbirds = make_sname_btree(&birds);
        remove_file(destination).expect("Cleanup test failed");

        let last_10 = get_last_10(&sightings);
        let arg = "0SHG#d=2001.09.02#c=jabl";

        let result = add_sighting(last_10, arg, &birds ,&sbirds, &mut sightings);
        let pos = result.unwrap();

        assert_eq!(pos,9);
    }
    
    
    #[ignore]
    #[test]
    fn t013_validate1() {
        // let mut options = SettingsText::new("");

        let source = "./test/store/sightings/sightings.bin";
        let destination = "./test/sights.bin";
        copy(source,destination).expect("Failed to copy");
        let sightings = Sightings::load(destination).unwrap();
        remove_file(destination).expect("Cleanup test failed");

        let source = "./test/store/species/birds.bin";
        let destination = "./test/birds.bin";
        copy(source,destination).expect("Failed to copy");
        let birds = Species::load(destination).unwrap();
        let sbirds = make_sname_btree(&birds);
        remove_file(destination).expect("Cleanup test failed");

        let mut si = sightings.get(7).unwrap().clone();
        si.seen = false;
        si.adult = false;

        let temp = si.clone();
        let res = temp.validate(&sbirds);
        assert_eq!(res.is_err(),true);

        si.photo = true;
        si.comments.push_str("hello this is Joannie, I'm sorry but I'm not home");
        si.comments.push_str("hello this is Joannie, I'm sorry but I'm not home");
        si.comments.push_str("If you leave me your name and number 0123456789 0123456789 01234567890");
        println!("Len is {}",si.comments.len());
        
        // let res2 = validate_sighting(&sbirds, &mut si);
        si = si.validate(&sbirds).unwrap();

        si.town = "benoni".to_string();
        let res3 = si.validate(&sbirds);
        if res3.is_ok() {
            assert_eq!(res3.unwrap().town ,"Benoni");
        }
    }


    #[ignore]
    #[test]
    fn t014_edit_a_sighting() {
        // let mut options = SettingsText::new("");

        let source = "./test/store/sightings/sightings.bin";
        let destination = "./test/sights.bin";
        copy(source,destination).expect("Failed to copy");
        let mut sightings = Sightings::load(destination).unwrap();
        remove_file(destination).expect("Cleanup test failed");

        let source = "./test/store/species/birds.bin";
        let destination = "./test/birds.bin";
        copy(source,destination).expect("Failed to copy");
        let birds = Species::load(destination).unwrap();
        let sbirds = make_sname_btree(&birds);
        remove_file(destination).expect("Cleanup test failed");

        let mut wn = WhatNumber::new();
        let to_edit = sightings.get(0).unwrap().clone();
        wn.is_number = true;
        wn.number = Some(1);
        wn.sighting = Some(to_edit);
        let arg = "SHG#d=2001.09.02#c=jabl#a=Around";

        let result = edit_sighting(arg, wn, &birds , &sbirds, &mut sightings);
        let pos = result.unwrap();

        assert_eq!(pos,8);
    }
    
    
    #[ignore]
    #[test]
    fn t015_do_booleans() {
        // let mut options = SettingsText::new("");

        let source = "./test/store/sightings/sightings.bin";
        let destination = "./test/sights.bin";
        copy(source,destination).expect("Failed to copy");
        let sightings = Sightings::load(destination).unwrap();
        remove_file(destination).expect("Cleanup test failed");

        // let source = "./test/store/species/birds.bin";
        // let destination = "./test/birds.bin";
        // copy(source,destination).expect("Failed to copy");
        // let birds = Species::load(destination).unwrap();
        // let sbirds = make_sname_btree(&birds);
        // remove_file(destination).expect("Cleanup test failed");

        let s1 = sightings.get(1).unwrap().clone();
        let arg = "fart#a=around#d=2000.01.01";
        let result = s1.clone().do_booleans(arg, false);
        if result.is_ok(){
            let ans = result.unwrap();
            assert_eq!(ans.nest,true);
            assert_eq!(ans.date,999302400);
        }
        
        let arg = "fart#a=around#prm#d=2000.01.01";
        let result2 = s1.do_booleans(arg, false);
        assert_eq!(result2.is_ok(),false);
        
    }
    
    
    #[ignore]
    #[test]
    fn t016_do_places() {
        let source = "./test/store/sightings/sightings.bin";
        let destination = "./test/sights.bin";
        copy(source,destination).expect("Failed to copy");
        let sightings = Sightings::load(destination).unwrap();
        remove_file(destination).expect("Cleanup test failed");
        
        let source = "./test/store/species/birds.bin";
        let destination = "./test/birds.bin";
        copy(source,destination).expect("Failed to copy");
        let birds = Species::load(destination).unwrap();
        // let sbirds = make_sname_btree(&birds);
        remove_file(destination).expect("Cleanup test failed");
        
        let mut s1 = sightings.get(1).unwrap().clone();
        let arg = "fart#a=around#d=2000.01.01";
        s1 = s1.do_booleans(arg, false).unwrap();
        s1 = s1.do_places(arg, &birds).unwrap();
        
        assert_eq!(s1.breeding,false);
        assert_eq!(s1.location,"around");

    }


    #[ignore]
    #[test]
    fn t017_do_edit() {
        // let mut options = SettingsText::new("");

        let source = "./test/store/sightings/sightings.bin";
        let destination = "./test/sights.bin";
        copy(source,destination).expect("Failed to copy");
        let mut sightings = Sightings::load(destination).unwrap();
        remove_file(destination).expect("Cleanup test failed");
        
        let source = "./test/store/species/birds.bin";
        let destination = "./test/birds.bin";
        copy(source,destination).expect("Failed to copy");
        let birds = Species::load(destination).unwrap();
        let sbirds = make_sname_btree(&birds);
        remove_file(destination).expect("Cleanup test failed");
        
        let sub = "2";
        let yes = what_number(sub, &sbirds, &sightings);

        let arg = "fart#a=around#d=2000.01.01";
        let result = edit_sighting(arg, yes, &birds, &sbirds, &mut sightings);

        assert_eq!(result.unwrap(),0);
    }

    #[ignore]
    #[test]
    fn t018_add_sighting() {
        // let mut options = SettingsText::new("");

        let source = "./test/store/sightings/sightings.bin";
        let destination = "./test/sights.bin";
        copy(source,destination).expect("Failed to copy");
        let mut sightings = Sightings::load(destination).unwrap();
        remove_file(destination).expect("Cleanup test failed");
        
        let source = "./test/store/species/birds.bin";
        let destination = "./test/birds.bin";
        copy(source,destination).expect("Failed to copy");
        let birds = Species::load(destination).unwrap();
        let sbirds = make_sname_btree(&birds);
        remove_file(destination).expect("Cleanup test failed");
        
        // Lets try addition
        let mut sub = "0fart#c=spho#d=2022.04.11#a=Sven's Office";
        let mut last_10 = get_last_10(&sightings);
        let mut result = add_sighting(last_10, &sub, 
                                        &birds, &sbirds, &mut sightings);
        assert_eq!(result.unwrap(),16037);
        
        sub = "fart#c=spho#d=2022.04.11#a=Sven's Office";
        last_10 = get_last_10(&sightings);
        result = add_sighting(last_10, &sub, 
                                &birds, &sbirds, &mut sightings);
            
        assert_eq!(result.is_err(),true);
    }


    #[ignore]
    #[test]
    fn t019_edit_sighting() {
        // let mut options = SettingsText::new("");

        let source = "./test/store/sightings/sightings.bin";
        let destination = "./test/sights.bin";
        copy(source,destination).expect("Failed to copy");
        let mut sightings = Sightings::load(destination).unwrap();
        remove_file(destination).expect("Cleanup test failed");
        
        let source = "./test/store/species/birds.bin";
        let destination = "./test/birds.bin";
        copy(source,destination).expect("Failed to copy");
        let birds = Species::load(destination).unwrap();
        let sbirds = make_sname_btree(&birds);
        remove_file(destination).expect("Cleanup test failed");
        
        let arg1 = "1";
        let arg2 = "mfapr#d=2022.04.11";
        let old = what_number(&arg1, &sbirds, &sightings);
        let result = edit_sighting(arg2, old, &birds, &sbirds, &mut sightings);
        if result.is_ok(){
            let ans = result.unwrap();
            assert_eq!(ans,16036);
        }
    }
















} // end of all tests








