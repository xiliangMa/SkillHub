use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id).uuid().primary_key().default(Expr::cust("gen_random_uuid()")))
                    .col(ColumnDef::new(User::Email).string().not_null().unique_key())
                    .col(ColumnDef::new(User::PasswordHash).string())
                    .col(ColumnDef::new(User::Name).string())
                    .col(ColumnDef::new(User::AvatarUrl).string())
                    .col(ColumnDef::new(User::Provider).string().not_null().default("email"))
                    .col(ColumnDef::new(User::ProviderId).string())
                    .col(ColumnDef::new(User::Role).string().not_null().default("user"))
                    .col(ColumnDef::new(User::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(User::UpdatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Skill::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Skill::Id).uuid().primary_key().default(Expr::cust("gen_random_uuid()")))
                    .col(ColumnDef::new(Skill::GithubOwner).string().not_null())
                    .col(ColumnDef::new(Skill::GithubRepo).string().not_null())
                    .col(ColumnDef::new(Skill::Name).string().not_null())
                    .col(ColumnDef::new(Skill::Description).text())
                    .col(ColumnDef::new(Skill::SkillContent).text())
                    .col(ColumnDef::new(Skill::ReadmeContent).text())
                    .col(ColumnDef::new(Skill::Stars).integer().not_null().default(0))
                    .col(ColumnDef::new(Skill::Forks).integer().not_null().default(0))
                    .col(ColumnDef::new(Skill::Language).string())
                    .col(ColumnDef::new(Skill::Tags).json())
                    .col(ColumnDef::new(Skill::InstallCommand).text())
                    .col(ColumnDef::new(Skill::Price).decimal(10, 2).not_null().default(0))
                    .col(ColumnDef::new(Skill::Marketplace).boolean().not_null().default(false))
                    .col(ColumnDef::new(Skill::DownloadedCount).integer().not_null().default(0))
                    .col(ColumnDef::new(Skill::LastSyncedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Skill::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Skill::UpdatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .unique_key([Skill::GithubOwner, Skill::GithubRepo])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Favorite::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Favorite::Id).uuid().primary_key().default(Expr::cust("gen_random_uuid()")))
                    .col(ColumnDef::new(Favorite::UserId).uuid().not_null())
                    .col(ColumnDef::new(Favorite::SkillId).uuid().not_null())
                    .col(ColumnDef::new(Favorite::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .foreign_key(ForeignKey::create().name("fk_favorite_user").from(Favorite::Table, Favorite::UserId).to(User::Table, User::Id).on_delete(ForeignKeyAction::Cascade))
                    .foreign_key(ForeignKey::create().name("fk_favorite_skill").from(Favorite::Table, Favorite::SkillId).to(Skill::Table, Skill::Id).on_delete(ForeignKeyAction::Cascade))
                    .unique_key([Favorite::UserId, Favorite::SkillId])
                    .to_owned(),
            )
            .await?;

        manager.create_index(IndexCreateStatement::new().table(Skill::Table).name("idx_skills_stars").col(Skill::Stars).order(sea_orm::sea_strum::IntoStaticStr::into_static(Order::Desc)).to_owned()).await?;
        manager.create_index(IndexCreateStatement::new().table(Skill::Table).name("idx_skills_updated").col(Skill::UpdatedAt).to_owned()).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Favorite::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Skill::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(User::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(Iden)]
pub enum User { Table, Id, Email, PasswordHash, Name, AvatarUrl, Provider, ProviderId, Role, CreatedAt, UpdatedAt }

#[derive(Iden)]
pub enum Skill { Table, Id, GithubOwner, GithubRepo, Name, Description, SkillContent, ReadmeContent, Stars, Forks, Language, Tags, InstallCommand, Price, Marketplace, DownloadedCount, LastSyncedAt, CreatedAt, UpdatedAt }

#[derive(Iden)]
pub enum Favorite { Table, Id, UserId, SkillId, CreatedAt }
