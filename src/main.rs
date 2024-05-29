// use std::fs;
// use std::path::Path;
// use geo::{LineString, Point};
// use geo::algorithm::euclidean_length::EuclideanLength;
// use proj::Proj;
// use std::io;
// use geojson::GeoJson;




// fn calculate_total_distance_km(tab_file_path: &Path) -> std::result::Result<f64, Box<dyn std::error::Error>> {
//     let data = fs::read_to_string(tab_file_path)?;

//     println!("Reading file: {:?}", tab_file_path);
//     println!("Content: {}", data);  // Log the content to see what is being parsed

//     let geojson = data.parse::<GeoJson>()?;

//     let proj = Proj::new_known_crs("EPSG:4326", "EPSG:4087", None)
//         .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Projection not found"))?;

//     let mut total_distance_meters = 0.0;
//     if let GeoJson::FeatureCollection(collection) = geojson {
//         for feature in collection.features {
//             if let Some(geometry) = feature.geometry {
//                 match geometry.value {
//                     geojson::Value::LineString(line_string_coords) => {
//                         let line_string: LineString<f64> = line_string_coords.iter()
//                             .map(|coord| {
//                                 let (x, y) = proj.convert((coord[0], coord[1]))
//                                     .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
//                                 Ok(Point::new(x, y))
//                             })
//                             .collect::<std::result::Result<Vec<_>, Box<dyn std::error::Error>>>()?
//                             .into();
//                         total_distance_meters += line_string.euclidean_length();
//                     },
//                     _ => {}
//                 }
//             }
//         }
//     }

//     Ok(total_distance_meters / 1000.0)
// }

// fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
//     let directory_path = Path::new("/home/adesoji/Downloads/spectranet_task/Final_route");

//     for entry in fs::read_dir(directory_path)? {
//         let entry = entry?;
//         let path = entry.path();

//         if path.extension().and_then(|ext| ext.to_str()) == Some("TAB") {
//             let total_distance_km = calculate_total_distance_km(&path)?;
//             println!(
//                 "Total distance for {}: {:.2} km",
//                 path.file_name().unwrap().to_string_lossy(),
//                 total_distance_km
//             );
//         }
//     }

//     Ok(())
// }


// use std::fs;
// use std::path::Path;
// use geo::{LineString, Point};
// use geo::algorithm::euclidean_length::EuclideanLength;
// use proj::Proj;
// use std::io;
// use geojson::GeoJson;

// /// Function to calculate the total distance in kilometers from a GeoJSON file.
// fn calculate_total_distance_km(geojson_file_path: &Path) -> std::result::Result<f64, Box<dyn std::error::Error>> {
//     // Read the GeoJSON file content
//     let data = fs::read_to_string(geojson_file_path)?;

//     // Parse the GeoJSON data
//     let geojson = data.parse::<GeoJson>()?;

//     // Initialize the projection
//     let proj = Proj::new_known_crs("EPSG:4326", "EPSG:4087", None)
//         .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Projection not found"))?;

//     let mut total_distance_meters = 0.0;
//     if let GeoJson::FeatureCollection(collection) = geojson {
//         for feature in collection.features {
//             if let Some(geometry) = feature.geometry {
//                 if let geojson::Value::LineString(line_string_coords) = geometry.value {
//                     let line_string: LineString<f64> = line_string_coords.iter()
//                         .map(|coord| {
//                             let (x, y) = proj.convert((coord[0], coord[1]))
//                                 .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
//                             Ok(Point::new(x, y))
//                         })
//                         .collect::<std::result::Result<Vec<_>, Box<dyn std::error::Error>>>()?
//                         .into();
//                     total_distance_meters += line_string.euclidean_length();
//                 }
//             }
//         }
//     }

//     // Convert meters to kilometers and return the total distance
//     Ok(total_distance_meters / 1000.0)
// }

// fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
//     // Directory containing your .geojson files
//     let directory_path = Path::new("/home/adesoji/Downloads/spectranet_task/Final_route");

//     // Loop through all .geojson files in the directory and calculate total distances
//     for entry in fs::read_dir(directory_path)? {
//         let entry = entry?;
//         let path = entry.path();

//         if path.extension().and_then(|ext| ext.to_str()) == Some("geojson") {
//             let total_distance_km = calculate_total_distance_km(&path)?;
//             println!(
//                 "Total distance for {}: {:.2} km",
//                 path.file_name().unwrap().to_string_lossy(),
//                 total_distance_km
//             );
//         }
//     }

//     Ok(())
// }






// use std::process::Command;
// use std::path::Path;
// use std::fs;
// use std::io::{self, ErrorKind};

// use geo::{EuclideanLength, LineString, Point};
// use geojson::GeoJson;
// use proj::Proj;

// /// Converts MapInfo TAB files to GeoJSON using GDAL's ogr2ogr.
// fn convert_tab_to_geojson(tab_file_path: &Path, geojson_file_path: &Path) -> io::Result<()> {
//     let output = Command::new("ogr2ogr")
//         .arg("-f")
//         .arg("GeoJSON")
//         .arg(geojson_file_path)
//         .arg(tab_file_path)
//         .output()?;

