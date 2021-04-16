pub mod smmo_player;
pub mod world_boss;
mod orphanage;
mod item;

#[cfg(feature = "discord")]
use crate::models::world_boss::WorldBoss;
use serenity::builder::CreateEmbed;

pub trait SmmoModel {
    const TYPE_NAME: &'static str;
}

// impl<T: SmmoModel> SmmoModel for Vec<T> {
//     const TYPE_NAME: &'static str = "Vec";

//     fn to_embed<'a, 'b>(&'a self, embed: &'b mut CreateEmbed) -> &'b mut CreateEmbed {
//         embed.fields(self.into_iter().map(|t| t.to_field()))
//     }

//     fn to_field(&self) -> (String, String, bool) {
//         ("test".into(), "test".into(), true)
//     }
// }