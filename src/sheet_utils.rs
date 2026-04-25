extern crate google_sheets4 as sheets4;
extern crate hyper;

use std::path::PathBuf;

use crate::config::{Config, Google};
use tracing::{debug, trace, warn};

use csv::{ReaderBuilder, StringRecord};
use google_sheets4::api::{ClearValuesRequest, ClearValuesResponse};
use google_sheets4::common;
use google_sheets4::hyper_util::client::legacy::connect::HttpConnector;
use hyper_rustls::HttpsConnector;
use sheets4::Result;
use sheets4::api::ValueRange;
use sheets4::{Sheets, hyper_rustls, hyper_util, yup_oauth2};

/// Resolves a sheet GID to its title by calling spreadsheets.get.
#[tracing::instrument(skip(config), err)]
pub async fn resolve_gid_to_name(
    config: &Config,
    spreadsheet_id: &str,
    gid: i32,
) -> anyhow::Result<String> {
    let hub = build_hub(config).await.map_err(anyhow::Error::from)?;
    let (_, spreadsheet) = hub
        .spreadsheets()
        .get(spreadsheet_id)
        .doit()
        .await
        .map_err(anyhow::Error::from)?;
    let sheets = spreadsheet.sheets.unwrap_or_default();
    debug!("spreadsheet has {} sheet(s)", sheets.len());
    let name = sheets.into_iter().find_map(|s| {
        let props = s.properties?;
        if props.sheet_id == Some(gid) { props.title } else { None }
    });
    match &name {
        Some(n) => debug!("gid {gid} resolved to {n:?}"),
        None => warn!("no sheet with gid {gid} found in spreadsheet {spreadsheet_id}"),
    }
    name.ok_or_else(|| anyhow::anyhow!("no sheet with gid {gid} in spreadsheet {spreadsheet_id}"))
}

/// Builds an A1-notation range string with the sheet name properly quoted.
pub fn a1_range(sheet_name: &str, range: &str) -> String {
    let escaped = sheet_name.replace('\'', "''");
    format!("'{escaped}'!{range}")
}

#[tracing::instrument(skip(config), err)]
pub async fn get_sheet_data(
    config: &Config,
    sheet_id: &str,
    range: &str,
) -> Result<(common::Response, ValueRange)> {
    let hub = build_hub(config).await?;
    let encoded_range = range.replace('/', "%2F");
    debug!("values_get range={encoded_range}");
    hub.spreadsheets()
        .values_get(sheet_id, &encoded_range)
        .doit()
        .await
}

#[tracing::instrument(skip(config), err)]
pub async fn clear_tab(
    config: &Config,
    sheet_id: &str,
    tab_name: &str,
) -> Result<ClearValuesResponse> {
    let hub = build_hub(config).await?;
    let encoded_tab = tab_name.replace('/', "%2F");
    trace!("values_clear tab={encoded_tab}");
    hub.spreadsheets()
        .values_clear(ClearValuesRequest::default(), sheet_id, &encoded_tab)
        .doit()
        .await
        .map(|(_, res)| res)
}

#[tracing::instrument(skip(config), err)]
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

    debug!("uploading {} row(s) to tab {:?}", records.len(), tab_name);
    if records.is_empty() {
        warn!("CSV file {path:?} contains no rows — tab will remain empty after clear");
    }

    let encoded_tab = tab_name.replace('/', "%2F");
    trace!("values_append tab={encoded_tab}");
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
        .values_append(req, sheet_id, &encoded_tab)
        .value_input_option("USER_ENTERED")
        .include_values_in_response(false)
        .doit()
        .await?;

    Ok(())
}

#[tracing::instrument(skip(config), err)]
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
