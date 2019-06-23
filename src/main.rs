extern crate clap;
extern crate glob;

use glob::glob;
use clap::{Arg, App, AppSettings};

use std::fs;

fn main() 
{
  let path_arg_name = "path";
  let args = App::new("cha-to-txt")
    .about("Rename .cha to .txt")
    .setting(AppSettings::ArgRequiredElseHelp)
    .arg(Arg::with_name(path_arg_name)
    .help("path to the top directory with .cha files"))
    .get_matches();

  let path = args.value_of(path_arg_name)
    .expect("You didn't supply a path");
  let search = String::from(path) + "/**/*.cha";
  let paths = glob(&search)
    .expect("Could not find paths in glob")
    .map(|p| p.expect("Bad individual path in glob"));

  for path in paths {
    match fs::rename(&path, &path.with_extension("txt")) {
      Ok(_) => (),
      Err(reason) => panic!("{}", reason),
    };
  }
}