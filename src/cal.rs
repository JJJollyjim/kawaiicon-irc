use chrono::{offset::Utc, DateTime};
use ics::{
    properties::{DtEnd, DtStart, Location, Summary},
    Event, ICalendar,
};
use uuid::Uuid;

pub fn make_cal(
    events: impl Iterator<Item = (DateTime<Utc>, DateTime<Utc>, String, Option<String>)>,
) -> Vec<u8> {
    // The ICalendar object is what is later written to the file.
    let mut calendar = ICalendar::new("2.0", "kawaiicon-ics-hax");

    calendar.push(ics::components::Property::new(
        "X-WR-CALNAME",
        "Kawaiicon Schedule ฅ(•ㅅ•❀)ฅ",
    ));
    calendar.push(ics::components::Property::new(
        "X-WR-CALDESC",
        "Kawaiicon Schedule ฅ(•ㅅ•❀)ฅ",
    ));

    let namespace = Uuid::from_bytes([
        0xA5, 0x33, 0x44, 0x62, 0xAA, 0x2D, 0x44, 0xE4, 0x82, 0xB8, 0x0C, 0x93, 0xC9, 0x9E, 0xC9,
        0x26,
    ]);

    let stamp = Utc::now();

    for (start, end, head, subhead) in events {
        let mut hashed = start.to_string().into_bytes();
        hashed.extend_from_slice(head.as_bytes());

        let mut ev = Event::new(
            Uuid::new_v5(&namespace, &hashed).to_string(),
            fmt_date(&stamp),
        );
        ev.push(Summary::new(head));
        if let Some(subhead) = subhead {
            ev.push(Location::new(subhead));
        }
        ev.push(DtStart::new(fmt_date(&start)));
        ev.push(DtEnd::new(fmt_date(&end)));

        // TODO link here
        // ev.push(Description::new());

        calendar.add_event(ev);
    }

    let mut res = Vec::new();
    calendar.write(&mut res).unwrap();
    res
}

fn fmt_date(d: &DateTime<Utc>) -> String {
    d.format("%Y%m%dT%H%M%SZ").to_string()
}
