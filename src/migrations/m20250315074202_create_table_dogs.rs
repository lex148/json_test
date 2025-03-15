use welds::errors::Result;
use welds::migrations::{create_table, types::Type, MigrationStep, TableState};

pub(super) fn step(_state: &TableState) -> Result<MigrationStep> {
    let m = create_table("dogs")
        .id(|c| c("id", Type::IntBig))
        .column(|c| c("name", Type::String))
        .column(|c| c("test_json", Type::Raw("json".to_string())));
    Ok(MigrationStep::new("m20250315074202_create_table_dogs", m))
}
