// Copyright 2022 Singularity Data
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use pgwire::pg_response::{PgResponse, StatementType};
use risingwave_common::error::ErrorCode::InternalError;
use risingwave_common::error::{Result, RwError};
use risingwave_sqlparser::ast::ObjectName;

use crate::binder::Binder;
use crate::session::OptimizerContext;

pub async fn handle_create_database(
    context: OptimizerContext,
    database_name: ObjectName,
    is_not_exist: bool,
) -> Result<PgResponse> {
    let session = context.session_ctx;
    let database_name = Binder::resolve_database_name(database_name)?;

    {
        let catalog_reader = session.env().catalog_reader();
        let reader = catalog_reader.read_guard();
        if reader.get_database_by_name(&database_name).is_ok() && !is_not_exist {
            return Err(RwError::from(InternalError(format!(
                "database {:?} already exists",
                database_name,
            ))));
        }
    }

    let catalog_writer = session.env().catalog_writer();
    catalog_writer.create_database(&database_name).await?;
    Ok(PgResponse::empty_result(StatementType::CREATE_DATABASE))
}
