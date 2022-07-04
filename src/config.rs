/**
 * This module provides configurational constants and default values for the
 * application.
 */
use clap::Parser;
use std::env;
use std::str::FromStr;

//==============================================================================
// Constants
//==============================================================================

const DEFAULT_APP_STATUS_URL: &str = "http://localhost:3000/status";
const DEFAULT_APP_SUBMIT_URL: &str = "http://localhost:3000/chunk";
const DEFAULT_APP_STATUS_INTERVAL: u64 = 100;
const DEFAULT_APP_BUFFER_SIZE_LIMIT: usize = 100;
const DEFAULT_APP_API_KEY_NAME: &str = "api-key";
const DEFAULT_APP_API_KEY_VALUE: &str = "x11-sentinel-client";
const DEFAULT_APP_METADATA_QUERY_INTERVAL: i64 = 600000;
const DEFAULT_APP_IDLE_TIMEOUT: u64 = 10000;

//==============================================================================
// Structs
//==============================================================================

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    /// URL of the status API endpoint.
    #[clap(long, value_parser)]
    pub status_url: Option<String>,

    /// URL of the submission API endpoint.
    #[clap(long, value_parser)]
    pub submit_url: Option<String>,

    /// Query interval of the client's status in seconds.
    #[clap(long, value_parser)]
    pub status_interval: Option<u64>,

    /// Upper limit for the event buffer's size. When the event buffer's size
    /// reaches this number it triggers a submission.
    #[clap(long, value_parser)]
    pub buffer_size_limit: Option<usize>,

    /// Name of the API key that is sent with every submission request.
    #[clap(long, value_parser)]
    pub api_key_name: Option<String>,

    /// Value of the API key that is sent with every submission request.
    #[clap(long, value_parser)]
    pub api_key_value: Option<String>,

    /// Query interval of the platform specific metadata in milliseconds.
    #[clap(long, value_parser)]
    pub metadata_query_interval: Option<i64>,

    /// If no new event is generated for this number of milliseconds, a
    /// submission gets triggered.
    #[clap(long, value_parser)]
    pub idle_timeout: Option<u64>,
}

impl Config {
    /// Constructor method for the `Config` object.
    pub fn new() -> Config {
        let mut config = Config::parse();
        config.set_status_url();
        config.set_submit_url();
        config.set_status_interval();
        config.set_buffer_size_limit();
        config.set_api_key_name();
        config.set_api_key_value();
        config.set_metadata_query_interval();
        config.set_idle_timeout();
        return config;
    }

    /// Setter method for the `status_url` field.
    fn set_status_url(&mut self) -> () {
        match &self.status_url {
            Some(_value) => (),
            None => {
                self.status_url = Some(get_env_var_or(
                    "APP_STATUS_URL",
                    DEFAULT_APP_STATUS_URL.to_string(),
                ))
            }
        }
    }

    /// Setter method for the `submit_url` field.
    fn set_submit_url(&mut self) -> () {
        match &self.submit_url {
            Some(_value) => (),
            None => {
                self.submit_url = Some(get_env_var_or(
                    "APP_SUBMIT_URL",
                    DEFAULT_APP_SUBMIT_URL.to_string(),
                ))
            }
        }
    }

    /// Setter method for the `status_interval` field.
    fn set_status_interval(&mut self) -> () {
        match &self.status_interval {
            Some(_value) => (),
            None => {
                self.status_interval = Some(get_env_var_or(
                    "APP_STATUS_INTERVAL",
                    DEFAULT_APP_STATUS_INTERVAL,
                ))
            }
        }
    }
    /// Setter method for the `buffer_size_limit` field.
    fn set_buffer_size_limit(&mut self) -> () {
        match &self.buffer_size_limit {
            Some(_value) => (),
            None => {
                self.buffer_size_limit = Some(get_env_var_or(
                    "APP_BUFFER_SIZE_LIMIT",
                    DEFAULT_APP_BUFFER_SIZE_LIMIT,
                ))
            }
        }
    }

    /// Setter method for the `api_key_name` field.
    fn set_api_key_name(&mut self) -> () {
        match &self.api_key_name {
            Some(_value) => (),
            None => {
                self.api_key_name = Some(get_env_var_or(
                    "APP_API_KEY_NAME",
                    DEFAULT_APP_API_KEY_NAME.to_string(),
                ))
            }
        }
    }

    /// Setter method for the `api_key_value` field.
    fn set_api_key_value(&mut self) -> () {
        match &self.api_key_value {
            Some(_value) => (),
            None => {
                self.api_key_value = Some(get_env_var_or(
                    "APP_API_KEY_VALUE",
                    DEFAULT_APP_API_KEY_VALUE.to_string(),
                ))
            }
        }
    }

    /// Setter method for the `metadata_query_interval` field.
    fn set_metadata_query_interval(&mut self) -> () {
        match &self.metadata_query_interval {
            Some(_value) => (),
            None => {
                self.metadata_query_interval = Some(get_env_var_or(
                    "APP_METADATA_QUERY_INTERVAL",
                    DEFAULT_APP_METADATA_QUERY_INTERVAL,
                ))
            }
        }
    }

    /// Setter method for the `idle_timeout` field.
    fn set_idle_timeout(&mut self) -> () {
        match &self.idle_timeout {
            Some(_value) => (),
            None => {
                self.idle_timeout = Some(get_env_var_or(
                    "APP_IDLE_TIMEOUT",
                    DEFAULT_APP_IDLE_TIMEOUT,
                ))
            }
        }
    }
}

//==============================================================================
// Internal functions
//==============================================================================

/// Generic function to get an environment variable and parse it to the desired
/// type. If the environment variable is not defined or cannot be parsed a
/// default value is returned.
fn get_env_var_or<T: FromStr>(name: &str, default: T) -> T {
    let var: T = match env::var(name) {
        Ok(string) => match string.parse() {
            Ok(value) => value,
            Err(_error) => default,
        },
        Err(_error) => default,
    };
    return var;
}
