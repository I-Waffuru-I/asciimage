use image::{Luma,imageops};

fn main() {
    let args : Vec<String> = std::env::args().collect();
    let file_path =args[1].clone();
    let result_width : u32 = args[2].parse().expect("Invalid width");
    let result_height : u32 = args[3].parse().expect("Invalid height");
    dbg!(result_width);
    dbg!(result_height);
    
    let st_len = file_path.len();
    if st_len < 4 {
        println!("File name is too short!");
        return
    }
    if file_path.find(".png").is_none() {
        println!("File isn't png!");
        return
    }

    let img = if let Ok(i) = image::open(file_path) { i } else { return };
    let img_gs = if let Some(i) = img.grayscale().as_mut_luma8() { i.clone() } else {
        println!("Couldn't convert image to luma8");
        return 
    };
    let img_dims = img_gs.dimensions();

    
    let term_dims = term_size::dimensions().expect("Couldn't get the terminal dimensions!");
    let term_dims : (u32,u32) = (term_dims.0.try_into().unwrap(), term_dims.1.try_into().unwrap());
    let step_w = (img_dims.0 / term_dims.0) + 1; // how many pixels to group horizontally
    let step_h = (img_dims.1 / term_dims.1) + 1; // how many pixels to group vertivally
    let term_pix_total = (term_dims.0 * term_dims.1) as usize;
    let pix_total = (result_height * result_width) as usize;




    let mut chunked_pixels : Vec<u8> = vec![0; pix_total]; 
    let mut chunked_counters : Vec<usize> = vec![0;pix_total];

    // dbg!(&chunked_pixels);
    for (i, pix) in img_gs.pixels().enumerate() {
        let pos   = i / (step_w * step_h) as usize;
        let current_lum = if let Some(l) = chunked_pixels.get(pos) {l} else {&0};
        let count = if let Some(s) =  chunked_counters.get(pos) {s} else {&0};
        let uc = *count as u8; 
        
        let lum = get_lum_val(uc, current_lum, pix);

        // println!("i {i}  pos {pos}, lum {lum}");
        chunked_counters[pos] = count+1;
        chunked_pixels[pos] = lum;
    }

    let mut final_img_str = String::new();
    let resized = imageops::resize(&img_gs, result_width, result_height, imageops::Triangle);
    // chunked pixels now contains all pixels to display
    // for (i,p) in chunked_pixels.iter().enumerate() {

    // this solves basically everything I was trying to do above
    for (i, p) in resized.pixels().enumerate() {

        let ch = get_char_from_lum(&p.0[0]);
        final_img_str.push(ch);
        if i % result_width as usize == 0 {
            final_img_str.push('\n');
        }
    };
    println!("{final_img_str}");
    println!("term size : {:?}", term_dims);
    println!("img size : {:?}", img_dims);
    println!("step w {step_w}, step h {step_h},");
    println!("pix total {}",&pix_total);
    println!("term pix total {}",term_pix_total);


}


fn get_lum_val(count : u8, current_lum : &u8 ,pix : &Luma<u8>) -> u8 {
    let new_count = (count+1) as u32;
    let curr_32 = current_lum.clone() as u32;
    let old_lum = curr_32 * count as u32;
    let new_lum = old_lum + pix.0[0] as u32;
    let result = new_lum / new_count;
    result as u8
}

fn get_char_from_lum(pix :&u8) -> char {

    match pix / 32 {
        0 => '.',
        1 => ':',
        2 => 'º',
        3 => 'o',
        4 => '¤',
        5 => '0',
        6 => 'Ø',
        7 => '&',
        8 => '#',
        _ => {
            panic!("ERROR pix with value {pix} couldn't be parsed")
        }
    }
}