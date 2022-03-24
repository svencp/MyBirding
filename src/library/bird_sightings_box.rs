/*
    My module for drawing the bird sighting on the terminal

    2022.01.27  Sven Ponelat

*/

use crate::library::my_file_funcs::*;
use crate::library::bird_species::*;
use crate::library::bird_sightings::*;
use crate::library::settings::*;
use draw_box::{Char};
use termion::{color, style};
use thousands::{Separable};
// use textwrap::{fill, Options};
use substring::Substring;





#[rustfmt::skip]
// const my_blue:  termion::color::Rgb = termion::color::Rgb(10,100,245);
// const MY_LIGHT_BLUE:  termion::color::Rgb = termion::color::Rgb(50,200,255);
// const my_normal: termion::color::Rgb = termion::color::Rgb(177,177,177);
// const MY_DARK_GRAY: termion::color::Rgb = termion::color::Rgb(50,50,50);

// const style: draw_box::Style = draw_box::Style::Thick;

const SPACE:    &str       =   " ";
const MARGIN:   &str       = "   ";  
const PADDING1: &str       = "   ";
const PADDING2: &str       =  "  ";

const PAD1_LEN: usize        = PADDING1.len();
const PAD2_LEN: usize        = PADDING2.len();
const R_SIDE_TEXT_LEN: usize = 18;

const BLOCK_LEN:      usize     = 123;
// const FAMILY_LEN:     usize     =  60;
const NAME_LEN:       usize     =  40;
const COMMENT_LEN:    usize     =  42;
const SEEN_LEN:       usize     =  10;
const INDEX_HALF_LEN: usize     =   5;



