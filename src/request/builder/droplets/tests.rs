use super::*;

use httpmock::Method::POST;
use httpmock::MockServer;
use url::Url;

use crate::tests::{BEARER_HEADER, TEST_TOKEN};
use crate::{DoManager, DoRequest};

#[test]
fn test_enable_backups() {
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/droplets/1234/actions")
            .header("Authorization", &BEARER_HEADER)
            .json_body(json!({"type":"enable_backups"}));

        then.status(200)
            .header("Content-Type", "application/json; charset=utf-8")
            .header("ratelimit-limit", "1200")
            .header("ratelimit-remaining", "774")
            .header("ratelimit-reset", "1415984218")
            .body(include_str!("test_enable_backups_response.json"));
    });

    let url = Url::parse(&server.base_url()).unwrap();

    let domgr = DoManager::builder()
        .token(TEST_TOKEN)
        .endpoint_url(url)
        .build();
    let action = domgr.droplet("1234").enable_backups().retrieve().unwrap();

    assert_eq!(action.id as u32, 36804745);
    assert_eq!(action.status, "in-progress");
    assert_eq!(action.action_type, "enable_backups");

    mock.assert();
}

#[test]
fn test_disable_backups() {
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/droplets/1234/actions")
            .header("Authorization", &BEARER_HEADER)
            .json_body(json!({"type":"disable_backups"}));

        then.status(200)
            .header("Content-Type", "application/json; charset=utf-8")
            .header("ratelimit-limit", "1200")
            .header("ratelimit-remaining", "774")
            .header("ratelimit-reset", "1415984218")
            .body(include_str!("test_disable_backups_response.json"));
    });

    let url = Url::parse(&server.base_url()).unwrap();

    let domgr = DoManager::builder()
        .token(TEST_TOKEN)
        .endpoint_url(url)
        .build();
    let action = domgr.droplet("1234").disable_backups().retrieve().unwrap();

    assert_eq!(action.id as u32, 36804745);
    assert_eq!(action.status, "in-progress");
    assert_eq!(action.action_type, "disable_backups");

    mock.assert();
}

#[test]
fn test_reboot() {
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/droplets/1234/actions")
            .header("Authorization", &BEARER_HEADER)
            .json_body(json!({"type":"reboot"}));

        then.status(200)
            .header("Content-Type", "application/json; charset=utf-8")
            .header("ratelimit-limit", "1200")
            .header("ratelimit-remaining", "774")
            .header("ratelimit-reset", "1415984218")
            .body(include_str!("test_reboot_response.json"));
    });

    let url = Url::parse(&server.base_url()).unwrap();

    let domgr = DoManager::builder()
        .token(TEST_TOKEN)
        .endpoint_url(url)
        .build();
    let action = domgr.droplet("1234").reboot().retrieve().unwrap();

    assert_eq!(action.id as u32, 36804748);
    assert_eq!(action.status, "in-progress");
    assert_eq!(action.action_type, "reboot");

    mock.assert();
}
