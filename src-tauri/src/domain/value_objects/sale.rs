use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Debug, Clone, Serialize, Deserialize, Display)]
pub enum PaymentMethod {
    #[strum(serialize = "cash")]
    Cash,
    #[strum(serialize = "credit_card")]
    CreditCard,
    #[strum(serialize = "bank_transfer")]
    BankTransfer,
    #[strum(serialize = "other")]
    Other,
}
