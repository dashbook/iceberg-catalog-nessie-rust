use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use iceberg_rust::{
    catalog::{
        bucket::{parse_bucket, Bucket, ObjectStoreBuilder},
        identifier::Identifier,
        namespace::Namespace,
        tabular::{Tabular, TabularMetadata},
        Catalog,
    },
    error::Error as IcebergError,
    materialized_view::MaterializedView,
    table::Table,
    util::strip_prefix,
    view::View,
};
use object_store::ObjectStore;

use crate::{
    apis::{configuration, v1_api},
    models::{
        self, CommitMeta, Content, ContentKey, ContentV1, Operation, Operations,
        PutExpectedContent, ReferenceV1,
    },
};

#[derive(Debug)]
pub struct NessieCatalog {
    configuration: configuration::Configuration,
    builder: ObjectStoreBuilder,
    main_hash: Option<String>,
}

impl NessieCatalog {
    pub async fn new(
        configuration: configuration::Configuration,
        builder: ObjectStoreBuilder,
    ) -> Result<Self, IcebergError> {
        let main_hash = if let ReferenceV1::BranchV1 {
            name: _,
            metadata: _,
            hash,
        } = v1_api::get_reference_by_name(&configuration, "main", None).await?
        {
            Ok(hash)
        } else {
            Err(IcebergError::InvalidFormat(
                "Hash doesn't belong to a branch".to_owned(),
            ))
        }?;
        Ok(NessieCatalog {
            configuration,
            builder,
            main_hash,
        })
    }

    pub fn new_with_hash(
        configuration: configuration::Configuration,
        builder: ObjectStoreBuilder,
        main_hash: Option<String>,
    ) -> Self {
        NessieCatalog {
            configuration,
            builder,
            main_hash,
        }
    }
}

