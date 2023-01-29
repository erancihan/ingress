/*
   This is the module for updating ingress.yml
   It recieves the ingress.yml file, and replaces the local one
       stored in `/home/ingress/ingress.yml`.
   Then it calls the `update_nginx` function to update the NGINX config.
   Then it calls the `reload_nginx` function to reload the NGINX config.
*/
use axum::{extract::Multipart, Json};
use serde::Serialize;
use std::{fs::File, io::Write};
use tracing::error;

use crate::workers::update_nginx_confs;

#[derive(Serialize)]
pub struct UpdateIngressYMLResponse {
    message: String,
}

pub async fn update_ingressyml(mut multipart: Multipart) -> Json<UpdateIngressYMLResponse> {
    // TODO: check authentication

    let mut file_in = None;
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        if name == "file" {
            file_in = Some(data);
        }
    }

    match File::create("./opt/ingress.new.yml") {
        Ok(mut filestream) => match file_in {
            Some(data) => {
                filestream.write_all(&data).unwrap();
            }
            None => {
                error!("No file found");
            }
        },
        Err(error) => {
            error!("Error: {}", error);
        }
    }

    // trigger async update_nginx worker
    let response = UpdateIngressYMLResponse {
        message: "config update started...".to_string(),
    };

    tokio::spawn(async {
        update_nginx_confs().await;
    });

    Json(response)
}
