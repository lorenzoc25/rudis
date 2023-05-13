mod get;

pub use get::Get;

mod set;
use httparse::{Request, Status};
pub use set::Set;

#[derive(Debug)]
pub enum Command {
    Set(Set),
    Get(Get),
    Invalid,
}

impl Command {
    pub fn from_bytes(http_request_buff: &[u8]) -> Command {
        if http_request_buff.len() == 0 {
            return Command::Invalid;
        }

        let mut headers = [httparse::EMPTY_HEADER; 16];
        let req = parse_request(&http_request_buff, &mut headers).unwrap();

        let args = split_on_path(req.path.unwrap());

        match args[0].to_uppercase().as_str() {
            "GET" => {
                if args.len() < 2 || args[1].is_empty() || args.len() > 2 {
                    return Command::Get(Get::new_invalid());
                }
                Command::Get(Get::from_key(args[1]))
            }
            "SET" => {
                if args.len() < 3 || args[1].is_empty() || args[2].is_empty() || args.len() > 3 {
                    return Command::Set(Set::new_invalid());
                }
                Command::Set(Set::from_key_val(args[1], args[2]))
            }
            _ => Command::Invalid,
        }
    }
}

fn parse_request<'a>(
    buff: &'a [u8],
    headers: &'a mut [httparse::Header<'a>; 16],
) -> Option<Request<'a, 'a>> {
    let mut req = Request::new(headers);
    match req.parse(buff) {
        Ok(Status::Complete(_)) => Some(req),
        _ => None,
    }
}

fn split_on_path(input: &str) -> Vec<&str> {
    let all: Vec<&str> = input.split("/").collect();
    if let Some((_, rest)) = all.split_first() {
        return rest.to_vec();
    }
    all
}
