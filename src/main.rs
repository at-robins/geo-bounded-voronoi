use arguments::CommandLineArguments;
use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parses command line arguments.
    let cl_args = CommandLineArguments::parse();
    let input_file = cl_args.point_set_file();
    let output_dir = cl_args.output_directory();

    let voronoi = todo!();

    let output_voronoi_path = output_dir.join("geo_bound_voronoi.json");

    serde_json::to_writer(std::fs::File::create(output_voronoi_path)?, &voronoi)?;
    Ok(())
}

mod arguments;
