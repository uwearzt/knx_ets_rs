// ------------------------------------------------------------------------------
// Copyright 2019 Uwe Arzt, mail@uwe-arzt.de
// SPDX-License-Identifier: Apache-2.0
// ------------------------------------------------------------------------------

use knx_rs::address::Address;

use std::str::FromStr;

use std::collections::HashMap;

use std::io::{BufRead, BufReader};
use std::fs::File;

pub struct Ets {
    project: String,
    maingroup_name: HashMap<u8, String>,
    middlegroup_name: HashMap<(u8, u8), String>,
    address_name: HashMap<Address, String>,
}

// ------------------------------------------------------------------------------
impl Ets {
    pub fn new(opc_file: &str) -> Ets {
        let mut ets = Ets {
            project: String::new(),
            maingroup_name: HashMap::new(),
            middlegroup_name: HashMap::new(),
            address_name: HashMap::new(),
        };
        ets.ets_read(opc_file);
        ets
    }
    fn ets_read(&mut self, opc_file: &str) {
        let file = File::open(opc_file).unwrap();
        let mut buf = BufReader::new(file);

        let _ = buf.read_line(&mut self.project);
        self.project = self.project.trim().to_string();

        for line in buf.lines() {
            Ets::parse_ets_opc_line(self, &line.unwrap());
        }
    }
    fn parse_ets_opc_line(&mut self, line: &str) {
        let (main, middle, address, name, _eis) = parse_line(line).unwrap().1;
        let address = Address::from_str(address).unwrap();
        // println!("{}, {}", address, name);
        self.maingroup_name.insert(address.main(), main.to_string());
        self.middlegroup_name
            .insert(address.middle(), middle.to_string());
        self.address_name.insert(address, name.to_string());
    }

    pub fn project_string(&self) -> &str {
        &self.project
    }
    pub fn main_string(&self, main: u8) -> &str {
        self.maingroup_name.get(&main).unwrap()
    }
    pub fn middle_string(&self, middle: (u8, u8)) -> &str {
        self.middlegroup_name.get(&middle).unwrap()
    }
    pub fn address_string(&self, address: &Address) -> &str {
        self.address_name.get(&address).unwrap()
    }

    pub fn print(&self) {
        println!("-----------------------------");
        println!("project: {}", self.project);
        println!("-----------------------------");
        for (maingroup, ref maingroup_string) in self.maingroup_name.iter() {
            println!("{}:\t{}", maingroup, maingroup_string);
        }
        println!("-----------------------------");
        for (&(maingroup, middlegroup), ref middlegroup_string) in self.middlegroup_name.iter() {
            println!("{}/{}:\t{}", maingroup, middlegroup, middlegroup_string);
        }
        println!("-----------------------------");
        for (address, ref address_string) in self.address_name.iter() {
            println!("{}:\t{}", address, address_string);
        }
        println!("-----------------------------");
    }
}

// ------------------------------------------------------------------------------
named!(parse_line<&str, (&str, &str, &str, &str, &str)>,
    do_parse!(
        main: take_until_and_consume!(".") >>
        middle: take_until_and_consume!(".") >>
        address: take_until_and_consume!("\t") >>
        name: take_until_and_consume!("\t") >>
        eis: take_until_and_consume!("\t") >>
        (main, middle, address, name, eis)
    )
);
