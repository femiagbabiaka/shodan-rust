extern crate hyper;
extern crate hyper_native_tls;

mod shodan {

    use hyper::client::Client;
    use hyper_native_tls::NativeTlsClient;
    use hyper::net::HttpsConnector;

    // BaseUrl is the basis for all of our api requests.
    const BASE_URL: &'static str = "https://api.shodan.io";

    #[derive(Debug)]
    pub struct ShodanClient {
        api_key: &'static str,
    }

    impl ShodanClient {
        fn create_http_client(&self) -> Client {
            let ssl = NativeTlsClient::new().unwrap();
            let connector = HttpsConnector::new(ssl);
            let client = Client::with_connector(connector);

            client
        }
        pub fn new(api_key: &'static str) -> ShodanClient {
            ShodanClient { api_key: api_key }
        }

        pub fn host_info(&self, ip_address: &str) -> Result<String, &'static str> {
            use std::io::Read;

            let client = self.create_http_client();

            let mut body = String::new();

            let formatted_url = format!("{}/shodan/host/{}?key={}",
                                        BASE_URL,
                                        ip_address,
                                        self.api_key);

            let mut response = client.get(&*formatted_url)
                .send()
                .unwrap();

            response.read_to_string(&mut body).unwrap();

            Ok(body)
        }

        pub fn search(&self, query: &str, facets: &str) -> Result<String, &'static str> {}
    }
}

fn main() {
    use shodan::ShodanClient;

    let client_response = ShodanClient::new("")
        .host_info("52.7.53.123");

    let body = client_response.unwrap();
    println!("{}", body);
}
