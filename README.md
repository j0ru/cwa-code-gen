# CWA QR CODE GENERATOR

Lately a CWA update intruduced an interesting new feature
that could help with environments where the usual detection method wouldn't work that well.
It works with QR codes that are prominently placed in your restaurant/office/whatever and are scanned by the user.
In case of an infection the user publishes the embedded id from the QR code and the other users can check against
the code to determine if they were exposed to any risk.
It is recommended to exchange the generated code every day but doing that from the app can get anoying
since the only option for export is to generate a code inside the app and export it as pdf in an A4 format.
This repo is a proof of concept for a program that you could run from a systemd timer or similar and expose the code
on a signage board or similiar without any interaction needed.

## This is not an official app and I can't make any guaranties regarding correctness and functionality. It is fully untested and was done purely out of interest!

```
CWA Code Generator 0.1.0
Folke 'joru' Gleumes <folke@gleumes.org>

USAGE:
    cwa-qr-generator [OPTIONS] --description <description> --address <address> --type <INTEGER>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --address <address>                                            Event address
        --default-checkin-time <default_check_in_length_in_minutes>    [default: 15]
        --description <description>                                    Event description
        --end-time <end-time>
            end time of the event in ISO 8601

    -o, --output <FILE>
            Output location for the Qr code. If not provided the qr code will be printed to the
            terminal

        --prefix <prefix>
            Url from your countries  [default: https://e.coronawarn.app?v=1#]

        --start-time <start-time>
            start time of the event in ISO 8601

        --type <INTEGER>
            LOCATION_TYPE_UNSPECIFIED = 0
            LOCATION_TYPE_PERMANENT_OTHER = 1
            LOCATION_TYPE_TEMPORARY_OTHER = 2
            LOCATION_TYPE_PERMANENT_RETAIL = 3
            LOCATION_TYPE_PERMANENT_FOOD_SERVICE = 4
            LOCATION_TYPE_PERMANENT_CRAFT = 5
            LOCATION_TYPE_PERMANENT_WORKPLACE = 6
            LOCATION_TYPE_PERMANENT_EDUCATIONAL_INSTITUTION = 7
            LOCATION_TYPE_PERMANENT_PUBLIC_BUILDING = 8
            LOCATION_TYPE_TEMPORARY_CULTURAL_EVENT = 9
            LOCATION_TYPE_TEMPORARY_CLUB_ACTIVITY = 10
            LOCATION_TYPE_TEMPORARY_PRIVATE_EVENT = 11
            LOCATION_TYPE_TEMPORARY_WORSHIP_SERVICE = 12
```
