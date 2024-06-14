use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

#[derive(Display, EnumIter, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum PaymentState {
    Pending,
    Paid,
    Rejected
}
