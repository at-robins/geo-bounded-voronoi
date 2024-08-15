use arguments::CommandLineArguments;
use clap::Parser;
use input::BoundedPointSet;
use voronoi::compute_voronoi;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parses command line arguments.
    let cl_args = CommandLineArguments::parse();
    let input_file = cl_args.point_set_file();
    let output_path = cl_args.output_path();

    // Parses the input file.
    let point_set_input: BoundedPointSet =
        serde_json::from_reader(std::fs::File::open(input_file)?)?;

    // Creats the Voronoi representation and saves it to the output file.
    let voronoi = compute_voronoi(point_set_input)?;
    serde_json::to_writer(std::fs::File::create(output_path)?, &voronoi)?;

    Ok(())
}

mod arguments;
mod input;
mod voronoi;
