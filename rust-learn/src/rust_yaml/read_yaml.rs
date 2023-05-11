use serde::{Serialize, Deserialize};
use log::{debug, error, log_enabled, info, Level};
use serde::de::DeserializeOwned;
use schemars::schema::RootSchema;
use std::path::PathBuf;


#[derive(Serialize, Deserialize, Debug)]
pub struct Profiles {
    pub active: String,
}
// 用来接收application.yml解析结果
#[derive(Serialize, Deserialize, Debug)]
pub struct EnvConfig {
    pub profiles: Profiles,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Mysql {
    pub host: String,
    pub port: u32,
    pub username: String,
    pub password: String,
    pub db_name: String,
}
// 用来接收application-dev.yml解析结果
#[derive(Serialize, Deserialize, Debug)]
pub struct GlobalConfig {
    // 解析对应的mysql配置
    pub mysql: Mysql,
    // 还可以添加其他需要解析的配置

}
// 加载指定配置文件
fn load_config<T>(path: &str) -> Option<T> where T: DeserializeOwned {
    // 1.通过std::fs读取配置文件内容
    // 2.通过serde_yaml解析读取到的yaml配置转换成json对象
    match serde_yaml::from_str::<RootSchema>(&std::fs::read_to_string(path).expect(&format!("failure read file {}", path))) {
        Ok(root_schema) => {
            // 通过serde_json把json对象转换指定的model
            let data = serde_json::to_string_pretty(&root_schema).expect("failure to parse RootSchema");
            let config = serde_json::from_str::<T>(&*data).expect(&format!("failure to format json str {}",&data));
            // 返回格式化结果
            Some(config)
        }
        Err(err) => {
            // 记录日志
            info!("{}",err);
            // 返回None
            None
        }
    }
}

// 加载目标文件application.yml
fn load_env_config() -> Option<EnvConfig> {
    let mut config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    config_path.push("src/application.yml");
    load_config::<EnvConfig>(config_path.to_str().unwrap())
}
// 根据环境加载application-{}.yml文件
fn load_global_config_from_env(active: String) -> Option<GlobalConfig> {
    let path = format!("application-{}.yml", active);
    let mut config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    config_path.push("src");
    config_path.push(&path);
    load_config::<GlobalConfig>(config_path.to_str().unwrap())
}
// 真正对外暴露的方法，根据application.yml指定的环境变量动态加载对应的配置文件
pub fn load_global_config() -> Option<GlobalConfig> {
    if let Some(env_config) = load_env_config() {
        return load_global_config_from_env(env_config.profiles.active);
    }
    None
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn load_config_test() {
        match load_global_config() {
            None => {
                println!("None");
            }
            Some(config) => {
                println!("{:#?}", config);
            }
        }
    }
}

// use lazy_static::lazy_static;
// // use crate::load_config::init_load_config;
// // use crate::load_config::models::GlobalConfig;
// use serde::de::DeserializeOwned;
// use schemars::schema::RootSchema;
// lazy_static! {
//     pub static ref GLOBAL_CONFIG:GlobalConfig=init_load_config::load_global_config().unwrap();
// }
