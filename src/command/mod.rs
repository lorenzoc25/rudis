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
    pub fn from_bytes(buff: &[u8]) -> Command {
        if buff.len() == 0 {
            return Command::Invalid;
        }

        let mut headers = [httparse::EMPTY_HEADER; 16];

        let req = parse_request(&buff, &mut headers).unwrap();

        let args = split_on_path(req.path.unwrap());
        println!("{:?}", args);
        match args[0].to_uppercase().as_str() {
            "GET" => Command::Get(Get::from_key(args[1])),
            "SET" => Command::Set(Set::from_key_val(args[1], args[2])),
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
