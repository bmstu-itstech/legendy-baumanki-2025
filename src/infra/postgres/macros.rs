#[macro_export]
macro_rules! with_client {
    ($pool:expr, $body:expr) => {{
        let obj = $pool
            .get()
            .await
            .map_err(|err| AppError::Internal(err.into()))?;
        let client: &Client = obj.client();
        $body(client).await
    }};
}

#[macro_export]
macro_rules! with_transaction {
    ($pool:expr, $body:expr) => {{
        let mut obj = $pool
            .get()
            .await
            .map_err(|err| AppError::Internal(err.into()))?;
        let tx = obj
            .transaction()
            .await
            .map_err(|err| AppError::Internal(err.into()))?;
        let res = $body(&tx).await?;
        tx.commit()
            .await
            .map_err(|err| AppError::Internal(err.into()))?;
        Ok(res)
    }};
}
