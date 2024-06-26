use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

#[derive(Display, EnumIter, Clone, Debug, Deserialize, Serialize)]
pub enum PaymentMethod {
    Card,
    QrCode,
}
