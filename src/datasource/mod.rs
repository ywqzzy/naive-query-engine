/*
 * @Author: Veeupup
 * @Date: 2022-05-12 16:08:23
 * @Email: code@tanweime.com
*/

mod csv;
mod empty;
mod memory;

use crate::error::Result;
use arrow::{datatypes::SchemaRef, record_batch::RecordBatch};

pub trait TableSource {
    fn schema(&self) -> SchemaRef;

    // TODO(veeupup): return async stream record batch
    /// for scan
    fn scan(&self, projection: Option<Vec<usize>>) -> Result<Vec<RecordBatch>>;
}