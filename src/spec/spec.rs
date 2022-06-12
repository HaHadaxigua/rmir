use std::collections::BTreeMap;

use docker_compose_types::{Networks, Service, Volume};
use serde::{Serialize, Deserialize};

// ---------------------------------------------------------------------------------- Spec


#[derive(Debug, Serialize, Deserialize)]
pub struct Spec {
    #[serde(skip_serializing_if = "Option::is_none")]
    service_config_map: Option<BTreeMap<String, Service>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_config_map: Option<BTreeMap<String, Volume>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_config_map: Option<BTreeMap<String, Networks>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    param_config_list: Option<Vec<ParamConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_config_list: Option<Vec<FileConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    template_config_list: Option<Vec<TemplateConfig>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    before_script_list: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after_script_list: Option<Vec<String>>,
}

// ---------------------------------------------------------------------------------- ParamConfig

#[derive(Debug, Serialize, Deserialize)]
pub struct ParamConfig {
    name: String,
    kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<serde_yaml::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema: Option<serde_yaml::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validation: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    internal: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    essential: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    no_save: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alias: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    extension: Option<BTreeMap<String, serde_yaml::Value>>,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_param_config() {
        let param_config = ParamConfig {
            name: "1".to_string(),
            kind: "string".to_string(),
            description: None,
            value: Option::from(serde_yaml::Value::String("hhh".to_string())),
            env: None,
            schema: None,
            validation: None,
            internal: None,
            required: None,
            essential: None,
            no_save: None,
            alias: None,
            extension: None,
        };

        let s = serde_yaml::to_string(&param_config).unwrap();
        let config: ParamConfig = serde_yaml::from_str(s.as_str()).unwrap();
        println!("{:?}", config)
    }
}

// ---------------------------------------------------------------------------------- TemplateConfig

#[derive(Debug, Serialize, Deserialize)]
struct TemplateConfig {
    name: String,
    content: String,
}

// ---------------------------------------------------------------------------------- FileConfig

#[derive(Debug, Serialize, Deserialize)]
struct FileConfig {
    path: Option<String>,
    policy: Option<String>,
    content: Option<String>,
    source: Option<String>,
    permission: Option<isize>,
    uid: Option<isize>,
    gid: Option<isize>,
}