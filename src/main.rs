use std::io::prelude::*;
use std::net::TcpStream;
use std::str::from_utf8;

const MY_CALLSIGN: &str = "KM6***";
const MY_SSID: &str = "10";
const APRS_IS_URL: &str = "noam.aprs2.net:14580";
const APRS_IS_PASSWORD: &str = "*****";
const VIA_CALLSIGN: &str = "APRS";
const SMS_GATE_PREFIX: &str = ":SMSGTE :@";
const DEST_PHONE_NUMBER: &str = "310*******";
const MESSAGE: &str = "This message has been sent via APRS SMS Gate";

fn main() -> std::io::Result<()> {
    let mut buf = [0; 128];

    // 1. Connect to APRS server
    let mut stream = TcpStream::connect(APRS_IS_URL)?;
    stream.read(&mut buf)?; // Expected msg: "# aprsc 2.1.8-gf8824e8"
    println!("{}", from_utf8(&buf).unwrap());

    // 2. Login to APRS server
    stream.write(format!("user {} pass {}\n", MY_CALLSIGN, APRS_IS_PASSWORD).as_bytes())?;
    stream.read(&mut buf)?; // Expected ack: "# logresp MY_CALLSIGN verified, server T2SJC"
    println!("{}", from_utf8(&buf).unwrap());

    // 3. Send a message
    let s = format!(
        "{}-{}>{}:{}{} {}\n",
        MY_CALLSIGN, MY_SSID, VIA_CALLSIGN, SMS_GATE_PREFIX, DEST_PHONE_NUMBER, MESSAGE
    );
    stream.write(s.as_bytes())?;
    println!("{}", &s);

    Ok(())
}
