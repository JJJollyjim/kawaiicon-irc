# ✨ Kawaiicon ICalendar schedule ✨

ohai ／(･ ω ･)＼

This thing scrapes https://kawaiicon.org/schedule and generates an .ics file for your favorite calendaring software! ／(^ ω ^)＼

The hosted version is at https://kwiius.com/kawaiicon.ics

## Architecture

It runs as an aws lambda function, and caches the generated calendar for 5 minutes (or when the lambda process is killed, whichever comes first).

## Android

Stock Android doesn't support subscribing to this link directly... ／(˃ᆺ˂)＼

Either subscribe in Google Calendar on the web (and set that calendar to sync to your phone), or sync it directly to your device with an app like https://f-droid.org/en/packages/at.bitfire.icsdroid/

----


