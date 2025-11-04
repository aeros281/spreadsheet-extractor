extern crate google_sheets4 as sheets4;
extern crate hyper;

use std::path::PathBuf;

use crate::config::{Config, Google};

use csv::{ReaderBuilder, StringRecord};
use google_sheets4::api::{ClearValuesRequest, ClearValuesResponse};
use google_sheets4::common;
use google_sheets4::hyper_util::client::legacy::connect::HttpConnector;
use hyper_rustls::HttpsConnector;
use sheets4::Result;
use sheets4::api::ValueRange;
use sheets4::{Sheets, hyper_rustls, hyper_util, yup_oauth2};

pub async fn get_sheet_data(
    config: &Config,
    sheet_id: &str,
    range: &str,
) -> Result<(common::Response, ValueRange)> {
    let hub = build_hub(config).await?;
    hub.spreadsheets().values_get(sheet_id, range).doit().await
}

pub async fn clear_tab(
    config: &Config,
    sheet_id: &str,
    tab_name: &str,
) -> Result<ClearValuesResponse> {
    let hub = build_hub(config).await?;
    hub.spreadsheets()
        .values_clear(ClearValuesRequest::default(), sheet_id, tab_name)
        .doit()
        .await
        .map(|(_, res)| res)
}

pub async fn write_page(
    config: &Config,
    sheet_id: &str,
    tab_name: &str,
    path: &str,
) -> anyhow::Result<()> {
    clear_tab(config, sheet_id, tab_name).await?;

    let mut rdr = ReaderBuilder::new().has_headers(false).from_path(path)?;

    let records = rdr
        .records()
        .collect::<std::result::Result<Vec<StringRecord>, csv::Error>>()?;

    let req = ValueRange {
        major_dimension: None,
        range: Some(tab_name.to_string()),
        values: Some(
            records
                .into_iter()
                .map(|s| {
                    s.iter()
                        .map(|s| serde_json::Value::String(s.to_string()))
                        .collect()
                })
                .collect(),
        ),
    };
    let hub = build_hub(config).await?;

    hub.spreadsheets()
        .values_append(req, sheet_id, tab_name)
        .value_input_option("USER_ENTERED")
        .include_values_in_response(false)
        .doit()
        .await?;

    Ok(())
}

pub async fn build_hub(config: &Config) -> Result<Sheets<HttpsConnector<HttpConnector>>> {
    let google_config: &Google = config
        .google
        .as_ref()
        .expect("No google configuration found");

    let service_account_path = build_path(&google_config.client_secret_path);
    let token_storage_path = build_path(&google_config.token_storage_path);

    debug!(
        "expanded service account path = {}",
        service_account_path.display()
    );
    debug!(
        "expanded token storage path = {}",
        token_storage_path.display()
    );

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
    Ok(hub)
}

fn build_path(val: &str) -> PathBuf {
    simple_expand_tilde::expand_tilde(val).unwrap_or(PathBuf::from(val))
}
