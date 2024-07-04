use crate::{SharedState, ApiError};
use csv_async::AsyncDeserializer;
use futures::stream::StreamExt;
use tokio::sync::mpsc;

#[derive(Debug, serde::Deserialize, Clone)]
pub struct Record {
    year: String,
    industry_aggregation_nz: String,
    industry_code_nz: String,
    industry_name_nz: String,
    units: String,
    variable_code: String,
    variable_name: String,
    variable_category: String,
    value: String,
    industry_code_anz: String,
}

pub async fn process_csv(state: SharedState, data: String) -> Result<(), ApiError> {
    let batch_size = 10_000;
    let (tx, mut rx) = mpsc::channel(batch_size);

    tokio::spawn(async move {
        let rdr = AsyncDeserializer::from_reader(data.as_bytes());

        let mut records = rdr.into_deserialize::<Record>();

        while let Some(record) = records.next().await {
            let _ = tx.send(record).await;
        }
    });

    let mut batched_records: Vec<Record> = vec![];

    while let Some(message) = rx.recv().await {
        batched_records.push(message?);

        if batched_records.len() >= batch_size {
            upload_data_db(state.clone(), batched_records.clone()).await?;
            batched_records.clear();
        }
    }

    if !batched_records.is_empty() {
        upload_data_db(state.clone(), batched_records).await?;
    }

    Ok(())
}

pub async fn upload_data_db(state: SharedState, batched_records: Vec<Record>) -> Result<(), ApiError> {
    let mut database_tx = state.pool.begin().await?;

    for record in batched_records {
        sqlx::query(
            r#"
            insert into financial_year_data
            (year, industry_aggregation_nz, 
            industry_code_nz, industry_name_nz, 
            units, variable_code, variable_name, 
            variable_category, value,
            industry_code_anz) 
            values(?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        )
        .bind(record.year)
        .bind(record.industry_aggregation_nz)
        .bind(record.industry_code_nz)
        .bind(record.industry_name_nz)
        .bind(record.units)
        .bind(record.variable_code)
        .bind(record.variable_name)
        .bind(record.variable_category)
        .bind(record.value)
        .bind(record.industry_code_anz)
        .execute(&mut *database_tx)
        .await?;
    }

    database_tx.commit().await?;

    Ok(())
}
