use chrono::NaiveDateTime;
use clap::{App, Arg};

pub const TIME_PARSE_STRING: &str = "%Y-%m-%d %H:%M";

pub fn get_app() -> App<'static> {
    App::new("CWA Code Generator")
    .version("0.1.0")
    .author("Folke 'joru' Gleumes <folke@gleumes.org>")
    .arg(
      Arg::new("output")
        .short('o')
        .long("output")
        .value_name("FILE")
        .about("Output location for the Qr code. If not provided the qr code will be printed to the terminal")
        .takes_value(true)
        .required(false),
    )
    .arg(
      Arg::new("description")
        .long("description")
        .takes_value(true)
        .required(true)
        .validator(str_max_len)
        .about("Event description"),
    )
    .arg(
      Arg::new("address")
        .long("address")
        .takes_value(true)
        .required(true)
        .validator(str_max_len)
        .about("Event address"),
    )
    .arg(
      Arg::new("default_check_in_length_in_minutes")
        .long("default-checkin-time")
        .takes_value(true)
        .default_value("15")
        .about("the usual length people will spend at your event"),
    )
    .arg(
      Arg::new("type")
        .long("type")
        .takes_value(true)
        .required(true)
        .value_name("INTEGER")
        .validator(type_num)
        .about(
          "LOCATION_TYPE_UNSPECIFIED = 0\n\
          LOCATION_TYPE_PERMANENT_OTHER = 1\n\
          LOCATION_TYPE_TEMPORARY_OTHER = 2\n\
          LOCATION_TYPE_PERMANENT_RETAIL = 3\n\
          LOCATION_TYPE_PERMANENT_FOOD_SERVICE = 4\n\
          LOCATION_TYPE_PERMANENT_CRAFT = 5\n\
          LOCATION_TYPE_PERMANENT_WORKPLACE = 6\n\
          LOCATION_TYPE_PERMANENT_EDUCATIONAL_INSTITUTION = 7\n\
          LOCATION_TYPE_PERMANENT_PUBLIC_BUILDING = 8\n\
          LOCATION_TYPE_TEMPORARY_CULTURAL_EVENT = 9\n\
          LOCATION_TYPE_TEMPORARY_CLUB_ACTIVITY = 10\n\
          LOCATION_TYPE_TEMPORARY_PRIVATE_EVENT = 11\n\
          LOCATION_TYPE_TEMPORARY_WORSHIP_SERVICE = 12",
        ),
    )
    .arg(
      Arg::new("prefix")
        .long("prefix")
        .takes_value(true)
        .default_value("https://e.coronawarn.app?v=1#")
        .about("Url from your countries "),
    )
    .arg(
      Arg::new("start-time")
      .long("start-time")
      .about("start time of the event formatted as %Y-%m-%d %H:%M")
      .takes_value(true)
      .validator(timedate)
    )
    .arg(
      Arg::new("end-time")
      .long("end-time")
      .about("end time of the event formatted as %Y-%m-%d %H:%M")
      .takes_value(true)
      .validator(timedate)
    )
    .arg(
      Arg::new("dimensions")
      .long("dimensions")
      .about("dimensions of the resulting code image, i.e. '1000x1000'")
      .takes_value(true)
      .validator(dimensions)
    )
}

fn dimensions(val: &str) -> Result<(), String> {
    let dim: Vec<&str> = val.split("x").collect();
    if dim.len() == 2 {
        let h = dim.get(0).unwrap().parse::<u32>();
        let w = dim.get(1).unwrap().parse::<u32>();

        if h.is_ok() && w.is_ok() {
            Ok(())
        } else {
            Err("failed to parse".to_owned())
        }
    } else {
        Err("failed to parse".to_owned())
    }
}

fn str_max_len(val: &str) -> Result<(), String> {
    if val.len() <= 100 {
        Ok(())
    } else {
        Err(String::from("can't be longer than 100 characters"))
    }
}

fn type_num(val: &str) -> Result<(), String> {
    match val.to_string().parse::<u32>() {
        Ok(val) => {
            if val <= 12 {
                Ok(())
            } else {
                Err("choose one of the provided types. You can get a list with --help".to_owned())
            }
        }
        Err(_) => Err("has to be a valid number".to_owned()),
    }
}

fn timedate(val: &str) -> Result<(), String> {
    match NaiveDateTime::parse_from_str(val, TIME_PARSE_STRING) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
