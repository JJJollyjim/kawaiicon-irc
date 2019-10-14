use chrono::{offset::Utc, DateTime, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Weekday};
use chrono_tz::Pacific::Auckland;
use lazy_static::lazy_static;
use scraper::{Html, Selector};

pub fn parse(s: &str) -> Vec<(DateTime<Utc>, DateTime<Utc>, String, Option<String>)> {
    lazy_static! {
        static ref SEL_EVENT: Selector = Selector::parse("li.event").unwrap();
        static ref SEL_TIME1: Selector = Selector::parse("p.sched-time>strong").unwrap();
        static ref SEL_TIME2: Selector = Selector::parse("p.sched-time>span").unwrap();
        static ref SEL_HEADER: Selector = Selector::parse("h3").unwrap();
        static ref SEL_SUBHEAD: Selector = Selector::parse("p:not(.sched-time)").unwrap();
    }

    let doc = Html::parse_document(s);

    let event_elems = doc.select(&SEL_EVENT);

    let mut events = Vec::new();

    for e in event_elems {
        let time1 = e
            .select(&SEL_TIME1)
            .next()
            .unwrap()
            .text()
            .collect::<Vec<_>>()
            .join("");

        let time2 = e
            .select(&SEL_TIME2)
            .next()
            .unwrap()
            .text()
            .collect::<Vec<_>>()
            .join("");
        let time2_parts: Vec<_> = time2.split_whitespace().collect();

        let start = NaiveTime::parse_from_str(time1.trim(), "%I:%M%P").unwrap();
        let end = NaiveTime::parse_from_str(time2_parts[1], "%I:%M%P").unwrap();

        let dow: Weekday = time2_parts[2].parse().unwrap();
        let date = NaiveDate::from_isoywd(2019, 42, dow);

        let header = e
            .select(&SEL_HEADER)
            .next()
            .unwrap()
            .text()
            .collect::<Vec<_>>()
            .join("");

        let subhead = e
            .select(&SEL_SUBHEAD)
            .next()
            .map(|e| e.text().collect::<Vec<_>>().join(""));

        events.push((
            Auckland
                .from_local_datetime(&NaiveDateTime::new(date, start))
                .earliest()
                .unwrap()
                .with_timezone(&Utc),
            Auckland
                .from_local_datetime(&NaiveDateTime::new(date, end))
                .earliest()
                .unwrap()
                .with_timezone(&Utc),
            header,
            subhead,
        ));
    }

    // TODO add href in description
    // TODO don't crash on one bad event

    events
}

#[cfg(test)]
mod tests {
    #[test]
    fn kawaiicon_monday_3_30pm() {
        super::parse(include_str!("../sample.html"));
    }
}
