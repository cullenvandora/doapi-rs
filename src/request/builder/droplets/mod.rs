use std::fmt;

use reqwest::Method;

use crate::request::{DoRequest, RequestBuilder};
use crate::response;
use crate::Extend;

use serde_json::json;

// name (true)  String           The human-readable string you wish to use when
// displaying the
// Droplet name. The name, if set to a domain
// name managed in the
// DigitalOcean DNS management system, will
// configure a PTR record
// for the Droplet. The name set during creation
// will also determine
// the hostname for the Droplet in its internal
// configuration.
// region (true) String          The unique slug id for the region that you
// wish to deploy in
// size (true)   String          The unique slug identifier for the size that
// you wish to select
//                               for this Droplet.
// image         number (if using an image ID), or String (if using a public
// image slug) The image ID of a public or private image, or the unique slug
// identifier for a public image. This image will be the base image for your
// Droplet.    true
// ssh_keys      Array           An array containing the IDs or fingerprints of
// the SSH keys that you wish to embed in the Droplet's root account upon
// creation.
// backups       Boolean         A boolean indicating whether automated backups
// should be enabled for the Droplet. Automated backups can only be enabled
// when the Droplet is created.
// ipv6          Boolean         A boolean indicating whether IPv6 is enabled
// on the Droplet.
// private_networking  Boolean  A boolean indicating whether private networking
// is enabled for the Droplet. Private networking is currently only available
// in certain regions.
// user_data     String          A string of the desired User Data for the
// Droplet. User Data is currently only available in regions with metadata
// listed in their features.
#[derive(Serialize)]
pub struct Droplet {
    pub name: String,
    pub region: String,
    pub size: String,
    pub image: String,
    pub ssh_keys: Option<Vec<String>>,
    pub backups: bool,
    pub ipv6: bool,
    pub private_networking: bool,
    pub user_data: Option<String>,
}

#[derive(Serialize)]
struct DropletWithId {
    pub name: String,
    pub region: String,
    pub size: String,
    pub image: u64,
    pub ssh_keys: Option<Vec<String>>,
    pub backups: bool,
    pub ipv6: bool,
    pub private_networking: bool,
    pub user_data: Option<String>,
}

impl From<Droplet> for DropletWithId {
    fn from(droplet: Droplet) -> DropletWithId {
        DropletWithId {
            name: droplet.name,
            region: droplet.region,
            size: droplet.size,
            image: droplet.image[..].parse().unwrap(),
            ssh_keys: droplet.ssh_keys,
            backups: droplet.backups,
            ipv6: droplet.ipv6,
            private_networking: droplet.private_networking,
            user_data: droplet.user_data,
        }
    }
}

impl fmt::Display for Droplet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Name: {}\n\
             Region: {}\n\
             Size: {}\n\
             Image: {}\n\
             SSH Keys: {}\n\
             Backups Enabled: {}\n\
             IPv6 Enabled: {}\n\
             Private Networking Enabled: {}\n\
             User Data: {}\n",
            self.name,
            self.region,
            self.size,
            self.image,
            if let Some(ref v) = self.ssh_keys {
                v.iter()
                    .fold(String::new(), |acc, s| acc + &format!(" {},", s)[..])
            } else {
                "None".to_owned()
            },
            self.backups,
            self.ipv6,
            self.private_networking,
            if let Some(d) = self.user_data.clone() {
                d
            } else {
                "None".to_owned()
            }
        )
    }
}

impl fmt::Display for DropletWithId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Name: {}\n\
             Region: {}\n\
             Size: {}\n\
             Image: {}\n\
             SSH Keys: {}\n\
             Backups Enabled: {}\n\
             IPv6 Enabled: {}\n\
             Private Networking Enabled: {}\n\
             User Data: {}\n",
            self.name,
            self.region,
            self.size,
            self.image,
            if let Some(ref v) = self.ssh_keys {
                v.iter()
                    .fold(String::new(), |acc, s| acc + &format!(" {},", s)[..])
            } else {
                "None".to_owned()
            },
            self.backups,
            self.ipv6,
            self.private_networking,
            if let Some(d) = self.user_data.clone() {
                d
            } else {
                "None".to_owned()
            }
        )
    }
}

