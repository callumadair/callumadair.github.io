use core::time::Duration;

use entity::{
    image::{
        ActiveModel as ImageActiveModel,
        ImageEntity,
        ImageModel,
    },
    software::{
        SoftwareActiveModel,
        SoftwareEntity,
        SoftwareModel,
    },
};
use migration::{
    Migrator,
    MigratorTrait,
};
use sea_orm::{
    ActiveModelTrait,
    ConnectOptions,
    Database,
    DatabaseConnection,
    EntityTrait,
};

use super::types::{
    image::{
        CreateImageRequest,
        DeleteImageRequest,
    },
    software::{
        CreateSoftwareRequest,
        DeleteSoftwareRequest,
    },
};
use crate::{
    error::Result,
    repositories::traits::SoftwareRepository,
};

/// Exists for GET requests to query current app state.
#[derive(Debug, Clone)]
pub struct SeaOrmDataBaseConnection
{
    connection: DatabaseConnection,
}

impl SeaOrmDataBaseConnection
{
    pub async fn new(path: &str) -> Result<Self>
    {
        let mut opt = ConnectOptions::new(path);
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(10))
            .idle_timeout(Duration::from_secs(10))
            .max_lifetime(Duration::from_secs(30))
            .sqlx_logging(true);
        let db_conn: DatabaseConnection = Database::connect(opt).await?;
        // TODO(CA): This preseeds data for dev, lock this behind a
        // feature flag.
        Migrator::up(&db_conn, None).await?;

        Ok(Self {
            connection: db_conn,
        })
    }
}

impl SoftwareRepository for SeaOrmDataBaseConnection
{
    async fn create_image(
        &self,
        req: &CreateImageRequest,
    ) -> Result<ImageModel>
    {
        let new_entry = ImageActiveModel::builder()
            .software_tool_id(req.software_id().clone())
            .image_url(req.image_url().clone())
            .build();
        let entry: ImageModel = new_entry.insert(&self.connection).await?;

        Ok(entry)
    }

    async fn create_software(
        &self,
        req: &CreateSoftwareRequest,
    ) -> Result<SoftwareModel>
    {
        let new_entry = SoftwareActiveModel::builder()
            .name(req.name().clone())
            .short_desc(req.short_desc().clone())
            .long_desc(req.long_desc().clone())
            .web_link(req.web_link().clone())
            .build();
        let entry: SoftwareModel = new_entry.insert(&self.connection).await?;

        // Now that we have successfully created the software tool,
        // insert all the images information.
        // Prep all the images to be stored.
        for image_link in req.image_urls()
        {
            let create_image_req = CreateImageRequest::builder()
                .software_id(*entry.id())
                .image_url(&image_link.to_string())?
                .build();
            self.create_image(&create_image_req).await?;
        }

        Ok(entry)
    }

    async fn delete_image(
        &self,
        req: &DeleteImageRequest,
    ) -> Result<()>
    {
        let res = ImageEntity::delete_by_id(req.image_id().as_int())
            .exec(&self.connection)
            .await;

        match res
        {
            Ok(inner) =>
            {
                tracing::info!(
                    "Successfully deleted {} rows in the images table.",
                    inner.rows_affected
                );
                Ok(())
            }

            Err(db_err) =>
            {
                tracing::error!(
                    "Error encountered when trying to delete image database record {}",
                    db_err
                );
                Err(db_err.into())
            }
        }
    }

    async fn delete_software(
        &self,
        req: &DeleteSoftwareRequest,
    ) -> Result<()>
    {
        let res = SoftwareEntity::delete_by_id(req.id().as_int())
            .exec(&self.connection)
            .await;

        match res
        {
            Ok(inner) =>
            {
                tracing::info!(
                    "Successfully deleted {} rows in the software tools table.",
                    inner.rows_affected
                );
                Ok(())
            }
            Err(db_err) =>
            {
                tracing::error!(
                    "Error encountered when trying to delete software tools record: {}.",
                    db_err
                );
                Err(db_err.into())
            }
        }
    }

    async fn get_all_software(&self) -> Result<Vec<SoftwareModel>>
    {
        let software_entries: Vec<SoftwareModel> =
            SoftwareEntity::find().all(&self.connection).await?;
        Ok(software_entries)
    }

    async fn get_all_software_tools(&self) -> Result<Vec<shared::software::SoftwareTool>>
    {
        let software_entries = self.get_all_software().await?;

        let mut software_tools = Vec::with_capacity(software_entries.len());
        for entry in software_entries
        {
            software_tools.push(entry.to_software_tool(&self.connection).await?);
        }
        Ok(software_tools)
    }
}
