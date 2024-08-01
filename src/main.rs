use arguments::CommandLineArguments;
use clap::Parser;
use input::BoundedPointSet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parses command line arguments.
    let cl_args = CommandLineArguments::parse();
    let input_file = cl_args.point_set_file();
    let output_dir = cl_args.output_directory();

    let point_set_input: BoundedPointSet =
        serde_json::from_reader(std::fs::File::open(input_file)?)?;

    let voronoi = todo!();
    let output_voronoi_path = output_dir.join("geo_bound_voronoi.json");

    serde_json::to_writer(std::fs::File::create(output_voronoi_path)?, &voronoi)?;
    Ok(())
}

mod arguments;
mod input;
mod voronoi;
