

use std::env;
mod library;
use library::lib::{apt_update,Config,run};

fn main() {


    apt_update();

   


    let arguments: Vec<String> = env::args().collect();

    let package_name: Option<String> = arguments.get(1).cloned();
    let upgrade_flag: Option<String> = arguments.get(2).cloned();


    let config: Config = Config::new(package_name, upgrade_flag);

    run(&config);

    

   






    

    


    
        

        

        // install_package();
    

        

    

    std::process::exit(0);


    

   
    
}

