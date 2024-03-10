use plotters::prelude::*;
use xensieve::Sieve;

// not sure how to pass backend into a function
// fn draw(
//     sieve_x: &str,
//     sieve_y: &str,
//     ) -> Result<(), Box<dyn std::error::Error>> {
//     root.draw(&Circle::new((100, 100), 50, ShapeStyle::from(&RED).filled()))?;
//     Ok(())
// }

#[derive(Clone, Debug)]
pub(crate) struct DrawSpec {
    sieve_x: String,
    sieve_y: String,
    sieve_r: String,
    color: RGBAColor,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the path to the output SVG
    let path = "test-plot-circ.svg";
    let range = (0, 50000);
    let area = (1000, 1000);

    let root = SVGBackend::new(path, area).into_drawing_area();

    root.fill(&RGBColor(40, 40, 40))?;

    let draw_specs = vec![
        DrawSpec{
            sieve_x: "(200@2|270@0) & !120@10".to_string(),
            sieve_y: "(80@2|94@76) & !250@80".to_string(),
            sieve_r: "(18@2|20@0) & !60@10".to_string(),
            color: RGBColor(40, 40, 160).mix(0.8),
        },
        DrawSpec{
            sieve_x: "81@6 | 81@4 | 83@44".to_string(),
            sieve_y: "19@2 | 21@17 | 23@5 | 25@18".to_string(),
            sieve_r: "(8@2|10@0) & !60@10".to_string(),
            color: RGBColor(120, 50, 250).mix(0.5),
        },
        DrawSpec{
            sieve_x: "(20@2|27@0) & !120@10".to_string(),
            sieve_y: "(80@2|94@76) & !250@80".to_string(),
            sieve_r: "(3@2|7@0|5@4) & !(12@2|15@4)".to_string(),
            color: RGBColor(200, 200, 200).mix(0.4),
        },

    ];


    for ds in draw_specs {
        let sieve_x = Sieve::new(&ds.sieve_x);
        let sieve_y = Sieve::new(&ds.sieve_y);
        let sieve_r = Sieve::new(&ds.sieve_r);
        let mut r_iter = sieve_r.iter_interval(0..);

        // NOTE: need to zip post collected values, as each sieve might generate different numbers of values for a given range
        for (x, y) in std::iter::zip(
            sieve_x.iter_value(range.0 as i128..range.1 as i128).collect::<Vec<_>>(),
            sieve_y.iter_value(range.0 as i128..range.1 as i128).collect::<Vec<_>>(),
        ) {
            // print!("\n{:?} {:?}", x, y);
            root.draw(&Circle::new(
                (x as i32 % area.0 as i32, y as i32 % area.1 as i32),
                r_iter.next().unwrap() as i32,
                ShapeStyle::from(&ds.color).filled())
                )?;
        };
    }
    Ok(())
}