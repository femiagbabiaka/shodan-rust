extern crate hyper;
extern crate hyper_native_tls;

pub mod shodan {

    use hyper::client::{Client, Response};
    use hyper_native_tls::NativeTlsClient;
    use hyper::net::HttpsConnector;
    use std::io::Read;
    use hyper::Ok as hyper_ok;

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
        fn request(&self, api_method: &str, url: String) -> Response {
            let client = self.create_http_client();

            let response = match api_method {
                "GET" => {
                    client.get(&*url)
                        .send()
                        .unwrap()
                }
                _ => panic!("Invalid HTTP Verb."),
            };

            response
        }

        fn form_response(&self, mut response: Response) -> Result<String, String> {

            let mut body = String::new();

            if response.status == hyper_ok {
                response.read_to_string(&mut body).unwrap();

                return Ok(body);
            }

            response.read_to_string(&mut body).unwrap();

            Err(body)
        }
        pub fn new(api_key: &'static str) -> ShodanClient {
            ShodanClient { api_key: api_key }
        }

        pub fn host_info(&self, ip_address: &str) -> Result<String, String> {
            let formatted_url = format!("{}/shodan/host/{}?key={}",
                                        BASE_URL,
                                        ip_address,
                                        self.api_key);

            let response = self.request("GET", formatted_url);
            self.form_response(response)
        }

        pub fn search(&self, query: &str) -> Result<String, String> {
            let formatted_url = format!("{}/shodan/host/search?key={}&query={}",
                                        BASE_URL,
                                        self.api_key,
                                        query);

            let response = self.request("GET", formatted_url);
            self.form_response(response)
        }

        pub fn search_with_facets(&self, query: &str, facets: &str) -> Result<String, String> {
            let formatted_url = format!("{}/shodan/host/search?key={}&query={}&facets={}",
                                        BASE_URL,
                                        self.api_key,
                                        query,
                                        facets);

            let response = self.request("GET", formatted_url);
            self.form_response(response)
        }

        pub fn honeyscore(&self, ip_address: &str) -> Result<String, String> {
            let formatted_url = format!("{}/labs/honeyscore/{}?key={}",
                                        BASE_URL,
                                        ip_address,
                                        self.api_key);

            let response = self.request("GET", formatted_url);
            self.form_response(response)
        }
    }
}