use crate::{FromDbString, ToDbString};
use crate::schema::user_settings;
use crate::{DbResult, FarmDB};
use diesel::prelude::*;
use diesel::{QueryDsl, Queryable, Selectable, SelectableHelper};
use paste::paste;

#[derive(Identifiable, Queryable, Selectable)]
#[diesel(belongs_to(User))]
#[diesel(table_name = user_settings)]
pub struct UserSetting {
    pub id: i32,
    pub user_id: i32,
    #[diesel(column_name = "setting_name")]
    pub name: String,
    #[diesel(column_name = "setting_value")]
    pub value: Option<String>,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = user_settings)]
pub struct SetUserSetting {
    pub user_id: i32,
    #[diesel(column_name = "setting_name")]
    pub name: String,
    #[diesel(column_name = "setting_value")]
    pub value: Option<String>,
}

pub async fn settings_for_user(db: &FarmDB, user_id: i32) -> DbResult<Vec<UserSetting>> {
    let settings = db
        .run(move |conn| {
            user_settings::table
                .filter(user_settings::user_id.eq(user_id))
                .select(UserSetting::as_select())
                .load::<UserSetting>(conn)
        })
        .await?;
    Ok(settings)
}

macro_rules! typed_setting {
    ($n:ident, $t:ty) => {
        paste! {
        pub async fn [<get_ $n>](db: &FarmDB, user_id: i32, name: &str) -> DbResult<Option<$t>> {
            let name = name.to_string();
            db.run(move |conn| {
                let setting: Option<String> = user_settings::table
                    .filter(user_settings::user_id.eq(user_id))
                    .filter(user_settings::setting_name.eq(name))
                    .select(user_settings::setting_value)
                    .first::<Option<String>>(conn)
                    .optional()?
                    .flatten()
                    .map(|v| if v.is_empty() { None } else { Some(v) })
                    .flatten();
                let setting = if let Some(s) = setting {
                    Some(FromDbString::from_str(s)?)
                } else {
                    None
                };
                Ok(setting)
            }).await
        }


        pub async fn [<set_ $n>] (db: &FarmDB, user_id: i32, name: &str, value: Option<$t>,) -> DbResult<()> {
            let value = if let Some(value) = value {
                Some(value.to_db_string()?)
            } else {
                None
            };
            let setting = SetUserSetting {
                user_id,
                name: name.to_string(),
                value,
            };
            db.run(move |conn| {
                diesel::insert_into(user_settings::table)
                    .values(&setting)
                    .on_conflict((user_settings::user_id, user_settings::setting_name))
                    .do_update()
                    .set(&setting)
                    .execute(conn)?;
                Ok(())
            })
            .await
        }
            }
    };
}

typed_setting!(bool_user_setting, bool);
typed_setting!(i32_user_setting, i32);
typed_setting!(f32_user_setting, f32);
typed_setting!(string_user_setting, String);
