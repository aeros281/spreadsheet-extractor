extern crate google_sheets4 as sheets4;
extern crate hyper;

use std::path::PathBuf;

use crate::config::{Config, Google};

use google_sheets4::common;
use sheets4::Result;
use sheets4::api::ValueRange;
use sheets4::{Sheets, hyper_rustls, hyper_util, yup_oauth2};

pub async fn get_sheet_data(
    config: &Config,
    sheet_id: &str,
    range: &str,
) -> Result<(common::Response, ValueRange)> {
    let google_config: &Google = &config
        .google
        .as_ref()
        .expect("No google configuration found");

    let service_account_path = build_path(&google_config.client_secret_path);
    let token_storage_path = build_path(&google_config.token_storage_path);
    let secret = yup_oauth2::read_application_secret(service_account_path).await
        .expect("Cannot find the application secret, please make sure to set the config for google.client_secret_path");
    let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
        secret,
        yup_oauth2::InstalledFlowReturnMethod::Interactive,
    )
    .persist_tokens_to_disk(token_storage_path)
    .build()
    .await?;

    let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
        .build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .unwrap()
                .https_or_http()
                .enable_http1()
                .build(),
        );

    let hub = Sheets::new(client, auth);
    hub.spreadsheets().values_get(sheet_id, range).doit().await
}

fn build_path(val: &str) -> PathBuf {
    let expanded_path_str: std::borrow::Cow<str> = shellexpand::tilde(val);
    PathBuf::from(expanded_path_str.as_ref())
}