impl<'a, 't> RequestBuilder<'a, 't, response::Droplet> {
    fn action(&'a self, action_type: &str) -> RequestBuilder<'a, 't, response::Action> {
        let url = self.url.extend("actions").unwrap();
        // let body = format!("{{\"type\":\"{}\"}}", action_type);
        // self.domgr.request_builder(Method::POST, url).body(body)
        //let body = format!("{{\"type\":\"{}\"}}", action_type);
        self.domgr
            .request_builder(Method::POST, url)
            .json(&json!({ "type": action_type }))
    }

    /// Returns a request that can be used to enable backups on a droplet.
    ///
    /// **NOTE**: `id` may either be an image ID, or slug
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use doapi::DoManager;
    /// # use doapi::DoRequest;
    /// let domgr = DoManager::with_token("asfasdfasdf");
    /// match domgr.droplet("1234")
    ///            .enable_backups()
    ///            .retrieve() {
    ///     Ok(_)  => println!("Success"),
    ///     Err(_) => println!("Error")
    /// }
    /// ```
    pub fn enable_backups(&'a self) -> RequestBuilder<'a, 't, response::Action> {
        // POST: "https://api.digitalocean.com/v2/droplets/$ID/actions"
        // body:
        //      "type" : "enable_backups"
        self.action("enable_backups")
    }

    /// Returns a request that can be used to disable backups on a droplet.
    ///
    /// **NOTE**: `id` may either be an image ID, or slug
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use doapi::DoManager;
    /// # use doapi::DoRequest;
    /// let domgr = DoManager::with_token("asfasdfasdf");
    /// match domgr.droplet("1234")
    ///            .disable_backups()
    ///            .retrieve() {
    ///     Ok(_)  => println!("Success"),
    ///     Err(_) => println!("Error")
    /// }
    /// ```
    pub fn disable_backups(&'a self) -> RequestBuilder<'a, 't, response::Action> {
        // POST: "https://api.digitalocean.com/v2/droplets/$ID/actions"
        // body:
        //      "type" : "disable_backups"
        self.action("disable_backups")
    }

    /// Returns a request that can be used to reboot a droplet.
    ///
    /// **NOTE**: `id` may either be an image ID, or slug
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use doapi::DoManager;
    /// # use doapi::DoRequest;
    /// let domgr = DoManager::with_token("asfasdfasdf");
    /// match domgr.droplet("1234")
    ///            .reboot()
    ///            .retrieve() {
    ///     Ok(_)  => println!("Success"),
    ///     Err(_) => println!("Error")
    /// }
    /// ```
    pub fn reboot(&'a self) -> RequestBuilder<'a, 't, response::Action> {
        // POST: "https://api.digitalocean.com/v2/droplets/$ID/actions"
        // body:
        //      "type" : "reboot"
        self.action("reboot")
    }
    
    /// Returns a request that can be used to power cycle a droplet.
    ///
    /// **NOTE**: `id` may either be an image ID, or slug
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use doapi::DoManager;
    /// # use doapi::DoRequest;
    /// let domgr = DoManager::with_token("asfasdfasdf");
    /// match domgr.droplet("1234")
    ///            .power_cycle()
    ///            .retrieve() {
    ///     Ok(_)  => println!("Success"),
    ///     Err(_) => println!("Error")
    /// }
    /// ```
    pub fn power_cycle(&'a self) -> RequestBuilder<'a, 't, response::Action> {
        // POST: "https://api.digitalocean.com/v2/droplets/$ID/actions"
        // body:
        //      "type" : "power_cycle"
        self.action("power_cycle")
    }

    /// Returns a request that can be used to shutdown a droplet.
    ///
    /// **NOTE**: `id` may either be an image ID, or slug
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use doapi::DoManager;
    /// # use doapi::DoRequest;
    /// let domgr = DoManager::with_token("asfasdfasdf");
    /// match domgr.droplet("1234")
    ///            .shutdown()
    ///            .retrieve() {
    ///     Ok(_)  => println!("Success"),
    ///     Err(_) => println!("Error")
    /// }
    /// ```
    pub fn shutdown(&'a self) -> RequestBuilder<'a, 't, response::Action> {
        // POST: "https://api.digitalocean.com/v2/droplets/$ID/actions"
        // body:
        //      "type" : "shutdown"
        self.action("shutdown")
    }

