use std::env::args;

mod pm;
mod util;

fn main() {
    let args = args().collect::<Vec<String>>();

    if args.len() > 1  {
        if args[1] == "ia".to_owned() {
            pm::index::add();
        } else if args[1] == "ir".to_owned() {
            pm::index::remove();
        } else if args[1] == "iu".to_owned() {
            pm::index::update();
        } else if args[1] == "pi".to_owned() {
            pm::project::install();
        } else if args[1] == "pd".to_owned() {
            pm::project::dev_install();
        } else if args[1] == "pr".to_owned() {
            pm::project::remove();
        } else if args[1] == "pu".to_owned() {
            pm::project::update();
        } else if args[1] == "pc".to_owned() {
            pm::project::create();
        } else if args[1] == "ps".to_owned() {
            pm::project::scaffhold();
        } else if args[1] == "px".to_owned() {
            pm::project::execute();
        } else if args[1] == "gi".to_owned() {
            pm::global::install();
        } else if args[1] == "gr".to_owned() {
            pm::global::remove();
        } else if args[1] == "gu".to_owned() {
            pm::global::update();
        } else if args[1] == "gx".to_owned() {
            pm::global::execute();
        } else if args[1] == "ah".to_owned() {
            pm::about::help();
        } else if args[1] == "ac".to_owned() {
            pm::about::configure();
        } else if args[1] == "au".to_owned() {
            pm::about::update();
        }
    } else {
        pm::about::help();
    }
}
