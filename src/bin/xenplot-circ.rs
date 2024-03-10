use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the path to the output SVG
    let path = "test-plot-circ.svg";
    // Create a drawing area using the SVG backend
    let root = SVGBackend::new(path, (640, 480)).into_drawing_area();

    // Optionally, you can clear the background if needed
    root.fill(&WHITE)?;

    // Draw circles with various colors and sizes
    // Example: Draw a red circle
    root.draw(&Circle::new((100, 100), 50, ShapeStyle::from(&RED).filled()))?;

    // Example: Draw a blue circle
    root.draw(&Circle::new((300, 200), 100, ShapeStyle::from(&BLUE).filled()))?;

    // Example: Draw a green circle
    root.draw(&Circle::new((500, 350), 75, ShapeStyle::from(&GREEN).filled()))?;

    // It's not necessary to call `present` for SVGBackend, as the output is directly written to the file.

    Ok(())
}