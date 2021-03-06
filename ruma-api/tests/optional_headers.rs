use ruma_api::ruma_api;

ruma_api! {
    metadata: {
        description: "Does something.",
        method: GET,
        name: "no_fields",
        path: "/_matrix/my/endpoint",
        rate_limited: false,
        authentication: None,
    }

    request: {
        #[ruma_api(header = LOCATION)]
        location: Option<String>,
    }
    response: {
        #[ruma_api(header = LOCATION)]
        stuff: Option<String>,
    }
}
