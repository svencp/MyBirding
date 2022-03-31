mod library;

use library::bird_sightings::*;
use std::collections::{BTreeMap};
use std::process::exit;
use library::settings::*;
use library::my_file_funcs::*;
use library::bird_species::*;
use library::help::*;
use std::fs::copy;
use std::path::Path;
use std::env;
use termion::{color, style};
use thousands::{Separable};
use std::time::{SystemTime};
use crate::library::bird_species_box::*;
use crate::library::bird_species_support::*;
use crate::library::bird_sightings_supp::*;


const VERSION: &str = env!("CARGO_PKG_VERSION");



#[warn(unused_variables)]
#[rustfmt::skip]
fn main() {
    let now = SystemTime::now();
    let arguments: Vec<String> = env::args().collect();
    let mut command = None;
    let mut sub1 = None;
    let mut sub2 = None;
    // let mut sub3 = None;

    // It seems I need to do this,otherwise temporary variables get dropped
    match arguments.len() {
        2 => {
            command = Some(arguments[1].to_lowercase().trim().to_owned());
        },
        3 => {
            command = Some(arguments[1].to_lowercase().trim().to_owned());
            sub1 = Some(arguments[2].trim().to_owned());
        },
        4 => {
            command = Some(arguments[1].to_lowercase().trim().to_owned());
            sub1 = Some(arguments[2].trim().to_owned());
            sub2 = Some(arguments[3].trim().to_owned());
        }
        5 => {
            command = Some(arguments[1].to_lowercase().trim().to_owned());
            sub1 = Some(arguments[2].trim().to_owned());
            sub2 = Some(arguments[3].trim().to_owned());
            // sub3 = Some(arguments[4].trim().to_owned());
        },

        _ => { () }
    }

    let mut sbirds:     BTreeMap<String, Species>;
    let mut birds:  BTreeMap<String, Species> = BTreeMap::new();
    let mut file_change_birds = false;
    let mut file_change_sightings = false;
    let mut sightings: Vec<Sightings> = Vec::new();
    
    println!("");
    
    // Options
    let mut options = SettingsText::new(OPTIONS_FILENAME);
    
    // Birds
    let destination_birds = SPECIES_BIN_FILENAME;
    // **************************************************************************************                       Remove Later
    if ! Path::new(destination_birds).exists() {
        let source_birds = "./test/store/species/species.bin";
        copy(source_birds,destination_birds).expect("Failed to copy");
    }
    let birds_file = Species::load(destination_birds);
    let mut birds_file_ok = false;
    if birds_file.is_ok() {
        birds_file_ok = true;

        // This BTreeMap has the code as the key
        birds = birds_file.unwrap();
    }
    // This BTreeMap has the sname as the key
    sbirds = make_sname_btree(&birds);
    
    
    
    // Sightings
    let destination_sightings = SIGHTINGS_BIN_FILENAME;
    // **************************************************************************************                       Remove Later
    if ! Path::new(destination_sightings).exists() {
        let source_sights = "./test/store/sightings/sightings.bin";
        copy(source_sights,destination_sightings).expect("Failed to copy");
    }
    let sightings_file = Sightings::load(destination_sightings);
    let mut sightings_file_ok = false;
    if sightings_file.is_ok() {
        sightings_file_ok = true;
        sightings = sightings_file.unwrap();
    }
    



    // There are no arguments
    if arguments.len() < 2 {
        if birds_file_ok {
            let message = format!("You have a species database with {} birds in it.",birds.len().separate_with_spaces());
            feedback(Feedback::Info, message);
        } else {
            let message = format!("You have no species database.");
            feedback(Feedback::Warning, message);
        }
        if sightings_file_ok {
            let message = format!("You have a sightings database with {} records in it.",sightings.len().separate_with_spaces());
            feedback(Feedback::Info, message);
        } else {
            let message = format!("You have no observations in your sightings database.");
            feedback(Feedback::Warning, message);

        }

    // Too many arguments
    } else if arguments.len() >= 5 {
        let message = format!("There are too many arguments.");
        feedback(Feedback::Warning, message);
    
    
    //majority of arguments    
    } else {

        match command.unwrap().as_str() {
            "b"   => {     
                if_birds_length_is_zero(&birds);
                
                if sub1.is_some(){
                    let yes = what_code(&sub1.unwrap(), &mut birds);

                    match yes.what {
                        Code::Code => {
                            show_bird(yes, &mut options, &birds, &sightings);
                        }
                        Code::Number => {
                            show_bird(yes, &mut options, &birds, &sightings);
                        }
                        // if garbage was put in
                        Code::Neither => {
                            error_neither();
                        }
                    }
                }
                // sub1 is None -> Show lastSpeciesViewed
                else {
                    let no = get_last_species_viewed(&options, &mut birds);
                    show_bird(no, &mut options, &birds, &sightings);
                }                             
            } // end of "b"
            
            
            "ba"  => {
                // Need some clones as one cannot borrow mutably more than once
                let len = birds.clone().len();
                let mut cl_options = options.clone();
                let mut cl_birds = birds.clone();
                
                if sub1.is_some(){
                   
                    let result = add_species(sub1.unwrap(), 
                                &mut options, &mut birds, &mut sbirds, &sightings);

                    if result.is_err(){
                        if len == 0 {
                            show_template_bird(&mut cl_options, &cl_birds);

                        } 
                        // get last or zero position bird
                        else {
                            let no = get_last_species_viewed(&cl_options, &mut cl_birds);
                            show_bird(no, &mut cl_options, &cl_birds, &sightings);
                        }
                        
                        let message = result.err().unwrap();
                        feedback(Feedback::Error, message);
                        let message = format!("Showing last bird viewed is an indication of fields needed");
                        feedback(Feedback::Info, message);

                    } else {
                        file_change_birds = true;
                    }

                }  else {          // Is None
                    let no = get_last_species_viewed(&cl_options, &mut cl_birds);
                    show_bird(no, &mut cl_options, &cl_birds, &sightings);

                    let message = format!("You have not provided any details for adding a bird to the database");
                    feedback(Feedback::Warning, message);
                    let message = format!("Showing last bird viewed is an indication of fields needed");
                    feedback(Feedback::Info, message);
                    
                } //end of sub1.is_none()                                
            }// end of "ba"
            

            "bb"  => {
                let read = &options.get_number("lastSpeciesViewed");

                if *read == 0 {
                    show_bird_number(0, &mut options, &mut birds, &sightings);
                    let message = format!("You are at the beginning of the species database.",);
                    feedback(Feedback::Info, message); 
                } else {
                    let wanted = read - 1;
                    show_bird_number(wanted, &mut options, &mut birds, &mut sightings);
                }

                if sub1.is_some(){
                    let message = format!("bb does not take any arguments itsself.");
                    feedback(Feedback::Warning, message);
                }  //end of sub1.is_none()                                
            }// end of "bb"
            
            "bd"  => {
                if_birds_length_is_zero(&birds);
                
                if sub1.is_some(){
                    let yes = what_code(&sub1.unwrap(), &mut birds);
                    
                    match yes.what {
                        
                        Code::Code | Code::Number => {
                            if delete_bird(yes, &mut options, &mut birds, &mut sightings).is_ok(){
                                file_change_birds = true;
                                file_change_sightings = true;
                            }
                        }
                        
                        // if garbage was put in
                        Code::Neither => {
                            error_neither();
                            exit(17);
                        }
                    }
                } 
                // Is None 
                else {          
                    error_neither();
                    exit(17);
                } //end of sub1.is_none()                  
            } // end of "bd"
            
            
            "be"  => {
                if_birds_length_is_zero(&birds);
                
                if sub1.is_some(){
                    let yes = what_code(&sub1.unwrap(), &mut birds);
                    
                    match yes.what {
                        
                        Code::Code | Code::Number => {
                            if sub2.is_some(){
                                let arg_sub2 = sub2.unwrap();
                                
                                let result = edit_bird(arg_sub2, yes.clone(), 
                                &mut options, &mut birds, &mut sightings);
                                if result.is_ok(){
                                    
                                    file_change_birds = true;
                                    file_change_sightings = true;
                                }
                                // Result not good
                                else {
                                    error_last_arg_edit_invalid(yes.clone().species.unwrap().name);
                                }
                            }
                            // Sub2 is None
                            else {
                                error_last_arg_edit(yes.species.unwrap().name);
                            }
                        }
                        
                        // if garbage was put in
                        Code::Neither => {
                            error_neither();
                            exit(17);
                        }
                    }
                }
                
                // sub1 is None -> Show lastSpeciesViewed
                else {
                    let number: usize = options.get_number("lastSpeciesViewed");
                    show_bird_number(number, &mut options, &mut birds, &mut sightings);
                }  
            } // end of "be"
            
            "bex"  => {
                if_birds_length_is_zero(&birds);
                
                if sub1.is_some(){
                    let arg = sub1.unwrap().to_lowercase().trim().to_owned();
                    if arg != "csv"{
                        let message = format!("Wrong argument given: (either blank or csv)");
                        feedback(Feedback::Error, message);
                        exit(17);
                    }
                    let path = &Species::export_path("csv", &mut options);
                    let result = Species::export_csv(path, &birds);
                    if result.is_err(){
                        let message = result.err().unwrap();
                        feedback(Feedback::Error, message); 
                        exit(17)
                    }
                }
                
                // sub1 is None -> Default to json
                else {
                    //Build the file name 
                    let path = &Species::export_path("json", &mut options);
                    
                    //Export json
                    let result = Species::export(path, &birds);
                    if result.is_err(){
                        let message = result.err().unwrap();
                        feedback(Feedback::Error, message); 
                        exit(17)
                    }
                    let number = &birds.len();
                    let message = format!("{} records have been exported as a json file.",number);
                    feedback(Feedback::Info, message);
                }  
            } // end of "bex"
            
            
            "bf"  => {
                let read = &options.get_number("lastSpeciesViewed");
                let last = birds.len();
                
                if *read == last - 1 {
                    show_bird_number(*read, &mut options, &mut birds, &mut sightings);
                    let message = format!("You are at the end of the species database.",);
                    feedback(Feedback::Info, message); 
                } else {
                    let wanted = read + 1;
                    show_bird_number(wanted, &mut options, &mut birds, &mut sightings);
                }
                
                if sub1.is_some(){
                    let message = format!("bb does not take any arguments itsself.");
                    feedback(Feedback::Warning, message);
                }  //end of sub1.is_none()   
                
            } // end of "bf"
            
            
            "bim"  => {
                if sub1.is_some(){
                    let file = sub1.unwrap().trim().to_owned();
                    let ext = get_extension_from_filename(&file);
                    if ext.is_none(){
                        let message = format!("Wrong file extension given: (either json or csv)");
                        feedback(Feedback::Error, message);
                        exit(17);
                    }
                    
                    // lets clear out birds
                    birds = BTreeMap::new();
                    
                    match ext.unwrap() {
                        "csv" => {
                            let result = Species::import_csv(&file, &mut birds);
                            if result.is_err(){
                                let message = result.err().unwrap();
                                feedback(Feedback::Error, message);
                                exit(17);
                            }
                            file_change_birds = true;
                        }
                        
                        "json" => {
                            let result = Species::import(&file);
                            if result.is_err(){
                                let message = result.err().unwrap();
                                feedback(Feedback::Error, message);
                                exit(17);
                            }
                            birds = result.unwrap();
                            file_change_birds = true;
                        }
                        
                        // Other ones
                        _ => {
                            let message = format!("Wrong file extension given: (either json or csv)");
                            feedback(Feedback::Error, message);
                            exit(17);
                        }
                    }
                }
                // sub1 is NONE
                else {
                    let message = format!("bim needs another argument, which is the file name.");
                    feedback(Feedback::Error, message);
                    exit(17);
                }
                
            } // end of "bimp"
            
            
            "bl"  => {
                if_birds_length_is_zero(&birds);
                
                // Deal with sub1               
                if sub1.is_some(){
                    
                    let code = sub1.unwrap();
                    let result = show_code_range(&code, &mut options, &mut birds);
                    if result.is_err(){
                        let message = result.err().unwrap();
                        feedback(Feedback::Info, message);
                    }
                }  else {          // Is None
                    let message = format!("You should put in a shortened code as well.");
                    feedback(Feedback::Info, message);
                } //end of sub1.is_none()      
            } // end of "bl"
            

            // @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@    h     @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@                
            // @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@          @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@                
            "-help"|"-h"|"h"|"help"|"--help"|"--h" => {
                show_help(options.clone());
            }
            
            
            // @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@    o     @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@                
            // @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@          @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@                
            "o"    =>  {
                if_sightings_length_is_zero(&sightings);
                
                let number: usize;
                // Deal with sub1
                if sub1.is_some(){
                    let yes = what_number(&sub1.unwrap(),&sbirds, &sightings);
                    
                    // User supplied a number
                    if yes.is_number {
                        show_sightings_number(yes, &mut options, &sbirds, &mut sightings);
                    }      
                    
                    // it is not a legit number
                    else {
                        error_no_legit_number();
                    }
                }  
                // Is None
                else {  
                    // Normally the user puts in number, but in this case, the system does, so
                    // we have to add one to it.
                    number = options.get_number("lastSightingViewed") + 1;
                    let yes = what_number(&number.to_string(), &sbirds, &sightings);
                    
                    if yes.is_number {
                        show_sightings_number(yes, &mut options, &sbirds, &mut sightings);
                    }      
                } //end of sub1.is_none()
            }  // end of "o"
            
            
            "oa" => {
                let last_10 = get_last_10(&sightings);
                
                if sub1.is_some(){
                    let result = add_sighting(last_10, &sub1.unwrap(), 
                    &birds, &sbirds, &mut sightings);
                    
                    if result.is_err(){
                        let message = result.err().unwrap();
                        feedback(Feedback::Error, message);
                        exit(17);
                    }
                    
                    // Success
                    let new_10 = get_last_10(&sightings);
                    display_last_10(&options, new_10);
                    let non_zero = result.unwrap() + 1;
                    let yes = what_number(&non_zero.to_string(), &sbirds, &sightings);
                    
                    show_sightings_number(yes.clone(), &mut options, &sbirds, &mut sightings);
                    oa_display_addition(yes, sbirds, sightings.clone());
                    
                    file_change_sightings = true;
                }
                // Is None
                else {
                    let new_10 = get_last_10(&sightings);
                    display_last_10(&options, new_10);
                    
                    let number = options.get_number("lastSightingViewed") + 1;
                    let yes = what_number(&number.to_string(), &sbirds, &sightings);
                    
                    if yes.is_number {
                        show_sightings_number(yes, &mut options, &sbirds, &mut sightings);
                    }  
                    
                    let message = format!("oa needs an argument that contains the code for the sighting to be added.");
                    feedback(Feedback::Warning, message);
                    exit(17);
                }
                
            }
            
            
            "ob"  => {
                if_sightings_length_is_zero(&sightings);
                
                // Lets pretend the user put this number in (i.e. add 1 to it)
                let read = &options.get_number("lastSightingViewed") + 1;
                let no = what_number(&read.to_string(), &sbirds, &sightings);
                
                if no.number.unwrap() == 0 {
                    show_sightings_number(no, &mut options, &sbirds, &mut sightings);
                    let message = format!("You are at the beginning of the sightings database.",);
                    feedback(Feedback::Info, message); 
                } else {
                    // By running what_number again , another 1 will be deducted from index 
                    let sub = &no.number.unwrap().to_string();
                    let yes = what_number(sub, &sbirds, &sightings);
                    show_sightings_number(yes, &mut options, &sbirds, &mut sightings);
                }
                
                if sub1.is_some(){
                    let message = format!("ob does not take any arguments itsself.");
                    feedback(Feedback::Warning, message);
                }  //end of sub1.is_none()                                
            }// end of "ob"
            
            
            "od"  => {
                if_sightings_length_is_zero(&sightings);
                
                if sub1.is_some(){
                    let yes = what_number(&sub1.unwrap(), &sbirds, &sightings);
                    if !yes.is_number {
                        error_no_legit_number();
                        exit(17);
                    }
                    
                    delete_sighting(yes, &mut options, &mut sbirds, &mut sightings);
                    file_change_sightings = true;
                } 
                // Is None 
                else {          
                    error_neither();
                    exit(17);
                } //end of sub1.is_none()                  
            } // end of "bd"
            
            
            "oe" => {
                if_sightings_length_is_zero(&sightings);
                let last_10 = get_last_10(&sightings);
                
                if sub1.is_some(){
                    let old = what_number(&sub1.unwrap(), &sbirds, &sightings);
                    if !old.is_number {
                        error_no_legit_number();
                    }
                    if sub2.is_some(){
                        let result = edit_sighting(&sub2.unwrap(), old.clone(), 
                        &birds, &sbirds, &mut sightings);
                        if result.is_err(){
                            let message = result.err().unwrap();
                            feedback(Feedback::Error, message);
                            exit(17);
                        }
                        let new_pos = result.unwrap() + 1;
                        let new = what_number(&new_pos.to_string(), &sbirds, &sightings);
                        
                        let rev_10 = get_last_10(&sightings);
                        display_last_10(&options, rev_10);
                        
                        show_sightings_number(new.clone(), &mut options, &sbirds, &mut sightings);
                        show_edit(old.clone(), new.clone());
                        
                        file_change_sightings = true;
                    }
                    // sub2 is None
                    else {
                        display_last_10(&options, last_10);
                        show_sightings_number(old.clone(), &mut options, &sbirds, &mut sightings);
                        error_last_sight_arg_edit((old.number.unwrap() + 1).to_string());
                    }
                }
                
                // sub1 is None -> Show lastSightingViewed
                else {
                    let last_sight = (options.get_number("lastSightingViewed") + 1).to_string();
                    let yes = what_number(&last_sight, &sbirds, &sightings);
                    show_sightings_number(yes, &mut options, &sbirds, &mut sightings);
                }  
            }
            
            
            "oex"  => {
                if_sightings_length_is_zero(&sightings);
                
                if sub1.is_some(){
                    let arg = sub1.unwrap().to_lowercase().trim().to_owned();
                    if arg != "csv"{
                        let message = format!("Wrong argument given: (either blank or csv)");
                        feedback(Feedback::Error, message);
                        exit(17);
                    }
                    let path = &Sightings::export_path("csv", &mut options);
                    let result = Sightings::export_csv(path, &sightings);
                    if result.is_err(){
                        let message = result.err().unwrap();
                        feedback(Feedback::Error, message); 
                        exit(17)
                    }
                }
                
                // sub1 is None -> Default to json
                else {
                    //Build the file name 
                    let path = &Sightings::export_path("json", &mut options);
                    
                    //Export json
                    let result = Sightings::export(path, &sightings);
                    if result.is_err(){
                        let message = result.err().unwrap();
                        feedback(Feedback::Error, message); 
                        exit(17)
                    }
                    let number = &sightings.len();
                    let message = format!("{} records have been exported as a json file.",number);
                    feedback(Feedback::Info, message);
                }  
            } // end of "bex"
            
            
            "of"  => {
                if_sightings_length_is_zero(&sightings);
                
                // Lets pretend the user put this number in (i.e. add 1 to it)
                let read = &options.get_number("lastSightingViewed") + 1;
                let no = what_number(&read.to_string(), &sbirds, &sightings);
                let max = sightings.len();
                
                if no.number.unwrap() == max - 1 {
                    show_sightings_number(no, &mut options, &sbirds, &mut sightings);
                    let message = format!("You are at the end of the sightings database.",);
                    feedback(Feedback::Info, message); 
                } else {
                    // with all the subtractions taking place, we have to add 2 here
                    let sub = &(no.number.unwrap() + 2).to_string();
                    let yes = what_number(sub, &sbirds, &sightings);
                    show_sightings_number(yes, &mut options, &sbirds, &mut sightings);
                }
                
                if sub1.is_some(){
                    let message = format!("ob does not take any arguments itsself.");
                    feedback(Feedback::Warning, message);
                }  //end of sub1.is_none()                                
            }// end of "ob"
            
            
            
            "oim"  => {
                if sub1.is_some(){
                    let file = sub1.unwrap().trim().to_owned();
                    let ext = get_extension_from_filename(&file);
                    if ext.is_none(){
                        let message = format!("Wrong file extension given: (either json or csv)");
                        feedback(Feedback::Error, message);
                        exit(17);
                    }
                    
                    match ext.unwrap() {
                        "csv" => {
                            let result = Sightings::import_csv(&file, &sbirds);
                            if result.is_err(){
                                let message = result.err().unwrap();
                                feedback(Feedback::Error, message);
                                exit(17);
                            }
                            sightings = result.unwrap();
                            file_change_sightings = true;
                        }
                        
                        "json" => {
                            let result = Sightings::import(&file);
                            if result.is_err(){
                                let message = result.err().unwrap();
                                feedback(Feedback::Error, message);
                                exit(17);
                            }
                            sightings = result.unwrap();
                            file_change_sightings = true;
                        }
                        
                        // Other ones
                        _ => {
                            let message = format!("Wrong file extension given: (either json or csv)");
                            feedback(Feedback::Error, message);
                            exit(17);
                        }
                    }
                }
                // sub1 is NONE
                else {
                    let message = format!("bimp needs another argument, which is the file name.");
                    feedback(Feedback::Error, message);
                    exit(17);
                }
                
            } // end of "oim"
            
            
            "oz" => {
                if_sightings_length_is_zero(&sightings);
                
                let last_10 = get_last_10(&sightings);
                display_last_10(&options, last_10);
                
                let yes = what_number(&sightings.len().to_string(), &sbirds, &sightings);
                if yes.is_number {
                    show_sightings_number(yes, &mut options, &sbirds, &mut sightings);
                } 
                
                if sub1.is_some(){
                    let message = format!("oz does not take any arguments itsself.");
                    feedback(Feedback::Warning, message);
                }  //end of sub1.is_none()                                
            }// end of "oz"
            
            
            
            // @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@    s     @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@                
            // @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@          @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ 
            
            "so" => {
                if_birds_length_is_zero(&birds);
                if_sightings_length_is_zero(&sightings);
                
                if sub1.is_some(){
                    
                    let r_slice = get_searched_slice_of_sightings(&sub1.unwrap(), &sbirds, &sightings);
                    if r_slice.is_err(){
                        let message = r_slice.err().unwrap();
                        feedback(Feedback::Error, message);
                        exit(17);
                    }
                    if r_slice.clone().unwrap().1.len() == 0{
                        let message = format!("The search results ended with no success.");
                        feedback(Feedback::Info, message);
                        exit(17);
                    }
                    display_search_results( &options,  &sbirds,r_slice.clone().unwrap().0, r_slice.unwrap().1)
                    
                }  
                // Is None
                else {
                    let message = format!("You have not provided any substring for a query.");
                    feedback(Feedback::Error, message);
                }
            }//end of "sol"
            
            // @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@    v     @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@                
            // @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@          @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@                
            
            "v"|"V"|"-v"|"-V"|"version"|"Version"|"VERSION" => {
                println!("mybirding version: {}", VERSION);
            }
            
            
            
            
            
            
            
            
            
            
            
            
            


            // Not a valid first argument 
            _   => {
                let message = format!("Not a valid first argument ->  {}",arguments[1]);
                feedback(Feedback::Warning, message);
            } //end of _   


















        } //  end of match command {














    } // end of else   if arguments.len() 









    save_files(file_change_birds,birds, file_change_sightings, sightings);        
    
    cleanup(options.clone());    
        

    // Show reponse times
    let res_bool =  options.get_bool("showResponseTimes");
    if res_bool {
        show_response(now,options);
    }
    
    
} // end of Main



