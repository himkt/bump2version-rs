use std::fs;

use clap::{Parser, ValueEnum};
use regex::Regex;
use toml::Value;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(value_enum)]
    action: Action,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum Action {
    Major,
    Minor,
    Patch,
}

#[derive(Debug, Clone, Copy)]
pub struct Version {
    major: usize,
    minor: usize,
    patch: usize,
}

impl Version {
    pub fn to_str(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
    pub fn bump(self, action: Action) -> Version {
        let mut version = self;

        match action {
            Action::Major => {
                version.major += 1;
                version.minor = 0;
                version.patch = 0;
            }
            Action::Minor => {
                version.minor += 1;
                version.patch = 0;
            }
            Action::Patch => version.patch += 1,
        }

        version
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Cli::parse();
    println!("{:?}", opt);

    let content = fs::read_to_string("bumpversion.toml")?;
    let config = content.parse::<Value>()?;
    let bumpversion: &Value = config.get("bumpversion").unwrap();
    let version = bumpversion.get("version").unwrap().to_string();
    println!("{:?}", bumpversion);
    println!("{:?}", version);

    let pattern = Regex::new(r"(\d+)\.(\d+)\.(\d+)")?;
    let result = pattern.captures(&version).unwrap();
    println!("{:?}", result);

    let version_obj = Version {
        major: result.get(1).unwrap().as_str().parse::<usize>()?,
        minor: result.get(2).unwrap().as_str().parse::<usize>()?,
        patch: result.get(3).unwrap().as_str().parse::<usize>()?,
    };
    println!("{:?}", version_obj);
    let version_obj = version_obj.bump(opt.action);
    println!("{:?}", version_obj);
    Ok(())
}
