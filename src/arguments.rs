use std::path::PathBuf;

use clap::{crate_authors, Parser};
use getset::{CopyGetters, Getters};

/// A tool for generating the Voronoi diagramm of a point set bound by an arbitrary geometry.
#[derive(Parser, CopyGetters, Getters, Debug)]
#[command(author = crate_authors!("\n"), version, about, long_about = None)]
pub struct CommandLineArguments {
    /// The path to the JSON file containing the point set and bounding geometry.
    ///
    /// The structure must be as follows:
    /// ```json
    /// {
    ///     "points": [[0.0, 1.0], [1.0, 1.0], ...],
    ///     "bound": [[-0.5, -0.5], [0.0, 0.0], [1.0, 0.5], [-0.5, -0.5]]
    /// }
    /// ```
    #[getset(get = "pub")]
    point_set_file: PathBuf,
    /// The output directory [default: the parent directory of the point set file]
    #[arg(short, long)]
    output_directory: Option<PathBuf>,
}

impl CommandLineArguments {
    /// Returns the output directory.
    /// If no directory has been specified the parent directory of the input file is returned.
    pub fn output_directory(&self) -> PathBuf {
        self.output_directory
            .as_ref()
            .map(|output_dir| output_dir.to_path_buf())
            .unwrap_or_else(|| self.point_set_file_parent_directory())
    }

    /// Returns the directory that contains the point set file.
    fn point_set_file_parent_directory(&self) -> PathBuf {
        self.point_set_file
            .parent()
            .map(|parent| parent.to_path_buf())
            .unwrap_or("/".into())
    }
}
