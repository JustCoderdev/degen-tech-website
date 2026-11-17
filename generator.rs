use std::env;
use std::fs;
use std::process;
use std::env::Args;
use std::path::PathBuf;

// enum AvailablePage
// {
    // Index,
    // AboutUs,
    // Events,
    // Lectures,
    // All
// }

// enum AvailableCache
// {
    // Events,
    // Lectures,
    // All
// }

#[allow(dead_code)]
#[derive(PartialEq)]
enum ProgramState
{
    ParseFlags,
    ParseFlag(String), // current flag being parsed
    PrintHelpAndDie,
    // GeneratePage(AvailablePage),
    // RefreshCache(AvailableCache)
}

fn main()
{
    let outpath: PathBuf = PathBuf::from("./build");

    let navbarview: String = fs::read_to_string("./views/navbar.html").expect("Cannot read navbar view");
    let indexview: String = fs::read_to_string("./views/index.html").expect("Cannot read index view");
    let indexpage: String = generate_page(navbarview, indexview);

    fs::write(outpath.join("index.html"), indexpage).expect("Failed to write index.html");
}

// TODO: Pass the name of the bodyview to disable that button
fn generate_page(navbar: String, bodyview: String) -> String
{
r#"
<!DOCTYPE html>
<html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <meta http-equiv="X-UA-Compatible" content="ie=edge">
        <title>DegenTech</title>
        <link rel="stylesheet" href="assets/styles.css">
        <meta name="DegenTech.inc" content="Working professional group of VRC furs, geared towards STEMM and tech work and interests! We host regular meetups and try to foster connections among tech-y furries, friendly or professional!">
    </head>
    <body>
"#.to_owned()
+ &navbar
+ &bodyview
+ r#"
    </body>
</html>
"#
}

// ------------------------------------------------------------ //

fn _main()
{
    let mut program_state: ProgramState = ProgramState::ParseFlags;

    let mut outpath: PathBuf = PathBuf::from("./build");
    let mut args: Args = env::args();
    let program_name: String = args.next().expect("It's the fucking program invocation");

    for command in args
    {
        program_state = match command.as_str()
        {
            "help" => ProgramState::PrintHelpAndDie,
            "--outpath" => ProgramState::ParseFlag("outpath".to_string()),
            value => {
                if program_state == ProgramState::ParseFlag("outpath".to_string())
                {
                    let new_path: PathBuf = PathBuf::from(value);
                    if new_path.is_dir()
                    {
                        println!("Updated outpath: '{}' -> '{}'", outpath.display(), new_path.display());
                        outpath = new_path;
                        ProgramState::ParseFlags
                    }
                    else
                    {
                        println!("The provided outpath is not a directory: '{}'", new_path.display());
                        ProgramState::PrintHelpAndDie
                    }
                }
                else
                {
                    println!("Unknown argument '{command}'");
                    ProgramState::PrintHelpAndDie
                }
            }
        };

        if program_state == ProgramState::PrintHelpAndDie { break; }
    }

    match program_state
    {
        ProgramState::PrintHelpAndDie => {
            println!("{program_name} <flags> [command] <args>");
            println!("");
            println!("Flags: (NOTE if you put a path with the same name as a command it will be ignored :D)");
            println!("\t--outpath <path>\tpath to output end result into (default: `./build`)");
            println!("");
            println!("Commands:");
            println!("\thelp\t\t\tdisplay this message");
            println!("\tgenerate\t\tstitch together the full website");
            println!("\trefresh\t\t\trefresh the full cache");
            println!("\trefresh <cache>\t\trefresh only cache `lectures` or `events`");
            process::exit(0);
        },

        _ => println!("waf...?")
    }
}

