extern crate csv;
extern crate rand;

use serde::{Deserialize, Serialize};
use std::{fmt, fs::File, io::Write};

// use rand::distributions::WeightedIndex;
use rand::prelude::*;
// use chrono::{Datelike, Duration, Utc};

#[derive(Debug, serde::Deserialize)]
struct Employee {
    empid: String,
    gender: String,
    country: String,
    cost_centre: String,
}

#[derive(Debug, serde::Deserialize)]
struct Accidents {
    empid: String,
    injury_type: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct LeaveDump {
    empid: String,
    gender: String,
    leave_type: String,
    days_applied: u8,
    left_within_a_year: bool,
    business_unit: String,
    cost_centre: String,
    // process_or_activity: String,
}

impl fmt::Display for LeaveDump {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{},{},{},{},{},{},{}", //, {}",
            self.empid,
            self.gender,
            self.leave_type,
            self.days_applied,
            self.left_within_a_year,
            self.business_unit,
            self.cost_centre,
            // self.process_or_activity,
        )
    }
}

fn main() {
    let leave_type = ["Maternity", "Paternity", "Paid", "Casual", "Injury", "Sick"];

    let mut outfile = File::create("../output/leaves.csv").unwrap();

    let emp_file = File::open("../input/employee_DB.csv").unwrap();
    // let emp_file = File::open("../input/3_emps.csv").unwrap();
    let mut emp_rdr = csv::Reader::from_reader(emp_file);

    // the accidents file generated is copied to the inputs folder and a header rec. inserted to allow serde_deserialization
    let accidents_file = File::open("../input/accidents.csv").unwrap();
    let mut accidents_ds: Vec<Accidents> = Vec::new();

    let mut accidents_rdr = csv::Reader::from_reader(accidents_file);
    for accident in accidents_rdr.deserialize() {
        let accident_rec: Accidents = accident.unwrap();
        accidents_ds.push(accident_rec);
    }

    // Type 	Max_Days Applicable_to 	%Employee_Availing	Left within 1 year after returning
    // Maternity 	182	Females		8%			25% of 8%
    // Paternity 	21	Males + TG	7%			15% of 7%
    // Paid 		22	All		90%
    // Casual		12	All		30%
    // Injury		21	Only for LTI & MI (see Safety Fact Set tab)	34%
    // Sick Leave	5	All		8%

    for record in emp_rdr.deserialize() {
        let employee: Employee = record.unwrap();
        let mut outrec = LeaveDump {
            ..Default::default()
        };

        // default setting unless overridden below.
        outrec.left_within_a_year = false;
        // The default value of 0 here will serve as a flag for injury-related leaves.
        // The employee may or may not be injured and we need to know whether the rec. needs to be written out.
        outrec.days_applied = 0;

        for ltype in leave_type {
            match ltype {
                "Maternity" => {
                    if employee.gender == "Female" {
                        let n = (&mut rand::thread_rng()).gen_range(0..=100);
                        if n <= 8 {
                            // Employee is taking maternity leave and usually between 120 & 182 days.
                            outrec.leave_type = "Maternity".to_string();
                            outrec.days_applied = (&mut rand::thread_rng()).gen_range(120..=182);
                            // 25% chance of staff returning from maternity leave quitting within a year.
                            let n = (&mut rand::thread_rng()).gen_range(0..=100);
                            if n <= 25 {
                                outrec.left_within_a_year = true;
                            }
                        }
                    }
                }
                "Paternity" => {
                    if employee.gender != "Female" {
                        let n = (&mut rand::thread_rng()).gen_range(0..=100);
                        if n <= 7 {
                            // Employee is taking paternity leave and usually between 10 & 21 days.
                            outrec.leave_type = "Paternity".to_string();
                            outrec.days_applied = (&mut rand::thread_rng()).gen_range(10..=21);
                            // 15% chance of staff returning from paternity leave quitting within a year.
                            let n = (&mut rand::thread_rng()).gen_range(0..=100);
                            if n <= 15 {
                                outrec.left_within_a_year = true;
                            }
                        }
                    }
                }
                "Paid" => {
                    let n = (&mut rand::thread_rng()).gen_range(0..=100);
                    if n <= 90 {
                        outrec.leave_type = "Paid".to_string();
                        outrec.days_applied = (&mut rand::thread_rng()).gen_range(10..=22);
                        outrec.left_within_a_year = false;
                    }
                }
                "Casual" => {
                    let n = (&mut rand::thread_rng()).gen_range(0..=100);
                    if n <= 30 {
                        outrec.leave_type = "Casual".to_string();
                        outrec.days_applied = (&mut rand::thread_rng()).gen_range(5..=12);
                        outrec.left_within_a_year = false;
                    }
                }
                "Sick" => {
                    let n = (&mut rand::thread_rng()).gen_range(0..=100);
                    if n <= 5 {
                        outrec.leave_type = "Sick".to_string();
                        outrec.days_applied = (&mut rand::thread_rng()).gen_range(0..=5);
                        outrec.left_within_a_year = false;
                    }
                }
                // Simplistic scenario where an employee is not injured more than once during a year.
                "Injury" => {
                    for accident in accidents_ds.iter() {
                        if accident.empid == employee.empid
                            && (accident.injury_type == "Lost-time injury"
                                || accident.injury_type == "Medical injury")
                        {
                            outrec.leave_type = "Injury".to_string();
                            outrec.days_applied = (&mut rand::thread_rng()).gen_range(10..=21);
                            outrec.left_within_a_year = false;
                        }
                    }
                }
                _ => {}
            }
        }

        outrec.empid = employee.empid;
        outrec.gender = employee.gender;
        outrec.business_unit = employee.country;
        outrec.cost_centre = employee.cost_centre;

        // Write out a leave record only if a leave is applied.
        if outrec.days_applied > 0 {
            let outrec1 = format!("{}\n", outrec);
            outfile
                .write_all(outrec1.as_bytes())
                .expect("Could not write output");
        }
    }
}
