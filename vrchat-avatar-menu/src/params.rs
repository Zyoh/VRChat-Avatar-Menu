// Copyright (c) 2023 Zoe <zoe@zyoh.ca>

use std::{error::Error, io::Read, path::Path};

use regex::Regex;

#[derive(Debug)]
pub struct OscConfigParameter {
    pub name: String,
    pub ptype: String,
}

pub fn get_avatar_params(path: &Path) -> Result<Vec<OscConfigParameter>, Box<dyn Error>> {
    // Why wouldn't serde just work?

    let mut params: Vec<OscConfigParameter> = Vec::new();

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
            let param = OscConfigParameter {
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

pub struct SavedParameter {
    pub name: String,
    pub raw_value: f32,
}

impl SavedParameter {
    pub fn from_file(path: &Path) -> Result<Vec<Self>, Box<dyn Error>> {
        // Why wouldn't serde just work? pt.2
    
        let mut params: Vec<Self> = Vec::new();
    
        let mut file = std::fs::File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
    
        let mut name: Option<String> = None;
        let mut string_value: Option<String> = None;
    
        for line in contents.split(',') {
            let re = Regex::new(r#"\{"name":"(.+)""#)?;
            for cap in re.captures_iter(line) {
                let mut captures = cap.iter();
    
                if let Some(Some(_name)) = captures.nth(1) {
                    name = Some(_name.as_str().replace(' ', "_").to_string());
                }
            }
    
            let re = Regex::new(r#""value":(.+)\}"#)?;
            for cap in re.captures_iter(line) {
                let mut captures = cap.iter();
                if let Some(Some(_value)) = captures.nth(1) {
                    string_value = Some(_value.as_str().to_string());
                }
            }
    
            if name.is_none() {
                string_value = None;
            }

            let mut value: Option<f32> = None;

            if let Some(_raw_value) = &string_value {
            if let Ok(_raw_value) = _raw_value.parse::<f32>() {
                value = Some(_raw_value);
            }}
    
            if let (Some(_name), Some(_value)) = (&name, &value) {
                let param = Self {
                    name: _name.to_string(),
                    raw_value: _value.to_owned(),
                };
    
                params.push(param);
    
                name = None;
                string_value = None;
            }
        }
        
        Ok(params)
    }
}
