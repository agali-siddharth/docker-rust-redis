extern crate redis;

use redis::Commands;
use redis::RedisResult;
use std::env;

#[derive(Debug)]
enum Command {
  Set,
  Get,
}

fn get_server_string() -> String {
    let server = match env::var("redis_server") {
        Ok(x) => {x},
        Err(_e) => {"127.0.0.1".to_string()}
    };
    let mut str = "redis://".to_string();
    str.push_str(server.as_str());
    str.push_str("/");
    str
}

fn set_to_redis(key :String, value :String) -> Result<(), String> {
  let client = redis::Client::open(get_server_string().as_str()).unwrap();
  let mut con = client.get_connection().unwrap();

  let ret :RedisResult<()> = con.set(key, value);
  match ret {
    Ok(_) => Ok(()),
    _ => Err(format!("error in writing to db!")),
  }
}

fn get_from_redis(key: String) -> Result<String, String> {
  let client = redis::Client::open(get_server_string().as_str()).unwrap();
  let mut con = client.get_connection().unwrap();

  let ret :RedisResult<String> = con.get(key);
  match ret {
    Ok(x) => {println!("obtained - {}", x); Ok(x)},
    _ => {return Err(format!("error in getting from db!"))},
  }
}

fn process_set(args :&mut env::Args) -> Result<(), String> {
  if args.len() != 4 {
    return Err(format!("wrong number of arguments for set command : {}", args.len()));
  }

  let key = args.nth(2).unwrap();
  // nth() consumes all preceding elements as well as the current one. so the next argument to nth should be 0
  let value = args.nth(0).unwrap();

  set_to_redis(key,value)
}

fn process_get(args :&mut env::Args) -> Result<String, String> {
  if args.len() != 3 {
    return Err(format!("wrong number of arguments for get command : {}", args.len()));
  }
  let key = args.nth(2).unwrap();

  get_from_redis(key)
}

pub fn parse_args() -> Result<(), String> {
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

#[cfg(test)]
mod tests {
use crate::*;
    #[test]
    fn test_set_and_get() {
        assert_eq!(set_to_redis("test_val".to_string(), 100.to_string()).is_ok(), true);
        assert_eq!(get_from_redis("test_val".to_string()).unwrap(), 100.to_string());
    }
}
