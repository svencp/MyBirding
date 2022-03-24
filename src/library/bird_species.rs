/*
        This is the class for all Bird species

        2022.01.06   Sven Ponelat


*/

use crate::library::my_file_funcs::*;
use crate::library::bird_species_support::*;
use crate::library::bird_species_support::CodeOrNumber;
use crate::library::bird_species_box::*;
use crate::library::bird_sightings::*;
use std::path::Path;
use std::fs::{ OpenOptions };
use serde::{Serialize, Deserialize};
use substring::Substring;
use thousands::Separable;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::process::exit;
use inflections::{Inflect};
use std::cmp;
use std::fs::File;
use super::settings::{SettingsText};
use termion::{color, style};
use std::io::{BufReader, BufWriter, Write};



pub const SPECIES_BIN_FILENAME: &str = "./species.bin";
pub const CODE_LEN:  usize   = 10;
pub const POS_LEN:   usize   =  9;

#[allow(non_snake_case)]
#[derive(Clone, Debug,  Serialize, Deserialize, derivative::Derivative)]
#[derivative(Default)]
pub struct Species {
    pub sname: String,
    pub name: String,
    pub fname: String,
    pub code: String,
    pub order: String,
    pub family: String,
    pub status: String,
    pub aname: String,
    pub afname: String,
    pub acode: String,
    pub list: String,
}


impl Species {
    
    // make an empty species for compilers sake
    pub fn new() -> Species {
        Species { sname: "".to_string(), 
                name: "".to_string(), 
                fname: "".to_string(),  
                code: "".to_string(),  
                order: "".to_string(),  
                family: "".to_string(),  
                status: "".to_string(),  
                aname: "".to_string(),  
                afname: "".to_string(),  
                acode: "".to_string(),  
                list: "".to_string(),  
        }
    }

    // convert the bird name to the bird family name
    pub fn make_fname(name: &str) -> String {

        let mut interim = line_to_words(name);
        let count = interim.len();

        let last: String = interim.remove(count -1).to_string();
        let mut result: String = String::new();

        for value in interim.into_iter(){
            let temp = value.trim().to_lowercase();
            result = format!("{} {}", &result.trim(), temp);

        }

        let proper = format!("{} {}",last.to_lowercase(), result.trim().to_lowercase());
        let ret = limit_length(proper, NAME_39);
        return title_case(&ret);
    }
    
    /*  
    This functions creates a code where the first two letters of the family name
    and second word as well as the first letter of all  subsequent words are 
    extracted out. An integer is then appended, to make it unique,
    so as to be used as a key.
    
    Lets see if I can use this code to make the alternative code as well.
    */    
    pub fn make_code(fname: &str, birds: &BTreeMap<String,Species>) -> String {
        let work = fname.to_lowercase();
        let chunks: Vec<_> = work.split_whitespace().collect();
        
        let mut build_code: String = "".to_string();
        let mut count = 1;
        
        for name in chunks {
            match count {
                1..=2   => {
                    build_code.push(name.chars().nth(0).unwrap());
                    build_code.push(name.chars().nth(1).unwrap());
                    count += 1
                }
                _       => {
                    build_code.push(name.chars().nth(0).unwrap());
                    count += 1
                }
            }
        }
        
        // reset counter for finding number of codes that are the same
        count = 1;
        
        let code = build_code.clone();
        let mut word: String;
        
        
        // loop to add the right integer
        loop {
            if birds.contains_key( &build_code){
                // word = next_string(&code,count);
                word = format!("{}{}", &code, count);
                count += 1 ;
                build_code = word;
            }
            else {
                break
            }
        }

        let ret = limit_length(build_code, NAME_39);
        return ret
    }
    
    
    /*  
    This functions creates the alternative code where the first two letters of the 
    alternative family name {afname}and second word as well as the first letter of all  subsequent words are 
    extracted out. An integer is then appended, to make it unique,
    so as to be used as an alternative key.
    */    
    pub fn make_acode(afname: &str, birds: &BTreeMap<String,Species>) -> String {

        // We have to make a new treemap where the key will be the acode
        let mut amap: BTreeMap<String,Species> = BTreeMap::new();

        for (_k, v) in birds {
            if v.acode.len() > 0 {
                let value = v.clone();
                let key    = v.clone()
                                     .acode;

                amap.insert(key, value);
            }
        }

        Species::make_code(afname, &amap)
    }


    // Load all the bird species from binary file into map
    #[warn(unused_must_use)]
    pub fn load(bin_file: &str) -> Result<BTreeMap<String,Species>, String> {
        
        let decoded:  BTreeMap<String,Species>;
                
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
    
    
    pub fn save(bin_file: &str, birds: &BTreeMap<String,Species>) -> Result<(), String> {
        
        // let encoded:  BTreeMap<String,Species>;
        let encoded: Vec<u8> = bincode::serialize(birds).unwrap();
                
        // Lets open the bin file
        let mut file = match OpenOptions::new()
            .read(false)
            .write(true)
            .create(true)
            .truncate(true)
            .open(bin_file){
                Ok(content) => content,
                Err(_) => { return Err("Problem saving species bin file".to_string()); }
        };
        
        match file.write_all(&encoded) {
            Ok(file) => file,
            Err(_) => { return Err("Problem writing to species bin file".to_string()); }
        }

        Ok(())
    }


    // Functions for importing json and csv files. With csv ASSUME HEADER line
    pub fn import_csv(csv_file: &str, birds: &mut BTreeMap<String,Species>) -> Result<(), String> {
        let mut counter = 0;

        let ext = get_extension_from_filename(csv_file);
        if ext.is_none(){
            return Err("Not a valid file extension for importing bird species.".to_string());
        }
        

        let f = File::open(csv_file);
        if f.is_err(){
            return Err("Cannot open csv file for reading".to_string());
        }
        let reader = BufReader::new(f.unwrap());

        for line in reader.lines() {
            if line.is_ok(){
                // Don't read header line
                if counter == 0 {
                    counter += 1;
                    continue;
                }

                let r_line = get_species_from_line(&line.unwrap(), &birds);
                if r_line.is_err(){
                    let message = r_line.err().unwrap();
                    feedback(Feedback::Error, message);
                    let mess1 = format!("Problem converting line, around number {}", counter);
                    return Err(mess1);
                }
                
                let species = r_line.unwrap();
                if birds.insert(species.clone().code, species.clone()).is_some(){
                    return Err("Duplicate keys in species database.".to_string());
                }
                
                let mut assembly = String::new();
                assembly.push_str( &format!("Line: {}", counter));
                assembly.push_str( &format!(" imported {} with an sname of {}", species.name,species.sname));
                feedback(Feedback::Info, assembly);
                
                counter += 1;
            }
        }

        Ok(())
    }
    
    pub fn export(json_file: &str, birds: &BTreeMap<String,Species>) -> Result<(), String> {
        
        let path = Path::new(json_file);
        
        let serialized = serde_json::to_string_pretty(&birds);
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
            Ok(_)   => { Ok(()) } 
        }
    }



    // Functions for importing json and csv files. With csv ASSUME HEADER line
    pub fn import(json_file: &str) -> Result<BTreeMap<String,Species>, String> {
        let str_file  = std::fs::read_to_string(json_file );

        let content = match str_file {
            Ok(content) => { content },
            Err(_) => { return Err("Problem importing species json file".to_string()); }
        };
        
        let map: BTreeMap<String,Species> = match serde_json::from_str(&content){
            Ok(map) => map,
            Err(_) => { return Err("Problem converting species json file".to_string()); }
        };
        
        Ok(map)
    }
    
    // pub fn export(json_file: &str, birds: &BTreeMap<String,Species>) -> Result<(), String> {
        
    //     let path = Path::new(json_file);
        
    //     let serialized = serde_json::to_string_pretty(&birds);
    //     let mut file = match OpenOptions::new()
    //                             .read(false)
    //                             .write(true)
    //                             .create(true)
    //                             .truncate(true)
    //                             .open(path)  {
            
    //         Err(_) => { return Err("Problem exporting species json file".to_string()); }
    //         Ok(file)   => { file }
    //     };
        
    //     match file.write_all(serialized.unwrap().as_bytes()) {
    //         Err(_) => { return Err("Problem writing species json file".to_string()); } 
    //         Ok(_)   => { Ok(()) } 
    //     }
    // }


    pub fn export_csv(csv_file: &str, birds: &BTreeMap<String,Species>) -> Result<(), String> {
        
        let path = Path::new(csv_file);
        let f = match OpenOptions::new()
                                        .read(false)
                                        .write(true)
                                        .create(true)
                                        .truncate(true)
                                        .open(path)  {
            
            Err(_) => { return Err("Problem opening export to csv file".to_string()); }
            Ok(file)   => { file }
        };

        let mut file = BufWriter::new(f);
        
        // Do Header
        let header = "sname\tname\tfname\tcode\torder\tfamily\tstatus\taname\tafname\tacode\tlist\n";
        match file.write_all(header.as_bytes()) {
            Err(_) => { return Err("Problem writing species csv file".to_string()); } 
            _      => { } 
        }
        
        let mut counter = 0;

        for (_k,v) in birds {
            let mut assembly = String::new();

            assembly.push_str(&v.sname);
            assembly.push('\t');
            assembly.push_str(&v.name);
            assembly.push('\t');
            assembly.push_str(&v.fname);
            assembly.push('\t');
            assembly.push_str(&v.code);
            assembly.push('\t');
            assembly.push_str(&v.order);
            assembly.push('\t');
            assembly.push_str(&v.family);
            assembly.push('\t');
            assembly.push_str(&v.status);
            assembly.push('\t');
            assembly.push_str(&v.aname);
            assembly.push('\t');
            assembly.push_str(&v.afname);
            assembly.push('\t');
            assembly.push_str(&v.acode);
            assembly.push('\t');
            assembly.push_str(&v.list);
            assembly.push('\n');
            
            match file.write_all(assembly.as_bytes()) {
                Err(_) => { return Err("Problem writing species csv file".to_string()); } 
                _      => { counter += 1 } 
            }
        }
        let message = format!("The birds species csv file was exported with {} records in it.",counter);
        feedback(Feedback::Info, message);

        return Ok(())
    }
    

