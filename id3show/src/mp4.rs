use mp4ameta::Tag;
use std::error::Error;

/// Show the MP4 metadata
pub fn show_metadata(filename: &str, show_detail: bool) -> Result<(), Box<dyn Error>> {
    let tag = Tag::read_from_path(filename)?;

    log::trace!("Tag = {:?}", tag);
    for (data_ident, data) in tag.data() {
        match data {
            mp4ameta::Data::Reserved(res) => {
                if show_detail {
                    println!("  {} = {res:?} (Reserved)", data_ident);
                }
            }
            mp4ameta::Data::Utf8(d) => {
                println!("  {} = {d} (UTF-8)", data_ident);
            }
            mp4ameta::Data::Utf16(d) => {
                println!("  {} = {d} (UTF-16)", data_ident);
            }
            mp4ameta::Data::Jpeg(jpeg) => {
                if show_detail {
                    println!("  {} = {} bytes (JPEG)", data_ident, jpeg.len());
                }
            }
            mp4ameta::Data::Png(png) => {
                if show_detail {
                    println!("  {} = {} bytes (PNG)", data_ident, png.len());
                }
            }
            mp4ameta::Data::BeSigned(bes) => {
                if show_detail {
                    println!("  {} = {} bytes (Big-Endian Signed)", data_ident, bes.len());
                }
            }
            mp4ameta::Data::Bmp(bmp) => {
                if show_detail {
                    println!("  {} = {} bytes (BMP)", data_ident, bmp.len());
                }
            }
        }
    }

    // return safely
    Ok(())
}
