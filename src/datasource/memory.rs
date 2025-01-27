/*
 * @Author: Veeupup
 * @Date: 2022-05-12 16:14:35
 * @Email: code@tanweime.com
*/

use arrow::record_batch::RecordBatch;
use std::sync::Arc;

use super::{TableRef, TableSource};
use crate::{error::Result, logical_plan::schema::NaiveSchema};

#[derive(Debug, Clone)]
pub struct MemTable {
    schema: NaiveSchema,
    batches: Vec<RecordBatch>,
}

impl MemTable {
    #[allow(unused)]
    pub fn try_create(schema: NaiveSchema, batches: Vec<RecordBatch>) -> Result<TableRef> {
        Ok(Arc::new(Self { schema, batches }))
    }
}

impl TableSource for MemTable {
    fn schema(&self) -> &NaiveSchema {
        &self.schema
    }

    fn scan(&self, projection: Option<Vec<usize>>) -> Result<Vec<RecordBatch>> {
        if let Some(projection) = projection {
            let batches = self
                .batches
                .iter()
                .map(|record_batch| record_batch.project(projection.as_ref()).unwrap())
                .collect::<Vec<_>>();
            return Ok(batches);
        }
        Ok(self.batches.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::MemTable;
    use crate::datasource::TableSource;
    use crate::error::Result;
    use crate::logical_plan::schema::NaiveSchema;
    use arrow::array::Int32Array;
    use arrow::datatypes::{DataType, Field, Schema};
    use arrow::record_batch::RecordBatch;
    use std::sync::Arc;

    #[test]
    fn mem_table_test() -> Result<()> {
        let schema = Arc::new(Schema::new(vec![
            Field::new("a", DataType::Int32, false),
            Field::new("b", DataType::Int32, false),
            Field::new("c", DataType::Int32, false),
            Field::new("d", DataType::Int32, true),
        ]));
        let schema = NaiveSchema::from_qualified("t1", &schema);

        let batch = RecordBatch::try_new(
            schema.clone().into(),
            vec![
                Arc::new(Int32Array::from(vec![1, 2, 3])),
                Arc::new(Int32Array::from(vec![4, 5, 6])),
                Arc::new(Int32Array::from(vec![7, 8, 9])),
                Arc::new(Int32Array::from(vec![None, None, Some(9)])),
            ],
        )?;

        let mem_table = MemTable::try_create(schema, vec![batch])?;

        // scan
        let batches = mem_table.scan(Some(vec![2, 1]))?;
        let batch2 = &batches[0];

        assert_eq!(2, batch2.schema().fields().len());
        assert_eq!("t1.c", batch2.schema().field(0).name());
        assert_eq!("t1.b", batch2.schema().field(1).name());
        assert_eq!(2, batch2.num_columns());

        Ok(())
    }
}
