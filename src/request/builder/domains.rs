use reqwest::Method;

use crate::request::{DoRequest, RequestBuilder};
use crate::response;

impl<'a, 't> RequestBuilder<'a, 't, response::Domains> {
    /// Returns a `RequestBuilder` that can be used to create a new domain.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use doapi::DoManager;
    /// # use doapi::DoRequest;
    /// # let domgr = DoManager::with_token("asfasdfasdf");
    /// // ... domgr set up same as before
    /// match domgr.domains()
    ///            .create("super.com", "10.10.10.1")
    ///            .retrieve() {
    ///     Ok(domain) => println!("Domain: {}", domain),
    ///     Err(e)     => println!("Error: {}", e)
    /// }
    /// ```
    pub fn create(&'a self, name: &str, ip: &str) -> RequestBuilder<'a, 't, response::Domain> {
        // POST: "https://api.digitalocean.com/v2/domains"
        // body:
        //      "ip_address" : "192.168.1.1"
        //      "name" : "supercool.com"
        let url = self.domgr.endpoint_url.clone().join("domains").unwrap();
        let body = format!(r#"{{"name":{:?},"ip_address":{:?}}}"#, name, ip);
        self.domgr.request_builder(Method::POST, url).body(body)
    }
}

impl<'a, 't> RequestBuilder<'a, 't, response::Domain> {
    /// Returns a `RequestBuilder` that can be used delete an existing domain.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use doapi::DoManager;
    /// # use doapi::DoRequest;
    /// # let domgr = DoManager::with_token("asfasdfasdf");
    /// // ... domgr set up same as before
    /// match domgr.domain("super.com")
    ///            .delete()
    ///            .retrieve() {
    ///     Ok(_)  => println!("Success"),
    ///     Err(_) => println!("Error")
    /// }
    /// ```
    pub fn delete(&'a self) -> RequestBuilder<'a, 't, response::HeaderOnly> {
        // DELETE: "https://api.digitalocean.com/v2/domains/$id"
        self.domgr.request_builder(Method::DELETE, self.url.clone())
    }

    /// Returns a type of `RequestBuilder` which allows you make requests related to multiple DNS
    /// records or the concept of "DNS Records" as a whole
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use doapi::DoManager;
    /// # use doapi::DoRequest;
    /// # let domgr = DoManager::with_token("asfasdfasdf");
    /// // ... domgr set up same as before
    /// match domgr.domains()
    ///            .create("super.com", "10.10.10.1")
    ///            .retrieve() {
    ///     Ok(domain) => println!("Domain: {}", domain),
    ///     Err(e)     => println!("Error: {}", e)
    /// }
    /// ```
    pub fn dns_records(&'a self) -> RequestBuilder<'a, 't, response::DnsRecords> {
        // GET: "https://api.digitalocean.com/v2/domains/$DOMAIN/records"
        let url = self.url.join("records").unwrap();
        self.domgr.request_builder(Method::GET, url)
    }

    /// Returns type of `RequestBuilder` which allows you make requests related to a single DNS
    /// record
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use doapi::DoManager;
    /// # use doapi::DoRequest;
    /// # let domgr = DoManager::with_token("asfasdfasdf");
    /// // ... domgr set up same as before
    /// match domgr.domain("super.com")
    ///            .retrieve() {
    ///     Ok(domain) => println!("Domain: {}", domain),
    ///     Err(e)     => println!("Error: {}", e)
    /// }
    /// ```
    pub fn dns_record(&'a self, id: &str) -> RequestBuilder<'a, 't, response::DnsRecord> {
        // GET "https://api.digitalocean.com/v2/domains/$DOMAIN/records/$ID"
        // Something seems off here with respect to the path.
        let url = self.url.join(id).unwrap();
        self.domgr.request_builder(Method::GET, url)
    }
}

impl<'a, 't> DoRequest<response::Domain> for RequestBuilder<'a, 't, response::Domain> {}
