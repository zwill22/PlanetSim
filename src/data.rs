use crate::body::Body;
use crate::settings::Settings;
use polars::prelude::*;
use std::collections::HashMap;

fn family() -> Expr {
    let f = col("family");
    let mut expr = when(f.clone().eq(lit("N/A")))
        .then(lit("Star"))
        .when(f.clone().str().starts_with(lit("TN-")))
        .then(lit("TNO"));

    let families: HashMap<&str, &str> = [
        ("A-mb", "Asteroid"),
        ("PL-t", "Planet"),
        ("PL-j", "Planet"),
        ("AnaR", "Ananke family (retrograde)"),
        ("CarP", "Carpo family (prograde)"),
        ("CrmR", "Carme family (retrograde)"),
        ("HimP", "Himalia family (prograde)"),
        ("PasR", "Pasiphae family (retrograde)"),
        ("TheP", "Themisto family (prograde)"),
        ("ValP", "Valetudo family (prograde)"),
        ("GaAP", "Gallic-Albiorix family (prograde)"),
        ("GalP", "Gallic other family (prograde)"),
        ("InKP", "Inuit-Kiviuq family (prograde)"),
        ("InSP", "Inuit-Siarnaq family (prograde)"),
        ("InuP", "Inuit other family (prograde)"),
        ("KarR", "Kari family (retrograde)"),
        ("MunR", "Mundifari family (retrograde)"),
        ("NorR", "low inclination Norse family (retrograde)"),
        ("PhoR", "Phobe family (retrograde);"),
        ("REG", "Regular satellite"),
        ("SIR", "small inner regular"),
        ("IrrP", "outer irregular prograde"),
        ("IrrR", "outer irregular retrograde"),
    ]
    .iter()
    .cloned()
    .collect();

    for (family, description) in families {
        expr = expr.when(f.clone().eq(lit(family))).then(lit(description));
    }

    expr.otherwise(lit("Unknown")).alias("description")
}

fn satellite() -> Expr {
    when(col("description").eq(lit("Asteroid")))
        .then(false)
        .when(col("description").eq(lit("Planet")))
        .then(false)
        .when(col("description").eq(lit("Star")))
        .then(false)
        .when(col("description").eq(lit("TNO")))
        .then(false)
        .otherwise(true)
        .alias("satellite")
}

fn prograde() -> Expr {
    when(col("description").str().contains_literal(lit("retrograde")))
        .then(false)
        .otherwise(true)
        .alias("prograde")
}

fn dynamic_data() -> LazyFrame {
    let lf = LazyCsvReader::new(PlRefPath::new("data/dynamic.csv"))
        .finish()
        .unwrap();

    lf.select([
        col("name"),
        col("fam").alias("family"),
        col("a_prp(km)").cast(DataType::Float64).alias("a"),
        col("e_prp").cast(DataType::Float64).alias("e"),
        col("P (d)").cast(DataType::Float64).alias("period"),
    ])
    .with_column(family())
    .with_columns([satellite(), prograde()])
    .filter(
        col("description")
            .neq(lit("Unknown"))
            .and(col("name").is_not_null()),
    )
}

fn physical_data() -> LazyFrame {
    let lf = LazyCsvReader::new(PlRefPath::new("data/physical.csv"))
        .finish()
        .unwrap();

    lf.select([
        col("name"),
        (col("mean d (km)").cast(DataType::Float64) / Expr::from(2.0)).alias("radius"),
    ])
}

fn get_data() -> DataFrame {
    let dynamic = dynamic_data();
    let physical = physical_data();

    dynamic
        .join(physical, [col("name")], [col("name")], JoinArgs::default())
        .collect()
        .unwrap()
}

pub(crate) fn initialise(settings: &Settings) -> Vec<Body> {
    let df = get_data();

    let mut output = vec![];

    let names = df.column("name").unwrap().str().unwrap();
    let radii = df.column("radius").unwrap().f64().unwrap();
    let semi_major_axis = df.column("a").unwrap().f64().unwrap();
    let eccentricities = df.column("e").unwrap().f64().unwrap();
    let period = df.column("period").unwrap().f64().unwrap();
    let satellite = df.column("satellite").unwrap().bool().unwrap();
    let prograde = df.column("prograde").unwrap().bool().unwrap();

    for i in 0..names.len() {
        let name = names.get(i);

        let radius = radii.get(i).unwrap();
        let a = semi_major_axis.get(i);
        let e = eccentricities.get(i);
        if satellite.get(i).unwrap() {
            continue;
        }

        let t = period.get(i);
        let pro = prograde.get(i).unwrap();

        let body = Body::new(name, radius, a, e, pro, t, settings);

        output.push(body);
    }

    output
}
