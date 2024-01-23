cargo_component_bindings::generate!();
use subsetter;

struct Component;
impl bindings::Guest for Component {
    fn subset(
        data: Vec<u8>,
        index: u32,
        profile: bindings::Profile,
    ) -> Result<Vec<u8>, bindings::Error> {
        subsetter::subset(data.as_slice(), index, profile.into()).map_err(|x| x.into())
    }
}

impl From<bindings::Error> for subsetter::Error {
    fn from(value: bindings::Error) -> Self {
        match value {
            bindings::Error::UnknownKind => Self::UnknownKind,
            bindings::Error::InvalidOffset => Self::InvalidOffset,
            bindings::Error::MissingData => Self::MissingData,
            bindings::Error::InvalidData => Self::InvalidData,
            bindings::Error::MissingTable(x) => Self::MissingTable(x.into()),
        }
    }
}
impl From<subsetter::Error> for bindings::Error {
    fn from(value: subsetter::Error) -> Self {
        match value {
            subsetter::Error::UnknownKind => Self::UnknownKind,
            subsetter::Error::InvalidOffset => Self::InvalidOffset,
            subsetter::Error::MissingData => Self::MissingData,
            subsetter::Error::InvalidData => Self::InvalidData,
            subsetter::Error::MissingTable(x) => Self::MissingTable(x.into()),
        }
    }
}

impl From<bindings::exports::typst_community::subsetter::types::Tag> for subsetter::Tag {
    fn from(value: bindings::exports::typst_community::subsetter::types::Tag) -> Self {
        subsetter::Tag(value.tuple0.into())
    }
}
impl From<subsetter::Tag> for bindings::exports::typst_community::subsetter::types::Tag {
    fn from(value: subsetter::Tag) -> Self {
        bindings::exports::typst_community::subsetter::types::Tag {
            tuple0: (value.0.into()),
        }
    }
}

struct Profile(subsetter::Profile<'static>);
impl bindings::exports::typst_community::subsetter::types::GuestProfile for Profile {
    fn pdf(glyphs: Vec<u16>) -> bindings::exports::typst_community::subsetter::types::OwnProfile {
        bindings::exports::typst_community::subsetter::types::OwnProfile::new(
            subsetter::Profile::pdf(glyphs.as_slice()).into(),
        )
    }
}

impl From<Profile> for subsetter::Profile<'static> {
    fn from(value: Profile) -> Self {
        value.0
    }
}
impl From<subsetter::Profile<'static>> for Profile {
    fn from(value: subsetter::Profile) -> Self {
        Self(value)
    }
}
