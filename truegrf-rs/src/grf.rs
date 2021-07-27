use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
struct NewGRFIndustry {
    id: i32,
    available: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct NewGRFOptions {
    industries: Vec<NewGRFIndustry>,
}


fn write_pseudo_sprite(data: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut output = Vec::new();

    let len = data.len();
    output.extend(&[
        ((len >>  0) & 0xff) as u8,
        ((len >>  8) & 0xff) as u8,
        ((len >> 16) & 0xff) as u8,
        ((len >> 24) & 0xff) as u8,
    ]);

    output.extend(&[0xff]);
    output.extend(data);

    Ok(output)
}

pub fn write_grf(options: NewGRFOptions) -> std::io::Result<Vec<u8>> {
    let mut output = Vec::new();

    /* Write GRF container version 2 header. */
    output.extend(b"\x00\x00GRF\x82\r\n\x1a\n");
    /* TODO -- Sprite section offset. */
    output.extend(b"\x00\x00\x00\x00");
    /* Compression. OpenTTD currently only support no-compression (= 0). */
    output.extend(b"\x00");

    /* TODO -- Amount of sprites in the file; ignored by OpenTTD. */
    output.extend(write_pseudo_sprite(b"\x02\x00\x00\x00")?);

    /* Action8 - GRF metadata */
    output.extend(write_pseudo_sprite(b"\x08\x08TRU1TrueGRF Test\x00TrueGRF Test Description\x00")?);

    if !options.industries[0].available {
        output.extend(write_pseudo_sprite(b"\x00\x0a\x01\x01\x00\x08\xff")?);
    }
    if !options.industries[1].available {
        output.extend(write_pseudo_sprite(b"\x00\x0a\x01\x01\x01\x08\xff")?);
    }

    /* Final sprite marker. */
    output.extend(b"\x00\x00\x00\x00");

    Ok(output)
}
