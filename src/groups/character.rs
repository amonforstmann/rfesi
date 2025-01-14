use crate::prelude::*;

/// Endpoints for Character
pub struct CharacterGroup<'a> {
    pub(crate) esi: &'a Esi,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct CharacterPublicInfo {
    pub alliance_id: Option<u64>,
    pub birthday: String,
    pub bloodline_id: u64,
    pub corporation_id: u64,
    pub description: Option<String>,
    pub gender: String,
    pub name: String,
    pub race_id: u16,
    pub security_status: Option<f64>,
    pub title: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct CharacterCorporationHistoryItem {
    pub corporation_id: u64,
    pub is_deleted: Option<bool>,
    pub record_id: u64,
    pub start_date: String,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct CharacterPortraitInfo {
    pub px128x128: Option<String>,
    pub px256x256: Option<String>,
    pub px512x512: Option<String>,
    pub px64x64: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct CharacterAffiliation {
    pub alliance_id: Option<u64>,
    pub character_id: u64,
    pub corporation_id: u64,
    pub faction_id: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct Blueprint {
    pub item_id: u64,
    pub location_flag: String,
    pub location_id: u64,
    pub material_efficiency: u8,
    pub quantity: i32,
    pub runs: i64,
    pub time_efficiency: u8,
    pub type_id: u64,
}

impl<'a> CharacterGroup<'a> {
    api_get!(
        /// Get a character's public information.
        get_public_info,
        "get_characters_character_id",
        RequestType::Public,
        CharacterPublicInfo,
        (character_id: u64) => "{character_id}"
    );

    api_get!(
        /// Get a character's corporation history.
        get_history,
        "get_characters_character_id_corporationhistory",
        RequestType::Public,
        Vec<CharacterCorporationHistoryItem>,
        (character_id: u64) => "{character_id}"
    );

    api_get!(
        /// Get a character's portrait URLs on the image server.
        get_portrait,
        "get_characters_character_id_portrait",
        RequestType::Public,
        CharacterPortraitInfo,
        (character_id: u64) => "{character_id}"
    );

    api_post!(
        /// Get character affiliations.
        get_affiliation,
        "post_characters_affiliation",
        RequestType::Public,
        Vec<CharacterAffiliation>,
        ,
        character_ids: &[u64],
    );

    api_get!(
        /// Get character blueprints.
        get_blueprints,
        "get_characters_character_id_blueprints",
        RequestType::Authenticated,
        Vec<Blueprint>,
        (character_id: u64) => "{character_id}"
    );

    // more endpoints ...
}
