
use std::process::Command;
use std::env;

fn main() {


    apt_update();

   


    let arguments: Vec<String> = env::args().collect();

    let package_name: Option<&String> = arguments.get(1);
    let upgrade_flag: Option<&String> = arguments.get(2);

    match upgrade_flag {

        // if there is an upgrade flag
        Some(flag) => {

            //upgrades if there is an upgrade flag

            apt_upgrade();
            

            if flag.len() > 0 {

                std::process::exit(0);

            }

        },

        None => {

            // if there is no upgrade flag
        
            
           

             match package_name{
                //checks if there is a package to be installed

                Some(_) => {
                    //if there is a package to be installed, it does nothing
                },
                None => {
                    //if there is no package to be installed, it closes the process
                    println!("No package name providede");
                    std::process::exit(1);
                }



             }

            

        
        }
        
    }


    match package_name {
        Some(p) => {

            if p.len() != 0 {

                install_package(p);
                std::process::exit(0);

            }

        },

        None => {
            println!("No package name provided");
            std::process::exit(1);
        }
        
    }






    

    


    
        

        

        // install_package();
    

        

    

    std::process::exit(0);


    

   
    
}


fn apt_update(){

    Command::new("sudo")
        .arg("apt")
        .arg("update")
        .spawn()
        .expect("apt command failed to start")
        .wait_with_output()
        .expect("failed to wait for update");

}

fn install_package(package_name: &str){
    Command::new("sudo")
        .arg("apt")
        .arg("install")
        .arg(package_name)
        .spawn()
        .expect("apt command failed to start")
        .wait_with_output()
        .expect("failed to wait for update");
}

fn apt_upgrade(){
    Command::new("sudo")
        .arg("apt")
        .arg("upgrade")
        .spawn()
        .expect("apt command failed to start")
        .wait_with_output()
        .expect("failed to wait for update");
}
