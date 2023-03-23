fn main() {
    let band = "EC";
    let band_array = [
        "", "N1", "N2", "M1", "M2", "M3", "M4", "M5", "M6", "M7", "EC",
    ];

    let band_index = band_array.iter().position(|&x| x == band).unwrap();
    println!("Index of band {} is {}", &band, band_index);

    let appl_for = vec!["N1 to EC", "M4 to EC", "N1 to M2", "N1 to N2", "N1 to M4"];

    for band_range in appl_for.iter() {
        let (lower_bound, upper_bound) = get_bounds(band_range, band_array);

        //     let bounds: Vec<&str> = band_range.split(' ').collect();
        //     let lower_bound = bounds[0];
        //     let lower_bound_index = band_array.iter().position(|&x| x == lower_bound).unwrap();

        //     let upper_bound = bounds[2];
        //     let upper_bound_index = band_array.iter().position(|&x| x == upper_bound).unwrap();

        if lower_bound <= band_index && band_index <= upper_bound {
            println!("Band {} belongs to {}", &band, &band_range);
        } else {
            println!("Band {} DOES NOT belong to {}", &band, &band_range);
        }
    }
}

fn get_bounds(band_range: &&str, band_array: [&str; 11]) -> (usize, usize) {
    let bounds: Vec<&str> = band_range.split(' ').collect();

    let lower_bound = bounds[0];
    let lower_bound_index = band_array.iter().position(|&x| x == lower_bound).unwrap();

    let upper_bound = bounds[2];
    let upper_bound_index = band_array.iter().position(|&x| x == upper_bound).unwrap();

    (lower_bound_index, upper_bound_index)
}
