#### This calculates for multiple .TAB files in kilometer

import os
import geopandas as gpd
from shapely.ops import transform
import pyproj

def calculate_total_distance_km(tab_file_path):
    # Load the .tab file
    gdf = gpd.read_file(tab_file_path)

    # Ensure the GeoDataFrame is in WGS84 (latitude and longitude)
    gdf = gdf.to_crs('EPSG:4326')
   
    # Define a projection to meters (using World Equidistant Cylindrical, EPSG:4087)
    # to accurately measure distances
    project = pyproj.Transformer.from_crs(
        'EPSG:4326', # Source coordinate system (WGS84)
        'EPSG:4087', # Destination coordinate system (meters)
        always_xy=True
    ).transform
   
    # Calculate total distance by converting line lengths from degrees to meters
    total_distance_meters = gdf.geometry.apply(lambda geom: transform(project, geom).length).sum()

    # Convert meters to kilometers
    total_distance_km = total_distance_meters / 1000

    return total_distance_km

# Directory containing your .tab files
directory_path = '/home/adesoji/Downloads/spectranet_task/Final_route'

# Loop through all .tab files in the directory and calculate total distances
for filename in os.listdir(directory_path):
    if filename.endswith('.TAB'):
        tab_file_path = os.path.join(directory_path, filename)
        total_distance_km = calculate_total_distance_km(tab_file_path)
        print(f"Total distance for {filename}: {total_distance_km:.2f} km")
