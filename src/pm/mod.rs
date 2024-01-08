mod parser;
mod util;
mod exec;
mod init;
mod tests;

pub fn exec() {
    exec::exec();
}

pub fn create() {
    init::create();
}

pub fn global_exec() {}
