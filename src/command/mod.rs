mod get;

pub use get::Get;

mod set;
use httparse::Request;
use serde_json::{Result, Value};
pub use set::Set;

#[derive(Debug)]
pub enum Command {
    Set(Set),
    Get(Get),
    Invalid,
}

#[derive(Debug)]
struct Args {
    valid: bool,
    command: String,
    key: String,
    val: Option<String>,
}

impl Args {
    pub fn new_invalid(command_type: &str) -> Self {
        Args {
            valid: false,
            command: String::from(command_type),
            key: String::from(""),
            val: None,
        }
    }
}

impl Command {
    pub fn from_bytes(http_request_buff: &[u8]) -> Command {
        if http_request_buff.len() == 0 {
            return Command::Invalid;
        }

        let mut headers = [httparse::EMPTY_HEADER; 16];
        let mut req = Request::new(&mut headers);
        let result = req.parse(http_request_buff).unwrap();
        let n = result.unwrap();

        let arg = make_args(&req, http_request_buff, n);

        match arg.command.as_str() {
            "GET" => {
                if arg.valid == false {
                    return Command::Get(Get::new_invalid());
                }
                Command::Get(Get::from_key(arg.key))
            }
            "SET" => {
                if arg.valid == false {
                    return Command::Set(Set::new_invalid());
                }
                Command::Set(Set::from_key_val(arg.key, arg.val.unwrap()))
            }
            _ => Command::Invalid,
        }
    }
}

fn make_args(req: &Request, request_buff: &[u8], idx_of_body: usize) -> Args {
    let method = req.method.unwrap();
    let all_path_vec = split_on_path(req.path.unwrap());

    // using regular GET request (pass data through URL)
    if method == "GET" {
        let action = all_path_vec[0];
        match action.to_uppercase().as_str() {
            "GET" => {
                if all_path_vec.len() < 2 || all_path_vec[1].is_empty() || all_path_vec.len() > 2 {
                    return Args::new_invalid("GET");
                }
                let key = all_path_vec[1];
                return Args {
                    valid: true,
                    command: String::from("GET"),
                    key: String::from(key),
                    val: None,
                };
            }
            "SET" => {
                if all_path_vec.len() < 3
                    || all_path_vec[1].is_empty()
                    || all_path_vec[2].is_empty()
                    || all_path_vec.len() > 3
                {
                    return Args::new_invalid("SET");
                }
                let key = all_path_vec[1];
                let val = all_path_vec[2];
                return Args {
                    valid: true,
                    command: String::from("SET"),
                    key: String::from(key),
                    val: Some(String::from(val)),
                };
            }
            _ => return Args::new_invalid("INVALID"),
        }
    } else if method == "POST" {
        // using POST request (SET ONLY) that passes kv-pair through body
        if all_path_vec.len() != 1 {
            return Args::new_invalid("SET");
        }
        let body = request_buff[idx_of_body..].to_vec();
        match parse_json(&body) {
            Ok(value) => {
                if let Some(obj) = value.as_object() {
                    // if multiple kv pairs are passed, only the first one is used  for now
                    if let Some((key, value)) = obj.iter().next() {
                        return Args {
                            valid: true,
                            command: String::from("SET"),
                            key: key.to_string(),
                            val: Some(value.as_str().unwrap().to_string()),
                        };
                    }
                }
                return Args::new_invalid("SET");
            }
            Err(_) => return Args::new_invalid("SET"),
        }
    }
    Args::new_invalid("INVALID")
}

fn split_on_path(input: &str) -> Vec<&str> {
    let all: Vec<&str> = input.split("/").collect();
    if let Some((_, rest)) = all.split_first() {
        return rest.to_vec();
    }
    all
}

fn parse_json(bytes: &[u8]) -> Result<Value> {
    let value = serde_json::from_slice(bytes)?;
    Ok(value)
}
