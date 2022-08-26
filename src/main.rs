use actix_web::{get, web, App, Error, HttpResponse, HttpServer, Result};
use rand::Rng;
use serde::Serialize;

#[derive(Serialize)]
struct FuelUsage {
    fuelUsage: f64,
}

#[derive(Serialize)]
struct FailProbability {
    failProbability: f64,
}

#[get("/calculateDieselUsageForDistance/{distance}/{fuelUsagePer100KM}/{yearOfproduction}")]
async fn calculateDistance(data: web::Path<(f64, f64, f64)>) -> Result<HttpResponse, Error> {
    let fuel = (data.0) / 100.0 * data.1;
    let fuelUsage = FuelUsage { fuelUsage: fuel };
    Ok(HttpResponse::Ok().json(fuelUsage))
}

#[get("/probabilityOfUnitInjectorFail/{VIN}")]
async fn fail() -> Result<HttpResponse, Error> {
    let mut fail: f64 = rand::thread_rng().gen_range(0.0..101.0);
    fail = fail.round() / 100.0;
    let failProbability = FailProbability {
        failProbability: fail,
    };
    Ok(HttpResponse::Ok().json(failProbability))
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(calculateDistance).service(fail))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
