use plotters::prelude::*;
use xensieve::Sieve;

fn plot_sieves(
        sieve_strings: Vec<String>,
        value_min: i32,
        value_max: i32,
        ) -> Result<(), Box<dyn std::error::Error>> {

    let fig_w: u32 = 700;
    let fig_h: u32 = 480;
    let fig_y_label_size = 50;
    let count_col: i32 = sieve_strings.len().try_into()?;

    let root = SVGBackend::new("test-plot.svg", (fig_w, fig_h)).into_drawing_area();
    root.fill(&WHITE)?;

    // .caption("Sieve", ("sans-serif", 30))
    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(fig_y_label_size)
        .build_cartesian_2d(0..count_col, value_min-1..value_max+1)?;

    // make x labels transparent
    chart.configure_mesh()
        .x_label_style(("sans-serif", 20).into_font().color(&RGBAColor(0, 0, 0, 0.0)))
        .draw()?;

    // let column_data = vec![
    //     (0, vec![1, 7, 13], "4@2"), // Column at x=2 with stacks
    //     (1, vec![2, 3, 8, 20], "4@3"), // Column at x=5 with stacks
    //     // Add more columns as needed
    // ];

    let line_color = &RGBColor(30, 120, 120).mix(0.5);
    let line_thickness = 12; // Adjust the thickness of the line
    let col_width = (fig_w as i32 - fig_y_label_size) / count_col;

    // for (x, lines, label) in column_data {
    for (x, label) in sieve_strings.iter().enumerate() {
        let lines: Vec<i128> = Sieve::new(&label).iter_value(
                value_min as i128..value_max as i128).collect();

        for &y in &lines {
            // Draw a horizontal line at each specified y-value
            chart.draw_series(LineSeries::new(
                vec![(x as i32, y as i32), (x as i32 + 1, y as i32)],
                Into::<ShapeStyle>::into(line_color.stroke_width(line_thickness)),
            ))?;
        }

        // do not try to center label; instead, just shift from left bound by a margin
        let label_position = (
                fig_y_label_size + (col_width * x as i32) + 5,
                fig_h as i32 - 20,
                ); // Example calculation for label position

        root.draw_text(
            label,
            &("sans-serif", 16).into_font().color(&BLACK),
            label_position,
        )?;

    }
    Ok(())
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sieve_str = vec![
            "4@2".to_string(),
            "5@0".to_string(),
            "4@2|5@0".to_string(),
            "!30@10".to_string(),
            "(4@2|5@0) & !30@10".to_string(),
            ];
    plot_sieves(sieve_str, -10, 30)
}
