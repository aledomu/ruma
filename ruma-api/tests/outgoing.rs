use ruma_api::Outgoing;
use ruma_identifiers::UserId;

#[allow(unused)]
pub struct Thing<'t, T> {
    some: &'t str,
    t: &'t T,
}

#[derive(Debug)]
pub struct IncomingThing<T> {
    some: String,
    t: T,
}

#[derive(Outgoing)]
#[incoming_no_deserialize]
pub struct Request<'a, T> {
    pub abc: &'a str,
    pub thing: Thing<'a, T>,
    pub device_id: &'a ::ruma_identifiers::DeviceId,
    pub user_id: &'a UserId,
    pub bytes: &'a [u8],
    pub recursive: &'a [Thing<'a, T>],
}