mod cli;

use std::process::exit;
use cli::Cli;
use pocket::Pocket;

fn main() {

    let (passwd_opt, user_opt) = Cli::perform();

    let pocket = Pocket::new();

    if let Some(passwd) = passwd_opt {
        match login_server(passwd) {
            Ok(_) => {}
            Err(error) => {
                eprintln!("Server passwd mismatch:{error}");
                exit(1);
            }
        }
    }

    let user;
    if let Some(usr) = user_opt {
        user = usr;
        match execute(user) {
            Ok(_) => {}
            Err(error) => {
                eprintln!("{error}");
                exit(1);
            }
        }
    } else {
        eprintln!("User parameter missing");
        exit(1);
    }

}
