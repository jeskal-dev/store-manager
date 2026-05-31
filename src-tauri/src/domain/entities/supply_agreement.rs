use anyhow::Result;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::domain::value_objects::common::Money;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SupplyAgreement {
    pub id: Uuid,
    pub product_id: Uuid,
    pub supplier_id: Uuid,
    pub cost: Option<Money>,
    pub active: bool,
}

impl SupplyAgreement {
    pub fn new(
        product_id: Uuid,
        supplier_id: Uuid,
        cost: Option<Decimal>,
        active: bool,
    ) -> Result<Self> {
        let agreement = Self {
            id: Uuid::new_v4(),
            product_id,
            supplier_id,
            cost: match cost {
                Some(c) => Some(Money::new(c)?),
                None => None,
            },
            active,
        };
        Ok(agreement)
    }

    pub fn restore(
        id: Uuid,
        product_id: Uuid,
        supplier_id: Uuid,
        cost: Option<Decimal>,
        active: bool,
    ) -> Result<Self> {
        let agreement = Self {
            id,
            product_id,
            supplier_id,
            cost: match cost {
                Some(c) => Some(Money::new(c)?),
                None => None,
            },
            active,
        };
        Ok(agreement)
    }

    pub fn update_cost(&mut self, cost: Option<Decimal>) -> Result<()> {
        self.cost = match cost {
            Some(c) => Some(Money::new(c)?),
            None => None,
        };
        Ok(())
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn get_formatted_cost(&self) -> Option<String> {
        self.cost.as_ref().map(|c| c.formatted())
    }
}