#[async_trait]
impl Catalog for NessieCatalog {
    /// Lists all tables in the given namespace.
    async fn list_tables(&self, namespace: &Namespace) -> Result<Vec<Identifier>, IcebergError> {
        let tables = v1_api::get_entries(
            &self.configuration,
            "main",
            Some(&format!("entry.namespace.startsWith('${namespace}')")),
            None,
            None,
            None,
            None,
        )
        .await?;
        tables
            .entries
            .into_iter()
            .filter(|entry| entry.r#type == Some("ICEBERG_TABLE".to_string()))
            .map(|entry| Identifier::try_new(&entry.name.elements))
            .collect::<Result<Vec<Identifier>, IcebergError>>()
    }

    /// Lists all namespaces in the catalog.
    async fn list_namespaces(&self, parent: Option<&str>) -> Result<Vec<Namespace>, IcebergError> {
        let namespaces = v1_api::get_namespaces(
            &self.configuration,
            "main",
            None,
            parent.map(|name| {
                models::Namespace::new(name.split(".").map(str::to_owned).collect(), HashMap::new())
            }),
        )
        .await?;
        namespaces
            .namespaces
            .first()
            .ok_or(IcebergError::NotFound(
                "Namespace".to_string(),
                "".to_string(),
            ))?
            .elements
            .iter()
            .map(|namespace| {
                Namespace::try_new(
                    &namespace
                        .split(".")
                        .map(str::to_owned)
                        .collect::<Vec<String>>(),
                )
            })
            .collect::<Result<Vec<Namespace>, IcebergError>>()
    }

    /// Check if a table exists
    async fn table_exists(&self, identifier: &Identifier) -> Result<bool, IcebergError> {
        v1_api::get_content(
            &self.configuration,
            &identifier.to_string(),
            None,
            Some("main"),
        )
        .await
        .map(|_| true)
        .map_err(IcebergError::from)
    }

    /// Drop a table and delete all data and metadata files.
    async fn drop_table(&self, identifier: &Identifier) -> Result<(), IcebergError> {
        v1_api::commit_multiple_operations(
            &self.configuration,
            "main",
            None,
            Some(Operations::new(
                CommitMeta::default(),
                vec![Operation::Delete {
                    key: Box::new(ContentKey::new(
                        identifier
                            .to_string()
                            .split(".")
                            .map(str::to_owned)
                            .collect(),
                    )),
                }],
            )),
        )
        .await
        .map(|_| ())
        .map_err(IcebergError::from)
    }

    /// Load a table.
    async fn load_table(self: Arc<Self>, identifier: &Identifier) -> Result<Tabular, IcebergError> {
        let path = v1_api::get_content(
            &self.configuration,
            &identifier.to_string(),
            None,
            Some("main"),
        )
        .await
        .map_err(IcebergError::from)
        .and_then(|x| match x {
            ContentV1::IcebergTableV1 {
                id: _id,
                metadata_location,
                snapshot_id: _snapshot_id,
                schema_id: _schema_id,
                spec_id: _spec_id,
                sort_order_id: _sort_order_id,
                metadata: _metadata,
            } => Ok(metadata_location),
            ContentV1::IcebergViewV1 {
                id: _id,
                metadata_location,
                version_id: _version_id,
                schema_id: _schema_id,
                sql_text: _sql_text,
                dialect: _dialect,
                metadata: _metadata,
            } => Ok(metadata_location),
            _ => Err(IcebergError::InvalidFormat("iceberg table".to_string())),
        })
        .map_err(IcebergError::from)?;
        let bucket = parse_bucket(&path)?;
        let object_store = self.builder.build(bucket)?;
        let bytes = object_store
            .get(&strip_prefix(&path).as_str().into())
            .await?
            .bytes()
            .await?;
        let metadata: TabularMetadata = serde_json::from_str(std::str::from_utf8(&bytes)?)?;
        let catalog: Arc<dyn Catalog> = self;
        match metadata {
            TabularMetadata::Table(metadata) => Ok(Tabular::Table(
                Table::new(
                    identifier.clone(),
                    Arc::clone(&catalog),
                    metadata,
                    &path.to_string(),
                )
                .await?,
            )),
            TabularMetadata::View(metadata) => Ok(Tabular::View(
                View::new(
                    identifier.clone(),
                    Arc::clone(&catalog),
                    metadata,
                    &path.to_string(),
                )
                .await?,
            )),
            TabularMetadata::MaterializedView(metadata) => Ok(Tabular::MaterializedView(
                MaterializedView::new(
                    identifier.clone(),
                    catalog.clone(),
                    metadata,
                    &path.to_string(),
                )
                .await?,
            )),
        }
    }

    /// Invalidate cached table metadata from current catalog.
    async fn invalidate_table(&self, _identifier: &Identifier) -> Result<(), IcebergError> {
        unimplemented!()
    }

    /// Register a table with the catalog if it doesn't exist.
    async fn register_table(
        self: Arc<Self>,
        identifier: Identifier,
        metadata_file_location: &str,
    ) -> Result<Tabular, IcebergError> {
        v1_api::commit_multiple_operations(
            &self.configuration,
            "main",
            self.main_hash.as_deref(),
            Some(Operations::new(
                CommitMeta::default(),
                vec![Operation::Put {
                    key: Box::new(ContentKey::new(
                        identifier
                            .to_string()
                            .split(".")
                            .map(str::to_owned)
                            .collect(),
                    )),
                    content: Box::new(Content::IcebergTable {
                        id: None,
                        metadata_location: metadata_file_location.to_string(),
                        snapshot_id: Some(1),
                        schema_id: Some(1),
                        spec_id: Some(1),
                        sort_order_id: Some(1),
                        metadata: None,
                    }),
                    expected_content: None,
                    metadata: None,
                    documentation: None,
                }],
            )),
        )
        .await
        .map(|_| ())
        .map_err(IcebergError::from)?;
        self.load_table(&identifier).await
    }

    /// Update a table by atomically changing the pointer to the metadata file
    async fn update_table(
        self: Arc<Self>,
        identifier: Identifier,
        metadata_file_location: &str,
        previous_metadata_file_location: &str,
    ) -> Result<Tabular, IcebergError> {
        v1_api::commit_multiple_operations(
            &self.configuration,
            "main",
            None,
            Some(Operations::new(
                CommitMeta::default(),
                vec![Operation::Put {
                    key: Box::new(ContentKey::new(
                        identifier
                            .to_string()
                            .split(".")
                            .map(str::to_owned)
                            .collect(),
                    )),
                    content: Box::new(Content::IcebergTable {
                        id: None,
                        metadata_location: metadata_file_location.to_string(),
                        snapshot_id: None,
                        schema_id: None,
                        spec_id: None,
                        sort_order_id: None,
                        metadata: None,
                    }),
                    expected_content: Some(Box::new(PutExpectedContent {
                        metadata_location: previous_metadata_file_location.to_string(),
                        id: None,
                        snapshot_id: None,
                        schema_id: None,
                        spec_id: None,
                        sort_order_id: None,
                        metadata: None,
                        metadata_location_history: vec![],
                        checkpoint_location_history: vec![],
                        last_checkpoint: None,
                        version_id: None,
                        sql_text: "".to_string(),
                        dialect: None,
                        elements: vec![],
                        properties: HashMap::new(),
                    })),
                    metadata: None,
                    documentation: None,
                }],
            )),
        )
        .await
        .map(|_| ())?;
        self.load_table(&identifier).await
    }

    /// Initialize a catalog given a custom name and a map of catalog properties.
    /// A custom Catalog implementation must have a no-arg constructor. A compute engine like Spark
    /// or Flink will first initialize the catalog without any arguments, and then call this method to
    /// complete catalog initialization with properties passed into the engine.
    async fn initialize(
        self: Arc<Self>,
        _properties: &HashMap<String, String>,
    ) -> Result<(), IcebergError> {
        unimplemented!()
    }

    /// Return the associated object store to the catalog
    fn object_store(&self, bucket: Bucket) -> Arc<dyn ObjectStore> {
        self.builder.build(bucket).unwrap()
    }
}

#[cfg(test)]
pub mod tests {
    use std::{collections::HashMap, sync::Arc};

