// Calculate the time it take to run this program which is 5.50 Seconds
use std::time::Instant;
use std::path::Path;
use std::io;
use std::fs;
use std::process::Command;
use std::io::ErrorKind;

use geo::EuclideanLength;
use geo::LineString;
use geo::Point;
use geojson::GeoJson;
use proj::Proj;



fn convert_tab_to_geojson(tab_file_path: &Path, geojson_file_path: &Path) -> io::Result<()> { // Enhanced Error Reporting
    println!("Attempting to convert TAB to GeoJSON for: {:?}", tab_file_path);
    let output = Command::new("ogr2ogr")
        .arg("-f")
        .arg("GeoJSON")
        .arg(geojson_file_path)
        .arg(tab_file_path)
        .output()?;

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        println!("Conversion failed: {}", error_message);
        return Err(io::Error::new(ErrorKind::Other, format!("Conversion failed for {:?}: {}", tab_file_path, error_message)));
    }

    println!("Conversion successful for: {:?}", tab_file_path);
    Ok(())
}

fn calculate_total_distance_km(geojson_file_path: &Path) -> std::result::Result<f64, Box<dyn std::error::Error>> {
    // Read the GeoJSON file content
    let data = fs::read_to_string(geojson_file_path)?;

    // Parse the GeoJSON data
    let geojson = data.parse::<GeoJson>()?;

    // Initialize the projection
    let proj = Proj::new_known_crs("EPSG:4326", "EPSG:4087", None)
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Projection not found"))?;

    let mut total_distance_meters = 0.0;
    if let GeoJson::FeatureCollection(collection) = geojson {
        for feature in collection.features {
            if let Some(geometry) = feature.geometry {
                if let geojson::Value::LineString(line_string_coords) = geometry.value {
                    let line_string: LineString<f64> = line_string_coords.iter()
                        .map(|coord| {
                            let (x, y) = proj.convert((coord[0], coord[1]))
                                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
                            Ok(Point::new(x, y))
                        })
                        .collect::<std::result::Result<Vec<_>, Box<dyn std::error::Error>>>()?
                        .into();
                    total_distance_meters += line_string.euclidean_length();
                }
            }
        }
    }

    // Convert meters to kilometers and return the total distance
    Ok(total_distance_meters / 1000.0)
}


fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now(); // Start timing

    let directory_path = Path::new("/home/adesoji/Downloads/ttask/Final_route");
    println!("Reading directory: {:?}", directory_path);

    if !directory_path.exists() {
        return Err(Box::new(io::Error::new(io::ErrorKind::NotFound, "Directory does not exist")));
    }

    for entry in fs::read_dir(directory_path)? {
        let entry = entry?;
        let path = entry.path();
        let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("").to_lowercase();
        println!("Found file: {:?}, extension: {:?}", path, extension);

        if extension == "tab" {
            println!("Processing TAB file: {:?}", path);
            let geojson_path = path.with_extension("geojson");
            if convert_tab_to_geojson(&path, &geojson_path).is_ok() {
                if let Ok(distance_km) = calculate_total_distance_km(&geojson_path) {
                    println!("Total distance for {}: {:.2} km", path.file_name().unwrap().to_string_lossy(), distance_km);
                } else {
                    println!("Failed to calculate distance for: {:?}", path);
                }
            }
        }
    }

    let duration = start.elapsed();
    println!("Total execution time: {:.2?} seconds", duration.as_secs_f64()); // Print total execution time in seconds

    Ok(())
}