    /// Returns a request that can be used to power off a droplet.
    ///
    /// **NOTE**: `id` may either be an image ID, or slug
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use doapi::DoManager;
    /// # use doapi::DoRequest;
    /// let domgr = DoManager::with_token("asfasdfasdf");
    /// match domgr.droplet("1234")
    ///            .power_off()
    ///            .retrieve() {
    ///     Ok(_)  => println!("Success"),
    ///     Err(_) => println!("Error")
    /// }
    /// ```
    pub fn power_off(&'a self) -> RequestBuilder<'a, 't, response::Action> {
        // POST: "https://api.digitalocean.com/v2/droplets/$ID/actions"
        // body:
        //      "type" : "power_off"
        self.action("power_off")
    }

    /// Returns a request that can be used to power on a droplet.
    ///
    /// **NOTE**: `id` may either be an image ID, or slug
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use doapi::DoManager;
    /// # use doapi::DoRequest;
    /// let domgr = DoManager::with_token("asfasdfasdf");
    /// match domgr.droplet("1234")
    ///            .power_on()
    ///            .retrieve() {
    ///     Ok(_)  => println!("Success"),
    ///     Err(_) => println!("Error")
    /// }
    /// ```
    pub fn power_on(&'a self) -> RequestBuilder<'a, 't, response::Action> {
        // POST: "https://api.digitalocean.com/v2/droplets/$ID/actions"
        // body:
        //      "type" : "power_on"
        self.action("power_on")
    }

    //     pub fn restore(mut self, img: &str) -> RequestBuilder<'t, response::Action> {
    //         // POST: "https://api.digitalocean.com/v2/droplets/$ID/actions"
    //         // body:
    //         //      "type" : "restore"
    //         //      "image": 12356          // number if ID, string if slug
    //         self.url.push_str("/actions");
    //         let image = match img.parse::<u64>() {
    //             Ok(_) => img.to_string(),
    //             Err(_) => format!("\"{}\"", img),
    //         };
    //         RequestBuilder {
    //             method: Method::Post,
    //             auth: self.auth,
    //             url: self.url,
    //             resp_t: PhantomData,
    //             body: Some(format!("{{\"type\":\"restore\",\"image\":{}}}", image)),
    //         }
    //     }

