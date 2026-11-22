//! The owner's logo as the canvas paints it: decoded from the project's PNG
//! file once, held as a texture under the picture's digest, and drawn where
//! the title block seats it.

use egui::{Color32, ColorImage, Context, Id, TextureHandle, TextureOptions};
use rspice_design_model::ContentDigest;
use rspice_design_model::design_management::DrawingSheetOwnerLogo;

/// The longest side the canvas keeps a logo's texture at. A title block's
/// owner cell is a few centimetres across, so this is finer than a screen
/// shows it at any zoom that frames the block; a larger picture is reduced.
const TEXTURE_SIDE_PX: usize = 2_048;

/// The most memory a logo's pixels may take while they are decoded: a
/// picture larger than this cannot be drawn, and is refused when it is
/// imported.
const DECODED_BYTES: usize = 256 * 1_048_576;

/// The pixels the canvas draws a logo's PNG file with, reduced to fit
/// [`TEXTURE_SIDE_PX`]; why not, said the way a person importing it reads it.
pub fn decode_owner_logo(png_bytes: &[u8]) -> Result<ColorImage, &'static str> {
    const UNREADABLE: &str = "The logo's PNG file cannot be read.";
    let mut decoder = png::Decoder::new(std::io::Cursor::new(png_bytes));
    decoder.set_transformations(png::Transformations::EXPAND | png::Transformations::STRIP_16);
    let mut reader = decoder.read_info().map_err(|_| UNREADABLE)?;
    let size = reader
        .output_buffer_size()
        .filter(|&size| size <= DECODED_BYTES)
        .ok_or("The logo is too large a picture to draw.")?;
    let mut buffer = vec![0; size];
    let frame = reader.next_frame(&mut buffer).map_err(|_| UNREADABLE)?;
    let pixels = &buffer[..frame.buffer_size()];
    let rgba: Vec<u8> = match frame.color_type {
        png::ColorType::Rgba => pixels.to_vec(),
        png::ColorType::Rgb => pixels
            .chunks_exact(3)
            .flat_map(|rgb| [rgb[0], rgb[1], rgb[2], u8::MAX])
            .collect(),
        png::ColorType::GrayscaleAlpha => pixels
            .chunks_exact(2)
            .flat_map(|ga| [ga[0], ga[0], ga[0], ga[1]])
            .collect(),
        png::ColorType::Grayscale => pixels
            .iter()
            .flat_map(|&gray| [gray, gray, gray, u8::MAX])
            .collect(),
        // Expanded to one of the above by the decoder.
        png::ColorType::Indexed => return Err(UNREADABLE),
    };
    let size = [frame.width as usize, frame.height as usize];
    if rgba.len() != size[0] * size[1] * 4 {
        return Err(UNREADABLE);
    }
    Ok(reduced(ColorImage::from_rgba_unmultiplied(size, &rgba)))
}

/// `image`, reduced by the least whole factor that brings its longer side
/// within [`TEXTURE_SIDE_PX`], each pixel the mean of the block it stands
/// for.
fn reduced(image: ColorImage) -> ColorImage {
    let [width, height] = image.size;
    let factor = width.max(height).div_ceil(TEXTURE_SIDE_PX);
    if factor <= 1 {
        return image;
    }
    let size = [width.div_ceil(factor), height.div_ceil(factor)];
    let mut pixels = Vec::with_capacity(size[0] * size[1]);
    for row in 0..size[1] {
        for column in 0..size[0] {
            let mut sum = [0_u32; 4];
            let mut count = 0_u32;
            for y in row * factor..((row + 1) * factor).min(height) {
                for x in column * factor..((column + 1) * factor).min(width) {
                    // Premultiplied, so a transparent pixel adds no colour.
                    let pixel = image.pixels[y * width + x];
                    for (total, channel) in sum.iter_mut().zip(pixel.to_array()) {
                        *total += u32::from(channel);
                    }
                    count += 1;
                }
            }
            let [r, g, b, a] = sum.map(|total| (total / count) as u8);
            pixels.push(Color32::from_rgba_premultiplied(r, g, b, a));
        }
    }
    ColorImage::new(size, pixels)
}

/// The texture the canvas paints `logo` with: made the first time a frame
/// paints it and kept under its digest, so a frame neither decodes nor
/// uploads it again. Importing another logo replaces it, and the last one's
/// texture is let go. `None` for a picture the canvas cannot decode.
pub fn owner_logo_texture(ctx: &Context, logo: &DrawingSheetOwnerLogo) -> Option<TextureHandle> {
    type Held = (ContentDigest, Option<TextureHandle>);
    let id = Id::new("rspice.drawing_sheet.owner_logo");
    if let Some((digest, texture)) = ctx.data(|data| data.get_temp::<Held>(id))
        && digest == logo.digest()
    {
        return texture;
    }
    let texture = decode_owner_logo(logo.png()).ok().map(|image| {
        ctx.load_texture(
            format!("rspice.owner_logo.{}", logo.digest()),
            image,
            TextureOptions::LINEAR,
        )
    });
    ctx.data_mut(|data| data.insert_temp::<Held>(id, (logo.digest(), texture.clone())));
    texture
}
