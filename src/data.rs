use crate::body::Body;
use graphics::color::{hex, BLUE, CYAN, GRAY, GREEN, RED, WHITE, YELLOW};
use polars::prelude::*;

fn colour(name: &str) -> [f32; 4] {
    match name {
        "Mercury" => GRAY,
        "Venus" => hex("FF8C00"),
        "Earth" => GREEN,
        "Mars" => RED,
        "Jupiter" => hex("FFA500"),
        "Saturn" => YELLOW,
        "Uranus" => CYAN,
        "Neptune" => BLUE,
        "Pluto" => hex("A52A2A"),
        "Vulcan" => RED,
        &_ => WHITE,
    }
}

pub(crate) fn initialise() -> Vec<Body> {
    let mut output = vec![];

    let sun = Body::new(695700.0, 0.0, 0.0, WHITE, 0.0, "Sol");
    output.push(sun);

    let lf = LazyCsvReader::new(PlRefPath::new("data/planets.csv"))
        .finish()
        .unwrap();

    let df = lf.select([
        col("planet"),
        (col("diameter") / Expr::from(2.0)).alias("radius"),
        col("perihelion"),
        col("aphelion"),
        col("orbital_eccentricity"),
        col("orbital_velocity").alias("v"),
    ])
    .collect()
    .unwrap();

    println!("{}", df);

    let names = df.column("planet").unwrap().str().unwrap();
    let radii = df.column("radius").unwrap().f64().unwrap();
    let p = df.column("perihelion").unwrap().f64().unwrap();
    let ap = df.column("aphelion").unwrap().f64().unwrap();
    let v = df.column("v").unwrap().f64().unwrap();

    for i in 0..names.len() {
        let name = names.get(i).unwrap();

        let radius = radii.get(i).unwrap();
        let peri = p.get(i).unwrap();
        let aph = ap.get(i).unwrap();
        let v0 = v.get(i).unwrap();

        let col = colour(name);


        let body = Body::new(radius, peri, aph, col, v0, name);

        output.push(body);
    }


    output
}
