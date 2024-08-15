use std::path::PathBuf;

use clap::{crate_authors, Parser};
use getset::{CopyGetters, Getters};

/// The default name of the output file.
const DEFAULT_OUTPUT_FILE_NAME: &str = "geo_bound_voronoi.json";

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
    /// The output path for the result JSON file [default: the output file is generated in the directory the point set file resides in]
    #[arg(short, long)]
    output_path: Option<PathBuf>,
}

impl CommandLineArguments {
    /// Returns the output path that has been specified.
    /// If none has been set the default output path is returned.
    pub fn output_path(&self) -> PathBuf {
        self.output_path
            .as_ref()
            .map(|output_file| output_file.to_path_buf())
            .unwrap_or_else(|| self.default_ouptut_path())
    }

    /// Returns the default output path.
    fn default_ouptut_path(&self) -> PathBuf {
        self.point_set_file
            .parent()
            .map(|parent| parent.to_path_buf().join(DEFAULT_OUTPUT_FILE_NAME))
            .unwrap_or(DEFAULT_OUTPUT_FILE_NAME.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_output_path_valid() {
        let args = CommandLineArguments {
            point_set_file: "/test/path/point_set.json".into(),
            output_path: None,
        };
        assert_eq!(
            args.default_ouptut_path(),
            PathBuf::from("/test/path").join(DEFAULT_OUTPUT_FILE_NAME)
        );
    }

    #[test]
    fn test_default_output_path_invalid() {
        let args = CommandLineArguments {
            point_set_file: "".into(),
            output_path: None,
        };
        assert_eq!(args.default_ouptut_path(), PathBuf::from(DEFAULT_OUTPUT_FILE_NAME));
    }

    #[test]
    fn test_output_path_set() {
        let output_path = PathBuf::from("test/output/path/file.json");
        let args = CommandLineArguments {
            point_set_file: "/test/path/point_set.json".into(),
            output_path: Some(output_path),
        };
        assert_eq!(args.output_path(), args.output_path.clone().unwrap());
    }

    #[test]
    fn test_output_path_unset_valid() {
        let args = CommandLineArguments {
            point_set_file: "/test/path/point_set.json".into(),
            output_path: None,
        };
        assert_eq!(args.output_path(), PathBuf::from("/test/path").join(DEFAULT_OUTPUT_FILE_NAME));
    }

    #[test]
    fn test_output_path_unset_invalid() {
        let args = CommandLineArguments {
            point_set_file: "".into(),
            output_path: None,
        };
        assert_eq!(args.output_path(), PathBuf::from(DEFAULT_OUTPUT_FILE_NAME));
    }
}
