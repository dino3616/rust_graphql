use crate::schemas::{
    root::Context,
    user::User,
};
use chrono::NaiveDateTime;
use juniper::{
    graphql_object,
    ID,
};

// オブジェクト型のリゾルバは木構造で言う「葉」になるので、変な処理は入れずに大体簡単なものでいい.
#[graphql_object(context=Context)]
impl User {
    fn id(&self) -> ID {
        ID::new(self.id.to_string())
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn profile(&self) -> String {
        self.profile.clone()
    }

    // NaiveDateTimeは組み込みスカラー型なので、そのまま返り値にしておｋ.
    fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    fn updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }
}
