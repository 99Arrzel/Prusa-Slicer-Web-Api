use actix_multipart::Multipart;
use actix_web::{get, post, App, HttpRequest, HttpResponse, HttpServer, Responder};
use futures_util::TryStreamExt;
use tokio::fs;
use tokio::io::AsyncWriteExt;
#[derive(serde::Serialize)]
struct SliceResponse {
    filament_used_mm: f32,
    filament_used_cm3: f32,
    filament_used_g: f32,
    estimated_printing_time: String,
    slicer_output: String,
}
const BASE_DIR: &str = "./";

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Working!")
}
#[post("/slice")]
async fn slice(mut payload: Multipart, req: HttpRequest) -> impl Responder {
    //benchmark time, get actual time
    let now = std::time::Instant::now();
    let mut current_count: usize = 0;
    let name = uuid::Uuid::new_v4();
    //    let mut stl = fs::File::create(format!("{}.stl", name.to_string()))
    //use base_dir
    let mut stl = fs::File::create(format!("{}{}.stl", BASE_DIR, name.to_string()))
        .await
        .unwrap();
    let mut ini = fs::File::create(format!("{}{}.ini", BASE_DIR, name.to_string()))
        .await
        .unwrap();
    loop {
        //2 files, config and stl, future may multiple stl 1 config
        if current_count > 2 {
            return HttpResponse::BadRequest().body("Max file count reached");
        }
        if let Ok(Some(mut field)) = payload.try_next().await {
            let filetype = field.content_type();
            if filetype.is_none() {
                return HttpResponse::BadRequest().body("Invalid file type");
            }
            let filename = field.content_disposition().get_filename().unwrap();
            if filename.ends_with("stl") {
                while let Ok(Some(chunk)) = field.try_next().await {
                    let _ = stl.write_all(&chunk).await.unwrap();
                }
            } else if filename.ends_with("ini") {
                while let Ok(Some(chunk)) = field.try_next().await {
                    let _ = ini.write_all(&chunk).await.unwrap();
                }
            } else {
                return HttpResponse::BadRequest().body("Invalid file type");
            }
        } else {
            break;
        }
        current_count += 1;
    }
    println!("Elapsed time (Write files): {:?}", now.elapsed());
    //execute cli prusa-slicer "./Main_Body_v2.stl" --load "./config.ini" -g --repair --loglevel 1
    let output = tokio::process::Command::new("prusa-slicer")
        .arg(format!("./{}.stl", name.to_string()))
        .arg("--load")
        .arg(format!("./{}.ini", name.to_string()))
        .arg("-g")
        .arg("--repair")
        .arg("--loglevel")
        .arg("1")
        .output()
        .await
        .unwrap_or_else(|e| panic!("failed to execute process: prusa-slicer {}", e));
    println!("Elapsed time (Generate GCODE): {:?}", now.elapsed());
    let grep = tokio::process::Command::new("grep")
        .arg("-A")
        .arg("5")
        .arg("filament used")
        .arg(format!("./{}.gcode", name.to_string()))
        .output()
        .await
        .unwrap_or_else(|e| panic!("failed to execute process: grep{}", e));
    //json response, divided by newlines, {filament_used_mm: line[0], and so on }
    //first split by newlines
    println!("Elapsed time (Grep file): {:?}", now.elapsed());
    let grep_output = String::from_utf8(grep.stdout).unwrap();
    println!("---------------GREP OUTPUT----------------");
    println!("{}", grep_output);
    println!("------------------------------------------");
    let grep_output = grep_output.split("\n");
    let grep_output: Vec<&str> = grep_output.collect();
    //array to json with serde

    let output_json = SliceResponse {
        filament_used_mm: grep_output[0].split("=").collect::<Vec<&str>>()[1]
            .trim()
            .parse::<f32>()
            .unwrap_or_else(|e| panic!("failed to parse: filament_used_mm {}", e)),
        filament_used_cm3: grep_output[1].split("=").collect::<Vec<&str>>()[1]
            .trim()
            .parse::<f32>()
            .unwrap_or_else(|e| panic!("failed to parse: filament_used_cm3 {}", e)),
        filament_used_g: grep_output[2].split("=").collect::<Vec<&str>>()[1]
            .trim()
            .parse::<f32>()
            .unwrap_or_else(|e| panic!("failed to parse: filament_used_g {}", e)),
        estimated_printing_time: grep_output[4].split("=").collect::<Vec<&str>>()[1]
            .trim()
            .to_string(),
        slicer_output: String::from_utf8(output.stdout)
            .unwrap_or_else(|e| panic!("failed to parse: output {}", e)),
    };
    let response = serde_json::to_string(&output_json).unwrap();
    //Finally, erase the files
    fs::remove_file(format!("./{}.stl", name.to_string()))
        .await
        .unwrap_or_else(|e| panic!("failed to remove: {}.stl {}", name.to_string(), e));
    fs::remove_file(format!("./{}.ini", name.to_string()))
        .await
        .unwrap_or_else(|e| panic!("failed to remove: {}.ini {}", name.to_string(), e));
    fs::remove_file(format!("./{}.gcode", name.to_string()))
        .await
        .unwrap();
    println!("Elapsed total time: {:?}", now.elapsed());
    return HttpResponse::Ok().body(response);
}
//slice receives 2 files and returns a json response of the cli execution

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(slice))
        .bind(("0.0.0.0", 3080))?
        .run()
        .await
}
