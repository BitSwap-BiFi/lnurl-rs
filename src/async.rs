//! LNURL by way of `reqwest` HTTP client.

use reqwest::Client;

use crate::api::*;
use crate::{decode_ln_url_response, Builder, Error};

#[derive(Debug)]
pub struct AsyncClient {
    client: Client,
}

impl AsyncClient {
    /// build an async client from a builder
    pub fn from_builder(builder: Builder) -> Result<Self, Error> {
        let mut client_builder = Client::builder();

        #[cfg(not(target_arch = "wasm32"))]
        if let Some(proxy) = &builder.proxy {
            client_builder = client_builder.proxy(reqwest::Proxy::all(proxy)?);
        }

        #[cfg(not(target_arch = "wasm32"))]
        if let Some(timeout) = builder.timeout {
            client_builder = client_builder.timeout(core::time::Duration::from_secs(timeout));
        }

        Ok(Self::from_client(client_builder.build()?))
    }

    /// build an async client from the base url and [`Client`]
    pub fn from_client(client: Client) -> Self {
        AsyncClient { client }
    }

    pub async fn make_request(&self, url: &str) -> Result<Box<dyn LnURLResponse>, Error> {
        let resp = self.client.get(url).send().await?;

        let txt = resp.error_for_status()?.text().await?;
        Ok(decode_ln_url_response(&txt))
    }

    pub async fn get_invoice(&self, callback: &str, msats: u64) -> Result<LnURLPayInvoice, Error> {
        let symbol = if callback.contains('?') { "&" } else { "?" };

        let resp = self
            .client
            .get(&format!("{}{}amount={}", callback, symbol, msats))
            .send()
            .await?;

        Ok(resp.error_for_status()?.json().await?)
    }
}