//     if !output.status.success() {
//         let error_message = String::from_utf8_lossy(&output.stderr);
//         return Err(io::Error::new(ErrorKind::Other, error_message.to_string()));
//     }

//     Ok(())
// }

// /// Function to calculate the total distance in kilometers from a GeoJSON file.
// /// This function remains the same as your previous implementation.
// /// Function to calculate the total distance in kilometers from a GeoJSON file.
// fn calculate_total_distance_km(geojson_file_path: &Path) -> std::result::Result<f64, Box<dyn std::error::Error>> {
//     // Read the GeoJSON file content
//     let data = fs::read_to_string(geojson_file_path)?;

//     // Parse the GeoJSON data
//     let geojson = data.parse::<GeoJson>()?;

//     // Initialize the projection
//     let proj = Proj::new_known_crs("EPSG:4326", "EPSG:4087", None)
//         .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Projection not found"))?;

//     let mut total_distance_meters = 0.0;
//     if let GeoJson::FeatureCollection(collection) = geojson {
//         for feature in collection.features {
//             if let Some(geometry) = feature.geometry {
//                 if let geojson::Value::LineString(line_string_coords) = geometry.value {
//                     let line_string: LineString<f64> = line_string_coords.iter()
//                         .map(|coord| {
//                             let (x, y) = proj.convert((coord[0], coord[1]))
//                                 .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
//                             Ok(Point::new(x, y))
//                         })
//                         .collect::<std::result::Result<Vec<_>, Box<dyn std::error::Error>>>()?
//                         .into();
//                     total_distance_meters += line_string.euclidean_length();
//                 }
//             }
//         }
//     }

//     // Convert meters to kilometers and return the total distance
//     Ok(total_distance_meters / 1000.0)
// }
// fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
//     let directory_path = Path::new("/home/adesoji/Downloads/spectranet_task/Final_route");
    
//     for entry in fs::read_dir(directory_path)? {
//         let entry = entry?;
//         let path = entry.path();

//         if path.extension().and_then(|ext| ext.to_str()) == Some("tab") {
//             let geojson_path = path.with_extension("geojson");

//             // Convert TAB to GeoJSON
//             convert_tab_to_geojson(&path, &geojson_path)?;

//             // Calculate the total distance from the converted GeoJSON file
//             let total_distance_km = calculate_total_distance_km(&geojson_path)?;
//             println!(
//                 "Total distance for {}: {:.2} km",
//                 path.file_name().unwrap().to_string_lossy(),
//                 total_distance_km
//             );
//         }
//     }

//     Ok(())
// }





// use geo::{EuclideanLength, LineString, Point};
// use geojson::GeoJson;
// use proj::Proj;
// use std::fs;
// use std::path::Path;
// use std::process::Command;
// use std::io::{self, ErrorKind};

// fn convert_tab_to_geojson(tab_file_path: &Path, geojson_file_path: &Path) -> io::Result<()> {
//     println!("Converting TAB to GeoJSON for: {:?}", tab_file_path);
//     let output = Command::new("ogr2ogr")
//         .arg("-f")
//         .arg("GeoJSON")
//         .arg(geojson_file_path)
//         .arg(tab_file_path)
//         .output()?;

//     if !output.status.success() {
//         let error_message = String::from_utf8_lossy(&output.stderr);
//         return Err(io::Error::new(ErrorKind::Other, error_message.to_string()));
//     }

//     Ok(())
// }

// fn calculate_total_distance_km(geojson_file_path: &Path) -> std::result::Result<f64, Box<dyn std::error::Error>> {
//     // Read the GeoJSON file content
//     let data = fs::read_to_string(geojson_file_path)?;

//     // Parse the GeoJSON data
//     let geojson = data.parse::<GeoJson>()?;

//     // Initialize the projection
//     let proj = Proj::new_known_crs("EPSG:4326", "EPSG:4087", None)
//         .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Projection not found"))?;

//     let mut total_distance_meters = 0.0;
//     if let GeoJson::FeatureCollection(collection) = geojson {
//         for feature in collection.features {
//             if let Some(geometry) = feature.geometry {
//                 if let geojson::Value::LineString(line_string_coords) = geometry.value {
//                     let line_string: LineString<f64> = line_string_coords.iter()
//                         .map(|coord| {
//                             let (x, y) = proj.convert((coord[0], coord[1]))
//                                 .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
//                             Ok(Point::new(x, y))
//                         })
//                         .collect::<std::result::Result<Vec<_>, Box<dyn std::error::Error>>>()?
//                         .into();
//                     total_distance_meters += line_string.euclidean_length();
//                 }
//             }
//         }
//     }

//     // Convert meters to kilometers and return the total distance
//     Ok(total_distance_meters / 1000.0)
// }
// fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
//     let directory_path = Path::new("/home/adesoji/Downloads/spectranet_task/Final_route");
//     println!("Reading directory: {:?}", directory_path);

