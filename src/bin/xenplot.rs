use plotters::prelude::*;


// fn main() -> Result<(), Box<dyn std::error::Error>> {

//     let root = BitMapBackend::new("test-plot.png", (640, 480)).into_drawing_area();
//     root.fill(&WHITE)?;
//     let mut chart = ChartBuilder::on(&root)
//         .caption("y=x^2", ("sans-serif", 20).into_font())
//         .margin(5)
//         .x_label_area_size(30)
//         .y_label_area_size(40)
//         .build_cartesian_2d(-1f32..1f32, -1.2f32..1f32)?;

//     chart.configure_mesh().draw()?;

//     chart
//         .draw_series(LineSeries::new(
//             (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x * x)),
//             &RED,
//         ))?
//         .label("y = x^2")
//         .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

//     chart
//         .configure_series_labels()
//         .background_style(&WHITE.mix(0.8))
//         .border_style(&BLACK)
//         .draw()?;

//     root.present()?;

//     Ok(())
// }


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Specify the file name and dimensions for the SVG output
    let fig_w: u32 = 700;
    let fig_h: u32 = 480;
    let fig_y_label_size = 50;
    let count_col = 3;

    let root = SVGBackend::new("test-plot.svg", (fig_w, fig_h)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Sieve", ("sans-serif", 30))
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(fig_y_label_size)
        .build_cartesian_2d(0..count_col, 0..20)?;

    chart.configure_mesh()
        .x_label_style(("sans-serif", 20).into_font().color(&RGBAColor(0, 0, 0, 0.0)))
        .draw()?;

    let column_data = vec![
        (0, vec![1, 2, 3], "4@2"), // Column at x=2 with stacks
        (1, vec![2, 3, 8, 20], "4@3"), // Column at x=5 with stacks
        // Add more columns as needed
    ];

    let line_color = &RGBColor(30, 120, 120).mix(0.5);
    let line_thickness = 12; // Adjust the thickness of the line
    let col_width = (fig_w as i32 - fig_y_label_size) / count_col;

    for (x, lines, label) in column_data {
        for &y in &lines {
            // Draw a horizontal line at each specified y-value
            chart.draw_series(LineSeries::new(
                vec![(x, y as i32), (x + 1, y as i32)],
                Into::<ShapeStyle>::into(line_color.stroke_width(line_thickness)),
            ))?;
        }

        let label_position = (
                fig_y_label_size + (col_width * x + col_width / 2),
                fig_h as i32 - 20,
                ); // Example calculation for label position

        root.draw_text(
            label,
            &("sans-serif", 16).into_font().color(&BLACK),
            label_position,
        )?;

    }


    // for (x, stacks) in column_data {
    //     let mut y_start = 0;
    //     for stack_height in stacks {
    //         for y in y_start..y_start + stack_height {
    //             // Draw a horizontal line for each unit of stack height
    //             chart.draw_series(LineSeries::new(
    //                 vec![(x, y as i32), (x + 1, y as i32)],
    //                 Into::<ShapeStyle>::into(line_color.stroke_width(line_thickness)),
    //             ))?;
    //         }
    //         y_start += stack_height;

    //         // chart
    //         //     .draw_series(std::iter::once(Rectangle::new(
    //         //         [(x, y_start), (x + 1, y_start + stack_height)],
    //         //         Into::<ShapeStyle>::into(&BLUE).filled(),
    //         //     )))?
    //         //     .label(format!("{}", stack_height))
    //         //     .legend(|(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], &BLUE));
    //         // y_start += stack_height;
    //     }
    // }

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}
