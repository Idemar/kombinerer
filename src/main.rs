mod args;

use args::Args;
use image::{
    imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};

fn main() -> Result<(), BildeDataError> {
    let args = Args::new();
    println!("{:?}", args);

    let (bilde_1, bilde_1_format) = finn_bilde_fra_mappe(args.bilde_1);
    let (bilde_2, bilde_2_format) = finn_bilde_fra_mappe(args.bilde_2);

    if bilde_1_format != bilde_2_format {
        return Err(BildeDataError::DiffBildeFormat);
    }

    let (bilde_1, bilde_2) = standarisert_størrelse(bilde_1, bilde_2);
    let mut output = FlytendeBilde::new(bilde_1.width(), bilde_1.height(), args.output);

    let kombinere_data = kombiner_bilder(bilde_1, bilde_2);

    output.sett_data(kombinere_data)?;

    image::save_buffer_with_format(
        output.name,
        &output.data,
        output.width,
        output.height,
        image::ColorType::Rgb8,
        bilde_1_format,
    )
    .unwrap();
    Ok(())
}

#[derive(Debug)]
enum BildeDataError {
    BufferForLiten,
    DiffBildeFormat,
}

struct FlytendeBilde {
    width: u32,
    height: u32,
    data: Vec<u8>,
    name: String,
}

impl FlytendeBilde {
    fn new(width: u32, height: u32, name: String) -> Self {
        let buffer_kapasitet = 3_655_744;
        let buffer: Vec<u8> = Vec::with_capacity(buffer_kapasitet);
        FlytendeBilde {
            width,
            height,
            data: buffer,
            name,
        }
    }
    fn sett_data(&mut self, data: Vec<u8>) -> Result<(), BildeDataError> {
        if data.len() > self.data.capacity() {
            return Err(BildeDataError::BufferForLiten);
        }
        self.data = data;
        Ok(())
    }
}

fn finn_bilde_fra_mappe(path: String) -> (DynamicImage, ImageFormat) {
    let bilde_leser = Reader::open(path).unwrap();
    let bilde_format = bilde_leser.format().unwrap();
    let bilde = bilde_leser.decode().unwrap();
    (bilde, bilde_format)
}

fn standarisert_størrelse(
    bilde_1: DynamicImage,
    bilde_2: DynamicImage,
) -> (DynamicImage, DynamicImage) {
    let (height, width) = få_minste_dimensjoner(bilde_1.dimensions(), bilde_2.dimensions());
    println!("Bredde: {}, Høyde: {}\n", width, height);
    if bilde_2.dimensions() == (width, height) {
        (bilde_1.resize_exact(width, height, Triangle), bilde_2)
    } else {
        (bilde_1, bilde_2.resize_exact(width, height, Triangle))
    }
}

fn få_minste_dimensjoner(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
    let pix_1 = dim_1.0 * dim_1.1;
    let pix_2 = dim_2.0 * dim_2.1;
    return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

fn kombiner_bilder(bilde_1: DynamicImage, bilde_2: DynamicImage) -> Vec<u8> {
    let vec_1 = bilde_1.to_rgb8().into_vec();
    let vec_2 = bilde_2.to_rgb8().into_vec();

    alternativ_pixels(vec_1, vec_2)
}

fn alternativ_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
    let mut kombinere_data = vec![0u8; vec_1.len()];

    let mut i = 0;
    while i < vec_1.len() {
        if i % 8 == 0 {
            kombinere_data.splice(i..=i + 3, sett_rgba(&vec_1, i, i + 3));
        } else {
            kombinere_data.splice(i..=i + 3, sett_rgba(&vec_2, i, i + 3));
        }
        i += 4;
    }

    kombinere_data
}

fn sett_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
    let mut rgba = Vec::new();
    for i in start..=end {
        let verdi = match vec.get(i) {
            Some(d) => *d,
            None => panic!("Indeks utenfor grensene"),
        };
        rgba.push(verdi);
    }
    rgba
}