    /// Returns a request that can be used to reset the password on a droplet.
    ///
    /// **NOTE**: `id` may either be an image ID, or slug
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use doapi::DoManager;
    /// # use doapi::DoRequest;
    /// let domgr = DoManager::with_token("asfasdfasdf");
    /// match domgr.droplet("1234")
    ///            .reset_password()
    ///            .retrieve() {
    ///     Ok(_)  => println!("Success"),
    ///     Err(_) => println!("Error")
    /// }
    /// ```
    pub fn reset_password(&'a self) -> RequestBuilder<'a, 't, response::Action> {
        // POST: "https://api.digitalocean.com/v2/droplets/$ID/actions"
        // body:
        //      "type" : "reset_password"
        self.action("reset_password")
    }
    //     pub fn resize(mut self, size: &str, disk: bool) -> RequestBuilder<'t, response::Action> {
    //         // POST: "https://api.digitalocean.com/v2/droplets/$ID/actions"
    //         // body:
    //         //      "type" : "resize"
    //         //      "disk" : bool
    //         //      "size" : "1gb"
    //         self.url.push_str("/actions");
    //         RequestBuilder {
    //             method: Method::Post,
    //             auth: self.auth,
    //             url: self.url,
    //             resp_t: PhantomData,
    //             body: Some(format!("{{\"type\":\"resize\",\"disk\":{},\"size\":{:?}}}",
    //                                disk.to_string(),
    //                                size)),
    //         }
    //     }
    //     pub fn rebuild(mut self, img: &str) -> RequestBuilder<'t, response::Action> {
    //         // POST: "https://api.digitalocean.com/v2/droplets/$ID/actions"
    //         // body:
    //         //      "type" : "rebuild"
    //         //      "image": 12356          // number if ID, string if slug
    //         self.url.push_str("/actions");
    //         let image = match img.parse::<u64>() {
    //             Ok(_) => img.to_string(),
    //             Err(_) => format!("\"{}\"", img),
    //         };
    //         RequestBuilder {
    //             method: Method::Post,
    //             auth: self.auth,
    //             url: self.url,
    //             resp_t: PhantomData,
    //             body: Some(format!("{{\"type\":\"rebuild\",\"image\":{}}}", image)),
    //         }
    //     }
    //     pub fn rename(mut self, name: &str) -> RequestBuilder<'t, response::Action> {
    //         // POST: "https://api.digitalocean.com/v2/droplets/$ID/actions"
    //         // body:
    //         //      "type" : "rename"
    //         //      "name" : "some name"
    //         self.url.push_str("/actions");
    //         RequestBuilder {
    //             method: Method::Post,
    //             auth: self.auth,
    //             url: self.url,
    //             resp_t: PhantomData,
    //             body: Some(format!("{{\"type\":\"rename\",\"name\":{:?}}}", name)),
    //         }
    //     }
    //     pub fn change_kernel(mut self, kernel: &str) -> RequestBuilder<'t, response::Action> {
    //         // POST: "https://api.digitalocean.com/v2/droplets/$ID/actions"
    //         // body:
    //         //      "type" : "change_kernel"
    //         //      "kernel" :  1234566
    //         self.url.push_str("/actions");
    //         RequestBuilder {
    //             method: Method::Post,
    //             auth: self.auth,
    //             url: self.url,
    //             resp_t: PhantomData,
    //             body: Some(format!("{{\"type\":\"change_kernel\",\"kernel\":{}}}", kernel)),
    //         }
    //     }
    pub fn enable_ipv6(&'a self) -> RequestBuilder<'a, 't, response::Action> {
        // POST: "https://api.digitalocean.com/v2/droplets/$ID/actions"
        // body:
        //      "type" : "shutdown"
        let url = self.url.extend("actions").unwrap();
        let body = String::from(r#"{"type":"enable_ipv6"}"#);
        self.domgr.request_builder(Method::POST, url).body(body)
    }
    pub fn enable_private_networking(&'a self) -> RequestBuilder<'a, 't, response::Action> {
        // POST: "https://api.digitalocean.com/v2/droplets/$ID/actions"
        // body:
        //      "type" : "shutdown"
        let url = self.url.extend("actions").unwrap();
        let body = String::from(r#"{"type":"enable_private_networking"}"#);
        self.domgr.request_builder(Method::POST, url).body(body)
    }
    //     pub fn snapshot(mut self, name: &str) -> RequestBuilder<'t, response::Action> {
    //         // POST: "https://api.digitalocean.com/v2/droplets/$ID/actions"
    //         // body:
    //         //      "type" : "snapshot"
    //         //      "name" : "some name"
    //         self.url.push_str("/actions");
    //         RequestBuilder {
    //             method: Method::Post,
    //             auth: self.auth,
    //             url: self.url,
    //             resp_t: PhantomData,
    //             body: Some(format!("{{\"type\":\"snapshot\",\"name\":{:?}}}", name)),
    //         }
    //     }
    pub fn upgrade(&'a self) -> RequestBuilder<'a, 't, response::Action> {
        // POST: "https://api.digitalocean.com/v2/droplets/$ID/actions"
        // body:
        //      "type" : "shutdown"
        let url = self.url.extend("actions").unwrap();
        let body = String::from(r#"{"type":"upgrade"}"#);
        self.domgr.request_builder(Method::POST, url).body(body)
    }
    //     pub fn action(mut self, id: &str) -> RequestBuilder<'t, response::Action> {
    //         // GET: "https://api.digitalocean.com/v2/droplets/$ID/actions/$ACTION_ID"
    //         self.url.push_str("/actions/");
    //         self.url.push_str(id);
    //         RequestBuilder::new(self.auth, self.url)
    //     }
    //     pub fn kernels(mut self) -> RequestBuilder<'t, response::Kernels> {
    //         // GET: "https://api.digitalocean.com/v2/droplets/$ID/kernels"
    //         self.url.push_str("/kernels");
    //         RequestBuilder::new(self.auth, self.url)
    //     }
    //     pub fn snapshots(mut self) -> RequestBuilder<'t, response::Snapshots> {
    //         // GET: "https://api.digitalocean.com/v2/droplets/$ID/snapshots"
    //         self.url.push_str("/snapshots");
    //         RequestBuilder::new(self.auth, self.url)
    //     }
    //     pub fn backups(mut self) -> RequestBuilder<'t, response::Backups> {
    //         // GET: "https://api.digitalocean.com/v2/droplets/$ID/backups"
    //         self.url.push_str("/backups");
    //         RequestBuilder::new(self.auth, self.url)
    //     }
    //     pub fn actions(mut self) -> RequestBuilder<'t, response::Actions> {
    //         // GET: "https://api.digitalocean.com/v2/droplets/$ID/actions"
    //         self.url.push_str("/actions");
    //         RequestBuilder::new(self.auth, self.url)
    //     }
    //     pub fn delete(self) -> RequestBuilder<'t, response::HeaderOnly> {
    //         // DELETE: "https://api.digitalocean.com/v2/droplets/$ID"
    //         RequestBuilder {
    //             method: Method::Delete,
    //             auth: self.auth,
    //             url: self.url,
    //             resp_t: PhantomData,
    //             body: None,
    //         }
    //     }
    //     pub fn show(&self) -> RequestBuilder<'t, response::Droplet> { unimplemented!() }
    //     pub fn neighbors(mut self) -> RequestBuilder<'t, response::Droplets> {
    //         // GET: "https://api.digitalocean.com/v2/droplets/$ID/neighbors"
    //         self.url.push_str("/neighbors");
    //         RequestBuilder::new(self.auth, self.url)
    //     }
}

// impl<'t> RequestBuilder<'t, response::Droplets> {
//     pub fn create(self, droplet: &Droplet) -> RequestBuilder<'t, response::Droplet> {
//         // POST:
//         // body:
//         //      "name" : ""             // true
//         //      "region" : ""           // true
//         //      "size" : ""             // true
//         //      "image" : ""            // true
//         //      "ssh_keys" : ""
//         //      "backups" : ""
//         //      "ipv6" : ""
//         //      "private_networking" : ""
//         //      "user_data" : ""

//         // FIXME: don't unwrap()

//         if droplet.image[..].parse::<u64>().is_ok() {
//             let d = DropletWithId::from_droplet(droplet);
//             return RequestBuilder {
//                 method: Method::Post,
//                 auth: self.auth,
//                 url: self.url,
//                 resp_t: PhantomData,
//                 body: Some(serde_json::to_string(&d).ok().unwrap()),
//             };
//         }

//         RequestBuilder {
//             method: Method::Post,
//             auth: self.auth,
//             url: self.url,
//             resp_t: PhantomData,
//             body: Some(serde_json::to_string(droplet).ok().unwrap()),
//         }

//     }
//     pub fn neighbors(self) -> RequestBuilder<'t, response::Neighbors> {
//         // GET: "https://api.digitalocean.com/v2/reports/droplet_neighbors"
//         RequestBuilder::new(self.auth,
//                             "https://api.digitalocean.com/v2/reports/droplet_neighbors")
//     }
//     pub fn upgrades(self) -> RequestBuilder<'t, response::ResponseStringArray> {
//         // GET: "https://api.digitalocean.com/v2/droplet_upgrades"
//         RequestBuilder::new(self.auth,
//                             "https://api.digitalocean.com/v2/droplet_upgrades")
//     }
// }

impl<'a, 't> DoRequest<response::Droplet> for RequestBuilder<'a, 't, response::Droplet> {}

#[cfg(test)]
mod tests;
