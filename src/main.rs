use std::fs;

extern crate yaml_rust;
use yaml_rust::YamlLoader;

extern crate curl;
use curl::easy::Easy;

struct GoogleDomainsDDNS {
    user_name: String,
    password: String,
    address: String,
}

impl GoogleDomainsDDNS {
    pub fn load_from_yaml() -> GoogleDomainsDDNS {
        let yaml_str = fs::read_to_string("ddns_conf.yaml").unwrap();
        let results = YamlLoader::load_from_str(&yaml_str).unwrap();
        let result = &results[0];
        let user_name = result["config"]["user"].as_str().unwrap();
        let password = result["config"]["password"].as_str().unwrap();
        let address = result["config"]["address"].as_str().unwrap();

        GoogleDomainsDDNS {
            user_name: user_name.to_string(),
            password: password.to_string(),
            address: address.to_string(),
        }
    }

    pub fn execute(self) {
        let mut easy = Easy::new();
        easy.url("https://domains.google.com/checkip").unwrap();
        easy.write_function(move |data| {
            let global_ip = String::from_utf8(data.to_vec()).unwrap();
            let mut easy = Easy::new();
            let http = format!(
                "https://{}:{}@domains.google.com/nic/update?hostname={}&myip={}",
                &self.user_name, &self.password, &self.address, global_ip
            );

            easy.url(http.as_str()).unwrap();
            easy.write_function(|response| {
                let response_str = String::from_utf8(response.to_vec()).unwrap();
                println!("{}", response_str);
                
                Ok(response.len())
            })
            .unwrap();
            easy.perform().unwrap();
            Ok(data.len())
        })
        .unwrap();
        easy.perform().unwrap();
    }
}

fn main() {
    let gddns = GoogleDomainsDDNS::load_from_yaml();
    gddns.execute();
}