    pub fn export_path(extension: &str, options: &mut SettingsText ) -> String {
        let mut assembly = "./".to_string();
        assembly.push_str(&options.date_time_str());
        assembly.push_str("species.");
        assembly.push_str(extension);
        assembly
    }
 
    
    pub fn build_species(
        birds: &BTreeMap<String,Species>,
        sname: String,
        name: String,
        order: String,
        family: String,
        status: String,
        aname: String,
        list: String)    -> Result<Species, String> {            
            
        // Validation time and some processing                                       -- sname
        let mut i_sname = sname.trim().to_lowercase();
        if i_sname.len() < 5 {
            return Err("Not a valid scientific name".to_string());
        } else {
            let result = Species::validate_sname(&i_sname);
            if result.is_err(){
                return Err(result.err().unwrap());
            }
            i_sname = limit_length(result.unwrap(), NAME_39);
        }
        
        // Validation time and some processing                                        -- name
        let t_name =  Species::validate_name(&name.clone());
        if t_name.is_err(){
            return Err(t_name.err().unwrap());
        }                                  
        let i_name: String = t_name.unwrap();
        let i_fname: String;
        let i_code: String;

        if i_name.len() < 3 {
            return Err("Not a valid name".to_string());
        } else {
            i_fname = Species::make_fname(&i_name).to_string();
            i_code = Species::make_code(&i_fname, &birds);
        }


        // Validation time and some processing                                       -- order
        let s_order = order.trim().to_title_case();
        // let i_order: String = order.trim().to_title_case();  
        let i_order = limit_length(s_order, NAME_39);                                 
        if i_order.len() < 3 {
            return Err("Not a valid order".to_string());
        } 


        // Validation time and some processing                                       -- family
        let res_f = Species::validate_family(&family);
        if res_f.is_err(){
            return Err(res_f.err().unwrap());
        }
        // let i_family: String = res_f.unwrap();
        let j_family: String = res_f.unwrap();
        let i_family = limit_length(j_family, FAMILY_59);
        if i_family.len() < 3 {
            return Err("Not a valid family".to_string());
        } 


        // Validation time and some processing                                       -- aname, afname, acode
        let mut i_aname = "".to_string();
        let mut i_afname = "".to_string();
        let mut i_acode = "".to_string();
        if aname.len() > 0 {
            let result  = Species::validate_name(&aname);
            if result.is_err(){
                return Err(result.err().unwrap());
            }
            let t_aname =  result.unwrap();                                  
            // let t_aname: String = t_aname.clone().to_title_case().to_string();

            if t_aname.len() < 3 {
                return Err("Not a valid alternative name".to_string());
            }
            let t_afname = Species::make_fname(&t_aname.clone());
            let j_acode = Species::make_acode(&t_afname.clone(), &birds);
            i_acode = limit_length(j_acode, NAME_39);
            i_afname = limit_length(t_afname, NAME_39);
            i_aname = limit_length(t_aname, NAME_39);

        }


        // Validation time and some processing                                       -- status
        let mut i_status: String = "".to_string();
        if status.len() > 0 {
            let temp = status.trim().to_string();
            // let j_status = title_case(&temp);
            let j_status = temp.to_uppercase();
            i_status = limit_length(j_status, NAME_39);

        } 


        // Validation time and some processing                                       -- list
        let mut i_list: String = "".to_string();
        if list.len() > 0 {
            // i_list = list.trim().to_title_case().to_string();
            let j_list = list.trim().to_title_case().to_string();
            i_list = limit_length(j_list, NAME_39);
        } 

        Ok(Species {
            sname:  i_sname,
            name:   i_name,
            fname:  i_fname,
            code:   i_code,
            order:  i_order,
            family: i_family,
            status: i_status,
            aname:  i_aname,
            afname: i_afname,
            acode:  i_acode,
            list:   i_list
        })
            
    }   // end of build_species


    //Function to validate a species (normally coming in from csv file)
    pub fn validate_species(
        birds: &BTreeMap<String,Species>,
        sname:  String,
        name:   String,
        // fname:  String,
        code:   String,
        order:  String,
        family: String,
        status: String,
        aname:  String,
        // afname: String,
        acode:  String,
        list:   String)     -> Result<Species, String> {

        let mut species: Species;
        let r_species = Species::build_species(birds, sname.clone(), name.clone(), order.clone(), 
                                            family.clone(), status.clone(), aname.clone(), list.clone());
        if r_species.is_err() {
            return Err("Species cannot be built".to_string());
        }
        species = r_species.unwrap();

        // Code
        let ans = is_code_valid(code.clone(), &species);
        if !ans {
            return Err("Code is not valid".to_string());
        }
        species.code = code.clone();
        
        if birds.contains_key(&code){
            return Err("Code is already in the database".to_string());
        }

        // sname
        let mut contains_sname = false;
        let mut contains_acode = false;
        for (_k,v) in birds {
            if sname == v.sname{
                contains_sname = true;
            }
            if acode.len() != 0 {
                if acode == v.acode{
                    contains_acode = true;
                }
            }
        }
        if contains_sname {
            return Err("Sname is already in the database".to_string());
        }
        //acode
        if contains_acode {
            return Err("Alt. code is already in the database".to_string());
        }
        species.acode = acode;

        return Ok(species)
    }







    pub fn validate_sname(input: &str) -> Result<String, String> {
        let interim = line_to_words(input);
        let mut result: String = String::new();

        if interim.len() != 2 {
            let error = format!("Scientific name has wrong number of terms -> {}",interim.len());
            return Err(error.to_string());
        }

        for name in interim.into_iter(){
            let temp = name.trim().to_lowercase();
            result = format!("{} {}", &result, temp);

        }

        Ok(sentence_case(&result.trim()))
    }


    // This function can probably be used for validating the alternative name as well.
    pub fn validate_name(input: &str) -> Result<String, String> {
        let interim = line_to_words(input);
        let mut result: String = String::new();

        if interim.len() > 6 {
            let error = format!("The name seems a bit too long -> {}",interim.len());
            return Err(error.to_string());
        }

        for name in interim.into_iter(){
            let temp = name.trim().to_lowercase();
            result = format!("{} {}", &result, temp);

        }

        Ok(title_case(result.trim()))
    }


    // This function for specialized family case.
    pub fn validate_family(input: &str) -> Result<String, String> {
        let mut low = input.trim().to_lowercase();
        if low.len() > 60 {
            low = low[0..60].to_string();
        }
        let interim = line_to_words(&low);
        let mut result: String = String::new();


        for name in interim.into_iter() {
            match &name[..1] {
                "(" => {
                    let len = name.clone().len();
                    if len > 1 {
                        let temp = name.substring(1, len);
                        let res = format!("({} ",uppercase_first_letter(temp));
                        result.push_str(&res);
                    }
                    else {
                        result.push_str("( ");
                    }
                }

                "a" => {
                    if name.clone() != "and" {
                        let ret = &title_case(&name);
                        let res = format!("{} ",ret);
                        result.push_str(&res);
                    }
                    else {
                        result.push_str("and ");
                    }
                }
                
                _ => {
                    let ret = &title_case(&name);
                    let res = format!("{} ",ret);
                    result.push_str(&res);
                }
            } 
        } 

        Ok(result.trim().to_string())
    } //end of validate_family
    
    
    
    
    
    
    
    
    
}// end of impl Species

























// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@


#[rustfmt::skip]
// A function to show the species at the TreeMap index
pub fn show_bird_number<'a>(z_index: usize, options: &'a mut SettingsText, birds: &'a mut BTreeMap<String, Species>,
                        sightings: &'a Vec<Sightings> ) ->  &'a mut SettingsText {
       
    let k_species = get_species_from_index(z_index, birds).unwrap();   
    let observations = get_array_of_sname_indices_from_records(&k_species.1.sname, sightings);
    
    // lets show the species
    *options = show_species( observations.len(),
                              z_index,
                                         &options, 
                                    &k_species.1,
                                         );
    
    options.set_value_for_key("lastSpeciesViewed", z_index.to_string()).expect("Option File Problems");  
    options
}
    
    
// // A function to get species with just the scientific name
// pub fn get_sname_value(sname: &str, birds:  &BTreeMap<String, Species>) -> Result<Species, String> {

//     let mut species = None; 
//     //lets find that bird
//     for (_k, v) in birds { 
//         if sname.to_lowercase() == v.sname.to_lowercase() {
//             // species = Some(v.clone());
//             species = Some(v);
//             break;
//         }
//     }

