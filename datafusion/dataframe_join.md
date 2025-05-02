pub fn join(
    self,
    right: DataFrame,
    join_type: JoinType,
    left_cols: &[&str],
    right_cols: &[&str],
    filter: Option<Expr>,
) -> Result<DataFrame>

Join this DataFrame with another DataFrame using explicitly specified columns and an optional filter expression.

See join_on for a more concise way to specify the join condition. Since DataFusion will automatically identify and optimize equality predicates there is no performance difference between this function and join_on

left_cols and right_cols are used to form “equijoin” predicates (see example below), which are then combined with the optional filter expression. If left_cols and right_cols contain ambiguous column references, they will be disambiguated by prioritizing the left relation for left_cols and the right relation for right_cols.

Note that in case of outer join, the filter is applied to only matched rows.
Example

let ctx = SessionContext::new();

let left = ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?;

let right = ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?

  .select(vec![

    col("a").alias("a2"),

    col("b").alias("b2"),

    col("c").alias("c2")])?;

// Perform the equivalent of `left INNER JOIN right ON (a = a2 AND b = b2)`

// finding all pairs of rows from `left` and `right` where `a = a2` and `b = b2`.

let join = left.join(right, JoinType::Inner, &["a", "b"], &["a2", "b2"], None)?;

let expected = vec![

    "+---+---+---+----+----+----+",

    "| a | b | c | a2 | b2 | c2 |",

    "+---+---+---+----+----+----+",

    "| 1 | 2 | 3 | 1  | 2  | 3  |",

    "+---+---+---+----+----+----+"

];

assert_batches_sorted_eq!(expected, &join.collect().await?);

Source
pub fn join_on(
    self,
    right: DataFrame,
    join_type: JoinType,
    on_exprs: impl IntoIterator<Item = Expr>,
) -> Result<DataFrame>

Join this DataFrame with another DataFrame using the specified expressions.

Note that DataFusion automatically optimizes joins, including identifying and optimizing equality predicates.
Example

let ctx = SessionContext::new();

let left = ctx

    .read_csv("tests/data/example.csv", CsvReadOptions::new())

    .await?;

let right = ctx

    .read_csv("tests/data/example.csv", CsvReadOptions::new())

    .await?

    .select(vec![

        col("a").alias("a2"),

        col("b").alias("b2"),

        col("c").alias("c2"),

    ])?;


// Perform the equivalent of `left INNER JOIN right ON (a != a2 AND b != b2)`

// finding all pairs of rows from `left` and `right` where

// where `a != a2` and `b != b2`.

let join_on = left.join_on(

    right,

    JoinType::Inner,

    [col("a").not_eq(col("a2")), col("b").not_eq(col("b2"))],

)?;

let expected = vec![

    "+---+---+---+----+----+----+",

    "| a | b | c | a2 | b2 | c2 |",

    "+---+---+---+----+----+----+",

    "+---+---+---+----+----+----+"

];
