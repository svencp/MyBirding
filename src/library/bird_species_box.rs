/*
    My module for drawing the bird species on the terminal

    2022.01.20  Sven Ponelat

*/

use crate::library::bird_sightings::*;
use crate::library::my_file_funcs::*;
use crate::library::bird_species::*;
use crate::library::settings::*;
use draw_box::{Char};
use termion::{color, style};
use thousands::{Separable};
use std::collections::BTreeMap;
use super::bird_species_support::CodeOrNumber;


const SPACE:    &str       =   " ";
const MARGIN:   &str       = "   ";  
const PADDING1: &str       = "   ";
const PADDING2: &str       =  "  ";

const PAD1_LEN: usize        = PADDING1.len();
const PAD2_LEN: usize        = PADDING2.len();
const R_SIDE_TEXT_LEN: usize = 18;

const BLOCK_LEN:      usize     = 123;
const FAMILY_LEN:     usize     =  60;
const NAME_LEN:       usize     =  40;
const INDEX_HALF_LEN: usize     =   4;



// A function to show the data of the species in a box
pub fn show_species(observations: usize, bird_number: usize, options: &SettingsText, bird: &Species) -> SettingsText {
    let mut tempo = options.clone();
    let my_green: color::Rgb = options.clone().get_color("myGreen");
    let my_normal_gray: color::Rgb = options.clone().get_color("myNormalGray");
    let style = tempo.get_box_style("speciesBoxStyle");
    let show_index = options.get_bool("showSpeciesIndex");

    let ul = Char::upper_left(style);
    let ur = Char::upper_right(style);
    let ll = Char::lower_left(style);
    let lr = Char::lower_right(style);
    let v  = Char::vertical(style);
    let h  = Char::horizontal(style);
    
    let non_zero = bird_number + 1;
    let index = justify(non_zero.separate_with_spaces(), 2 * INDEX_HALF_LEN + 1, Justify::Center);         
    // let total = justify(total_birds.separate_with_spaces(), 10, Justify::Center); 
    let observations = justify(observations.separate_with_spaces(), 10, Justify::Left);      

    // Top line
    print!("{}{}",color::Fg(my_green),"\n\n");
    print!("{}{}", MARGIN, ul); 
    print!("{}", &repeat_char(h, BLOCK_LEN));   
    print!("{}{}", ur, "\n");    
    
    print!("{}{}{}{}\n", MARGIN, v, &repeat_char(SPACE.to_string(), BLOCK_LEN), v);                  // empty body line
    print!("{}{}{}{}\n", MARGIN, v, &repeat_char(SPACE.to_string(), BLOCK_LEN), v);                  // empty body line
    
    // Name - Observations
    let name = justify(bird.name.clone(), NAME_LEN, Justify::Left);         
    print!("{}{}{}", MARGIN, v, PADDING1); 
    let name_text = justify("Name:".to_string(), R_SIDE_TEXT_LEN, Justify::Left);         
    let observation_text = justify("Observations:".to_string(), R_SIDE_TEXT_LEN, Justify::Left);         
    let obs = justify(observations, NAME_LEN, Justify::Left);  
    // lets make function to underline
    underline('N', &name_text);
    print!("{}{}{}", color::Fg(my_normal_gray), name, style::Reset); 
    print!("{}{}{}", color::Fg(my_green), observation_text, style::Reset); 
    print!("{}{}{}", color::Fg(my_normal_gray), obs, style::Reset); 
    let name_pad_len = BLOCK_LEN - PAD1_LEN - PAD2_LEN - (2 * R_SIDE_TEXT_LEN ) - NAME_LEN - NAME_LEN;
    let add_space = &repeat_char(SPACE.to_string(), name_pad_len);
    print!("{}{}{}{}\n", color::Fg(my_green), PADDING2, add_space,  v); 
    
    print!("{}{}{}{}\n", MARGIN, v, &repeat_char(SPACE.to_string(), BLOCK_LEN), v);                  // empty body line
    
    // Sname
    let sname = justify(bird.sname.clone(), NAME_LEN, Justify::Left);         
    print!("{}{}{}", MARGIN, v, PADDING1); 
    let sname_text = justify("Scientific Name:".to_string(), R_SIDE_TEXT_LEN, Justify::Left);         
    underline('S', &sname_text);
    print!("{}{}{}{}", color::Fg(my_normal_gray), style::Italic, sname, style::Reset); 
    let sname_pad_len: usize = BLOCK_LEN - PAD1_LEN - R_SIDE_TEXT_LEN - NAME_LEN;
    print!("{}{}{}\n", color::Fg(my_green), &repeat_char(SPACE.to_string(), sname_pad_len), v); 
    
    print!("{}{}{}{}\n", MARGIN, v, &repeat_char(SPACE.to_string(), BLOCK_LEN), v);                  // empty body line
    
    
    // Family
    let family = justify(bird.family.clone(), FAMILY_LEN, Justify::Left);         
    print!("{}{}{}", MARGIN, v, PADDING1); 
    let family_text = justify("Family:".to_string(), R_SIDE_TEXT_LEN, Justify::Left);         
    underline('m', &family_text);
    print!("{}{}{}", color::Fg(my_normal_gray), family, style::Reset); 
    let family_pad_len: usize = BLOCK_LEN - PAD1_LEN - R_SIDE_TEXT_LEN  - FAMILY_LEN;
    print!("{}{}{}\n", color::Fg(my_green), &repeat_char(SPACE.to_string(), family_pad_len), v); 
    
    print!("{}{}{}{}\n", MARGIN, v, &repeat_char(SPACE.to_string(), BLOCK_LEN), v);                  // empty body line
    
    
    // Order - List
    let order = justify(bird.order.clone(), NAME_LEN, Justify::Left);         
    print!("{}{}{}", MARGIN, v, PADDING1); 
    let order_text = justify("Order:".to_string(), R_SIDE_TEXT_LEN, Justify::Left);         
    let list_text = justify("List:".to_string(), R_SIDE_TEXT_LEN, Justify::Left);  
    let list = justify(bird.list.clone(), NAME_LEN, Justify::Left);   
    underline('r', &order_text);
    print!("{}{}{}", color::Fg(my_normal_gray), order, style::Reset); 
    print!("{}", color::Fg(my_green));
    underline('L', &list_text);
    print!("{}", style::Reset); 
    print!("{}{}{}", color::Fg(my_normal_gray), list, style::Reset); 
    let order_pad_len: usize = BLOCK_LEN - PAD1_LEN - PAD2_LEN - (2 * R_SIDE_TEXT_LEN) - NAME_LEN - NAME_LEN;
    let add_space = &repeat_char(SPACE.to_string(), order_pad_len);
    print!("{}{}{}{}\n", color::Fg(my_green), PADDING2, add_space,  v);
    
    print!("{}{}{}{}\n", MARGIN, v, &repeat_char(SPACE.to_string(), BLOCK_LEN), v);                  // empty body line
    

    // Status
    let status = justify(bird.status.clone(), NAME_LEN, Justify::Left);
    let aname = justify(bird.aname.clone(), NAME_LEN, Justify::Left);  
    print!("{}{}{}", MARGIN, v, PADDING1); 
    let status_text = justify("Status:".to_string(), R_SIDE_TEXT_LEN, Justify::Left);         
    let aname_text = justify("Alt. Name:".to_string(), R_SIDE_TEXT_LEN, Justify::Left);         
    underline('u', &status_text);
    print!("{}", style::Reset); 
    print!("{}{}{}", color::Fg(my_normal_gray), status, style::Reset); 
    print!("{}", color::Fg(my_green)); 
    let status_pad_len: usize = BLOCK_LEN - PAD1_LEN - PAD2_LEN - (2 * R_SIDE_TEXT_LEN) - NAME_LEN - NAME_LEN;
    let add_space = &repeat_char(SPACE.to_string(), status_pad_len);
    underline('e', &aname_text);
    print!("{}", style::Reset); 
    print!("{}{}{}", color::Fg(my_normal_gray), aname, style::Reset); 
    print!("{}{}{}{}\n", color::Fg(my_green), PADDING2, add_space,  v);
    
    print!("{}{}{}{}\n", MARGIN, v, &repeat_char(SPACE.to_string(), BLOCK_LEN), v);                  // empty body line
    
    
    // Code - Alt. Code
    let code = justify(bird.code.clone(), NAME_LEN, Justify::Left);         
    print!("{}{}{}", MARGIN, v, PADDING1); 
    let code_text = justify("Code:".to_string(), R_SIDE_TEXT_LEN, Justify::Left);         
    underline('C', &code_text);
    print!("{}{}{}", color::Fg(my_normal_gray), code, style::Reset); 
    // let mut acode = "".to_string();
    let acode_text = justify("Alt. Code:".to_string(), R_SIDE_TEXT_LEN, Justify::Left);         
    let acode = justify(bird.acode.clone(), NAME_LEN, Justify::Left); 
    print!("{}", color::Fg(my_green)); 
    print!("{}", &acode_text); 
    // underline('d', &acode_text);
    let code_pad_len: usize = BLOCK_LEN - PAD1_LEN - PAD2_LEN - (2 * R_SIDE_TEXT_LEN) - NAME_LEN - NAME_LEN;
    let add_space = &repeat_char(SPACE.to_string(), code_pad_len);
    print!("{}", style::Reset); 
    print!("{}{}{}", color::Fg(my_normal_gray), acode, style::Reset); 
    print!("{}{}{}{}\n", color::Fg(my_green), PADDING2, add_space,  v);
    
    print!("{}{}{}{}\n", MARGIN, v, &repeat_char(SPACE.to_string(), BLOCK_LEN), v);                  // empty body line
    
    
    // Fname - Alt. Fname
    let fname = justify(bird.fname.clone(), NAME_LEN, Justify::Left); 
    let afname = justify(bird.afname.clone(), NAME_LEN, Justify::Left); 
    print!("{}{}{}", MARGIN, v, PADDING1); 
    let fname_text = justify("Family Name:".to_string(), R_SIDE_TEXT_LEN, Justify::Left);         
    print!("{}{}", fname_text, style::Reset); 
    print!("{}{}{}", color::Fg(my_normal_gray), fname, style::Reset); 
    let afname_text = justify("Alt. Family Name:".to_string(), R_SIDE_TEXT_LEN, Justify::Left);         
    let fname_pad_len: usize = BLOCK_LEN - PAD1_LEN - PAD2_LEN - (2 * R_SIDE_TEXT_LEN) - NAME_LEN - NAME_LEN;
    let add_space = &repeat_char(SPACE.to_string(), fname_pad_len);
    print!("{}{}{}", color::Fg(my_green), afname_text, style::Reset); 
    print!("{}{}{}", color::Fg(my_normal_gray), afname, style::Reset); 
    print!("{}{}{}{}\n", color::Fg(my_green), PADDING2, add_space,  v);
    
    print!("{}{}{}{}\n", MARGIN, v, &repeat_char(SPACE.to_string(), BLOCK_LEN), v);                  // empty body line
    print!("{}{}{}{}\n", MARGIN, v, &repeat_char(SPACE.to_string(), BLOCK_LEN), v);                  // empty body line
    
    
    // Bottom line
    let h1  = Char::horizontal(style);
    let h2  = Char::horizontal(style);
    print!("{}{}",style::Reset, color::Fg(my_green));
    print!("{}{}", MARGIN, ll); 
    if show_index {
        let lhs:usize = ( BLOCK_LEN / 2 ) -  INDEX_HALF_LEN ;
        print!("{}", &repeat_char(h1, lhs-1));   
        print!("{}", index);   
        print!("{}", &repeat_char(h2, lhs+1));   
        print!("{}{}", lr, "\n");    
    } else {
        print!("{}{}\n",&repeat_char(h1, BLOCK_LEN), lr);                  // empty body line
    }
    
    print!("{}{}",color::Fg(my_green),"\n");
    
    return tempo
    
    
}  // end of show_species


