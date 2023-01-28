use std::{fs::File, io::{Write}};
/*
   This is the module for updating ingress.yml
   It recieves the ingress.yml file, and replaces the local one
       stored in `/home/ingress/ingress.yml`.
   Then it calls the `update_nginx` function to update the NGINX config.
   Then it calls the `reload_nginx` function to reload the NGINX config.
*/
use axum::{extract::Multipart, Json};
// use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct UpdateIngressYMLResponse {
    message: String,
}

#[derive(Deserialize)]
pub struct UpdateIngressYMLPayload {
    ingress_yml: String,
}

pub async fn update_ingressyml(mut multipart: Multipart) -> Json<UpdateIngressYMLResponse> {
    // TODO: check authentication

    let mut file = None;
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        if name == "file" {
            file = Some(data);
        }
    }

    match File::create("./opt/ingress.new.yml") {
        Ok(mut f) => {
            match file {
                Some(data) => {
                    f.write_all(&data).unwrap();
                },
                None => {
                    println!("No file found");
                }
            }
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    let response = UpdateIngressYMLResponse {
        message: "config update started...".to_string(),
    };

    Json(response)
}
