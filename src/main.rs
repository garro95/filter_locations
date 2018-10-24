extern crate chrono;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate quicli;
extern crate serde_json;
extern crate serde;

use chrono::{DateTime,
             naive::NaiveDateTime,
             offset::{Utc, Local}};
use quicli::prelude::*;

use std::fs::File;
use std::str::FromStr;

fn deserialize_timestamp<'de, D>(deserializer: D) -> std::result::Result<DateTime<Utc>, D::Error>
where D: serde::de::Deserializer<'de> {
    let s: String = serde::Deserialize::deserialize(deserializer)?;
    let n = u64::from_str(s.as_str());
    match n {
        Ok(n) => Ok(DateTime::from_utc(NaiveDateTime::from_timestamp((n/1000) as i64, (n%1000*1000) as u32), Utc)),
        Err(_) => Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(s.as_str()), &"a string containing numbers")),
    }
}

#[derive(Serialize, Deserialize)]
struct Location {
    #[serde(rename = "timestampMs", deserialize_with = "deserialize_timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "latitudeE7")]
    latitude: i32,
    #[serde(rename = "longitudeE7")]
    longitude: i32,
    accuracy: u32
}

#[derive(Serialize, Deserialize)]
struct Locations {
    locations: Vec<Location>
}

#[derive(StructOpt)]
enum Period {
    #[structopt(name = "from")]
    From{
        from: DateTime<Local>
    },
    #[structopt(name = "to")]
    To{
        to: DateTime<Local>
    },
    #[structopt(name = "fromto")]
    FromTo{
        from: DateTime<Local>,
        to: DateTime<Local>
    }
}

#[derive(StructOpt)]
/// Filter locations data 
struct Interface {
    filename: String,
    #[structopt(subcommand)]
    period: Option<Period>,
    
}

main!(|args: Interface| {
    let file = File::open(&args.filename)?;
    let locations: Locations = serde_json::from_reader(file)?;

    let iter = locations.locations.par_iter();
    let iter = iter.filter(|x| {
        match args.period {
            None => true,
            Some(Period::From{from}) => x.timestamp > from.with_timezone(&Utc),
            Some(Period::To{to}) => x.timestamp < to.with_timezone(&Utc),
            Some(Period::FromTo{from, to}) => x.timestamp > from.with_timezone(&Utc) && x.timestamp < to.with_timezone(&Utc),
        }
    });

    println!("Loaded {} locations", locations.locations.par_iter().count());
    println!("Filtered {} locations", iter.count());
});