//     if species.is_some(){
//         let ans = species.unwrap().clone();
//         return Ok(ans)
//     }
//     return Err("The scientific name given does not exist in species database".to_string())
// }


// A function to get species with just the scientific name
pub fn get_sname_as_key_and_return_value(sname: &str, sbirds:  &BTreeMap<String, Species>) -> Result<Species, String> {

    let r_species = sbirds.get(sname);
    if r_species.is_some(){
        let species = r_species.unwrap().clone();
        return Ok(species);
    }
    return Err("The scientific name given does not exist in species database".to_string())
}


// Deconstruct the given string so that a species may be built
// Incoming field name , outgoing the value of the field
// pub fn deconstruct_bird_str(field: String, mut arg: String, ) -> Result<String, String> {
pub fn deconstruct_bird_str(ch: char, arg: &str, ) -> Result<String, String> {
    let ret: String;
    let mut str = arg.to_string();
    str.retain(|c| !r#"""#.contains(c));

    let split = str.split("#");
    let vec: Vec<&str>   = split.collect();

    for i in vec {
        let i_temp = i.split("=");
        let indi: Vec<&str> = i_temp.collect();
        if indi.len() == 2 {
            let first = indi[0].to_lowercase().chars().nth(0).unwrap();
            if first == ch {
                ret = indi[1].to_string();
                return Ok(ret);
            }
        } else  {
            return Err("There is a malformed part of the string".to_string());
        }
    }

    return Err("Wanted term is not in string".to_string());
}


// Function to prepare the species to be built and return result
pub fn prepare_species_adding(arg: String, sbirds: &BTreeMap<String,Species>, 
                        birds: &BTreeMap<String,Species>) -> Result<Species, String> {
    
    let mut needed = 0;
    let name: String;
    let sname: String;
    let family: String;
    let order: String;
    let status: String;
    let list: String;
    let aname: String;


    let r_name = deconstruct_bird_str('n', &arg);
    if r_name.is_ok() {
        needed += 1;
        name = limit_length(r_name.unwrap(),NAME_39);
    } else {
        return Err("The needed name of the bird is required (either not given or malformed)".to_string());
    }
    
    let r_sname = deconstruct_bird_str('s', &arg);
    if r_sname.is_ok() {
        needed += 1;
        sname = limit_length(r_sname.unwrap(),NAME_39);
    } else {
        return Err("The needed scientific name of the bird is required (either not given or malformed)".to_string());
    }
    
    let r_family = deconstruct_bird_str('m', &arg);
    if r_family.is_ok() {
        needed += 1;
        family = limit_length(r_family.unwrap(),FAMILY_59);
    } else {
        return Err("The needed family of the bird is required (either not given or malformed)".to_string());
    }
    
    let r_order = deconstruct_bird_str('r', &arg);
    if r_order.is_ok() {
        needed += 1;
        order = limit_length(r_order.unwrap(),NAME_39);
    } else {
        return Err("The needed order of the bird is required (either not given or malformed)".to_string());
    }
    
    let r_status = deconstruct_bird_str('u', &arg);
    if r_status.is_ok() {
        status = limit_length(r_status.unwrap(),NAME_39);
    } else {
        status = "".to_string();
    }
    
    let r_list = deconstruct_bird_str('l', &arg);
    if r_list.is_ok() {
        list = limit_length(r_list.unwrap(),NAME_39);
    } else {
        list = "".to_string();
    }
    
    let r_aname = deconstruct_bird_str('e', &arg);
    if r_aname.is_ok() {
        aname = limit_length(r_aname.unwrap(),NAME_39);
    } else {
        aname = "".to_string();
    }

    // Check if all needed are there
    if needed != 4 {
        return Err("Not all 4 required names are there to make a species (either not given or malformed)".to_string());
    }

    // Check if scientific name exists already
    // let check = get_sname_value(&sname,&birds);
    let check = sbirds.get(&sname);
    if check.is_some(){
        return Err("The species with that scientific name exists already.".to_string());
    }

    let result = Species::build_species(&birds, sname, name, order, family, status, aname, list);
    if result.is_err(){
        return Err(result.err().unwrap());
    }

    Ok(result.unwrap())
}


// Add a bird , if successful
pub fn add_species<'a>(arg: String, options: &'a mut SettingsText, birds: &'a mut BTreeMap<String,Species>, 
        sbirds: &'a mut BTreeMap<String,Species>, sightings: &Vec<Sightings> ) -> Result<(), String> {

    let mut yes = CodeOrNumber::new();
    let result = prepare_species_adding(arg, &sbirds, &birds);
    if result.is_err(){
        return Err(result.err().unwrap());
    }

    let key = result.clone().unwrap().code;
    let species = result.unwrap();
    let insert = birds.insert(key.clone(), species.clone());
    
    // This indicates an error
    if insert.is_some(){
        return Err("Adding species error -> key was updated and not inserted".to_string());
    }
    
    // for safety's sake going to insert sbirds as well
    sbirds.insert(species.clone().sname, species.clone());

    let index = get_index_from_code(&key, &birds).unwrap();
    
    yes.code = Some(key.clone());
    yes.what = Code::Code;
    yes.species = Some(species.clone());
    yes.number = Some(index);

    show_bird(yes, options, &birds, &sightings);

    let message = format!("The bird: {}{}{}{} with the code: {}{}{}{} has been added to your bird species database.", 
                        style::Italic, color::Fg(color::Yellow), species.name, style::Reset, 
                        style::Italic, color::Fg(color::Yellow), key.clone(),  style::Reset);
    feedback(Feedback::Info, message);

    // let tuple = (options, birds);
    // return Ok(tuple)

    // let tuple = (options, birds);
    Ok(())
}



// Delete bird species with that number, as well as all the sightings attached to that species
// BUT remember the user does not see 0-based indeces
pub fn delete_bird<'a>(con: CodeOrNumber , options: &'a mut SettingsText, birds: &'a mut BTreeMap<String,Species>, 
            sightings: &'a mut Vec<Sightings> ) -> Result<( &'a mut SettingsText, &'a mut BTreeMap<String,Species>, 
            &'a mut Vec<Sightings>), String>{
    
    //Show the bird first
    show_bird(con.clone(), options, &birds, &sightings);

    // let mut counter = 0;
    let index = con.number.unwrap();
    let species = con.species.unwrap();
    let observations = get_array_of_sname_indices_from_records(&species.clone().sname, sightings);

    // we have to remove the sightings in order to avoid orphans            
    for o  in observations.clone() {
        sightings.remove(o);
        // counter += 1;
    }
     
    // message the deletion of observations
    if observations.len() > 0 {
        let message = format!("{}{}{} observations (sightings) were deleted", 
                color::Fg(color::Yellow), observations.len(), style::Reset);
        feedback(Feedback::Info, message);
    } else {
        let message = "No observations (sightings) were attached to this bird.".to_string();
        feedback(Feedback::Info, message);
    }
    
    let result = birds.remove(&species.code);
    if result.is_some(){
        let bird_number = index + 1;
        let message = format!("The bird species:  {}{}{} {} with the code:  {}{}{} {} in position  {}{}{} {} was deleted.\n",
                        color::Fg(color::Yellow), style::Italic ,&species.name, style::Reset,
                        color::Fg(color::Yellow), style::Italic ,&species.code, style::Reset,
                        color::Fg(color::Yellow), style::Italic ,&bird_number.separate_with_spaces(), style::Reset);
        feedback(Feedback::Info, message);
        
    } else {
        return Err("Problems deleting species".to_string());
    }
    
    let tuple = (options, birds, sightings);
    return Ok(tuple)  
} 
        
        
// get the tail of the code range (eg. ga -> gb)        
pub fn get_code_range_end(code: &str) -> String {
    let lower = code.to_lowercase();
    let trim = lower.trim();
    let mut ret = String::new();
    
    // If empty
    if trim.len() == 0 {
        return "".to_string();
    }

    let len = trim.len();
    let start = &trim[..len-1].to_string();
    ret.push_str(start);

    let end = &trim[len-1..].to_string();
    // If "z" then return "zz"
    if end == "z" {
        ret.push_str("zz");
        return ret
    }

    let my_char = end.chars().next().unwrap().to_string();
    let next = move_shift(&my_char,1);
    ret.push_str(&next);

    return ret
}        
        
 
// Show all the species that fall into the range of the code
pub fn show_code_range<'a>(code: &str, options: &'a mut SettingsText, birds: &'a mut BTreeMap<String,Species>) 
                    -> Result<(&'a mut SettingsText, &'a mut BTreeMap<String,Species>), String> {
    
    let my_normal_gray: color::Rgb = options.clone().get_color("myNormalGray");

    let end = get_code_range_end(&code);
    let r_map = get_code_slice_of_birds(code, &end, birds);

    if r_map.is_err() {
        return Err(r_map.err().unwrap());
    }

    for (k,v) in r_map.unwrap() {
        let r_index = get_index_from_code(&k, birds).unwrap() + 1;                            //Add one
        let pos_text = justify(r_index.to_string(), POS_LEN, Justify::Left); 
        let code_text = justify(k.clone(), CODE_LEN, Justify::Left); 
        let fname_text = justify(v.clone().fname, NAME_39, Justify::Left); 
        let sname_text = justify(v.clone().sname, NAME_39, Justify::Left); 
        print!("  {}{}", color::Fg(my_normal_gray), code_text);  
        print!("{}", pos_text);  
        print!("{}{}\n", fname_text, sname_text);  
    }
    print!("{}\n", style::Reset); 
    let tuple = (options, birds);
    Ok(tuple)
}
        

     
        

