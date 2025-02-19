// Copyright 2023 RisingWave Labs
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use risingwave_common::array::{ArrayRef, DataChunk};
use risingwave_common::error::Result;
use risingwave_common::types::Datum;
use risingwave_common::util::iter_util::ZipEqFast;
use risingwave_connector::source::SourceColumnDesc;

pub(crate) trait SourceChunkBuilder {
    fn build_columns<'a>(
        column_descs: &[SourceColumnDesc],
        rows: impl IntoIterator<Item = &'a Vec<Datum>>,
        chunk_size: usize,
    ) -> Result<Vec<ArrayRef>> {
        let mut builders: Vec<_> = column_descs
            .iter()
            .map(|k| k.data_type.create_array_builder(chunk_size))
            .collect();

        for row in rows {
            for (datum, builder) in row.iter().zip_eq_fast(&mut builders) {
                builder.append(datum);
            }
        }

        Ok(builders
            .into_iter()
            .map(|builder| builder.finish().into())
            .collect())
    }

    fn build_datachunk(
        column_desc: &[SourceColumnDesc],
        rows: &[Vec<Datum>],
        chunk_size: usize,
    ) -> Result<DataChunk> {
        let columns = Self::build_columns(column_desc, rows, chunk_size)?;
        Ok(DataChunk::new(columns, rows.len()))
    }
}
