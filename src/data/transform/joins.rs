use crate::error::Result;
use datafusion::prelude::*;

/// Join two tables using an inner join
pub async fn inner_join(
    ctx: &SessionContext,
    left_table: &str,
    right_table: &str,
    join_columns: &[(&str, &str)],
) -> Result<DataFrame> {
    let left_df = ctx.table(left_table).await?;
    let right_df = ctx.table(right_table).await?;
    
    let join_expr = join_columns.iter()
        .map(|(left, right)| col(format!("{left_table}.{left}")).eq(col(format!("{right_table}.{right}"))))
        .reduce(datafusion::prelude::Expr::and)
        .unwrap_or(lit(true));
    
    Ok(left_df.join(right_df, JoinType::Inner, &[], &[], Some(join_expr))?)
}

/// Join two tables using a left join
pub async fn left_join(
    ctx: &SessionContext,
    left_table: &str,
    right_table: &str,
    join_columns: &[(&str, &str)],
) -> Result<DataFrame> {
    let left_df = ctx.table(left_table).await?;
    let right_df = ctx.table(right_table).await?;
    
    let join_expr = join_columns.iter()
        .map(|(left, right)| col(format!("{left_table}.{left}")).eq(col(format!("{right_table}.{right}"))))
        .reduce(datafusion::prelude::Expr::and)
        .unwrap_or(lit(true));
    
    Ok(left_df.join(right_df, JoinType::Left, &[], &[], Some(join_expr))?)
}

/// Join multiple tables sequentially
pub async fn multi_join(
    ctx: &SessionContext,
    tables: &[&str],
    join_columns: &[&str],
    join_type: JoinType,
) -> Result<DataFrame> {
    if tables.is_empty() {
        return Err(crate::error::IdsError::Validation("No tables provided for join".to_string()));
    }
    
    if tables.len() == 1 {
        return ctx.table(tables[0]).await.map_err(std::convert::Into::into);
    }
    
    let mut result_df = ctx.table(tables[0]).await?;
    
    for i in 1..tables.len() {
        let right_df = ctx.table(tables[i]).await?;
        
        let join_expr = join_columns.iter()
            .map(|col_name| {
                col(format!("{}.{}", tables[0], col_name)).eq(col(format!("{}.{}", tables[i], col_name)))
            })
            .reduce(datafusion::prelude::Expr::and)
            .unwrap_or(lit(true));
        
        result_df = result_df.join(right_df, join_type, &[], &[], Some(join_expr))?;
    }
    
    Ok(result_df)
}

/// Execute a SQL join query
pub async fn sql_join(
    ctx: &SessionContext,
    query: &str,
) -> Result<DataFrame> {
    Ok(ctx.sql(query).await?)
}

/// Create a temporary joined view
pub async fn create_joined_view(
    ctx: &SessionContext,
    view_name: &str,
    tables: &[&str],
    join_columns: &[&str],
    join_type: &str,
) -> Result<()> {
    if tables.len() < 2 {
        return Err(crate::error::IdsError::Validation("At least two tables are required for a join view".to_string()));
    }
    
    let join_cols = join_columns.join(" AND ");
    let mut sql = format!("CREATE OR REPLACE TEMPORARY VIEW {} AS\nSELECT * FROM {}", view_name, tables[0]);
    
    for i in 1..tables.len() {
        sql.push_str(&format!("\n{} JOIN {} ON {}", join_type, tables[i], join_cols));
    }
    
    ctx.sql(&sql).await?;
    Ok(())
}