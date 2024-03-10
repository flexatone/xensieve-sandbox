use plotters::prelude::*;
use xensieve::Sieve;

fn plot_sieves(
        file_name: &str,
        sieve_strings: Vec<String>,
        range: (i32, i32), // value range used to plot sieve
        col_width: u32,
        ) -> Result<(), Box<dyn std::error::Error>> {

    let count_col: i32 = sieve_strings.len().try_into()?;
    let fig_y_label_size = 50;
    let fig_w: u32 = fig_y_label_size as u32 + col_width * count_col as u32;
    let fig_h: u32 = 700;
    let fig_x_label_size = 180; // bottom space for vertical labels
    let line_color = &RGBColor(30, 30, 180).mix(0.6);
    let line_thickness = 5; // Adjust the thickness of the line

    let root = SVGBackend::new(file_name, (fig_w, fig_h)).into_drawing_area();
    // root.fill(&WHITE)?;
    root.fill(&RGBColor(240, 240, 240))?;

    // .caption("Sieve", ("sans-serif", 30))
    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .x_label_area_size(fig_x_label_size)
        .y_label_area_size(fig_y_label_size)
        .build_cartesian_2d(0..count_col, range.0-1..range.1+1)?;

    // make x labels transparent
    chart.configure_mesh()
        .x_label_style(("sans-serif", 20).into_font().color(&RGBAColor(0, 0, 0, 0.0)))
        .draw()?;

    let col_width = (fig_w as i32 - fig_y_label_size) / count_col;

    // for (x, lines, label) in column_data {
    for (x, label) in sieve_strings.iter().enumerate() {
        let lines: Vec<i128> = Sieve::new(&label).iter_value(
                range.0 as i128..range.1 as i128).collect();

        for &y in &lines {
            // Draw a horizontal line at each specified y-value
            chart.draw_series(LineSeries::new(
                vec![(x as i32, y as i32), (x as i32 + 1, y as i32)],
                Into::<ShapeStyle>::into(line_color.stroke_width(line_thickness)),
            ))?;
        }

        // do not try to center label; instead, just shift from left bound by a margin
        let label_position = (
                (fig_y_label_size + 30) + ((col_width - 4) * x as i32),
                fig_h as i32 - fig_x_label_size,
                ); // Example calculation for label position

        root.draw_text(
            label,
            &("sans-serif", 20).into_font().color(&BLACK).transform(FontTransform::Rotate90),
            label_position,
        )?;
    }
    Ok(())
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    plot_sieves(
        "test-plot-a.svg",
        vec![
            "4@2".to_string(),
            "5@0".to_string(),
            "4@2|5@0".to_string(),
            "!30@10".to_string(),
            "(4@2|5@0) & !30@10".to_string(),
            ],
        (-30, 30),
        60,
    )?;

    plot_sieves(
        "test-plot-b.svg",
        vec![
            "10@2|10@3".to_string(),
            "5@0".to_string(),
            "!(5@0|5@2)".to_string(),
            "15@7".to_string(),
            ],
        (-30, 30),
        60,
    )?;

    Ok(())
}
