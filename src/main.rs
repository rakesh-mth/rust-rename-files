extern crate clap;
extern crate glob;

use glob::glob;
use clap::{Arg, App, AppSettings};

use std::fs;

fn main() 
{
  let path_arg_name = "path";
  let path_arg_src_ext = "src_fmt";
  let path_arg_dst_ext = "dst_fmt";
  let args = App::new("src-fmt-to-dst-fmt")
    .about("Rename .src_fmt to .dst_fmt")
    .setting(AppSettings::ArgRequiredElseHelp)
    .arg(Arg::with_name(path_arg_name).help("path to the top directory with .src_fmt files"))
    .arg(Arg::with_name(path_arg_src_ext).help("source file extention"))
    .arg(Arg::with_name(path_arg_dst_ext).help("destination file extention"))
    .get_matches();

  let path = args.value_of(path_arg_name)
    .expect("You didn't supply a path");
  let src_ext = args.value_of(path_arg_src_ext)
    .expect("You didn't supply source extention");
  let dst_ext = args.value_of(path_arg_dst_ext)
    .expect("You didn't supply destination extention");

  let search = String::from(path) + "/**/*." + src_ext;
  let paths = glob(&search)
    .expect("Could not find paths in glob")
    .map(|p| p.expect("Bad individual path in glob"));

  for path in paths {
    match fs::rename(&path, &path.with_extension(dst_ext)) {
      Ok(_) => (),
      Err(reason) => panic!("{}", reason),
    };
  }
}