/*
A function to show the data of the species
*/
pub fn show_sighting(options: &mut SettingsText, rec_num: usize, species: Species, sighting: Sightings) -> &mut SettingsText {

    let my_blue: color::Rgb = options.clone().get_color("myBlue");
    let my_light_blue: color::Rgb = options.clone().get_color("myLightBlue");
    let my_normal: color::Rgb = options.clone().get_color("myNormalGray");
    let my_dark_gray: color::Rgb = options.clone().get_color("myDarkGray");
    let style = options.clone().get_box_style("speciesBoxStyle");
    let show_index = true;

    let ul = Char::upper_left(style);
    let ur = Char::upper_right(style);
    let ll = Char::lower_left(style);
    let lr = Char::lower_right(style);
    let v  = Char::vertical(style);
    let h  = Char::horizontal(style);

    // Change to non-zero bases index
    let non_zero = rec_num + 1;
    let index = justify(non_zero.separate_with_spaces(), 2 * INDEX_HALF_LEN + 1, Justify::Center); 
    let comments = divide_and_give3(sighting.clone().comments);

    // Top line
    print!("{}{}",color::Fg(my_blue),"\n\n");
    print!("{}{}", MARGIN, ul); 
    print!("{}", &repeat_char(h, BLOCK_LEN));   
    print!("{}{}", ur, "\n");    
    
    print!("{}{}{}{}\n", MARGIN, v, &repeat_char(SPACE.to_string(), BLOCK_LEN), v);                  // empty body line
    print!("{}{}{}{}\n", MARGIN, v, &repeat_char(SPACE.to_string(), BLOCK_LEN), v);                  // empty body line
    

    
    // Name - Observations
    let name = justify(species.clone().name, NAME_LEN, Justify::Left);         
    print!("{}{}{}", MARGIN, v, PADDING1); 
    let name_text = justify("Name:".to_string(), R_SIDE_TEXT_LEN, Justify::Left);         
    // let observation_text = justify("Observations:".to_string(), R_SIDE_TEXT_LEN, Justify::Left);         
    // let observations = justify(observations, NAME_LEN, Justify::Left);  
    let seen =  justify("SEEN".to_string(), SEEN_LEN, Justify::Left); 
    let heard =  justify("HEARD".to_string(), SEEN_LEN, Justify::Left); 
    let ringed =  justify("RINGED".to_string(), SEEN_LEN, Justify::Left); 
    let breeding =  justify("BREEDING".to_string(), SEEN_LEN, Justify::Left); 
    let nonbreeding =  justify("NON-BREEDING".to_string(), SEEN_LEN, Justify::Left); 
    print!("{}{}{}",color::Fg(my_blue), name_text, style::Reset);
    print!("{}{}{}", color::Fg(my_normal), name, style::Reset); 
    if sighting.seen {
        print!("{}", color::Fg(my_light_blue));
        underline('S', &seen); 
        print!("{}", style::Reset);
    } else {
        print!("{}", color::Fg(my_dark_gray));
        underline('S', &seen); 
        print!("{}", style::Reset);
    }
    if sighting.heard {
        print!("{}", color::Fg(my_light_blue));
        underline('H', &heard); 
        print!("{}", style::Reset);
    } else {
        print!("{}", color::Fg(my_dark_gray));
        underline('H', &heard); 
        print!("{}", style::Reset);
    }
    if sighting.ringed  {
        print!("{}", color::Fg(my_light_blue));
        underline('R', &ringed); 
        print!("{}", style::Reset);
    } else {
        print!("{}", color::Fg(my_dark_gray));
        underline('R', &ringed); 
        print!("{}", style::Reset);
    }
    if sighting.breeding  {
        print!("{}", color::Fg(my_light_blue));
        underline('B', &breeding); 
        print!("{}", style::Reset);
    } else {
        print!("{}", color::Fg(my_dark_gray));
        underline('B', &breeding); 
        print!("{}", style::Reset);
    }
    if sighting.nonbreeding  {
        print!("{}", color::Fg(my_light_blue));
        underline('N', &nonbreeding); 
        print!("{}", style::Reset);
    } else {
        print!("{}", color::Fg(my_dark_gray));
        underline('N', &nonbreeding); 
        print!("{}", style::Reset);
    }
    let name_pad_len: usize = BLOCK_LEN - PAD1_LEN - PAD2_LEN - R_SIDE_TEXT_LEN  - NAME_LEN - 52 ;
    let add_space = &repeat_char(SPACE.to_string(), name_pad_len);
    print!("{}{}{}{}\n", color::Fg(my_blue), PADDING2, add_space,  v); 
    
    print!("{}{}{}{}\n", MARGIN, v, &repeat_char(SPACE.to_string(), BLOCK_LEN), v);                  // empty body line
    
    
    // Sname
    let sname = justify(sighting.clone().sname, NAME_LEN, Justify::Left);         
    print!("{}{}{}", MARGIN, v, PADDING1); 
    let sname_text = justify("Scientific Name:".to_string(), R_SIDE_TEXT_LEN, Justify::Left);         
    print!("{}{}{}",color::Fg(my_blue), sname_text, style::Reset);
    print!("{}{}{}{}", color::Fg(my_normal), style::Italic, sname, style::Reset); 
    
    let nest =  justify("NEST".to_string(), SEEN_LEN, Justify::Left); 
    let eggs =  justify("EGGS".to_string(), SEEN_LEN, Justify::Left); 
    let chicks =  justify("CHICKS".to_string(), SEEN_LEN, Justify::Left); 
    let immature =  justify("IMMATURE".to_string(), SEEN_LEN, Justify::Left); 
    let dead =  justify("DEAD".to_string(), SEEN_LEN, Justify::Left); 
    
    if sighting.nest {
        print!("{}", color::Fg(my_light_blue));
        underline('T', &nest); 
        print!("{}", style::Reset);
    } else {
        print!("{}", color::Fg(my_dark_gray));
        underline('T', &nest); 
        print!("{}", style::Reset);
    }
    if sighting.eggs {
        print!("{}", color::Fg(my_light_blue));
        underline('G', &eggs); 
        print!("{}", style::Reset);
    } else {
        print!("{}", color::Fg(my_dark_gray));
        underline('G', &eggs); 
        print!("{}", style::Reset);
    }
    if sighting.chicks  {
        print!("{}", color::Fg(my_light_blue));
        underline('C', &chicks); 
        print!("{}", style::Reset);
    } else {
        print!("{}", color::Fg(my_dark_gray));
        underline('C', &chicks); 
        print!("{}", style::Reset);
    }
    if sighting.immature  {
        print!("{}", color::Fg(my_light_blue));
        underline('I', &immature); 
        print!("{}", style::Reset);
    } else {
        print!("{}", color::Fg(my_dark_gray));
        underline('I', &immature); 
        print!("{}", style::Reset);
    }
    if sighting.dead  {
        print!("{}", color::Fg(my_light_blue));
        underline('E', &dead); 
        print!("{}", style::Reset);
    } else {
        print!("{}", color::Fg(my_dark_gray));
        underline('E', &dead); 
        print!("{}", style::Reset);
    }
    let sname_pad_len: usize = BLOCK_LEN - PAD1_LEN - PAD2_LEN - R_SIDE_TEXT_LEN  - NAME_LEN - 48 ;
    print!("{}{}{}\n", color::Fg(my_blue), &repeat_char(SPACE.to_string(), sname_pad_len), v); 
    
    print!("{}{}{}{}\n", MARGIN, v, &repeat_char(SPACE.to_string(), BLOCK_LEN), v);                  // empty body line
    
    
    // Status
    let status = justify(species.clone().status, NAME_LEN, Justify::Left);         
    print!("{}{}{}", MARGIN, v, PADDING1); 
    let status_text = justify("Status:".to_string(), R_SIDE_TEXT_LEN, Justify::Left);         
    print!("{}{}{}",color::Fg(my_blue), status_text, style::Reset);
    print!("{}", style::Reset); 
    print!("{}{}{}", color::Fg(my_normal), status, style::Reset); 
    // let fname = justify(species.clone().fname, NAME_LEN, Justify::Left);         
    // print!("{}{}{}", MARGIN, v, PADDING1); 
    // let fname_text = justify("Family Name:".to_string(), R_SIDE_TEXT_LEN, Justify::Left);         
    // print!("{}{}{}",color::Fg(my_blue), fname_text, style::Reset);
    // print!("{}{}{}", color::Fg(my_normal), fname, style::Reset); 
    let male =  justify("MALE".to_string(), SEEN_LEN, Justify::Left); 
    let female =  justify("FEMALE".to_string(), SEEN_LEN, Justify::Left); 
    let adult =  justify("ADULT".to_string(), SEEN_LEN, Justify::Left); 
    let photo =  justify("PHOTO".to_string(), SEEN_LEN, Justify::Left); 
    if sighting.male {
        print!("{}", color::Fg(my_light_blue));
        underline('M', &male); 
        print!("{}", style::Reset);
    } else {
        print!("{}", color::Fg(my_dark_gray));
        underline('M', &male); 
        print!("{}", style::Reset);
    }
    if sighting.female  {
        print!("{}", color::Fg(my_light_blue));
        underline('F', &female); 
        print!("{}", style::Reset);
    } else {
        print!("{}", color::Fg(my_dark_gray));
        underline('F', &female); 
        print!("{}", style::Reset);
    }
    if sighting.adult  {
        print!("{}", color::Fg(my_light_blue));
        underline('A', &adult); 
        print!("{}", style::Reset);
    } else {
        print!("{}", color::Fg(my_dark_gray));
        underline('A', &adult); 
        print!("{}", style::Reset);
    }
    if sighting.photo  {
        print!("{}", color::Fg(my_light_blue));
        underline('P', &photo); 
        print!("{}", style::Reset);
    } else {
        print!("{}", color::Fg(my_dark_gray));
        underline('P', &photo); 
        print!("{}", style::Reset);
    }
    let family_pad_len: usize = BLOCK_LEN - PAD1_LEN - R_SIDE_TEXT_LEN  - NAME_LEN - 40;
    print!("{}{}{}\n", color::Fg(my_blue), &repeat_char(SPACE.to_string(), family_pad_len), v); 
    
    print!("{}{}{}{}\n", MARGIN, v, &repeat_char(SPACE.to_string(), BLOCK_LEN), v);                  // empty body line
    
    
    // Code    & Province
    let mut code = species.clone().code;
    code = justify(code, NAME_LEN, Justify::Left);         
    print!("{}{}{}", MARGIN, v, PADDING1); 
    let code_text = justify("Code:".to_string(), R_SIDE_TEXT_LEN, Justify::Left);         
    // print!("{}", &code_text); 
    print!("{}", color::Fg(my_blue)); 
    underline('C', &code_text); 
    print!("{}", style::Reset); 
    print!("{}{}{}", color::Fg(my_normal), code, style::Reset); 
    let province_text = justify("Province/State:".to_string(), R_SIDE_TEXT_LEN, Justify::Left);    
    let province = justify(sighting.clone().province, NAME_LEN, Justify::Left);  
    print!("{}", color::Fg(my_blue)); 
    underline('P', &province_text); 
    print!("{}", style::Reset); 
    print!("{}{}", color::Fg(my_normal), province); 
    let status_pad_len: usize = BLOCK_LEN - PAD1_LEN - ( 2 * R_SIDE_TEXT_LEN ) - ( 2 * NAME_LEN)  ;
    print!("{}{}{}\n", color::Fg(my_blue), &repeat_char(SPACE.to_string(), status_pad_len), v); 
    
    print!("{}{}{}{}\n", MARGIN, v, &repeat_char(SPACE.to_string(), BLOCK_LEN), v);                  // empty body line
    
    
    // Date   & Country
    let tempo = options.clone().get_date_string(sighting.date);
    let date = justify(tempo, NAME_LEN, Justify::Left);         
    print!("{}{}{}", MARGIN, v, PADDING1); 
    let date_text = justify("Date:".to_string(), R_SIDE_TEXT_LEN, Justify::Left);   
    print!("{}",color::Fg(my_blue));
    underline('D', &date_text);
    print!("{}", style::Reset);      
    print!("{}{}{}", color::Fg(my_normal), date, style::Reset); 
    let country_text = justify("Country:".to_string(), R_SIDE_TEXT_LEN, Justify::Left);         
    let country = justify(sighting.clone().country, NAME_LEN, Justify::Left);  
    print!("{}", color::Fg(my_blue)); 
    underline('t', &country_text);
    print!("{}", style::Reset); 
    print!("{}{}", color::Fg(my_normal), country); 
    let status_pad_len: usize = BLOCK_LEN - PAD1_LEN - ( 2 * R_SIDE_TEXT_LEN ) - ( 2 * NAME_LEN)  ;
    print!("{}{}{}\n", color::Fg(my_blue), &repeat_char(SPACE.to_string(), status_pad_len), v); 
    
    print!("{}{}{}{}\n", MARGIN, v, &repeat_char(SPACE.to_string(), BLOCK_LEN), v);                  // empty body line
    
    
    // Location    & Comments
    let tempo = sighting.clone().location;
    let location = justify(tempo, NAME_LEN, Justify::Left);         
    print!("{}{}{}", MARGIN, v, PADDING1); 
    let location_text = justify("Location:".to_string(), R_SIDE_TEXT_LEN, Justify::Left);   
    print!("{}",color::Fg(my_blue));      
    underline('a', &location_text);
    print!("{}",style::Reset);      
    print!("{}{}", color::Fg(my_normal), location); 
    let comments_text = justify("Comments:".to_string(), R_SIDE_TEXT_LEN, Justify::Left);         
    let comment = justify(comments[0].clone(), COMMENT_LEN, Justify::Left);  
    print!("{}", color::Fg(my_blue)); 
    underline('o', &comments_text);
    print!("{}", style::Reset); 
    print!("{}{}", color::Fg(my_normal), comment); 
    let status_pad_len: usize = BLOCK_LEN - PAD1_LEN - ( 2 * R_SIDE_TEXT_LEN)  - NAME_LEN - COMMENT_LEN;
    print!("{}{}{}\n", color::Fg(my_blue), &repeat_char(SPACE.to_string(), status_pad_len), v); 
    
    // Line below
    let first_space: usize = PAD1_LEN + ( 2 * R_SIDE_TEXT_LEN ) + NAME_LEN;
    print!("{}{}{}", MARGIN, v, &repeat_char(SPACE.to_string(), first_space));                  // empty body line
    let comment = justify(comments[1].clone(), COMMENT_LEN, Justify::Left);  
    print!("{}{}", color::Fg(my_normal), comment); 
    let comment_pad_len: usize = BLOCK_LEN - first_space - COMMENT_LEN;
    print!("{}{}{}\n", color::Fg(my_blue), &repeat_char(SPACE.to_string(), comment_pad_len), v); 
    
    
    
    // Town
    let tempo = sighting.clone().town;
    let town = justify(tempo, NAME_LEN, Justify::Left);         
    print!("{}{}{}", MARGIN, v, PADDING1); 
    let town_text = justify("Town:".to_string(), R_SIDE_TEXT_LEN, Justify::Left);   
    print!("{}",color::Fg(my_blue));      
    underline('w', &town_text);
    print!("{}",style::Reset);      
    print!("{}{}", color::Fg(my_normal), town); 
    print!("{}", &repeat_char(SPACE.to_string(), R_SIDE_TEXT_LEN ));
    let comment = justify(comments[2].clone(), COMMENT_LEN, Justify::Left);  
    print!("{}", comment);
    let comment_pad_len: usize = BLOCK_LEN - PAD1_LEN - ( 2 * R_SIDE_TEXT_LEN ) - COMMENT_LEN - NAME_LEN;
    print!("{}{}{}\n", color::Fg(my_blue), &repeat_char(SPACE.to_string(), comment_pad_len), v); 
    

    // Lines below
    print!("{}{}{}{}\n", MARGIN, v, &repeat_char(SPACE.to_string(), BLOCK_LEN), v);                  // empty body line
    print!("{}{}{}{}\n", MARGIN, v, &repeat_char(SPACE.to_string(), BLOCK_LEN), v);                  // empty body line


   // Bottom line
    let h1  = Char::horizontal(style);
    let h2  = Char::horizontal(style);
    print!("{}{}",style::Reset, color::Fg(my_blue));
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
    
    print!("{}{}",color::Fg(my_blue),"\n");



    
    return options
    
    
}  // end of show_sighting

// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@


// A functions (used mainly in my comments) to divide a long string into 3 equal parts
// and complement the three strings with spaces if need be.
pub fn divide_and_give3(text: String) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    let mut modified: String = String::from(&text);

    if text.len() > 120 {
        modified =  text.substring(0, 119).to_string();
    }
    
    modified =  textwrap::fill(&modified,42);
    let split: Vec<&str> = modified.split("\n").collect();

    for i in 0..split.len(){
        res.insert(i, split[i].to_string());
    }

    // add two empty strings in case vector is only one long
    while res.len() < 3 {
        res.push("".to_string());
    }

    res
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
    fn t001_divide_string() {
        // let mut mess1 = "";
        // let mut mess2 = "".to_string();

        // let mut t_str = String::from(mess1);
        let t_str = "".to_string();

        let yebo = divide_and_give3(t_str);
        // for i in 0..3 {
        //     mess2.push_str(&yebo[i])
        // }

        assert_eq!(yebo.len(),3);
    }

    
    #[ignore]
    #[test]
    fn t002_divide_string() {
        let mess1 = "   This is how the string starts but who    ";
        let mut t_str = String::from(mess1);
        t_str = t_str.trim().to_string();

        let yebo = divide_and_give3(t_str);

        assert_eq!(yebo[0].len(),"This is how the string starts but who".len());
    }


    #[ignore]
    #[test]
    fn t003_divide_string() {
        let mess1 = "This is how the string starts but who ";
        let mess2 = "knows how it will end, there may be a case ";
        let mess3 = "I better include 7x45 or 76 555 or even a lot bigger number, that pushes 120 char";
        let mut t_str = String::from(mess1);
        t_str.push_str(mess2);
        t_str.push_str(mess3);
        t_str = t_str.trim().to_string();
        let mut count = 0;
                
        let yebo = divide_and_give3(t_str);
        for i in 0..3 {
            count += yebo[i].len();
        }
        assert_eq!(count,117);
    }


    #[ignore]
    #[test]
    fn t004_divide_string() {
        let mess1 = "1234567890123456789012345678901234567890 12345678901234567890 knows how it will end, there may be a case".to_string();
                 
        let yebo = divide_and_give3(mess1);

        assert_eq!(yebo.len(),3);
    }


    #[ignore]
    #[test]
    fn t005_divide_string() {
        let mess1 = "".to_string();
        let yebo = divide_and_give3(mess1);

        assert_eq!(yebo.len(),3);
    }



} // end of all tests




















