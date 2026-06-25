use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::store::Store;
use crate::domain::repositories::store::StoreRepository;
use crate::domain::value_objects::common::{Code, Name, PhoneNumber, TextValue};

use crate::shared::criteria::{Criteria, PaginatedResult};

use super::super::dtos::shared::CriteriaInput;
use super::super::dtos::store::{CreateStoreInput, StoreOutput, UpdateStoreInput};

#[async_trait]
pub trait ForStoreUseCases {
    async fn create(&self, input: CreateStoreInput) -> Result<StoreOutput>;
    async fn update(&self, id: Uuid, input: UpdateStoreInput) -> Result<StoreOutput>;
    async fn delete(&self, id: Uuid) -> Result<()>;
    async fn search(&self, input: CriteriaInput) -> Result<PaginatedResult<StoreOutput>>;
}

pub struct ForStoreInteractor<R: StoreRepository> {
    repo: R,
}

impl<R: StoreRepository> ForStoreInteractor<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<R: StoreRepository + Sync> ForStoreUseCases for ForStoreInteractor<R> {
    async fn create(&self, input: CreateStoreInput) -> Result<StoreOutput> {
        if self.repo.exists_by_code(&input.store_code).await? {
            anyhow::bail!("A store with this code already exists");
        }

        let store = Store::new(
            input.store_code,
            input.name,
            input.address,
            input.phone,
            input.active,
        )?;
        self.repo.create(&store).await?;
        Ok(StoreOutput::from(&store))
    }

    async fn update(&self, id: Uuid, input: UpdateStoreInput) -> Result<StoreOutput> {
        let mut store = self
            .repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Store not found"))?;

        if let Some(ref store_code) = input.store_code {
            if *store_code != store.store_code.value() {
                if self.repo.exists_by_code(store_code).await? {
                    anyhow::bail!("A store with this code already exists");
                }
                store.store_code = Code::new(store_code.clone())?;
            }
        }
        if let Some(ref name) = input.name {
            store.name = Name::new(name.clone())?;
        }
        if let Some(ref address) = input.address {
            store.address = TextValue::new(address.clone(), 200)?;
        }
        if let Some(ref phone) = input.phone {
            store.phone = match phone {
                Some(p) => Some(PhoneNumber::new(p.clone())?),
                None => None,
            };
        }
        if let Some(active) = input.active {
            if active {
                store.activate();
            } else {
                store.deactivate();
            }
        }

        self.repo.update(&store).await?;
        Ok(StoreOutput::from(&store))
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        self.repo.delete(id).await?;
        Ok(())
    }

    async fn search(&self, input: CriteriaInput) -> Result<PaginatedResult<StoreOutput>> {
        let criteria: Criteria<()> = input.try_into()?;
        let result = self.repo.search(criteria.into()).await?;
        Ok(PaginatedResult {
            data: result.data.into_iter().map(|s| StoreOutput::from(&s)).collect(),
            meta: result.meta,
        })
    }
}