// To make some of the functions more readable: see if code is in given range
pub fn is_code_in_range(start: &str, end: &str, k: &str) -> bool {
    
    let ret = k.ge(start) && k.lt(end);
    return ret
}
        
    
// Edit Bird via the code
pub fn edit_bird<'a>(arg: String, con: CodeOrNumber, options: &'a mut SettingsText, birds: &'a mut BTreeMap<String,Species>, 
            sightings: &'a mut Vec<Sightings>) -> Result<( &'a mut SettingsText, &'a mut BTreeMap<String,Species>, 
            &'a mut Vec<Sightings>), String>{

    let mut shuffled_species: Option<Species> = None;
    let code_change: bool;

    // Let oldest records show first
    let mut obs = get_array_of_sname_indices_from_records(&con.clone().species.unwrap().sname, &sightings);
    obs.sort();

    // Lets get the species 
    let old_key = con.clone().code.unwrap();
    let old_species = birds.get_key_value(&old_key.clone()).unwrap().1.clone();
    let old_position = con.number.unwrap() + 1; 

    let initial_result = prepare_species_edit(arg, old_species.clone(), &birds);
    if initial_result.is_err(){
        return Err(initial_result.err().unwrap());
    }
    
    let result = initial_result.unwrap();
    let improved_species = result.0;
    let new_key = improved_species.clone().code;
    let sname_change = result.1;

    // Sometimes the change remains the same
    if old_species.clone().code == improved_species.clone().code {
        code_change = false;
    } else {
        code_change = true;
    }
    
    //Deal with sname change - all records must be changed that have that sname
    if sname_change {
        // Display observation record numbers that will incur changes
        println!("Below are the {} records numbers of the bird species that will have their scientific name changed (sname)",
                                obs.len()    );
        for o in obs {
            let mut s = sightings.get_mut(o).unwrap();
            let si_clone = s.clone();
            s.sname = improved_species.clone().sname;
            display_sname_changes(o, con.clone(), options, si_clone );
        }
    }
    
    // Deal with code change
    if code_change {
        // Is there an entry with the new code already, if so remove and keep it
        shuffled_species = birds.remove(&improved_species.code); 
    }
    
    // Delete the old entry with its key
    let result = birds.remove_entry(&old_key);
    if result.is_none(){
        return Err("Deletion error in editing bird with old key".to_string())
    }
    
    //Lets insert the edited species
    let insertion_new = birds.insert(new_key, improved_species.clone());  
    
    
    // Lets deal with the shuffled one
    if shuffled_species.is_some(){
        
        let s = shuffled_species.unwrap();
        let result = Species::build_species(&birds, s.sname, s.name, s.order, 
            s.family, s.status, s.aname, s.list); 
            if result.is_ok(){
                let insertion_shuffle = birds.insert(result.clone().unwrap().code, result.unwrap());
                
                if insertion_shuffle.is_none(){
                    // let tuple = (options, birds, sightings);
                    // return Ok(tuple) 
                }
                else {
                    return Err("Error ocurred in editing shuffled species".to_string())
                }
            }
        } 

        
    // Make a new CodeOrNumber
    let con_new = what_code(&improved_species.code, birds);
    let new_position = con_new.number.unwrap() + 1;
    
    show_bird(con_new, options, birds, sightings);
    if code_change {
        be_display_change("code", &old_species.code, &improved_species.code);
    }
    
    if old_position != new_position {
        be_display_change("position", &old_position.to_string(), &new_position.to_string(),);
    }


    if insertion_new.is_none(){
        let tuple = (options, birds, sightings);
        return Ok(tuple) 
    } 
    else {
        return Err("Insertion error in editing bird with new key".to_string())
    }
}
 



// Function to prepare the species to be built and return result
pub fn prepare_species_edit(arg: String, old_species: Species, birds: &BTreeMap<String,Species>) 
            -> Result<(Species, bool, bool), String> {
    
    let mut change_sname = false;
    let mut change_name = false;
    let change_code: bool;

    let name: String;
    let sname: String;
    let family: String;
    let order: String;
    let status: String;
    let list: String;
    let aname: String;
    let mut code: String =String::new();

    let r_name = deconstruct_bird_str('n', &arg);
    if r_name.is_ok() {
        name = r_name.unwrap();
        change_name = true;
    } else {
        name = old_species.name;
    }
    
    let r_sname = deconstruct_bird_str('s', &arg);
    if r_sname.is_ok() {
        sname = r_sname.unwrap();
        change_sname = true;
    } else {
        sname = old_species.sname;
    }

    let r_family = deconstruct_bird_str('m', &arg);
    if r_family.is_ok() {
        family = r_family.unwrap();
    } else {
        family = old_species.family;
    }

    let r_order = deconstruct_bird_str('o', &arg);
    if r_order.is_ok() {
        order = r_order.unwrap();
    } else {
        order = old_species.order;
    }
    
    let r_status = deconstruct_bird_str('u', &arg);
    if r_status.is_ok() {
        status = r_status.unwrap();
    } else {
        status = old_species.status;
    }
    
    let r_list = deconstruct_bird_str('l', &arg);
    if r_list.is_ok() {
        list = r_list.unwrap();
    } else {
        list = old_species.list;
    }
    
    let r_aname = deconstruct_bird_str('e', &arg);
    if r_aname.is_ok() {
        aname = r_aname.unwrap();
    } else {
        aname = old_species.aname;
    }

    let result = Species::build_species(&birds, sname, name, order, family, status, aname, list);
    if result.is_err(){
        return Err(result.err().unwrap());
    }
    let mut species = result.unwrap();


    let r_code = deconstruct_bird_str('c', &arg);
    if r_code.is_ok() {
        code = r_code.unwrap();
        let is_valid = is_code_valid(code.clone(), &species ); 
        if !is_valid {
            return Err("The given code is not valid".to_string());
        }
        change_code = true;
    } else {
        if !change_name {
            species.code = old_species.code;
            change_code = false
        }
        // Name change has taken place
        else {
            change_code = true;
            code = species.clone().code;
        }
    }


    // If code has changed , we need to change it
    if change_code {
        let res = is_code_valid(code.clone(), &species );   
        if res {
            species.code = code;
        }
    }

    let tuple = (species, change_sname, change_code);
    Ok(tuple)
}


// To check if the desired code meets all the rules for a code change
pub fn is_code_valid(code: String, species: &Species ) -> bool {
    let nope = false;

    let work = species.fname.to_lowercase();
    let chunks: Vec<_> = work.split_whitespace().collect();
    
    let mut build_code: String = "".to_string();
    let mut count = 1;
    
    for name in chunks {
        match count {
            1..=2   => {
                build_code.push(name.chars().nth(0).unwrap());
                build_code.push(name.chars().nth(1).unwrap());
                count += 1
            }
            _       => {
                build_code.push(name.chars().nth(0).unwrap());
                count += 1
            }
        }
    }

    // code is just a number
    if code.parse::<usize>().is_ok(){
        return false;
    }

    let len_b = build_code.len();
    let len_c = code.len();
    let m = cmp::min(len_b,len_c);

    //If the code is not big enough
    if len_b > len_c {
        return false
    }

    // code is empty string
    if m == 0 {
        return false
    }

    // let mut resultant = code[0..m].replace(&build_code, "");
    let mut resultant = str::replace(&code, &code[0..m], "");
    if resultant.len() == 0 {
        return true
    } else {
        resultant.push('1');

        let result = resultant.parse::<usize>();
        if result.is_ok(){
            return true;
        }
    }
    return nope;
}


// Function to split a csv line into Species parts and return species (tab delimited)
pub fn get_species_from_line(line: &str, birds: &BTreeMap<String,Species> ) -> Result<Species,String> {
    let temp = line.split("\t");
    let vec: Vec<&str> = temp.collect();
    if vec.len() != 11 {
        return Err("Line does not have 11 fields".to_string());
    }

    let ret = Species::validate_species(birds, vec[0].to_string(),
                                vec[1].to_string(), vec[3].to_string(), vec[4].to_string(), 
                               vec[5].to_string(), vec[6].to_string(),
                                vec[7].to_string(), vec[9].to_string(), vec[10].to_string());

    if ret.is_err(){
        return Err(ret.err().unwrap());
    }

    return Ok(ret.unwrap())
}























// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ Errors @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@        @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

// This was coded to reduce the number of lines in the main.rs
// This should make readability a bit better
pub fn if_birds_length_is_zero(birds: &BTreeMap<String,Species>){

    if birds.len() == 0 {
        let message = format!("You have no birds in the species database.");
        feedback(Feedback::Warning, message);
        exit(17);
    }
}


pub fn error_neither(){
    let mut assembled = String::new();

    let message = format!("No legitimate {}{}{} or ",
        color::Fg(color::Yellow), "code", style::Reset);       
    assembled.push_str(&message);

    let message = format!("{}{}{} has been given as a sub-argument.",
        color::Fg(color::Yellow), "number", style::Reset);       
    assembled.push_str(&message);
    
    feedback(Feedback::Warning, assembled);
}


