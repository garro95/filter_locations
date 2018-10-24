/*
 *  Copyright © 2018 Gianmarco Garrisi
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

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
    /// Select all the locations from a certain moment. The time instant must be expressed in [RFC3339](https://tools.ietf.org/html/rfc3339) format: e.g. 1996-12-19T16:39:57-08:00
    #[structopt(name = "from")]
    From{
        from: DateTime<Local>
    },
    /// Select all the locations collected until a certain moment. The time instant must be expressed in [RFC3339](https://tools.ietf.org/html/rfc3339) format: e.g. 1996-12-19T16:39:57-08:00
    #[structopt(name = "to")]
    To{
        to: DateTime<Local>
    },
    /// Select all the locations collected within a temporal window. The time instants must be expressed in [RFC3339](https://tools.ietf.org/html/rfc3339) format: e.g. 1996-12-19T16:39:57-08:00
    #[structopt(name = "fromto")]
    FromTo{
        from: DateTime<Local>,
        to: DateTime<Local>
    }
}

#[derive(StructOpt)]
/// Filter locations data 
struct Interface {
    /// The file containing the data to filter
    filename: String,
    /// Add a filter based on a time period
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
