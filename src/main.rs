use subprocess::Exec;
use std::path::PathBuf;
use std::fs;
use actix_web::{get, web, App, HttpServer};
use serde::Deserialize;



fn run_aqua(name: String) -> String {
    let aqua_dir = PathBuf::from("./aqua");
    let aqua_path = fs::canonicalize(&aqua_dir).unwrap();
    let cmd = format!("fluence run -i {} -f 'helloWorld(\"{}\")' --env=kras", aqua_path.display().to_string(), name);
    println!("cmd: {:?}", cmd);

    let output = {
        Exec::shell(cmd)
    }.capture(); //?.stdout_str();

    match output {
        Ok(so) => so.stdout_str(),
        Err(e) => format!("subprocess error calling fcli: {}", e),
    }
}

#[derive(Deserialize)]
struct AquaData {
    hello_param:String,
    // script_name: String,
}

// this handler gets called if the query deserializes into `Info` successfully
// otherwise a 400 Bad Request error response is returned
#[get("/")]
async fn index(info: web::Query<AquaData>) -> String {
    let info = info.into_inner();
    let res = run_aqua(info.hello_param);
    format!("aqua helloWorld response: {}", res)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}