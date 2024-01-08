use std::env::args;

mod about;
mod pm;

fn main() {
    let args = args().collect::<Vec<String>>();

    if args.len() > 1  {
        if args[1] == "x".to_owned() {
            pm::exec();
        } else if args[1] == "c".to_owned() {
            pm::create();
        } else if args[1] == "g".to_owned() {
            pm::global_exec();
        } else if args[1] == "h".to_owned() {
            about::help();
        } 
    } else {
        about::help();
    }
}
