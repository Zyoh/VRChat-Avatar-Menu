// Copyright (c) 2023 Zoe <zoe@zyoh.ca>

use std::{error::Error, io::Read};

use regex::Regex;

#[derive(Debug)]
pub struct Parameter {
    pub name: String,
    pub ptype: String,
}

pub fn get_avatar_params<P>(path: P) -> Result<Vec<Parameter>, Box<dyn Error>> where P: AsRef<std::path::Path> {
    // Why wouldn't serde just work?

    let mut params: Vec<Parameter> = Vec::new();

    let mut file = std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut name: Option<String> = None;
    let mut ptype: Option<String> = None;

    for line in contents.lines() {
        let re = Regex::new(r#"^\s+"name": "(.+)","#)?;
        for cap in re.captures_iter(line) {
            let mut captures = cap.iter();

            if let Some(Some(_name)) = captures.nth(1) {
                name = Some(_name.as_str().replace(' ', "_").to_string());
            }
        }

        let re = Regex::new(r#"^\s+"type": "(\S+)""#)?;
        for cap in re.captures_iter(line) {
            let mut captures = cap.iter();
            if let Some(Some(_ptype)) = captures.nth(1) {
                ptype = Some(_ptype.as_str().to_string());
            }
        }

        if name.is_none() {
            ptype = None;
        }

        if let (Some(_name), Some(_ptype)) = (&name, &ptype) {
            let param = Parameter {
                name: _name.to_string(),
                ptype: _ptype.to_string(),
            };

            params.push(param);

            name = None;
            ptype = None;
        }
    }
    
    Ok(params)
}