    use iceberg_rust::{
        catalog::{identifier::Identifier, Catalog},
        spec::{
            schema::Schema,
            types::{PrimitiveType, StructField, StructTypeBuilder, Type},
        },
        table::table_builder::TableBuilder,
    };
    use object_store::memory::InMemory;

    use crate::{
        apis::{configuration::Configuration, v1_api},
        catalog::{NessieCatalog, ObjectStoreBuilder},
        models::Namespace,
    };

    fn configuration() -> Configuration {
        Configuration {
            base_path: "http://localhost:19120/api".to_string(),
            user_agent: None,
            client: reqwest::Client::new(),
            basic_auth: None,
            oauth_access_token: None,
            bearer_access_token: None,
            api_key: None,
        }
    }

    #[tokio::test]
    async fn test_create_update_drop_table() {
        let configuration = configuration();

        let _ = dbg!(
            v1_api::create_namespace(
                &configuration,
                Namespace {
                    id: None,
                    elements: vec!["load_table".to_owned()],
                    properties: HashMap::new(),
                },
                "main",
                None,
                Some(Namespace {
                    id: None,
                    elements: Vec::new(),
                    properties: HashMap::new(),
                }),
            )
            .await
        )
        .is_ok();

        let catalog: Arc<dyn Catalog> = Arc::new(
            NessieCatalog::new(
                configuration,
                ObjectStoreBuilder::Memory(Arc::new(InMemory::new())),
            )
            .await
            .expect("Failed to create catalog"),
        );
        let identifier = Identifier::parse("load_table.table3").unwrap();
        let schema = Schema {
            schema_id: 1,
            identifier_field_ids: Some(vec![1, 2]),
            fields: StructTypeBuilder::default()
                .with_struct_field(StructField {
                    id: 1,
                    name: "one".to_string(),
                    required: false,
                    field_type: Type::Primitive(PrimitiveType::String),
                    doc: None,
                })
                .with_struct_field(StructField {
                    id: 2,
                    name: "two".to_string(),
                    required: false,
                    field_type: Type::Primitive(PrimitiveType::String),
                    doc: None,
                })
                .build()
                .unwrap(),
        };
        let mut builder = TableBuilder::new(identifier.clone(), catalog.clone()).unwrap();
        builder
            .location("/")
            .with_schema((1, schema))
            .current_schema_id(1);
        let mut table = builder.build().await.expect("Failed to create table.");

        let exists = Arc::clone(&catalog)
            .table_exists(&identifier)
            .await
            .expect("Table doesn't exist");
        assert_eq!(exists, true);

        let metadata_location = table.metadata_location().to_string();

        let transaction = table.new_transaction(None);
        transaction.commit().await.expect("Transaction failed.");

        let new_metadata_location = table.metadata_location().to_string();

        assert_ne!(metadata_location, new_metadata_location);

        let _ = catalog
            .drop_table(&identifier)
            .await
            .expect("Failed to drop table.");

        Arc::clone(&catalog)
            .table_exists(&identifier)
            .await
            .expect_err("Table still exists");
    }
}
