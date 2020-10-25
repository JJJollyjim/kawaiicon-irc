use chrono::{offset::Utc, DateTime, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Weekday};
use chrono_tz::Pacific::Auckland;
use lazy_static::lazy_static;
use scraper::{Html, Selector};

pub fn parse(s: &str) -> Vec<(DateTime<Utc>, DateTime<Utc>, String, Option<String>)> {
    lazy_static! {
        static ref SEL_H2_OR_UL: Selector = Selector::parse("main>h2, main>ul").unwrap();
        static ref SEL_LI: Selector = Selector::parse("li").unwrap();
    }

    let doc = Html::parse_document(s);

    let h2s_and_uls = doc.select(&SEL_H2_OR_UL);

    let mut events = Vec::new();

    let mut date: Option<NaiveDate> = None;

    for e in h2s_and_uls {
        if e.value().name() == "h2" {
            let dow: Option<Weekday> = e.text().collect::<Vec<_>>().join("").parse().ok();
            if let Some(dow) = dow {
                date = Some(NaiveDate::from_isoywd(2020, 44, dow));
            }
        } else {
            for event in e.select(&SEL_LI) {
                let text = event.text().collect::<Vec<_>>().join("");
                let spc_idx = text.find(' ').unwrap();

                let start_time = NaiveTime::parse_from_str(&text[0..spc_idx], "%H:%M").unwrap();
                let rest = &text[(spc_idx+1)..];

                let (title, subtitle) = if let Some(sep_idx) = rest.rfind(" - ") {
                    (&rest[0..sep_idx], Some(&rest[(sep_idx + 3)..]))
                } else {
                    (rest, None)
                };

                let dt = Auckland
                        .from_local_datetime(&NaiveDateTime::new(date.expect("date none in event"), start_time))
                        .earliest()
                        .unwrap()
                        .with_timezone(&Utc);

                events.push((
                    dt,
                    dt,
                    title.to_owned(),
                    subtitle.map(|s| s.to_owned()),
                ));
            }
        }
    }

    events
}

#[cfg(test)]
mod tests {
    #[test]
    fn chcon() {
        println!(
            "{:?}", super::parse(include_str!("../chcon.html"))
        );
    }
}
