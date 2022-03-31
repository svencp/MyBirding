/*  A file to keep functions that I use on files here
And I had to add sentence case here too

2022.03.29   Sven Ponelat

*/

use termion::{color, style};
use super::settings::SettingsText;

// Function to show help
pub fn show_help(options: SettingsText){
    let my_gray: color::Rgb = options.clone().get_color("myLightGray");
    let my_geen: color::Rgb = options.clone().get_color("myGreen");
    let my_blue: color::Rgb = options.clone().get_color("myBlue");
    let my_lightblue: color::Rgb = options.clone().get_color("myLightBlue");
   
    print!("");
    print!("{}",color::Fg(my_geen));
    print!("Dealing with bird species ===========================================================================================================================");
    print!("{}\n",style::Reset);
    
    print!("b");
    print!("\t\t\t\t\t\t\t\t\t\t{}",color::Fg(my_gray));
    print!("last bird species viewed");
    print!("{}\n",style::Reset);
    
    print!("b\tcode or index ");
    print!("\t\t\t\t\t\t\t\t{}",color::Fg(my_gray));
    print!("view bird species with given code or index number (e.g.)");
    print!("{}\n",style::Reset);
    
    print!("b\tspho ");
    print!("\t\t\t\t\t\t\t\t\t{}",color::Fg(my_gray));
    print!("view the house sparrow");
    print!("{}\n",style::Reset);
    
    print!("ba\targument in inverted commas with # as a separator " );
    print!("\t\t\t{}",color::Fg(my_gray));
    print!("adding a new bird to the database (e.g.)");
    print!("{}\n",style::Reset);
    
    print!("ba\t\"n=dodo#s=raphus cucullatus#r=columbiformes#m=columbidae#u=Extinct\"" );
    print!("\t{}",color::Fg(my_gray));
    print!("adding the Dodo");
    print!("{}\n",style::Reset);
    
    print!("bb" );
    print!("\t\t\t\t\t\t\t\t\t\t{}",color::Fg(my_gray));
    print!("go back one bird species from the last viewed species");
    print!("{}\n",style::Reset);
    
    print!("bd\tcode or index" );
    print!("\t\t\t\t\t\t\t\t{}",color::Fg(my_gray));
    print!("delete a bird from species database, also deletes all sightings with that bird species (e.g.)");
    print!("{}\n",style::Reset);
    
    print!("bd\tspho" );
    print!("\t\t\t\t\t\t\t\t\t{}",color::Fg(my_gray));
    print!("deletes the house sparrow");
    print!("{}\n",style::Reset);
    
    print!("be\tspho\t\"u=Rare#l=Europe\"" );
    print!("\t\t\t\t\t\t{}",color::Fg(my_gray));
    print!("edits the house sparrow field of status and list");
    print!("{}\n",style::Reset);
    
    print!("bex" );
    print!("\t\t\t\t\t\t\t\t\t\t{}",color::Fg(my_gray));
    print!("exports birds file to a json formatted file");
    print!("{}\n",style::Reset);
    
    print!("bex\tcsv" );
    print!("\t\t\t\t\t\t\t\t\t{}",color::Fg(my_gray));
    print!("exports birds file to a csv formatted file");
    print!("{}\n",style::Reset);
    
    print!("bf" );
    print!("\t\t\t\t\t\t\t\t\t\t{}",color::Fg(my_gray));
    print!("go forward one bird species from the last viewed species");
    print!("{}\n",style::Reset);
    
    print!("bim\t/Home/mine/bird_file.csv" );
    print!("\t\t\t\t\t\t{}",color::Fg(my_gray));
    print!("import a bird file either .json or .csv that will replace all birds");
    print!("{}\n",style::Reset);
    
    print!("");
    print!("Help ================================================================================================================================================");
    print!("\n");
    
    print!("help" );
    print!("\t\t\t\t\t\t\t\t\t\t{}",color::Fg(my_gray));
    print!("this help screen ");
    print!("{}\n",style::Reset);
    
    print!("");
    print!("{}",color::Fg(my_blue));
    print!("Dealing with bird sightings =========================================================================================================================");
    print!("{}\n",style::Reset);
    
    print!("o");
    print!("\t\t\t\t\t\t\t\t\t\t{}",color::Fg(my_gray));
    print!("last bird sighting viewed");
    print!("{}\n",style::Reset);
    
    print!("o\tindex ");
    print!("\t\t\t\t\t\t\t\t\t{}",color::Fg(my_gray));
    print!("view bird sighting at the given index number (e.g.)");
    print!("{}\n",style::Reset);
    
    print!("o\t909 ");
    print!("\t\t\t\t\t\t\t\t\t{}",color::Fg(my_gray));
    print!("view bird sighting at position 909");
    print!("{}\n",style::Reset);
    
    print!("oa\t\"sha#d=29#c=duye#a=Home#w=London#p=WA1#t=UK\"");
    print!("\t\t\t\t{}",color::Fg(my_gray));
    print!("adding a sighting ['sha' are the codes for seen heard adult] (another example is)");
    print!("{}\n",style::Reset);
    
    print!("oa\t\"0sha#c=duye\"");
    print!("\t\t\t\t\t\t\t\t{}",color::Fg(my_gray));
    print!("adding a sighting [0 is the shorcut for the last location]");
    print!("{}\n",style::Reset);
    
    print!("ob");
    print!("\t\t\t\t\t\t\t\t\t\t{}",color::Fg(my_gray));
    print!("go back one bird sighting from the last viewed sighting");
    print!("{}\n",style::Reset);
    
    print!("od\t1");
    print!("\t\t\t\t\t\t\t\t\t{}",color::Fg(my_gray));
    print!("delete the first sighting from the database");
    print!("{}\n",style::Reset);
    
    print!("oe\t1\t\"d=2019.01.25\"");
    print!("\t\t\t\t\t\t\t{}",color::Fg(my_gray));
    print!("change the date on the first sighting");
    print!("{}\n",style::Reset);
    
    print!("oex\t/Home/mine/sightings_file.csv");
    print!("\t\t\t\t\t\t{}",color::Fg(my_gray));
    print!("exports sightings data file to a csv format");
    print!("{}\n",style::Reset);
    
    print!("oex");
    print!("\t\t\t\t\t\t\t\t\t\t{}",color::Fg(my_gray));
    print!("exports sightings data file to a json format");
    print!("{}\n",style::Reset);
    
    print!("of");
    print!("\t\t\t\t\t\t\t\t\t\t{}",color::Fg(my_gray));
    print!("go forward one bird sighting from the last viewed sighting");
    print!("{}\n",style::Reset);
    
    print!("oim\t/Home/mine/sight_file.csv" );
    print!("\t\t\t\t\t\t{}",color::Fg(my_gray));
    print!("import a bird sightings file that is .csv formatted and that will replace all sightings");
    print!("{}\n",style::Reset);
    
    print!("oim" );
    print!("\t\t\t\t\t\t\t\t\t\t{}",color::Fg(my_gray));
    print!("import a bird sightings file that is .json formatted and that will replace all sightings");
    print!("{}\n",style::Reset);
    
    print!("oz" );
    print!("\t\t\t\t\t\t\t\t\t\t{}",color::Fg(my_gray));
    print!("shows last 10 locations with the last sighting in the database");
    print!("{}\n",style::Reset);
    
    print!("");
    print!("{}",color::Fg(my_lightblue));
    print!("Searching ===========================================================================================================================================");
    print!("{}\n",style::Reset);
    
    print!("so\tsar" );
    print!("\t\t\t\t\t\t\t\t\t{}",color::Fg(my_gray));
    print!("list all sightings of birds that were adults and seen and ringed");
    print!("{}\n",style::Reset);
    
    
    
    
    
    print!("");
    print!("Version =============================================================================================================================================");
    print!("\n");
    
    print!("v");
    print!("\t\t\t\t\t\t\t\t\t\t{}",color::Fg(my_gray));
    print!("program version");
    print!("{}\n\n",style::Reset);
    
    
    
    
    
    
    
    
    
    















    
    
    
}






















