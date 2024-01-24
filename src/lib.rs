cargo_component_bindings::generate!();
use bindings::exports::typst_community::subsetter::types;
use subsetter;

pub struct Component;
impl bindings::Guest for Component {
    fn subset(
        data: Vec<u8>,
        index: u32,
        profile: bindings::exports::typst_community::subsetter::types::Profile,
    ) -> Result<Vec<u8>, types::Error> {
        todo!()
    }
}

pub struct Profile(subsetter::Profile<'static>);
impl types::GuestProfile for Profile {
    fn pdf(glyphs: Vec<u16>) -> types::OwnProfile {
        todo!()
    }
}
