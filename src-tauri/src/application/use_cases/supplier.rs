use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::supplier::Supplier;
use crate::domain::repositories::supplier::SupplierRepository;
use crate::domain::value_objects::common::{Code, Name, PhoneNumber, TextValue};

use crate::shared::criteria::{Criteria, PaginatedResult};

use super::super::dtos::shared::CriteriaInput;
use super::super::dtos::supplier::{CreateSupplierInput, UpdateSupplierInput};

#[async_trait]
pub trait ForSupplierUseCases {
    async fn create(&self, input: CreateSupplierInput) -> Result<Supplier>;
    async fn update(&self, id: Uuid, input: UpdateSupplierInput) -> Result<Supplier>;
    async fn delete(&self, id: Uuid) -> Result<()>;
    async fn search(&self, input: CriteriaInput) -> Result<PaginatedResult<Supplier>>;
}

pub struct ForSupplierInteractor<R: SupplierRepository> {
    repo: R,
}

impl<R: SupplierRepository> ForSupplierInteractor<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<R: SupplierRepository + Sync> ForSupplierUseCases for ForSupplierInteractor<R> {
    async fn create(&self, input: CreateSupplierInput) -> Result<Supplier> {
        if self.repo.exists_by_code(&input.supplier_code).await? {
            anyhow::bail!("A supplier with this code already exists");
        }

        let supplier = Supplier::new(
            input.name,
            input.supplier_code,
            input.contact_name,
            input.phone,
            input.address,
            input.active,
        )?;
        self.repo.create(&supplier).await?;
        Ok(supplier)
    }

    async fn update(&self, id: Uuid, input: UpdateSupplierInput) -> Result<Supplier> {
        let mut supplier = self
            .repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Supplier not found"))?;

        if let Some(ref supplier_code) = input.supplier_code {
            if *supplier_code != supplier.supplier_code.value() {
                if self.repo.exists_by_code(supplier_code).await? {
                    anyhow::bail!("A supplier with this code already exists");
                }
                supplier.supplier_code = Code::new(supplier_code.clone())?;
            }
        }
        if let Some(ref name) = input.name {
            supplier.name = Name::new(name.clone())?;
        }
        if let Some(ref contact_name) = input.contact_name {
            supplier.contact_name = match contact_name {
                Some(cn) => Some(TextValue::new(cn.clone(), 100)?),
                None => None,
            };
        }
        if let Some(ref phone) = input.phone {
            supplier.phone = match phone {
                Some(p) => Some(PhoneNumber::new(p.clone())?),
                None => None,
            };
        }
        if let Some(ref address) = input.address {
            supplier.address = match address {
                Some(addr) => Some(TextValue::new(addr.clone(), 200)?),
                None => None,
            };
        }
        if let Some(active) = input.active {
            if active {
                supplier.activate();
            } else {
                supplier.deactivate();
            }
        }

        self.repo.update(&supplier).await?;
        Ok(supplier)
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        self.repo.delete(id).await?;
        Ok(())
    }

    async fn search(&self, input: CriteriaInput) -> Result<PaginatedResult<Supplier>> {
        let criteria: Criteria<()> = input.try_into()?;
        self.repo.search(criteria.into()).await
    }
}