//  ======================================================================================================================================  









































// This function saves the option file
pub fn cleanup(options: SettingsText){
    if options.export(OPTIONS_FILENAME).is_err(){
        let message = format!("Problem with writing options file");
        feedback(Feedback::Error, message)
    }
    
}        


// Function to update the data files
pub fn save_files(b_birds: bool, birds: BTreeMap<String,Species>, b_sightings: bool, sightings: Vec<Sightings>){
    if b_birds {
        let result = Species::save(SPECIES_BIN_FILENAME, &birds);
        if result.is_err(){
            let message = format!("Problem with writing birds csp file");
            feedback(Feedback::Error, message)
        } else {
            let records = &birds.len();
            let message = format!("{} bird species records have been saved.", records);
            feedback(Feedback::Info, message)
        }
    }
    
    if b_sightings{
        let result = Sightings::save(SIGHTINGS_BIN_FILENAME, &sightings);
        if result.is_err(){
            let message = format!("Problem with writing sightings csp file");
            feedback(Feedback::Error, message)
        } else {
            let records = &sightings.len();
            let message = format!("{} bird sightings records have been saved.", records);
            feedback(Feedback::Info, message)
        }
    }
}        


// Function to show response times
pub fn show_response(now: SystemTime, options: SettingsText){
    let my_normal_gray: color::Rgb = options.clone().get_color("myNormalGray");
    let duration = now.elapsed().unwrap().as_millis();
    let message = format!("Program runtime is: {:?}ms", duration);
    print!("{}{}{}", color::Fg(my_normal_gray), message, style::Reset); 
}        











        