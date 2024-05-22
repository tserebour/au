

pub mod lib{


    pub struct Config{
        pub package_name: Option<String>,
        pub upgrade_flag: Option<String>
    }

    impl Config {

        pub fn new(package_name: Option<String>, upgrade_flag: Option<String>) -> Config {
            Config {
                package_name,
                upgrade_flag,
            }
        }
    }







use std::process::Command;




pub fn apt_update(){

    Command::new("sudo")
        .arg("apt")
        .arg("update")
        .spawn()
        .expect("apt command failed to start")
        .wait_with_output()
        .expect("failed to wait for update");

    println!("Done apt Update >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");

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

        println!("Done installing package >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>")

}

fn apt_upgrade(){
    Command::new("sudo")
        .arg("apt")
        .arg("upgrade")
        .spawn()
        .expect("apt command failed to start")
        .wait_with_output()
        .expect("failed to wait for update");
    println!("Done apt Upgrade >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>")

}


pub fn run(config: &Config){
    match &config.upgrade_flag {

        // if there is an upgrade flag
        Some(flag) => {

            //upgrades if there is an upgrade flag

            if flag == "-u"{
                apt_upgrade();
            }
            

            match &config.package_name{
                //checks if there is a package to be installed

                Some(p) => {
                    //if there is a package to be installed,

                    if p.len() == 0{
                        //if there is no package to be installed, it closes the process
                        println!("Upgrade done, No package to be installed");
                        std::process::exit(1);
                    }

                },
                None => {
                    //if there is no package to be installed, it closes the process
                    println!("Upgrade done, No package to be installed");

                    std::process::exit(1);
                }



             }

        },

        None => {

            // if there is no upgrade flag
        
            
           

             match config.package_name{
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


    match &config.package_name {
        Some(p) => {

            if p.len() != 0 {

                install_package(p.as_ref());
                std::process::exit(0);

            }

        },

        None => {
            println!("No package name provided");
            std::process::exit(1);
        }
        
    }
}






}