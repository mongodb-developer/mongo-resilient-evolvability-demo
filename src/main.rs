use std::env;
use std::error::Error;
use std::process::exit;

mod app1;
use app1::app1_main;

mod app2;
use app2::app2_main;

const APP1_ID: &str = "app1";
const APP2_ID: &str = "app2";

// Main bootstrap function which starts app1 or app2 depending on the command line args passed in
//
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (appid, url) = get_appid_and_url_args_or_exit();

    match appid.as_str() {
        APP1_ID => app1_main(&url).await?,
        APP2_ID => app2_main(&url).await?,
        _ => {
            eprintln!(
                "\nERROR: Application id parameter must have the value '{}' or '{}'\n",
                APP1_ID, APP2_ID
            );
            exit(1);
        }
    }

    Ok(())
}

// Extract the Application ID + URL parameters passed on the command line or exit if not provided
//
fn get_appid_and_url_args_or_exit() -> (String, String) {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!(
            "\nERROR: An application id ('app1' or 'app2') + the MongoDB URL both need to be \
            provided as arguments\n"
        );
        exit(1);
    }

    if !args[2].starts_with("mongodb") {
        eprintln!(
            "\nERROR: The second parameter (URL) must be a valid MongoDB URL starting with the \
            text 'mongodb'\n"
        );
        exit(1);
    }

    (args[1].to_string(), args[2].to_string())
}
