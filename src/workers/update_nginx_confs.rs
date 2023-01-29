use std::time::Duration;
use tokio::time;
use tracing::debug;
pub async fn update_nginx_confs() {
    debug!("update_nginx_confs invoke");

    // TODO: implement update_nginx_confs

    time::sleep(Duration::from_secs(5)).await;

    debug!("update_nginx_confs done");

    // TODO: reload nginx
}
