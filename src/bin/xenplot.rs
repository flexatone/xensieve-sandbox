use plotters::prelude::*;
use xensieve::Sieve;


fn plot_sieves(
    file_name: &str,
    sieve_strings: Vec<String>,
    range: (i32, i32), // value range used to plot sieve
    band_height: u32,
    fig_width: u32,
    ) -> Result<(), Box<dyn std::error::Error>> {

    let count_row: i32 = sieve_strings.len().try_into()?;
    let fig_x_label_size = 20;

    // do I need to include margin in this?
    let fig_h: u32 = fig_x_label_size as u32 + band_height * count_row as u32;
    let fig_y_label_size = 180; // left space for y labels
    let fig_margin: u32 = 10;

    let line_colors = vec![
        RGBColor(90, 90, 90),
        RGBColor(140, 140, 140),
    ];
    let line_thickness = 5; // Adjust the thickness of the line

    let root = SVGBackend::new(file_name, (fig_width, fig_h)).into_drawing_area();
    root.fill(&RGBColor(220, 220, 230))?;
    // .caption("Sieve", ("sans-serif", 30));

    let mut chart = ChartBuilder::on(&root)
        .margin(fig_margin)
        .x_label_area_size(fig_x_label_size)
        .y_label_area_size(fig_y_label_size)
        .build_cartesian_2d(range.0-1..range.1+1, 0..count_row)?;

    // make x labels transparent
    chart.configure_mesh()
        .y_label_style(("sans-serif", 20).into_font().color(&RGBAColor(0, 0, 0, 0.0)))
        .draw()?;

    // subtract bottom labels from height, then divide by rows
    let band_height = (fig_h as i32 - fig_x_label_size) / count_row;

    for (y, label) in sieve_strings.iter().enumerate() {
        let lines: Vec<i128> = Sieve::new(&label).iter_value(
                range.0 as i128..range.1 as i128).collect();

        // the charting positions go from bottom to top, so invert and shift
        let y_pos: i32 = count_row - y as i32 - 1;
        for &x in &lines {
            // Draw a horizontal line at each specified y-value
            chart.draw_series(LineSeries::new(
                vec![(x as i32, y_pos), (x as i32, y_pos + 1)],
                Into::<ShapeStyle>::into(line_colors[y % 2].stroke_width(line_thickness)),
            ))?;
        }
        // do not try to center label; instead, just shift from left bound by a margin
        let label_position = (
            fig_margin as i32, // horizontal position
            20 + ((band_height - 4) * y as i32),
            );

        root.draw_text(
            label,
            &("sans-serif", 18).into_font().color(&BLACK),
            label_position,
        )?;
    }
    Ok(())
}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    // this show shows the example of using a union group first, then selectively removing lines by intersecting with a complement group
    plot_sieves(
        "test-plot-a.svg",
        vec![
            "4@2".to_string(),
            "5@0".to_string(),
            "4@2|5@0".to_string(),
            "!30@10".to_string(),
            "(4@2|5@0) & !30@10".to_string(),
            ],
        (0, 50),
        25, // bar height
        660, // fig width
    )?;

    // plot_sieves(
    //     "test-plot-b.svg",
    //     vec![
    //         "10@2|10@3".to_string(),
    //         "5@0".to_string(),
    //         "!(5@0|5@2)".to_string(),
    //         "15@7".to_string(),
    //         ],
    //     (-30, 30),
    //     36,
    // )?;

    Ok(())
}
