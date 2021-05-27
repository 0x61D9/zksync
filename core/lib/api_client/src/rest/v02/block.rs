use crate::rest::client::{Client, Result};

use zksync_api_types::v02::{
    pagination::{IdOrLatest, PaginationQuery},
    Response,
};
use zksync_types::{tx::TxHash, BlockNumber};

impl Client {
    pub async fn block_by_number(&self, block_position: &str) -> Result<Response> {
        self.get_with_scope(super::API_V02_SCOPE, &format!("blocks/{}", block_position))
            .send()
            .await
    }

    pub async fn block_transactions(
        &self,
        pagination_query: &PaginationQuery<IdOrLatest<TxHash>>,
        block_position: &str,
    ) -> Result<Response> {
        self.get_with_scope(
            super::API_V02_SCOPE,
            &format!("blocks/{}/transactions", block_position),
        )
        .query(&pagination_query)
        .send()
        .await
    }

    pub async fn block_pagination(
        &self,
        pagination_query: &PaginationQuery<IdOrLatest<BlockNumber>>,
    ) -> Result<Response> {
        self.get_with_scope(super::API_V02_SCOPE, "blocks")
            .query(pagination_query)
            .send()
            .await
    }
}