#[rustfmt::skip]
// A function to prepare data for Show_species function
pub fn show_bird<'a>(con: CodeOrNumber, options: &'a mut SettingsText, birds: &BTreeMap<String,Species>, 
                 sightings: &Vec<Sightings>) -> &'a mut SettingsText {

    let species = birds.get_key_value(&con.clone().code.unwrap());
    let observations = get_array_of_sname_indices_from_records(&species.clone().unwrap().1.sname, sightings);

    // lets show the species
    *options = show_species( observations.len(),
                              con.clone().number.unwrap(),
                                         &options, 
                                    &species.unwrap().1,
                                         );

    options.set_value_for_key("lastSpeciesViewed", con.clone().number.unwrap().to_string()).expect("Option File Problems");

    return options;
}


// Show a non-existing bird in user database, as a reference. Needs to be deleted later. 
pub fn show_template_bird<'a>(options: &'a mut SettingsText, birds: &'a BTreeMap<String,Species>) -> &'a mut SettingsText {

    let sname = "Streptopelia capicola".to_string();
    let name = "Ring-necked Dove".to_string();
    let order = "Columbiformes".to_string();
    let family = "Columbidae (Pigeons and Doves)".to_string();
    let status = "LC".to_string();
    let aname = "Cape Turtle-dove".to_string();
    let list = "Southern Africa".to_string();

    let result = Species::build_species(&birds, sname, name, order, family, status, aname, list).unwrap();

    // lets show the species
    *options = show_species( 0,
                              0,
                                         &options, 
                                    &result,
                                         );

    return options;
}












