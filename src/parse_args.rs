#[derive(PartialEq)]
pub enum Error {
    TooFewArgs,
    CantParseSize,
    BadArgs
}

/// The struct in charge of parsing arguments
///
/// Handles getting the file name as a string, and the size as a tuple of u32s,
/// but not opening the image itself. That's done in the main function.
///
/// ```
/// let args = ArgParser::new(std::env::args().skip(1).collect()).unwrap();
/// println!("Path: {}, Size: {:?}", args.path(), args.size());
/// ```
pub struct ArgParser {
    path: String,
    size: Option<(u32, u32)>
}

impl ArgParser {
    pub fn new<T: ToString>(args: Vec<T>) -> Result<Self, Error> {
        if args.len() == 0 {
            return Err(Error::TooFewArgs)
        } else if args.len() > 2 {
            return Err(Error::BadArgs)
        }

        // Would love to use .and_then() here, but sadly try! doesn't like closures.
        let size = match args.get(1) {
            Some(string) => Some(try!(parse_size(string))),
            None => None
        };

        Ok(ArgParser {
            path: args[0].to_string(),
            size: size
        })
    }

    /// Returns the path
    pub fn path(&self) -> &String {
        &self.path
    }

    /// Returns the size (optional)
    pub fn size(&self) -> &Option<(u32, u32)> {
        &self.size
    }
}

fn parse_size<T: ToString>(s: &T) -> Result<(u32, u32), Error> {
    let string = s.to_string();

    let components = string.splitn(2, 'x')
        .filter_map(|x| x.parse().ok())
        .collect::<Vec<u32>>();

    if components.len() == 2 {
        Ok((components[0], components[1]))
    } else {
        Err(Error::CantParseSize)
    }
}

#[cfg(test)]
mod tests {
    use super::Error;
    use super::ArgParser;

    #[test]
    fn test_only_path() {
        let parser = ArgParser::new(vec!["image.png"]);
        match parser {
            Ok(args) => {
                assert!(*args.path() == "image.png");
                assert!(*args.size() == None);
            }
            _ => panic!()
        }
    }

    #[test]
    fn test_path_and_size() {
        let parser = ArgParser::new(vec!["image.png", "50x25"]);
        match parser {
            Ok(args) => {
                assert!(*args.path() == "image.png");
                assert!(*args.size() == Some((50, 25)));
            },
            _ => panic!()
        }
    }

    #[test]
    fn test_too_few_args() {
        let parser = ArgParser::new::<&str>(vec![]);
        match parser {
            Err(e) => assert!(e == Error::TooFewArgs),
            _ => panic!()
        }
    }

    #[test]
    fn test_cant_parse_size() {
        let parser = ArgParser::new(vec!["image.png", "junk"]);
        match parser {
            Err(e) => assert!(e == Error::CantParseSize),
            _ => panic!()
        }
    }

    #[test]
    fn test_bad_args() {
        let parser = ArgParser::new(vec!["image.png", "50x25", "junk"]);
        match parser {
            Err(e) => assert!(e == Error::BadArgs),
            _ => panic!()
        }
    }
}
