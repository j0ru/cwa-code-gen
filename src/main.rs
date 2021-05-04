use image::{ImageBuffer, Luma};
use prost::Message;
use qrcode::QrCode;
use rand::rngs::StdRng;
use rand::RngCore;
use rand::SeedableRng;

mod cli;

// protobuf from https://github.com/corona-warn-app/cwa-app-android/blob/main/Server-Protocol-Buffer/src/main/proto/internal/pt/trace_location.proto
pub mod items {
    include!(concat!(
        env!("OUT_DIR"),
        "/de.rki.coronawarnapp.server.protocols.internal.pt.rs"
    ));
}

fn main() {
    let app = cli::get_app();
    let matches = app.get_matches();

    let description = matches.value_of("description").unwrap();
    let address = matches.value_of("address").unwrap();
    let prefix = matches.value_of("prefix").unwrap();
    let default_check_in_length_in_minutes = matches
        .value_of("default_check_in_length_in_minutes")
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let r#type = matches.value_of("type").unwrap().parse::<i32>().unwrap();

    let start_time = if let Some(time_string) = matches.value_of("start-time") {
        Some(
            chrono::NaiveDateTime::parse_from_str(time_string, cli::TIME_PARSE_STRING)
                .unwrap()
                .timestamp(),
        )
    } else {
        None
    };

    let end_time = if let Some(time_string) = matches.value_of("end-time") {
        Some(
            chrono::NaiveDateTime::parse_from_str(time_string, cli::TIME_PARSE_STRING)
                .unwrap()
                .timestamp(),
        )
    } else {
        None
    };

    // public cert from https://github.com/corona-warn-app/cwa-app-android/blob/main/prod_environments.json
    let cn_public = include_bytes!("public_cert");

    let tl = items::TraceLocation {
        version: 1,
        description: description.to_owned(),
        address: address.to_owned(),
        start_timestamp: start_time.unwrap_or(0) as u64,
        end_timestamp: end_time.unwrap_or(0) as u64,
    };

    // generate a secure random id
    let mut rng = StdRng::from_entropy();
    let mut rng_buf: [u8; 16] = [0; 16];
    rng.fill_bytes(&mut rng_buf);

    let cwa_nd = items::CrowdNotifierData {
        version: 1,
        public_key: cn_public.to_vec(),
        cryptographic_seed: rng_buf.to_vec(),
    };

    let cwa_ld = items::CwaLocationData {
        default_check_in_length_in_minutes,
        version: 1,
        r#type,
    };

    let mut cwa_ld_buf = vec![];
    cwa_ld_buf.reserve(cwa_ld.encoded_len());
    cwa_ld.encode(&mut cwa_ld_buf).unwrap();

    let payload = items::QrCodePayload {
        version: 1,
        location_data: Some(tl),
        crowd_notifier_data: Some(cwa_nd),
        vendor_data: cwa_ld_buf,
    };

    let mut buf = vec![];
    buf.reserve(payload.encoded_len());
    payload.encode(&mut buf).unwrap();

    let mut link = prefix.to_owned();
    link.push_str(&base64_url::encode(&buf));

    let dimensions: Option<Vec<u32>> = if let Some(dim) = matches.value_of("dimensions") {
        Some(dim.split('x').map(|d| d.parse::<u32>().unwrap()).collect())
    } else {
        None
    };

    let code = QrCode::new(link).unwrap();
    if let Some(output_path) = matches.value_of("output") {
        let mut render = code.render::<Luma<u8>>();
        if let Some(dim) = dimensions {
            let width = dim.get(0).unwrap();
            let height = dim.get(1).unwrap();
            {
                let dim = if height > width { width } else { height };
                render.max_dimensions(*dim, *dim);
            }
            let qrcode = render.build();
            let mut image = ImageBuffer::from_pixel(*width, *height, Luma([255]));
            image::imageops::overlay(
                &mut image,
                &qrcode,
                (width - qrcode.width()) / 2,
                (height - qrcode.height()) / 2,
            );
            image.save(output_path).unwrap();
        } else {
            render.build().save(output_path).unwrap();
        }
    } else {
        let string = code.render().light_color(' ').dark_color('█').build();
        println!("{}", string);
    }
}
