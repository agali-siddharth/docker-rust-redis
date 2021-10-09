extern crate redis;

use redis::Commands;
use redis::RedisResult;
use std::env;

#[derive(Debug)]
enum Command {
  Set,
  Get,
}

fn process_set(args :&mut env::Args) -> Result<(), String> {
  if args.len() != 4 {
    return Err(format!("wrong number of arguments for set command : {}", args.len()));
  }

  let client = redis::Client::open("redis://127.0.0.1/").unwrap();
  let mut con = client.get_connection().unwrap();
  let key = args.nth(2).unwrap();
  // nth() consumes all preceding elements as well as the current one. so the next argument to nth should be 0
  let value = args.nth(0).unwrap();

  let ret :RedisResult<()> = con.set(key, value);
  match ret {
    Ok(_) => Ok(()),
    _ => Err(format!("error in writing to db!")),
  }
}

fn process_get(args :&mut env::Args) -> Result<String, String> {
  if args.len() != 3 {
    return Err(format!("wrong number of arguments for get command : {}", args.len()));
  }
  let client = redis::Client::open("redis://127.0.0.1/").unwrap();
  let mut con = client.get_connection().unwrap();
  let key = args.nth(2).unwrap();

  let ret :RedisResult<String> = con.get(key);
  match ret {
    Ok(x) => {println!("obtained - {}", x); Ok(x)},
    _ => {return Err(format!("error in getting from db!"))},
  }
}

fn parse_args() -> Result<(), String> {
  let mut args = env::args();
  let mut orig_args = env::args();
  if args.len() < 2 {
    return Err(format!("wrong number of arguments : {}", args.len()));
  }
  let command = args.nth(1).unwrap();
  let cmd = match command.as_str() {
    "set" => Command::Set,
    "get" => Command::Get,
    _ => { return Err(format!("invalid command {}", command)); },
  };

  match cmd {
    Command::Set => {process_set(&mut orig_args)?;},
    Command::Get => {process_get(&mut orig_args)?;},
  }

  Ok(())
}

fn main() {
/*
    println!("Hello, world!");
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();

    let mynum :i32 = con.get("mynum").unwrap();
    println!("mynum = {:?}", mynum);

    let _ : () = con.set("mynum", 20).unwrap();

    let mynum :i32 = con.get("mynum").unwrap();
    println!("mynum = {:?}", mynum);
*/


    match parse_args() {
        Err(e) => {println!("operation failed - {}", e); std::process::exit(1); },
        _ => {redis::cmd("SAVE");},
    }

    
}