pub fn error_last_arg_edit(name: String) {
    let mut assembled = String::new();
    
    let message = format!("The last argument editing the {}{}{} was missing.",
    color::Fg(color::Yellow), name, style::Reset);       
    assembled.push_str(&message);
    
    feedback(Feedback::Warning, assembled);
}


pub fn error_last_arg_edit_invalid(name: String) {
    let mut assembled = String::new();
    
    let message = format!("The last argument editing the {}{}{} was invalid.",
    color::Fg(color::Yellow), name, style::Reset);       
    assembled.push_str(&message);
    
    feedback(Feedback::Warning, assembled);
}






















        
        
        
        
        
        
        
        
































// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@  Tests  @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@         @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::copy};
    use substring::Substring;
    use std::fs::remove_file;

    
    #[ignore]
    #[test]
    fn t001_make_fname() {
        let name = "  oLd   BlUe   eYes  ";
        let fname = Species::make_fname(name);

        assert_eq!(fname,"Eyes Old Blue".to_string());
    }
    

    #[ignore]
    #[test]
    fn t002_import() {
        let source = "/DATA/programming/Rust/mybirding/test/store/species/species4_import.json";
        let destination = "./test/birds_import.json";
        copy(source,destination).expect("Failed to copy");
        
        let birds = Species::import(destination).unwrap();
        let fname = "sugar sugar";
        let code = Species::make_code(fname, &birds);
        
        //cleanup
        remove_file(destination).expect("Cleanup test failed");
        
        assert_eq!(code, "susu2".to_string());
    }
    
    
    #[ignore]
    #[test]
    fn t003_make_acode() {
        let source = "/DATA/programming/Rust/mybirding/test/store/species/species4_import.json";
        let destination = "./test/birds_import.json";
        copy(source,destination).expect("Failed to copy");
        
        let birds = Species::import(destination).unwrap();
        
        let afname = "Sparrow Lix";
        let code = Species::make_acode(afname, &birds);
        
        //cleanup
        remove_file(destination).expect("Cleanup test failed");
        
        assert_eq!(code, "spli2".to_string());
    }
    
    
    #[ignore]
    #[test]
    fn t004_build_species1() {

    // Birds
        let source = "/DATA/programming/Rust/mybirding/test/store/species/species4_import.json";
        let destination = "./test/birds_import.json";
        copy(source,destination).expect("Failed to copy");
        let birds = Species::import(destination).unwrap();
        remove_file(destination).expect("Cleanup test failed");
    
    // Lets build
        let sname = "Maximillian birdus".to_string();
        let name = "Cassius The Birdus".to_string();
        let order = "Ordering".to_string();
        let family = "Familing".to_string();
        let status = "".to_string();
        let list = "".to_string();
        let aname = "".to_string();

        let result = Species::build_species(&birds, sname, name, order, family, status, aname, list).unwrap();

        assert_eq!(result.status, "".to_string());
    }
    

    #[ignore]
    #[test]
    fn t005_build_species2() {

        let source = "/DATA/programming/Rust/mybirding/test/store/species/species4_import.json";
        let destination = "./test/birds_import.json";
        copy(source,destination).expect("Failed to copy");
        let birds = Species::import(destination).unwrap();
        remove_file(destination).expect("Cleanup test failed");
    
    // Lets build
        let sname = "maximiLLian birDus".to_string();
        let name = "Cassius The Birdus".to_string();
        let order = "Ordering".to_string();
        let family = "Familing".to_string();
        let status = "".to_string();
        let list = "".to_string();
        let aname = "".to_string();

        let result = Species::build_species(&birds, sname, name, order, family, status, aname, list).unwrap();

        assert_eq!(result.sname, "Maximillian birdus".to_string());
    }
    
    
    #[ignore]
    #[test]
    fn t006_build_species3() {

        let source = "/DATA/programming/Rust/mybirding/test/store/species/species4_import.json";
        let destination = "./test/birds_import.json";
        copy(source,destination).expect("Failed to copy");
        let birds = Species::import(destination).unwrap();
        remove_file(destination).expect("Cleanup test failed");

    // Lets build
        let sname = " Maximillian  birdus  ".to_string();
        let name = "lIxus  spirilius ".to_string();
        let order = "ordering".to_string();
        let family = "familing".to_string();
        let status = "LC".to_string();
        let aname = "  lIxus  spirilius ".to_string();
        let list = "Southern Australia".to_string();

        let result = Species::build_species(&birds, sname, name, order, family, status, aname, list).unwrap();

        assert_eq!(result.code, "spli".to_string());
    }
    
    
    #[ignore]
    #[test]
    fn t007_build_species4() {

        let source = "/DATA/programming/Rust/mybirding/test/store/species/species4_import.json";
        let destination = "./test/birds_import.json";
        copy(source,destination).expect("Failed to copy");
        let birds = Species::import(destination).unwrap();
        remove_file(destination).expect("Cleanup test failed");

    // Lets build
        let sname = " Maximillian  birdus  ".to_string();
        let name = "lIxus  spirilius ".to_string();
        let order = "ordering".to_string();
        let family = "familing".to_string();
        let status = "LC".to_string();
        let aname = "  lIxus  spirilius ".to_string();
        let list = "Southern Australia".to_string();

        let result = Species::build_species(&birds, sname, name, order, family, status, aname, list).unwrap();

        assert_eq!(result.acode, "spli2".to_string());
    }
    
    
    #[ignore]
    #[test]
    fn t008_build_species5() {

        let source = "/DATA/programming/Rust/mybirding/test/store/species/species4_import.json";
        let destination = "./test/birds_import.json";
        copy(source,destination).expect("Failed to copy");
        let birds = Species::import(destination).unwrap();
        remove_file(destination).expect("Cleanup test failed");

    // Lets build
        let sname = " Maximillian  birdus  ".to_string();
        let name = "  lIXus spirilius ".to_string();
        let order = "ordering".to_string();
        let family = "familing".to_string();
        let status = "LC".to_string();
        let aname = "  lIxus  spirilius ".to_string();
        let list = "Southern Australia".to_string();

        let result = Species::build_species(&birds, sname, name, order, family, status, aname, list).unwrap();

        assert_eq!(result.name, result.aname);
    }
    
    
    #[ignore]
    #[test]
    fn t009_export() {

        let source = "/DATA/programming/Rust/mybirding/test/store/species/species4_import.json";
        let destination = "./test/birds_import.json";
        copy(source,destination).expect("Failed to copy");
        let mut birds = Species::import(destination).unwrap();
        remove_file(destination).expect("Cleanup test failed");

    // Lets build
        let sname = " Maximillian  birdus  ".to_string();
        let name = "  lIXus spirilius ".to_string();
        let order = "ordering".to_string();
        let family = "familing".to_string();
        let status = "LC".to_string();
        let aname = "  lIxus  spirilius ".to_string();
        let list = "Southern Australia".to_string();

        let result = Species::build_species(&birds, sname, name, order, family, status, aname, list).unwrap();
        birds.insert(result.code.clone(), result);

        let file = "./test/birds_export.json";
        if Species::export(file, &birds).is_err(){
            println!("Something wrong saving file")
        }

        let nother_birds = Species::import(file).unwrap();
        remove_file(file).expect("Cleanup test failed");

        let new_bird = nother_birds.get("spli").unwrap().clone();

        assert_eq!(new_bird.aname, "Lixus Spirilius");
    }
    
    
    #[ignore]
    #[test]
    fn t010_validate_sname() {

        let example = "   hiGHus   seLLAsus   ";
        let result = Species::validate_sname(&example);
        
        assert_eq!(result.unwrap(), "Highus sellasus");
    }
    
    
    #[ignore]
    #[test]
    fn t011_validate_sname2() {

        let example = "   highus   sellasus  really ";
        let xxx = Species::validate_sname(&example);
        let snip = xxx.unwrap_err();
        let a = snip.substring(0, 5);
        
        assert_eq!(a, "Scien");
    }


    #[ignore]
    #[test]
    fn t012_validate_name1() {

        let example = "   pale  chanting  lesser   maybe greater sparrow hawk ";
        let xxx = Species::validate_name(&example);
        let snip = xxx.unwrap_err();
        let a = snip.substring(0, 8);
        
        assert_eq!(a, "The name");
    }


    #[ignore]
    #[test]
    fn t013_validate_name2() {

        let example = "  pale  chanting  goshawk ";
        let xxx = Species::validate_name(&example);
        let snip = xxx.unwrap();
        
        assert_eq!(snip, "Pale Chanting Goshawk");
    }


    #[ignore]
    #[test]
    fn t014_save() {

        let source = "/DATA/programming/Rust/mybirding/test/store/species/species4_import.json";
        let destination = "./test/birds_import.json";
        copy(source,destination).expect("Failed to copy");
        let mut birds = Species::import(destination).unwrap();
        remove_file(destination).expect("Cleanup test failed");

    // Lets build
        let sname = " Maximillian  birdus  ".to_string();
        let name = "  lIXus spirilius ".to_string();
        let order = "ordering".to_string();
        let family = "familing".to_string();
        let status = "LC".to_string();
        let aname = "  lIxus  spirilius ".to_string();
        let list = "Southern Australia".to_string();

        let result = Species::build_species(&birds, sname, name, order, family, status, aname, list).unwrap();
        birds.insert(result.code.clone(), result);

        let file = "./test/birds.bin";
        if Species::save(file, &birds).is_err(){
            println!("Something wrong saving file")
        }

        let nother_birds = Species::load(file).unwrap();
        remove_file(file).expect("Cleanup test failed");

        let new_bird = nother_birds.get("spli").unwrap().clone();
        assert_eq!(new_bird.aname, "Lixus Spirilius");
    }
    
    
    #[ignore]
    #[test]
    fn t015_get_sname_value() {

        let source = "/DATA/programming/Rust/mybirding/test/store/species/to_use.json";
        let destination = "./test/birds.json";
        copy(source,destination).expect("Failed to copy");
        let birds = Species::import(destination).unwrap();
        let sbirds = make_sname_btree(&birds);
        remove_file(destination).expect("Cleanup test failed");

        let sname = "Centropus burchellii";
        // let species = get_sname_value(sname, &birds).unwrap();
        let species = sbirds.get(sname).unwrap().clone();
        assert_eq!(species.order,"Cuculiformes");
    }


    #[ignore]
    #[test]
    fn t016_make_code() {
        let source = "/DATA/programming/Rust/mybirding/test/store/species/species4_import.json";
        let destination = "./test/birds_import.json";
        copy(source,destination).expect("Failed to copy");
        
        let birds = Species::import(destination).unwrap();
        
        //cleanup
        remove_file(destination).expect("Cleanup test failed");

        let fname = "    Sparrow-Hawk     Lixus  ";
        let code = Species::make_acode(fname, &birds);
        
        assert_eq!(code, "spli2".to_string());
    }


    #[ignore]
    #[test]
    fn t017_make_fname2() {
        let name = "  buff-Breasted     sparrow-Hawk   ";
        let fname = Species::make_fname(name);

        assert_eq!(fname,"Sparrow-hawk Buff-breasted".to_string());
    }


    #[ignore]
    #[test]
    fn t018_make_acode() {
        let source = "/DATA/programming/Rust/mybirding/test/store/species/species4_import.json";
        let destination = "./test/birds_import.json";
        copy(source,destination).expect("Failed to copy");
        
        let birds = Species::import(destination).unwrap();
        
        //cleanup
        remove_file(destination).expect("Cleanup test failed");

        let fname = "    Sparrow-Lark     Twitching-Humped  ";
        let code = Species::make_acode(fname, &birds);
        
        assert_eq!(code, "sptw".to_string());
    }


    #[ignore]
    #[test]
    fn t019_validate_family() {
        let name = "accipiTridae (hawks, eagles, and kites)";
        let ans = Species::validate_family(name).unwrap();
        
        
        assert_eq!(ans, "Accipitridae (Hawks, Eagles, and Kites)".to_string());
    }


    #[ignore]
    #[test]
    fn t020_validate_family() {
        let name = "CALcariidae ( longspurs And SnoW buntings )";
        let ans = Species::validate_family(name).unwrap();
        
        
        assert_eq!(ans, "Calcariidae ( Longspurs and Snow Buntings )".to_string());
    }


    #[ignore]
    #[test]
    fn t021_build_species6() {

        let source = "/DATA/programming/Rust/mybirding/test/store/species/species4_import.json";
        let destination = "./test/birds_import.json";
        copy(source,destination).expect("Failed to copy");
        let birds = Species::import(destination).unwrap();
        remove_file(destination).expect("Cleanup test failed");

    // Lets build
        let sname = " Maximillian  birdus  ".to_string();
        let name = "  lIXus spirilius ".to_string();
        let order = "ordering".to_string();
        let family = "pHoeniculidae (woodhoopoes And scimitarbills)".to_string();
        let status = "LC".to_string();
        let aname = "  lIxus  spirilius ".to_string();
        let list = "Southern Australia".to_string();

        let result = Species::build_species(&birds, sname, name, order, family, status, aname, list).unwrap();

        assert_eq!(result.name, result.aname);
    }
    
    
    #[ignore]
    #[test]
    fn t022_save2() {
        let mut birds: BTreeMap<String,Species> = BTreeMap::new();
        let source = "./test/store/species/to_use.json";
        let destination = "./test/birds_import.json";
        copy(source,destination).expect("Failed to copy");
        let result1 = Species::import(destination);
        if result1.is_ok(){
            birds = result1.unwrap();
        }
        remove_file(destination).expect("Cleanup test failed");
        
        let result2 = Species::save("./to_use.bin", &birds);
        let mut birds2: BTreeMap<String,Species> = BTreeMap::new();
        if result2.is_ok(){
            birds2 = Species::load("./to_use.bin").unwrap();
            remove_file("./to_use.bin").expect("Cleanup test failed");
        }
        let sbirds = make_sname_btree(&birds2);
        // let species = get_sname_value("Upupa africana",&birds2);
        let species = sbirds.get("Upupa africana").unwrap().clone();
        
        assert_eq!(species.name,"African Hoopoe");
    }


    #[ignore]
    #[test]
    fn t023_build_species7() {

        let source = "/DATA/programming/Rust/mybirding/test/store/species/species4_import.json";
        let destination = "./test/birds_import.json";
        copy(source,destination).expect("Failed to copy");
        let birds = Species::import(destination).unwrap();
        remove_file(destination).expect("Cleanup test failed");

    // Lets build
        let sname = " Maximillian  birdus  ".to_string();
        let name = "  black-throated  spirilius ".to_string();
        let order = "ordering".to_string();
        let family = "pHoeniculidae (woodhoopoes And scimitarbills)".to_string();
        let status = "LC".to_string();
        let aname = "  lIxus  spirilius ".to_string();
        let list = "Southern Australia".to_string();

        let result = Species::build_species(&birds, sname, name, order, family, status, aname, list).unwrap();

        assert_eq!(result.name, "Black-throated Spirilius");
    }


    #[ignore]
    #[test]
    fn t024_deconstruct1() {

    // Lets build
        let mut count = 0;
        let arg = "n=\"firstus birdus \"#s=\"prubie canaray \"#m=prunideiao #o=formes#u=LC ".to_string();
        let name = deconstruct_bird_str('n', &arg);
        if name.is_ok() {
            count += 1
        }
        let sname = deconstruct_bird_str('s', &arg);
        if sname.is_ok() {
            count += 1
        }
        let family = deconstruct_bird_str('m', &arg);
        if family.is_ok() {
            count += 1
        }
        let order = deconstruct_bird_str('o', &arg);
        if order.is_ok() {
            count += 1
        }
        let status = deconstruct_bird_str('u', &arg);
        if status.is_ok() {
            count += 1
        }
        assert_eq!(count,5);
    }


    #[ignore]
    #[test]
    fn t025_prepare1() {

        let source = "./test/store/species/to_use.json";
        let destination = "./test/birds_import.json";
        copy(source,destination).expect("Failed to copy");
        let birds = Species::import(destination).unwrap();
        let sbirds = make_sname_btree(&birds);
        remove_file(destination).expect("Cleanup test failed");

        // Lets build
        let arg = "n=\"firstus birdus\"#s=\"prubie canaray\"#m=prunideiao#r=formes#u=LC".to_string();
        let result = prepare_species_adding(arg, &sbirds, &birds);

        assert_eq!(result.is_ok(),true);
    }


    #[ignore]
    #[test]
    fn t026_prepare2() {

        let source = "./test/store/species/to_use.json";
        let destination = "./test/birds_import.json";
        copy(source,destination).expect("Failed to copy");
        let birds = Species::import(destination).unwrap();
        let sbirds = make_sname_btree(&birds);
        remove_file(destination).expect("Cleanup test failed");

        // Lets build
        let arg = "n=\"firstus birdus\"#s=\"prubie \"#m=prunideiao#r=formes#u=LC".to_string();
        let result = prepare_species_adding(arg, &sbirds, &birds);

        assert_eq!(result.err().unwrap()[0..5],"Scien".to_string());
    }


    #[ignore]
    #[test]
    fn t027_prepare3() {

        let source = "./test/store/species/to_use.json";
        let destination = "./test/birds_import.json";
        copy(source,destination).expect("Failed to copy");
        let birds = Species::import(destination).unwrap();
        let sbirds = make_sname_btree(&birds);
        remove_file(destination).expect("Cleanup test failed");

        // Lets build
        let arg = "n=\"firstus birdus\"#m=prunideiao#r=formes#u=LC".to_string();
        let result = prepare_species_adding(arg, &sbirds, &birds);

        assert_eq!(result.err().unwrap()[0..10],"The needed".to_string());
    }


    #[ignore]
    #[test]
    fn t028_prepare4() {

        let source = "./test/store/species/to_use.json";
        let destination = "./test/birds_import.json";
        copy(source,destination).expect("Failed to copy");
        let birds = Species::import(destination).unwrap();
        let sbirds = make_sname_btree(&birds);
        remove_file(destination).expect("Cleanup test failed");

        // Lets build
        let arg = "n=\"firstus birdus\"#s=\"prubie canaray\"#m=prunideiao#u=LC".to_string();
        let result = prepare_species_adding(arg, &sbirds, &birds);

        assert_eq!(result.err().unwrap().contains("order"), true);
    }


    #[ignore]
    #[test]
    fn t029_add_species1() {
        let source = "./test/store/species/to_use.json";
        let destination = "./test/birds_import.json";
        copy(source,destination).expect("Failed to copy");
        let mut birds = Species::import(destination).unwrap();
        let mut sbirds = make_sname_btree(&birds);
        remove_file(destination).expect("Cleanup test failed");

        let source = "./test/store/sightings/to_use.json";
        let destination = "./test/sights.json";
        copy(source,destination).expect("Failed to copy");
        let sights = Sightings::import(destination).unwrap();
        remove_file(destination).expect("Cleanup test failed");

        let source = "./test/store/settings/options_002.json";
        let destination = "./test/options.json";
        copy(source,destination).expect("Failed to copy");
        let mut opt = SettingsText::new(destination);
        remove_file(destination).expect("Cleanup test failed");

        // Lets build
        let arg = "n=\"firstus birdus\"#s=\"prubie canaray\"#m=prunideiao#r=formes#u=LC".to_string();
        let _result = add_species(arg, &mut opt, &mut birds, &mut sbirds, &sights);
        let num = opt.get_number("lastSpeciesViewed");

        assert_eq!(num, 704);
    }


    #[ignore]
    #[test]
    fn t030_show_bird_code2() {
        let source = "./test/store/species/to_use.json";
        let destination = "./test/birds_import.json";
        copy(source,destination).expect("Failed to copy");
        let mut birds = Species::import(destination).unwrap();
        let mut sbirds = make_sname_btree(&birds);
        remove_file(destination).expect("Cleanup test failed");

        let source = "./test/store/sightings/to_use.json";
        let destination = "./test/sights.json";
        copy(source,destination).expect("Failed to copy");
        let sights = Sightings::import(destination).unwrap();
        remove_file(destination).expect("Cleanup test failed");

        let source = "./test/store/settings/options_002.json";
        let destination = "./test/options.json";
        copy(source,destination).expect("Failed to copy");
        let mut opt = SettingsText::new(destination);
        remove_file(destination).expect("Cleanup test failed");

        // let num = birds.clone().len() + 1;
        
        // Lets build
        let arg = "n=\"firstus birdus\"#s=\"prubie canaray\"#m=prunideiao#r=formes#u=LC".to_string();
        let result = add_species(arg, &mut opt, &mut birds, &mut sbirds, &sights);
        
        // let tupleling = result.unwrap();

        assert_eq!(result.is_ok(),true);
    }
    

    #[ignore]
    #[test]
    fn t031_get_map_position() {
        let source = "./test/store/species/species5_import.json";
        let destination = "./test/birds_import.json";
        copy(source,destination).expect("Failed to copy");
        let birds = Species::import(destination).unwrap();
        remove_file(destination).expect("Cleanup test failed");

        let code = "jabl";
        let pos = get_index_from_code(code, &birds).unwrap();
        assert_eq!(pos,1);
        
        let source = "./test/store/species/to_use.json";
        let destination = "./test/birds_import.json";
        copy(source,destination).expect("Failed to copy");
        let birds2 = Species::import(destination).unwrap();
        remove_file(destination).expect("Cleanup test failed");
        
        let pos = get_index_from_code(code, &birds2).unwrap();
        assert_eq!(pos,4237);
    }


    #[ignore]
    #[test]
    fn t032_delete_bird_number1() {
        let source = "./test/store/species/species5_import.json";
        let destination = "./test/birds_import.json";
        copy(source,destination).expect("Failed to copy");
        let mut birds = Species::import(destination).unwrap();
        println!("length of birds is {}",birds.clone().len());
        remove_file(destination).expect("Cleanup test failed");
        
        let source = "./test/store/sightings/sightsSome.json";
        let destination = "./test/sights.json";
        copy(source,destination).expect("Failed to copy");
        let mut sightings = Sightings::import(destination).unwrap();
        println!("length of sightings is {}",sightings.clone().len());
        remove_file(destination).expect("Cleanup test failed");
        
        let source = "./test/store/settings/options_002.json";
        let destination = "./test/options.json";
        copy(source,destination).expect("Failed to copy");
        let mut options = SettingsText::new(destination);
        remove_file(destination).expect("Cleanup test failed");

        let code = "jabl";
        let species = birds.get_key_value(code.clone()).unwrap().1.to_owned();
        let pos = get_index_from_code(&code, &birds);
        let con = CodeOrNumber::make(Code::Code, code.to_string(), species, pos.clone().unwrap());
        assert_eq!(pos.clone().unwrap(), 1);

        
        // removing the blue jay
        if delete_bird(con, &mut options, &mut birds, &mut sightings).is_ok(){
            let last = options.get_number("lastSpeciesViewed");
            assert_eq!(last, 1);
        }
        
        let dest_birds = "./test/birds.bin";
        let dest_sightings = "./test/sightings.json";
        let dest_options = "./test/options.json";
        
        if Sightings::export(dest_sightings, sightings).is_ok(){
            let s1 = Sightings::import(dest_sightings).unwrap();
            remove_file(dest_sightings).expect("Cleanup test failed");
            
            let num = s1.clone().len();
            assert_eq!(num, 45);
            
            let mut b: bool = false;
            for i in s1.clone() {
                if i.sname == "Cyanocitta cristata" {
                    b = true ;
                }
            }
            assert_eq!(b,false);
            
            b = false;
            for i in s1 {
                if i.sname == "Corvus brachyrhynchos" {
                    b = true ;
                }
            }
            assert_eq!(b,true);
        }
        if Species::export(dest_birds, &birds).is_ok(){
            let s1 = Species::import(dest_birds).unwrap();
            let pos = get_index_from_code(code, &s1);
            remove_file(dest_birds).expect("Cleanup test failed");
            
            assert_eq!(pos.is_err(),true);
            
        }
        if options.export(dest_options).is_ok(){
            let o2 = SettingsText::new(dest_options); 
            let last = o2.get_number("lastSpeciesViewed");
            remove_file(dest_options).expect("Cleanup test failed");

            assert_eq!(last,1);
        }
    }


    #[ignore]
    #[test]
    fn t033_delete_bird_code1() {
        let source = "./test/store/species/species5_import.json";
        let destination = "./test/birds_import.json";
        copy(source,destination).expect("Failed to copy");
        let mut birds = Species::import(destination).unwrap();
        println!("length of birds is {}",birds.clone().len());
        remove_file(destination).expect("Cleanup test failed");
        
        let source = "./test/store/sightings/sightsSome.json";
        let destination = "./test/sights.json";
        copy(source,destination).expect("Failed to copy");
        let mut sightings = Sightings::import(destination).unwrap();
        println!("length of sightings is {}",sightings.clone().len());
        remove_file(destination).expect("Cleanup test failed");
        
        let source = "./test/store/settings/options_002.json";
        let destination = "./test/options.json";
        copy(source,destination).expect("Failed to copy");
        let mut options = SettingsText::new(destination);
        remove_file(destination).expect("Cleanup test failed");

        let code = "cram";
        let species = birds.get_key_value(code.clone()).unwrap().1.to_owned();
        let pos = get_index_from_code(&code, &birds);
        let con = CodeOrNumber::make(Code::Code, code.clone().to_string(), species, pos.clone().unwrap());
        assert_eq!(pos.clone().unwrap(), 0);

        // removing the American crow
        let result = delete_bird(con.clone(), &mut options, &mut birds, &mut sightings);
        if result.is_ok(){
            let last = options.get_number("lastSpeciesViewed");
            assert_eq!(last, 0);
        }

        let dest_birds = "./test/birds.bin";
        let dest_sightings = "./test/sightings.json";
        let dest_options = "./test/options.json";
        
        if Sightings::export(dest_sightings, sightings).is_ok(){
            let s1 = Sightings::import(dest_sightings).unwrap();
            remove_file(dest_sightings).expect("Cleanup test failed");
            
            let num = s1.clone().len();
            assert_eq!(num, 44);
            
            let mut b: bool = false;
            for i in s1.clone() {
                if i.sname == "Cyanocitta cristata" {
                    b = true ;
                }
            }
            assert_eq!(b,true);
            
            b = false;
            for i in s1 {
                if i.sname == "Corvus brachyrhynchos" {
                    b = true ;
                }
            }
            assert_eq!(b,false);
        }
        if Species::export(dest_birds, &birds).is_ok(){
            let s1 = Species::import(dest_birds).unwrap();
            let pos = get_index_from_code(code, &s1);
            remove_file(dest_birds).expect("Cleanup test failed");
            
            assert_eq!(pos.is_err(),true);
            
        }
        if options.export(dest_options).is_ok(){
            let o2 = SettingsText::new(dest_options); 
            let last = o2.get_number("lastSpeciesViewed");
            remove_file(dest_options).expect("Cleanup test failed");

            assert_eq!(last,0);
        }
    }


    #[ignore]
    #[test]
    fn t034_range() {
        let source = "./test/store/species/species5_import.json";
        let destination = "./test/birds_import.json";
        copy(source,destination).expect("Failed to copy");
        let birds = Species::import(destination).unwrap();
        remove_file(destination).expect("Cleanup test failed");

        let start = "si".to_string();
        let end = "sy".to_string();

        let mut counter = 0;
        for (k,v) in birds {
            if k.ge(&start) && k.lt(&end) {
                println!("{:?}",v);
                counter += 1;
            }
        }
        assert_eq!(counter,2);
    }


    #[ignore]
    #[test]
    fn t035_get_code_range_end() {
        let source = "./test/store/species/to_use.json";
        let destination = "./test/birds_import.json";
        copy(source,destination).expect("Failed to copy");
        let birds = Species::import(destination).unwrap();
        remove_file(destination).expect("Cleanup test failed");

        let start = "spho";
        let end = get_code_range_end(start);
        assert_eq!(end,"sphp");

        let s1 = "";
        let e1 = get_code_range_end(s1);
        assert_eq!(e1,"");

        let s2 = "cz";
        let e2 = get_code_range_end(s2);
        assert_eq!(e2,"czz");

        let s3 = "q";
        let e3 = get_code_range_end(s3);
        assert_eq!(e3,"r");

        let s4 = "it";
        let e4 = get_code_range_end(s4);
        assert_eq!(e4,"iu");

        let s5 = "bugr".to_string();
        let e5 = get_code_range_end(&s5.clone());
        assert_eq!(e5,"bugs");

        let mut counter = 0;
        for (k,v) in birds {

            if k.ge(&s5) && k.lt(&e5) {
                println!("{:?}",v);
                counter += 1;
            }
        }
        assert_eq!(counter,18);
    }


    #[ignore]
    #[test]
    fn t036_code_is_in_range() {
        let s1 = "sp";
        let e1 = "sq";
        let k = "soz";
        let ans = is_code_in_range(s1,e1, k);
        assert_eq!(ans, false);

        let s1 = "sp";
        let e1 = "sq";
        let k = "sp1";
        let ans = is_code_in_range(s1,e1, k);
        assert_eq!(ans, true);

        let s1 = "bz";
        let e1 = "az";
        let k = "ay";
        let ans = is_code_in_range(s1,e1, k);
        assert_eq!(ans, false);

        let s1 = "az";
        let e1 = "bz";
        let k = "by";
        let ans = is_code_in_range(s1,e1, k);
        assert_eq!(ans, true);

        let s1 = "az";
        let e1 = "bz";
        let k = "ay";
        let ans = is_code_in_range(s1,e1, k);
        assert_eq!(ans, false);

    }


    #[ignore]
    #[test]
    fn t037_get_code_slice_of_birds() {
        let source = "./test/store/species/to_use.json";
        let destination = "./test/birds_import.json";
        copy(source,destination).expect("Failed to copy");
        let mut birds = Species::import(destination).unwrap();
        remove_file(destination).expect("Cleanup test failed");
        
        let s_code = "spho";
        let e_code = "sphp";

        let slice = get_code_slice_of_birds(s_code, e_code, &mut birds).unwrap();
        assert_eq!(slice.len(),2);
    }

    #[ignore]
    #[test]
    fn t038_is_valid_code() {
        let source = "./test/store/species/species5_import.json";
        let destination = "./test/birds_import.json";
        copy(source,destination).expect("Failed to copy");
        let mut birds = Species::import(destination).unwrap();
        remove_file(destination).expect("Cleanup test failed");

        // let s_code = "yuwh";
        let species = get_species_from_index(4, &mut birds).unwrap().1;

        let mut ans = is_code_valid("yuwh".to_string(), &species);
        assert_eq!(ans, true);

        ans = is_code_valid("yuwh3".to_string(), &species);
        assert_eq!(ans, true);

        ans = is_code_valid("yu1wh3".to_string(), &species);
        assert_eq!(ans, false);

        ans = is_code_valid("1yuwh3".to_string(), &species);
        assert_eq!(ans, false);

        ans = is_code_valid("5yuwh".to_string(), &species);
        assert_eq!(ans, false);

        ans = is_code_valid("yuw".to_string(), &species);
        assert_eq!(ans, false);

        ans = is_code_valid("yuwh32134".to_string(), &species);
        assert_eq!(ans, true);

        ans = is_code_valid("y".to_string(), &species);
        assert_eq!(ans, false);

        ans = is_code_valid("".to_string(), &species);
        assert_eq!(ans, false);

        ans = is_code_valid("2".to_string(), &species);
        assert_eq!(ans, false);
    }


    #[ignore]
    #[test]
    fn t039_edit_bird() {
        let source = "./test/store/species/species5_import.json";
        let destination = "./test/birds_import.json";
        copy(source,destination).expect("Failed to copy");
        let mut birds = Species::import(destination).unwrap();
        let sbirds = make_sname_btree(&birds);
        remove_file(destination).expect("Cleanup test failed");
        
        let source = "./test/store/sightings/sights5.json";
        let destination = "./test/sights.json";
        copy(source,destination).expect("Failed to copy");
        let mut sightings = Sightings::import(destination).unwrap();
        remove_file(destination).expect("Cleanup test failed");

        let source = "./test/store/settings/options_001.json";
        let destination = "./test/options_004.json";
        copy(source,destination).expect("Failed to copy");
        let mut options = SettingsText::new(destination);
        remove_file(destination).expect("Cleanup test failed");

        let sname = "Calidris pugnax";
        // let species = get_sname_value(sname, &birds).unwrap();
        let species = sbirds.get(sname).unwrap().clone();
        let index = get_index_from_code(&species.clone().code, &birds.clone()).unwrap();
        let con = CodeOrNumber::make(Code::Code, species.clone().code, species.clone(), index);
        assert_eq!(index, 2);

        // status
        let arg = "u=\"sven howzit buddy \"#s=\"Sigularity complexus\"#c=susu1".to_string(); 
        let result = edit_bird(arg, con, &mut options, &mut birds, &mut sightings);
        let tuple = result.unwrap();
        let s = tuple.2;
        let b2 = &tuple.1;
        let obs = get_array_of_sname_indices_from_records("Sigularity complexus", &mut s.clone());
        assert_eq!(obs.len(),2);

        let temp = b2.get_key_value("susu1");
        if temp.is_some(){
            assert_eq!(temp.unwrap().1.sname,"Sigularity complexus");
        }
        
        let temp2 = b2.get_key_value("susu");
        if temp2.is_some(){
            assert_eq!(temp2.unwrap().1.sname,"Calidris virgata");
        }
    }
    
    #[ignore]
    #[test]
    fn t040_export_path() {

        let mut s1 = SettingsText::new("./test/options_005.json");
        let ans = Species::export_path("cvs",&mut s1);
        
        assert_eq!(ans.substring(22, 33),"species.cvs");
    }


    #[ignore]
    #[test]
    fn t040_export_csv() {
        let source = "./test/store/species/species5_import.json";
        let destination = "./test/birds.json";
        copy(source,destination).expect("Failed to copy");
        let birds = Species::import(destination).unwrap();
        remove_file(destination).expect("Cleanup test failed");
        
        let res = Species::export_csv("./test/birds.csv", &birds);
        remove_file("./test/birds.csv").expect("Cleanup test failed");

        assert_eq!(res.is_ok(),true);
    }


    #[ignore]
    #[test]
    fn t041_get_species_from_line1() {
        let source = "./test/store/species/species5_import.json";
        let destination = "./test/birds.json";
        copy(source,destination).expect("Failed to copy");
        let birds = Species::import(destination).unwrap();
        remove_file(destination).expect("Cleanup test failed");

        let line = "Maximillian birdus	Cassius The Birdus	Birdus Cassius The	bicat	Ordering	Familing					";
        let species = get_species_from_line(line, &birds);
        if species.is_ok(){
            assert_eq!(species.unwrap().family,"Familing");
        }
    }
    
    
    #[ignore]
    #[test]
    fn t042_import_csv1() {
        let mut birds: BTreeMap<String, Species> = BTreeMap::new();

        let source = "./test/store/species/2_species.csv";
        let destination = "./test/birdings.csv";
        copy(source,destination).expect("Failed to copy");
        let imp = Species::import_csv(destination, &mut birds);
        remove_file(destination).expect("Cleanup test failed");

        if imp.is_ok(){
            assert_eq!(birds.len(),2);
        }
    }

    #[ignore]
    #[test]
    fn t042_import_csv2() {
        let mut birds: BTreeMap<String, Species> = BTreeMap::new();

        let source = "./test/store/species/bad1.csv";
        let destination = "./test/birdings.csv";
        copy(source,destination).expect("Failed to copy");
        let imp = Species::import_csv(destination, &mut birds);
        remove_file(destination).expect("Cleanup test failed");

        assert_eq!(imp.is_err(),true);
    }

    #[ignore]
    #[test]
    fn t043_import_csv3() {
        let mut birds: BTreeMap<String, Species> = BTreeMap::new();

        let source = "./test/store/species/bad2.csv";
        let destination = "./test/birdings.csv";
        copy(source,destination).expect("Failed to copy");
        let imp = Species::import_csv(destination, &mut birds);
        remove_file(destination).expect("Cleanup test failed");

        assert_eq!(imp.is_err(),true);
    }
    
    
    #[ignore]
    #[test]
    fn t044_deconstruct2() {
        // let source = "./test/store/species/to_use.json";
        // let destination = "./test/birds_import.json";
        // copy(source,destination).expect("Failed to copy");
        // let mut birds = Species::import(destination).unwrap();
        // remove_file(destination).expect("Cleanup test failed");
        
        let arg = "n=alexis carrus#s=gladus maximus#m=Sturnidae (Starlings)#r=Passeriformes#u=rare";
        
        let sname = deconstruct_bird_str('n', &arg);
        assert_eq!(sname.unwrap(),"alexis carrus");
        
        let status = deconstruct_bird_str('u', &arg);
        assert_eq!(status.unwrap(),"rare");
        


    }








































} // End of Test





