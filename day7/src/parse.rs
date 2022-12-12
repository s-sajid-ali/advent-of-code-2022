use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::error::Error;
use nom::error::ErrorKind;
//use nom::Err;
use nom::Finish;
use nom::IResult;

use crate::types::types::Line;

mod parse {

    use crate::parse::digit1;
    use crate::parse::tag;
    use crate::parse::IResult;

    pub fn parse_command(input: &str) -> IResult<&str, &str> {
        tag("$")(input)
    }

    pub fn parse_cd_command(input: &str) -> IResult<&str, &str> {
        tag(" cd")(input)
    }

    pub fn parse_ls_dirout(input: &str) -> IResult<&str, &str> {
        tag("dir")(input)
    }

    pub fn parse_ls_fileout(input: &str) -> IResult<&str, &str> {
        digit1(input)
    }
}

pub fn parse_line(input: &str) -> Option<Line> {
    match parse::parse_command(input).finish() {
        Ok((str1, _)) => {
            //println!("str1:{}, str2:{}", str1, str2);
            match parse::parse_cd_command(str1).finish() {
                Ok((dst, _)) => {
                    let cdcommand = Line::CdCommand {
                        location: dst.trim().to_string(),
                    };

                    return Some(cdcommand);
                }
                Err(e) => {
                    if e == Error::new(&str1, ErrorKind::Tag) {
                        //eprintln!("tag error when parsing cd command {}", e);
                        //println!("ls command inferred, given input : '{}'", str1);
                        return Some(Line::LsCommand);
                    } else {
                        eprintln!("generic error: {}", e);
                        return None;
                    }
                }
            }
        }
        Err(_) => match alt((parse::parse_ls_dirout, parse::parse_ls_fileout))(input).finish() {
            Ok((str1, str2)) => {
                if str2 == "dir" {
                    let dirname = Line::DirOutput {
                        name: (str1.trim().to_string()),
                    };
                    return Some(dirname);
                } else {
                    let fileout = Line::FileOutput {
                        size: (str2.parse::<usize>().expect("error parsing file size!")),
                        name: (str1.trim().to_string()),
                    };
                    return Some(fileout);
                }
            }
            Err(e) => {
                eprintln!("generic error when parsing file output: {}", e);
                return None;
            }
        },
    }
}
