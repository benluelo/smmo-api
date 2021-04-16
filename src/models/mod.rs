pub mod item;
pub mod orphanage;
pub mod smmo_player;
pub mod world_boss;

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
