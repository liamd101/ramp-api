use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct DataEntry {
    pub id: Option<String>,
    pub card_id: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub location_name: Option<String>,
    pub department_name: Option<String>,
    pub amount: Option<f64>,
    pub merchant_descriptor: Option<String>,
    pub merchant_name: Option<String>,
    pub merchant_category_code_description: Option<String>,
    pub acct_category_id: Option<String>,
    pub memo: Option<String>,
}

#[derive(Deserialize, Debug, Default)]
struct CardHolder {
    first_name: Option<String>,
    last_name: Option<String>,
    location_name: Option<String>,
    department_name: Option<String>,
}

#[derive(Deserialize, Debug, Default)]
struct AccountingCategory {
    category_id: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct Root {
    id: Option<String>,
    card_id: Option<String>,
    amount: Option<f64>,
    merchant_descriptor: Option<String>,
    merchant_name: Option<String>,
    merchant_category_code_description: Option<String>,
    memo: Option<String>,
    card_holder: CardHolder,
    accounting_categories: Vec<AccountingCategory>,
}

impl From<Root> for DataEntry {
    fn from(root: Root) -> Self {
        DataEntry {
            id: root.id,
            card_id: root.card_id,
            first_name: root.card_holder.first_name,
            last_name: root.card_holder.last_name,
            location_name: root.card_holder.location_name,
            department_name: root.card_holder.department_name,
            amount: root.amount,
            merchant_descriptor: root.merchant_descriptor,
            merchant_name: root.merchant_name,
            merchant_category_code_description: root.merchant_category_code_description,
            acct_category_id: root
                .accounting_categories
                .first()
                .map(|x| x.category_id.clone()),
            memo: root.memo,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Response {
    pub data: Vec<Root>,
}
