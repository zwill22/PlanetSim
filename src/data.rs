use crate::body::Body;
use crate::settings::Settings;
use polars::prelude::*;

fn dynamic_data() -> LazyFrame {
    let lf = LazyCsvReader::new(PlRefPath::new("data/dynamic.csv"))
        .finish()
        .unwrap();

    lf.filter(
        col("fam")
            .eq(lit("PL-t"))
            .or(col("fam").eq(lit("PL-j")))
            .or(col("fam").eq(lit("N/A")))
            .or(col("fam").eq(lit("A-mb")))
            .or(col("fam").str().contains_literal(lit("TN-")))
    )
    .select([
        col("name"),
        col("fam").alias("family"),
        col("a_prp(km)").cast(DataType::Float64).alias("a"),
        col("e_prp").cast(DataType::Float64).alias("e"),
        col("P (d)").cast(DataType::Float64).alias("period"),
    ])
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

    for i in 0..names.len() {
        let name = names.get(i);

        let radius = radii.get(i).unwrap();
        let a = semi_major_axis.get(i);
        let e = eccentricities.get(i);

        let t = period.get(i);

        let body = Body::new(name, radius, a, e, t, settings);

        output.push(body);
    }

    output
}
