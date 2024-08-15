# geo-bounded-voronoi

A small command line utility to convert a 2-dimensional input point set to finite Voronoi cells that are bound by an input polygon.

## Building

```bash
cargo build --release
```

## Usage

To display all options use the help flag:

```bash
geo-bounded-voronoi -h
```

The utility can be run as follows:

```bash
geo-bounded-voronoi -o path/to/output/directory/output.json path/to/input.json
```

## Input format

The input file must be a JSON object with 2 keys, that both are arrays of 2-dimensional points (2 element arrays).

- `points` - The points set that should be used to create the Voronoi diagramm. Duplicate points are allowed, but are filtered out. All invalid points (non-normal coordinates: infinite, NaN, sub-normal, ...) are also filtered out.
- `bound` - A simple polygon shape without interiors to be used as Voronoi cell bound. The shape will be centered around each point and its intersection with the according Voronoi cell will be used as final output. The array must contain only valid points (see `points`) and must contain at least 3 unique points to form a valid polygon exterior. The first and last point should be the same point to indicate a closed polygon. If the first and last point are not equal the polygon will be closed automatically.

Example input:

```json
    {
        "points": [[0.0, 1.0], [1.0, 1.0], ...],
        "bound": [[-0.5, -0.5], [0.0, 0.0], [1.0, 0.5], [-0.5, -0.5]]
    }
```

## Output format

The output is a JSON file with the default name `geo_bound_voronoi.json`. This file contains an array of cell objects. Each cell object has the 2 following keys:

- `site` - The original 2-dimensional point that produced this vVoronoi cell.
- `cell` - An array of 2-dimensional points that are the corners of the bounded Voronoi cell polygon.

Example output:

```json
    [
        {
            "site": [0.0, 1.0],
            "cell": [[-0.5, -0.5], [0.0, 0.0], [1.0, 0.5], [-0.5, -0.5]]
        },
        {
            "site": [1.0, 1.0],
            "cell": [[-0.5, -0.5], [0.42, 0.42], [1.42, 0.42], [-0.5, -0.5]]
        },
        ...
    ]
```
