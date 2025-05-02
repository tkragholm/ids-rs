
# DataFrame API

A DataFrame represents a logical set of rows with the same named columns,
similar to a [Pandas DataFrame] or [Spark DataFrame].

DataFrames are typically created by calling a method on [`SessionContext`], such
as [`read_csv`], and can then be modified by calling the transformation methods,
such as [`filter`], [`select`], [`aggregate`], and [`limit`] to build up a query
definition.

The query can be executed by calling the [`collect`] method.

DataFusion DataFrames use lazy evaluation, meaning that each transformation
creates a new plan but does not actually perform any immediate actions. This
approach allows for the overall plan to be optimized before execution. The plan
is evaluated (executed) when an action method is invoked, such as [`collect`].
See the [Library Users Guide] for more details.

The DataFrame API is well documented in the [API reference on docs.rs].
Please refer to the [Expressions Reference] for more information on
building logical expressions (`Expr`) to use with the DataFrame API.

## Example

The DataFrame struct is part of DataFusion's `prelude` and can be imported with
the following statement.

```rust
use datafusion::prelude::*;
```

Here is a minimal example showing the execution of a query using the DataFrame API.

```rust
use datafusion::prelude::*;
use datafusion::error::Result;
use datafusion::functions_aggregate::expr_fn::min;

#[tokio::main]
async fn main() -> Result<()> {
    let ctx = SessionContext::new();
    let df = ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?;
    let df = df.filter(col("a").lt_eq(col("b")))?
        .aggregate(vec![col("a")], vec![min(col("b"))])?
        .limit(0, Some(100))?;
    // Print results
    df.show().await?;
    Ok(())
}
```

[pandas dataframe]: https://pandas.pydata.org/pandas-docs/stable/reference/api/pandas.DataFrame.html
[spark dataframe]: https://spark.apache.org/docs/latest/sql-programming-guide.html
[`sessioncontext`]: https://docs.rs/datafusion/latest/datafusion/execution/context/struct.SessionContext.html
[`read_csv`]: https://docs.rs/datafusion/latest/datafusion/execution/context/struct.SessionContext.html#method.read_csv
[`filter`]: https://docs.rs/datafusion/latest/datafusion/dataframe/struct.DataFrame.html#method.filter
[`select`]: https://docs.rs/datafusion/latest/datafusion/dataframe/struct.DataFrame.html#method.select
[`aggregate`]: https://docs.rs/datafusion/latest/datafusion/dataframe/struct.DataFrame.html#method.aggregate
[`limit`]: https://docs.rs/datafusion/latest/datafusion/dataframe/struct.DataFrame.html#method.limit
[`collect`]: https://docs.rs/datafusion/latest/datafusion/dataframe/struct.DataFrame.html#method.collect
[library users guide]: ../library-user-guide/using-the-dataframe-api.md
[api reference on docs.rs]: https://docs.rs/datafusion/latest/datafusion/dataframe/struct.DataFrame.html
[expressions reference]: expressions
