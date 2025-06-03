use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
struct Car {
    make: String,
    model: String,
    year: u16,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body(r#"
        <html>
            <head>
                <title>Car Display App</title>
                <style>
                    body { font-family: Arial, sans-serif; margin: 40px; }
                    h1 { color: #333; }
                    a { color: #0066cc; text-decoration: none; }
                    a:hover { text-decoration: underline; }
                </style>
            </head>
            <body>
                <h1>Welcome to Car Display App</h1>
                <p><a href="/cars">View all cars</a></p>
            </body>
        </html>
    "#)
}

#[get("/cars")]
async fn cars_handler() -> impl Responder {
    let cars_file_path = "cars.json";
    
    match fs::read_to_string(cars_file_path) {
        Ok(data) => {
            match serde_json::from_str::<Vec<Car>>(&data) {
                Ok(cars) => {
                    let mut html = String::from(r#"
                        <html>
                        <head>
                            <title>Available Cars</title>
                            <style>
                                body { font-family: Arial, sans-serif; margin: 40px; }
                                h1 { color: #333; }
                                table { border-collapse: collapse; width: 100%; margin-top: 20px; }
                                th, td { text-align: left; padding: 12px; border-bottom: 1px solid #ddd; }
                                th { background-color: #f2f2f2; font-weight: bold; }
                                tr:hover { background-color: #f5f5f5; }
                                .back-link { margin-top: 20px; display: inline-block; }
                                .back-link a { color: #0066cc; text-decoration: none; }
                                .back-link a:hover { text-decoration: underline; }
                            </style>
                        </head>
                        <body>
                            <h1>List of Available Cars</h1>
                            <table>
                                <tr>
                                    <th>Make</th>
                                    <th>Model</th>
                                    <th>Year</th>
                                </tr>
                    "#);

                    for car in cars {
                        html.push_str(&format!(
                            "                <tr><td>{}</td><td>{}</td><td>{}</td></tr>\n",
                            car.make, car.model, car.year
                        ));
                    }
                    
                    html.push_str(r#"
                            </table>
                            <div class="back-link">
                                <a href="/">← Back to Home</a>
                            </div>
                        </body>
                        </html>
                    "#);
                    
                    HttpResponse::Ok().content_type("text/html").body(html)
                }
                Err(e) => HttpResponse::InternalServerError().body(format!(
                    "<h1>Error</h1><p>Error deserializing cars.json: {}</p><p><a href='/'>Back to Home</a></p>", 
                    e
                )),
            }
        }
        Err(_) => HttpResponse::InternalServerError().body(
            r#"<h1>Error</h1>
            <p>Error reading cars.json. Please ensure the file exists in the project root.</p>
            <p><a href='/'>Back to Home</a></p>"#
        ),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");
    println!("Make sure 'cars.json' exists in the project root directory!");
    
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(cars_handler)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}