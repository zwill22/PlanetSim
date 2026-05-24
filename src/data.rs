use crate::bodies::Bodies;
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

fn semi_major() -> Expr {
    col("a_prp(km)").fill_null(col("a_osc (km)")).alias("a")
}

fn eccentricity() -> Expr {
    col("e_prp").fill_null(col("e_osc")).alias("e")
}

fn period() -> Expr {
    col("P (d)_duplicated_0")
        .fill_null(col("P (d)"))
        .alias("period")
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

    lf.with_columns([semi_major(), eccentricity(), period()])
        .select([
            col("number"),
            col("name"),
            col("fam").alias("family"),
            col("a").cast(DataType::Float64),
            col("e").cast(DataType::Float64),
            col("period").cast(DataType::Float64),
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

pub(crate) struct Data {
    df: DataFrame,
}

impl Data {
    pub(crate) fn new() -> Data {
        let dynamic = dynamic_data();
        let physical = physical_data();

        let df = dynamic
            .join(physical, [col("name")], [col("name")], JoinArgs::default())
            .collect()
            .unwrap();

        Data { df }
    }

    fn get_str_column(&self, name: &str) -> &StringChunked {
        self.df.column(name).unwrap().str().unwrap()
    }

    fn get_bool_column(&self, name: &str) -> &BooleanChunked {
        self.df.column(name).unwrap().bool().unwrap()
    }

    fn get_f64_column(&self, name: &str) -> &Float64Chunked {
        self.df.column(name).unwrap().f64().unwrap()
    }

    pub(crate) fn get_bodies(&self, settings: &Settings) -> Bodies {
        let mut output = vec![];

        let names = self.get_str_column("name");
        let numbers = self.get_str_column("number");
        let radii = self.get_f64_column("radius");
        let semi_major_axis = self.get_f64_column("a");
        let eccentricities = self.get_f64_column("e");
        let period = self.get_f64_column("period");
        let satellites = self.get_bool_column("satellite");
        let prograde = self.get_bool_column("prograde");

        for i in 0..names.len() {
            let name = names.get(i);
            let number = numbers.get(i);

            let radius = radii.get(i).unwrap();
            if radius < 5.0 {
                continue;
            }
            let a = semi_major_axis.get(i);
            let e = eccentricities.get(i);
            let satellite = satellites.get(i).unwrap();
            if settings.out_of_focus(name, satellite, number) {
                continue;
            }

            let t = period.get(i);
            let pro = prograde.get(i).unwrap();

            let body = Body::new(name, radius, a, e, pro, t, settings);

            output.push(body);
        }

        Bodies::new(output)
    }
}
