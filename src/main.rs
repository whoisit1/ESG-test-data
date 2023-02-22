extern crate rand;

use serde::{Deserialize, Serialize};
use std::{
    fmt,
    fs::File,
    io::{Read, Write},
};

use rand::distributions::WeightedIndex;
use rand::prelude::*;
use random_string::generate;

use rand::distributions::Distribution;
use statrs::distribution::Normal;

use chrono::{Datelike, Duration, Utc};

#[derive(Debug, Serialize, Deserialize)]
struct CommonFactors {
    department: Vec<String>,
    dept_wtg: Vec<u8>, // Weightages for the various departments
    emp_type: Vec<String>,
    emp_type_wtg: Vec<u8>,
    emp_gender: Vec<String>,
    emp_gender_wtg: Vec<u8>,
    marital_status: Vec<String>,
    band: Vec<String>,
    band_wtg: Vec<u8>,
    reason_for_separation: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Country {
    country: String,
    location: Vec<String>,
    cost_centre: Vec<String>,
    religion: Vec<String>,
    ethnicities: Vec<String>,
    ethn_wtg: Vec<u8>,
    minorities: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Countries {
    countries: Vec<Country>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct Employee {
    empid: String,
    emp_type: String,
    dob: String,
    age: u8,
    doj: String,
    dos: String,
    reason_for_sep: String,
    gender: String,
    marital_status: String,
    band: String,
    mgmt_level: String,
    compensation: f32,
    country: String,
    location: String,
    department: String,
    cost_centre: String,
    religion: String,
    ethnicity: String,
    minority: String,
}

impl fmt::Display for Employee {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {:.2}, {}, {}, {}, {}, {}, {}, {}",
            self.empid,
            self.emp_type,
            self.dob,
            self.age,
            self.doj,
            self.dos,
            self.reason_for_sep,
            self.gender,
            self.marital_status,
            self.band,
            self.mgmt_level,
            self.compensation,
            self.country,
            self.location,
            self.department,
            self.cost_centre,
            self.religion,
            self.ethnicity,
            self.minority
        )
    }
}

fn main() {
    let mut file = File::open("../config/common_factors.json").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    let common_factors: CommonFactors = serde_json::from_str(&buf).unwrap();
    drop(file);

    let mut file = File::open("../config/countries.json").unwrap();
    let mut buf2 = String::new();
    file.read_to_string(&mut buf2).unwrap();
    let listed_countries: Countries = serde_json::from_str(&buf2).unwrap();
    drop(file);

    // Output file
    let mut file = File::create("../output/out.csv").unwrap();

    let charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";
    let now = Utc::now();

    // Not older than 62 years; erring on the higher side
    let earliest_bday = now - Duration::days(62 * 366);
    // Not younger than 18 years; erring on the higher side

    // Difference in days between the oldest and youngest employee
    let offset_days_range = (62 - 18) * 366 - 1;

    for _ in 0..1000 {
        let picked_country = listed_countries
            .countries
            .choose(&mut rand::thread_rng())
            .unwrap();

        let country = picked_country.country.to_string();
        let location = picked_country
            .location
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_string();

        let mut emp = Employee {
            ..Default::default()
        };

        emp.empid = generate(8, charset);
        // Skipping empname; not required for now

        let emp_type = &common_factors.emp_type;
        let wtg = &common_factors.emp_type_wtg;
        let dist = WeightedIndex::new(wtg).unwrap();
        emp.emp_type = emp_type[dist.sample(&mut rand::thread_rng())].to_string();

        let offset_days = rand::thread_rng().gen_range(0..offset_days_range);
        let dob = earliest_bday + Duration::days(offset_days);
        let age = ((now - dob).num_days() / 365) as u8;
        // Date of Joining = date of birth + 18 years + a random #days from then to Now
        let upper_range = (now - dob - Duration::days(18 * 366)).num_days();

        let doj = dob
            + Duration::days(18 * 366)
            + Duration::days(rand::thread_rng().gen_range(0..upper_range));

        let doj_until_now = (now - doj).num_days();
        let mut dos: Option<String> = None;

        // generate a random value [0..1)
        let n: f64 = (&mut rand::thread_rng()).gen::<f64>() * 100.0;
        // Using an attrition rate of 12%
        if let 0..=12 = n as u8 {
            let dos1 = doj + Duration::days(rand::thread_rng().gen_range(1..doj_until_now - 1));
            dos = Some(format!(
                "{}-{:02}-{:02}",
                dos1.year(),
                dos1.month(),
                dos1.day()
            ));
        };

        emp.minority = match n as u8 {
            0..=2 => {
                // This means I am hard-coding a ceiling of 2% for Minorities in any given selection
                picked_country
                    .minorities
                    .choose(&mut rand::thread_rng())
                    .unwrap()
                    .to_string()
            }
            _ => "".to_string(),
        };

        emp.dob = format!("{}-{:02}-{:02}", dob.year(), dob.month(), dob.day());
        emp.age = age;
        emp.doj = format!("{}-{:02}-{:02}", doj.year(), doj.month(), doj.day());
        // emp.dos = dos;

        // if let Some(x) = dos {
        //     x // a valid date-of-separation exists
        // } else {
        //     "".to_string()
        // },
        // I like match more than "if let"

        // let (dos1, rfs1) = match dos {
        //let (emp.dos, emp.reason_for_sep) = match dos {
        match dos {
            Some(x) => {
                //x,
                let rfs = &common_factors
                    .reason_for_separation
                    .choose(&mut rand::thread_rng())
                    .unwrap()
                    .to_string();
                // (&x.to_owned(), &rfs.to_owned())
                // format!("{}, {}", x, rfs.to_string())
                emp.dos = x;
                emp.reason_for_sep = rfs.to_string();
            }
            _ => {
                emp.dos = "".to_owned();
                emp.reason_for_sep = "".to_owned();
            } // (&"".to_string(), &"".to_string())}, // format!("{}, {}", "", ""),
        };

        // emp.dos = dos1.to_string();
        // emp.reason_for_sep = rfs1.to_string();

        let emp_gender = &common_factors.emp_gender;
        let wtg = &common_factors.emp_gender_wtg;
        let dist = WeightedIndex::new(wtg).unwrap();
        emp.gender = emp_gender[dist.sample(&mut rand::thread_rng())].to_string();

        emp.marital_status = common_factors
            .marital_status
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_string();

        let band = &common_factors.band;
        let wtg = &common_factors.band_wtg;
        let dist = WeightedIndex::new(wtg).unwrap();
        let band = band[dist.sample(&mut rand::thread_rng())].to_string();

        // We are receiving the band and the mean-monthly-compensation as ':' separated values
        let split: Vec<&str> = band.split(':').collect();
        emp.band = split[0].to_string();
        emp.mgmt_level = match emp.band.as_str() {
            // .as_str() {  // my first use of the "as_str()" method!
            "N1" | "N2" => "NML".to_string(),
            "M1" | "M2" => "JML".to_string(),
            "M3" | "M4" => "MML".to_string(),
            "M5" | "M6" => "SML".to_string(),
            "M7" | "EC" => "TML".to_string(),
            _ => "Unknown level".to_string(),
        };
        let mean_comp: f64 = split[1].parse().unwrap();
        // compensation is 2 stdev from mean compensation.
        emp.compensation = Normal::new(mean_comp, 2.0)
            .unwrap()
            .sample(&mut rand::thread_rng()) as f32;

        emp.country = country;
        emp.location = location;

        let department = &common_factors.department;
        let wtg = &common_factors.dept_wtg;
        let dist = WeightedIndex::new(wtg).unwrap();

        // Now the department is picked according to the given weights.
        emp.department = department[dist.sample(&mut rand::thread_rng())].to_string();

        emp.cost_centre = picked_country
            .cost_centre
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_string();

        emp.religion = picked_country
            .religion
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_string();

        let ethnicity = &picked_country.ethnicities;
        let wtg = &picked_country.ethn_wtg;
        let dist = WeightedIndex::new(wtg).unwrap();
        emp.ethnicity = ethnicity[dist.sample(&mut rand::thread_rng())].to_string();

        // use statrs::Bernoulli;
        // Bernoulli is a special case of Binomial where it is success or failure
        // with a given probability (here 20%) and only one trial.
        // let n: Bernoulli = Bernoulli::new(0.2).unwrap();
        // Could not get the below to work using a Bernoulli distribution :-(

        // generate a random value [0..1)
        let n: f64 = (&mut rand::thread_rng()).gen::<f64>() * 100.0;
        emp.minority = match n as u8 {
            0..=2 => {
                // This means I am hard-coding a ceiling of 2% for Minorities in any given selection
                picked_country
                    .minorities
                    .choose(&mut rand::thread_rng())
                    .unwrap()
                    .to_string()
            }
            _ => "".to_string(),
        };

        // println!("{}", emp);
        let emp1 = format!("{}\n", emp);

        file.write_all(emp1.as_bytes())
            .expect("Could not write output");
    }
}
