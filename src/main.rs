use std::fs::{self,File,OpenOptions};
use std::io::{self,BufRead,Write};
use std::path::Path;
use regex::Regex;

#[derive(Clone)]
struct Waypoint {
    id: String,
    name: String,
    latitude: f64,
    latitude_ns: String,
    latitude_int: u8,
    latitude_dec: f64,
    latitude_string: String,
    longitude: f64,
    longitude_we: String,
    longitude_int: u8,
    longitude_dec: f64,
    longitude_string: String,
}

impl Waypoint {
    fn nouveau() -> Waypoint {
        Waypoint {
            id: String::new(),
            name: String::new(),
            latitude: 0.0,
            latitude_ns: String::from("N"),
            latitude_int: 0,
            latitude_dec: 0.0,
            latitude_string: String::new(),
            longitude: 0.0,
            longitude_we: String::from("E"),
            longitude_int: 0,
            longitude_dec: 0.0,
            longitude_string: String::new(),
        }
    }

    fn convert(&mut self, mut chemin: &File) {
        if self.latitude.is_sign_negative() {
            self.latitude_ns = String::from("S");
        }

        self.latitude_int = self.latitude.abs().trunc() as u8;
        self.latitude_dec = self.latitude.abs().fract() * 60.0;
        self.latitude_string = format!("{};{:.6};{}", self.latitude_int, self.latitude_dec, self.latitude_ns);
        self.latitude_string = self.latitude_string.replace(".",",");

        if self.longitude.is_sign_negative() {
            self.longitude_we = String::from("W");
        }

        self.longitude_int = self.longitude.abs().trunc() as u8;
        self.longitude_dec = self.longitude.abs().fract() * 60.0;
        self.longitude_string = format!("{};{:.6};{}", self.longitude_int, self.longitude_dec, self.longitude_we);
        self.longitude_string = self.longitude_string.replace(".",",");

        println!("Exported waypoint :");
        println!("Latitude : {}°{:.6} {} -> {}", self.latitude_int, self.latitude_dec, self.latitude_ns, self.latitude_string);
        println!("Longitude : {}°{:.6} {} -> {}", self.longitude_int, self.longitude_dec, self.longitude_we, self.longitude_string);
        println!("-------------------------");

        writeln!(chemin, "{};{};{}", self.id, self.latitude_string, self.longitude_string).expect("Erreur d'écriture dans le fichier");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() -> std::io::Result<()> {
    let mut waypoints: Vec<Waypoint> = Vec::new();
    let mut wp = Waypoint::nouveau();
    let mut etape:u8 = 0;

    let re_waypoint_start = Regex::new(r#"<waypoint id="(?<id>\d{1,3})" name="(?<name>.*)" radius="(?<radius>\d+\.\d{6})">"#).unwrap();
    let re_waypoint_coordinates = Regex::new(r#"<position lat="(?<latitude>-?\d{1,2}\.\d{6})" lon="(?<longitude>-?\d{1,3}\.\d{6})"/>"#).unwrap();
    let re_waypoint_end = Regex::new(r"</waypoint>$").unwrap();

    let directory_rtz = String::from("rtz"); // Input folder for *.rtz files
    let directory_csv = String::from("csv"); // Output folder for *.csv files

    // Looks for routes in the "rtz" folder.
    for route in fs::read_dir(directory_rtz)? {
        let route = route?;

        // File opening
        if let Ok(lines) = read_lines(route.path()) {
            println!("Ouverture du fichier : {}", route.path().display());
            println!("-------------------------");
            
            for line in lines {
                if let Ok(ligne) = line {
                    if etape == 0 && re_waypoint_start.is_match(&ligne) {
                        etape = 1;
                        for cap in re_waypoint_start.captures_iter(&ligne) {
                            println!("Waypoint found :");
                            println!("Id : {} - Nom : {} - Radius : {}", &cap["id"], &cap["name"], &cap["radius"]);
                            wp.id = (&cap["id"]).to_string();
                            wp.name = (&cap["name"]).to_string();
                        }
                    }

                    if etape == 1 && re_waypoint_coordinates.is_match(&ligne) {
                        etape = 2;
                        for cap in re_waypoint_coordinates.captures_iter(&ligne) {
                            println!("Latitude : {} - Longitude : {}", &cap["latitude"], &cap["longitude"]);
                            wp.latitude = cap["latitude"].parse().unwrap();
                            wp.longitude = cap["longitude"].parse().unwrap();
                        }
                    }

                    if etape == 2 && re_waypoint_end.is_match(&ligne) {
                        etape = 0;
                        waypoints.push(wp.clone());
                        println!("-------------------------");
                    }
                }
            }
        }

        // Output to csv file
        println!("Export des waypoints dans le fichier {}/{}.csv", directory_csv, Path::new(&route.path()).file_stem().and_then(|s| s.to_str()).unwrap());
        println!("-------------------------");
        let fichier_export = OpenOptions::new()
            .append(true)
            .create(true)
            .open(directory_csv.to_owned() + "/" + Path::new(&route.path()).file_stem().and_then(|s| s.to_str()).unwrap() + ".csv")
            .expect("Erreur : impossible d'écrire dans le fichier.");

        for waypoint in waypoints.iter_mut() {
            waypoint.convert(&fichier_export);
        }

        waypoints.clear();
    }

    Ok(())
}