//     if !directory_path.exists() {
//         return Err(Box::new(io::Error::new(io::ErrorKind::NotFound, "Directory does not exist")));
//     }

//     for entry in fs::read_dir(directory_path)? {
//         let entry = entry?;
//         let path = entry.path();

//         println!("Found file: {:?}", path);

//         if path.extension().and_then(|ext| ext.to_str()) == Some("tab") {
//             let geojson_path = path.with_extension("geojson");
//             convert_tab_to_geojson(&path, &geojson_path)?;

//             let total_distance_km = calculate_total_distance_km(&geojson_path)?;
//             println!(
//                 "Total distance for {}: {:.2} km",
//                 path.file_name().unwrap().to_string_lossy(),
//                 total_distance_km
//             );
//         }
//     }

//     Ok(())
// }





// use geo::{EuclideanLength, LineString, Point};
// use geojson::GeoJson;
// use proj::Proj;
// use std::fs;
// use std::path::Path;
// use std::process::Command;
// use std::io::{self, ErrorKind};

// fn convert_tab_to_geojson(tab_file_path: &Path, geojson_file_path: &Path) -> io::Result<()> {
//     println!("Attempting to convert TAB to GeoJSON for: {:?}", tab_file_path);
//     let output = Command::new("ogr2ogr")
//         .arg("-f")
//         .arg("GeoJSON")
//         .arg(geojson_file_path)
//         .arg(tab_file_path)
//         .output()?;

//     if !output.status.success() {
//         let error_message = String::from_utf8_lossy(&output.stderr);
//         println!("Conversion failed: {}", error_message);
//         return Err(io::Error::new(ErrorKind::Other, error_message.to_string()));
//     }

//     println!("Conversion successful for: {:?}", tab_file_path);
//     Ok(())
// }

// fn calculate_total_distance_km(geojson_file_path: &Path) -> std::result::Result<f64, Box<dyn std::error::Error>> {
//     // Read the GeoJSON file content
//     let data = fs::read_to_string(geojson_file_path)?;

//     // Parse the GeoJSON data
//     let geojson = data.parse::<GeoJson>()?;

//     // Initialize the projection
//     let proj = Proj::new_known_crs("EPSG:4326", "EPSG:4087", None)
//         .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Projection not found"))?;

//     let mut total_distance_meters = 0.0;
//     if let GeoJson::FeatureCollection(collection) = geojson {
//         for feature in collection.features {
//             if let Some(geometry) = feature.geometry {
//                 if let geojson::Value::LineString(line_string_coords) = geometry.value {
//                     let line_string: LineString<f64> = line_string_coords.iter()
//                         .map(|coord| {
//                             let (x, y) = proj.convert((coord[0], coord[1]))
//                                 .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
//                             Ok(Point::new(x, y))
//                         })
//                         .collect::<std::result::Result<Vec<_>, Box<dyn std::error::Error>>>()?
//                         .into();
//                     total_distance_meters += line_string.euclidean_length();
//                 }
//             }
//         }
//     }

//     // Convert meters to kilometers and return the total distance
//     Ok(total_distance_meters / 1000.0)
// }

// fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
//     let directory_path = Path::new("/home/adesoji/Downloads/spectranet_task/Final_route");
//     println!("Reading directory: {:?}", directory_path);

//     if !directory_path.exists() {
//         return Err(Box::new(io::Error::new(io::ErrorKind::NotFound, "Directory does not exist")));
//     }

//     for entry in fs::read_dir(directory_path)? {
//         let entry = entry?;
//         let path = entry.path();
//         println!("Found file: {:?}", path);

//         if path.extension().and_then(|ext| ext.to_str()) == Some("tab") {
//             println!("Processing TAB file: {:?}", path);
//             let geojson_path = path.with_extension("geojson");
//             if convert_tab_to_geojson(&path, &geojson_path).is_ok() {
//                 if let Ok(distance_km) = calculate_total_distance_km(&geojson_path) {
//                     println!("Total distance for {}: {:.2} km", path.file_name().unwrap().to_string_lossy(), distance_km);
//                 } else {
//                     println!("Failed to calculate distance for: {:?}", path);
//                 }
//             }
//         }
//     }

//     Ok(())
// }













































use geo::{EuclideanLength, LineString, Point};
use geojson::GeoJson;
use proj::Proj;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::io::{self, ErrorKind};


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


// fn convert_tab_to_geojson(tab_file_path: &Path, geojson_file_path: &Path) -> io::Result<()> {
//     println!("Attempting to convert TAB to GeoJSON for: {:?}", tab_file_path);
//     let output = Command::new("ogr2ogr")
//         .arg("-f")
//         .arg("GeoJSON")
//         .arg(geojson_file_path)
//         .arg(tab_file_path)
//         .output()?;

//     if !output.status.success() {
//         let error_message = String::from_utf8_lossy(&output.stderr);
//         println!("Conversion failed: {}", error_message);
//         return Err(io::Error::new(ErrorKind::Other, error_message.to_string()));
//     }

//     println!("Conversion successful for: {:?}", tab_file_path);
//     Ok(())
// }

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

    Ok(())
}
