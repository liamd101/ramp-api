use crate::data::{ReimbursementRow, Transaction};
use anyhow::Result;
use odbc_api::{ConnectionOptions, Environment, IntoParameter};

pub fn insert_transaction_server(
    database_info: &crate::config::Database,
    data: Vec<Transaction>,
) -> Result<()> {
    let env = Environment::new()?;

    println!("{:?}", data[0].trx_date);

    let connection_string = format!(
        "
        Driver={};\
        Server={};\
        UID={};\
        PWD={};\
        DATABASE={};\
        ",
        database_info.driver,
        database_info.server,
        database_info.user.id,
        database_info.user.password,
        database_info.database
    );

    let conn =
        env.connect_with_connection_string(&connection_string, ConnectionOptions::default())?;

    for entry in data {
        conn.execute(
            "
INSERT INTO [dbo].[_DSI_RampTrx_tbl] (
    Id, Card_Id, Trx_Date, First_Name, Last_Name, Location_Name, Department_Name,
    Amount, Merchant_Descriptor, Merchant_Name, Merchant_Category_Code_Description,
    Acct_Category_Id, Memo
)
VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ",
            (
                &entry.id.into_parameter(),
                &entry.card_id.into_parameter(),
                &entry.trx_date.into_parameter(),
                &entry.first_name.into_parameter(),
                &entry.last_name.into_parameter(),
                &entry.location_name.into_parameter(),
                &entry.department_name.into_parameter(),
                &entry.amount.into_parameter(),
                &entry.merchant_descriptor.into_parameter(),
                &entry.merchant_name.into_parameter(),
                &entry.merchant_category_code_description.into_parameter(),
                &entry.acct_category_id.into_parameter(),
                &entry.memo.into_parameter(),
            ),
        )?;
    }

    Ok(())
}

pub fn insert_reimbursement_server(
    database_info: &crate::config::Database,
    data: Vec<ReimbursementRow>,
) -> Result<()> {
    let env = Environment::new()?;

    let connection_string = format!(
        "
        Driver={};\
        Server={};\
        UID={};\
        PWD={};\
        DATABASE={};\
        ",
        database_info.driver,
        database_info.server,
        database_info.user.id,
        database_info.user.password,
        database_info.database
    );

    let conn =
        env.connect_with_connection_string(&connection_string, ConnectionOptions::default())?;

    for entry in data {
        conn.execute(
            "
INSERT INTO [dbo].[_DSI_RampExpTrx_tbl] (
    Id, State, Trx_Date, User_Full_Name, Amount, Distance, Merchant_Name,
    Merchant_Category_Code_Description, Acct_Category_Id, Type, Memo
)
VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ",
            (
                &entry.id.into_parameter(),
                &entry.state.into_parameter(),
                &entry.trx_date.into_parameter(),
                &entry.user_full_name.into_parameter(),
                &entry.amount.into_parameter(),
                &entry.distance.into_parameter(),
                &entry.merchant.into_parameter(),
                &entry.name.into_parameter(),
                &entry.external_code.into_parameter(),
                &entry.reimb_type.into_parameter(),
                &entry.memo.into_parameter(),
            ),
        )?;
    }

    Ok(())
}
