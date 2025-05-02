# Crate Documentation

**Version:** 47.0.0

**Format Version:** 39

# Module `datafusion`

 [DataFusion] is an extensible query engine written in Rust that
 uses [Apache Arrow] as its in-memory format. DataFusion's target users are
 developers building fast and feature rich database and analytic systems,
 customized to particular workloads. See [use cases] for examples.

 "Out of the box," DataFusion offers [SQL] and [`Dataframe`] APIs,
 excellent [performance], built-in support for CSV, Parquet, JSON, and Avro,
 extensive customization, and a great community.
 [Python Bindings] are also available.

 DataFusion features a full query planner, a columnar, streaming, multi-threaded,
 vectorized execution engine, and partitioned data  sources. You can
 customize DataFusion at almost all points including additional data sources,
 query languages, functions, custom operators and more.
 See the [Architecture] section below for more details.

 [DataFusion]: https://datafusion.apache.org/
 [Apache Arrow]: https://arrow.apache.org
 [use cases]: https://datafusion.apache.org/user-guide/introduction.html#use-cases
 [SQL]: https://datafusion.apache.org/user-guide/sql/index.html
 [`DataFrame`]: dataframe::DataFrame
 [performance]: https://benchmark.clickhouse.com/
 [Python Bindings]: https://github.com/apache/datafusion-python
 [Architecture]: #architecture

 # Examples

 The main entry point for interacting with DataFusion is the
 [`SessionContext`]. [`Expr`]s represent expressions such as `a + b`.

 [`SessionContext`]: execution::context::SessionContext

 ## DataFrame

 To execute a query against data stored
 in a CSV file using a [`DataFrame`]:

 ```rust
 # use datafusion::prelude::*;
 # use datafusion::error::Result;
 # use datafusion::functions_aggregate::expr_fn::min;
 # use datafusion::arrow::array::RecordBatch;

 # #[tokio::main]
 # async fn main() -> Result<()> {
 let ctx = SessionContext::new();

 // create the dataframe
 let df = ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?;

 // create a plan
 let df = df.filter(col("a").lt_eq(col("b")))?
            .aggregate(vec![col("a")], vec![min(col("b"))])?
            .limit(0, Some(100))?;

 // execute the plan
 let results: Vec<RecordBatch> = df.collect().await?;

 // format the results
 let pretty_results = arrow::util::pretty::pretty_format_batches(&results)?
    .to_string();

 let expected = vec![
     "+---+----------------+",
     "| a | min(?table?.b) |",
     "+---+----------------+",
     "| 1 | 2              |",
     "+---+----------------+"
 ];

 assert_eq!(pretty_results.trim().lines().collect::<Vec<_>>(), expected);
 # Ok(())
 # }
 ```

 ## SQL

 To execute a query against a CSV file using [SQL]:

 ```
 # use datafusion::prelude::*;
 # use datafusion::error::Result;
 # use datafusion::arrow::array::RecordBatch;

 # #[tokio::main]
 # async fn main() -> Result<()> {
 let ctx = SessionContext::new();

 ctx.register_csv("example", "tests/data/example.csv", CsvReadOptions::new()).await?;

 // create a plan
 let df = ctx.sql("SELECT a, MIN(b) FROM example WHERE a <= b GROUP BY a LIMIT 100").await?;

 // execute the plan
 let results: Vec<RecordBatch> = df.collect().await?;

 // format the results
 let pretty_results = arrow::util::pretty::pretty_format_batches(&results)?
   .to_string();

 let expected = vec![
     "+---+----------------+",
     "| a | min(example.b) |",
     "+---+----------------+",
     "| 1 | 2              |",
     "+---+----------------+"
 ];

 assert_eq!(pretty_results.trim().lines().collect::<Vec<_>>(), expected);
 # Ok(())
 # }
 ```

 ## More Examples

 There are many additional annotated examples of using DataFusion in the [datafusion-examples] directory.

 [datafusion-examples]: https://github.com/apache/datafusion/tree/main/datafusion-examples

 # Architecture

 <!-- NOTE: The goal of this section is to provide a high level
 overview of how DataFusion is organized and then link to other
 sections of the docs with more details -->

 You can find a formal description of DataFusion's architecture in our
 [SIGMOD 2024 Paper].

 [SIGMOD 2024 Paper]: https://dl.acm.org/doi/10.1145/3626246.3653368

 ## Design Goals
 DataFusion's Architecture Goals are:

 1. Work “out of the box”: Provide a very fast, world class query engine with
    minimal setup or required configuration.

 2. Customizable everything: All behavior should be customizable by
    implementing traits.

 3. Architecturally boring 🥱: Follow industrial best practice rather than
    trying cutting edge, but unproven, techniques.

 With these principles, users start with a basic, high-performance engine
 and specialize it over time to suit their needs and available engineering
 capacity.

 ## Overview  Presentations

 The following presentations offer high level overviews of the
 different components and how they interact together.

 - [Apr 2023]: The Apache DataFusion Architecture talks
   - _Query Engine_: [recording](https://youtu.be/NVKujPxwSBA) and [slides](https://docs.google.com/presentation/d/1D3GDVas-8y0sA4c8EOgdCvEjVND4s2E7I6zfs67Y4j8/edit#slide=id.p)
   - _Logical Plan and Expressions_: [recording](https://youtu.be/EzZTLiSJnhY) and [slides](https://docs.google.com/presentation/d/1ypylM3-w60kVDW7Q6S99AHzvlBgciTdjsAfqNP85K30)
   - _Physical Plan and Execution_: [recording](https://youtu.be/2jkWU3_w6z0) and [slides](https://docs.google.com/presentation/d/1cA2WQJ2qg6tx6y4Wf8FH2WVSm9JQ5UgmBWATHdik0hg)
 - [July 2022]: DataFusion and Arrow: Supercharge Your Data Analytical Tool with a Rusty Query Engine: [recording](https://www.youtube.com/watch?v=Rii1VTn3seQ) and [slides](https://docs.google.com/presentation/d/1q1bPibvu64k2b7LPi7Yyb0k3gA1BiUYiUbEklqW1Ckc/view#slide=id.g11054eeab4c_0_1165)
 - [March 2021]: The DataFusion architecture is described in _Query Engine Design and the Rust-Based DataFusion in Apache Arrow_: [recording](https://www.youtube.com/watch?v=K6eCAVEk4kU) (DataFusion content starts [~ 15 minutes in](https://www.youtube.com/watch?v=K6eCAVEk4kU&t=875s)) and [slides](https://www.slideshare.net/influxdata/influxdb-iox-tech-talks-query-engine-design-and-the-rustbased-datafusion-in-apache-arrow-244161934)
 - [February 2021]: How DataFusion is used within the Ballista Project is described in _Ballista: Distributed Compute with Rust and Apache Arrow_: [recording](https://www.youtube.com/watch?v=ZZHQaOap9pQ)

 ## Customization and Extension

 DataFusion is designed to be highly extensible, so you can
 start with a working, full featured engine, and then
 specialize any behavior for your use case. For example,
 some projects may add custom [`ExecutionPlan`] operators, or create their own
 query language that directly creates [`LogicalPlan`] rather than using the
 built in SQL planner, [`SqlToRel`].

 In order to achieve this, DataFusion supports extension at many points:

 * read from any datasource ([`TableProvider`])
 * define your own catalogs, schemas, and table lists ([`catalog`] and [`CatalogProvider`])
 * build your own query language or plans ([`LogicalPlanBuilder`])
 * declare and use user-defined functions ([`ScalarUDF`], and [`AggregateUDF`], [`WindowUDF`])
 * add custom plan rewrite passes ([`AnalyzerRule`], [`OptimizerRule`]  and [`PhysicalOptimizerRule`])
 * extend the planner to use user-defined logical and physical nodes ([`QueryPlanner`])

 You can find examples of each of them in the [datafusion-examples] directory.

 [`TableProvider`]: crate::datasource::TableProvider
 [`CatalogProvider`]: crate::catalog::CatalogProvider
 [`LogicalPlanBuilder`]: datafusion_expr::logical_plan::builder::LogicalPlanBuilder
 [`ScalarUDF`]: crate::logical_expr::ScalarUDF
 [`AggregateUDF`]: crate::logical_expr::AggregateUDF
 [`WindowUDF`]: crate::logical_expr::WindowUDF
 [`QueryPlanner`]: execution::context::QueryPlanner
 [`OptimizerRule`]: datafusion_optimizer::optimizer::OptimizerRule
 [`AnalyzerRule`]:  datafusion_optimizer::analyzer::AnalyzerRule
 [`PhysicalOptimizerRule`]: datafusion_physical_optimizer::PhysicalOptimizerRule

 ## Query Planning and Execution Overview

 ### SQL

 ```text
                 Parsed with            SqlToRel creates
                 sqlparser              initial plan
 ┌───────────────┐           ┌─────────┐             ┌─────────────┐
 │   SELECT *    │           │Query {  │             │Project      │
 │   FROM ...    │──────────▶│..       │────────────▶│  TableScan  │
 │               │           │}        │             │    ...      │
 └───────────────┘           └─────────┘             └─────────────┘

   SQL String                 sqlparser               LogicalPlan
                              AST nodes
 ```

 1. The query string is parsed to an Abstract Syntax Tree (AST)
    [`Statement`] using [sqlparser].

 2. The AST is converted to a [`LogicalPlan`] and logical expressions
    [`Expr`]s to compute the desired result by [`SqlToRel`]. This phase
    also includes name and type resolution ("binding").

 [`Statement`]: https://docs.rs/sqlparser/latest/sqlparser/ast/enum.Statement.html

 ### DataFrame

 When executing plans using the [`DataFrame`] API, the process is
 identical as with SQL, except the DataFrame API builds the
 [`LogicalPlan`] directly using [`LogicalPlanBuilder`]. Systems
 that have their own custom query languages typically also build
 [`LogicalPlan`] directly.

 ### Planning

 ```text
             AnalyzerRules and      PhysicalPlanner          PhysicalOptimizerRules
             OptimizerRules         creates ExecutionPlan    improve performance
             rewrite plan
 ┌─────────────┐        ┌─────────────┐      ┌─────────────────┐        ┌─────────────────┐
 │Project      │        │Project(x, y)│      │ProjectExec      │        │ProjectExec      │
 │  TableScan  │──...──▶│  TableScan  │─────▶│  ...            │──...──▶│  ...            │
 │    ...      │        │    ...      │      │   DataSourceExec│        │   DataSourceExec│
 └─────────────┘        └─────────────┘      └─────────────────┘        └─────────────────┘

  LogicalPlan            LogicalPlan         ExecutionPlan             ExecutionPlan
 ```

 To process large datasets with many rows as efficiently as
 possible, significant effort is spent planning and
 optimizing, in the following manner:

 1. The [`LogicalPlan`] is checked and rewritten to enforce
    semantic rules, such as type coercion, by [`AnalyzerRule`]s

 2. The [`LogicalPlan`] is rewritten by [`OptimizerRule`]s, such as
    projection and filter pushdown, to improve its efficiency.

 3. The [`LogicalPlan`] is converted to an [`ExecutionPlan`] by a
    [`PhysicalPlanner`]

 4. The [`ExecutionPlan`] is rewritten by
    [`PhysicalOptimizerRule`]s, such as sort and join selection, to
    improve its efficiency.

 ## Data Sources

 ```text
 Planning       │
 requests       │            TableProvider::scan
 information    │            creates an
 such as schema │            ExecutionPlan
                │
                ▼
   ┌─────────────────────────┐         ┌───────────────┐
   │                         │         │               │
   │impl TableProvider       │────────▶│DataSourceExec │
   │                         │         │               │
   └─────────────────────────┘         └───────────────┘
         TableProvider
         (built in or user provided)    ExecutionPlan
 ```

 A [`TableProvider`] provides information for planning and
 an [`ExecutionPlan`]s for execution. DataFusion includes [`ListingTable`]
 which supports reading several common file formats, and you can support any
 new file format by implementing the [`TableProvider`] trait. See also:

 1. [`ListingTable`]: Reads data from Parquet, JSON, CSV, or AVRO
    files.  Supports single files or multiple files with HIVE style
    partitioning, optional compression, directly reading from remote
    object store and more.

 2. [`MemTable`]: Reads data from in memory [`RecordBatch`]es.

 3. [`StreamingTable`]: Reads data from potentially unbounded inputs.

 [`ListingTable`]: crate::datasource::listing::ListingTable
 [`MemTable`]: crate::datasource::memory::MemTable
 [`StreamingTable`]: crate::catalog::streaming::StreamingTable

 ## Plan Representations

 ### Logical Plans
 Logical planning yields [`LogicalPlan`] nodes and [`Expr`]
 representing expressions which are [`Schema`] aware and represent statements
 independent of how they are physically executed.
 A [`LogicalPlan`] is a Directed Acyclic Graph (DAG) of other
 [`LogicalPlan`]s, each potentially containing embedded [`Expr`]s.

 `LogicalPlan`s can be rewritten with [`TreeNode`] API, see the
 [`tree_node module`] for more details.

 [`Expr`]s can also be rewritten with [`TreeNode`] API and simplified using
 [`ExprSimplifier`]. Examples of working with and executing `Expr`s can be
 found in the [`expr_api`.rs] example

 [`TreeNode`]: datafusion_common::tree_node::TreeNode
 [`tree_node module`]: datafusion_expr::logical_plan::tree_node
 [`ExprSimplifier`]: crate::optimizer::simplify_expressions::ExprSimplifier
 [`expr_api`.rs]: https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/expr_api.rs

 ### Physical Plans

 An [`ExecutionPlan`] (sometimes referred to as a "physical plan")
 is a plan that can be executed against data. It a DAG of other
 [`ExecutionPlan`]s each potentially containing expressions that implement the
 [`PhysicalExpr`] trait.

 Compared to a [`LogicalPlan`], an [`ExecutionPlan`] has additional concrete
 information about how to perform calculations (e.g. hash vs merge
 join), and how data flows during execution (e.g. partitioning and
 sortedness).

 [cp_solver] performs range propagation analysis on [`PhysicalExpr`]s and
 [`PruningPredicate`] can prove certain boolean [`PhysicalExpr`]s used for
 filtering can never be `true` using additional statistical information.

 [cp_solver]: crate::physical_expr::intervals::cp_solver
 [`PruningPredicate`]: datafusion_physical_optimizer::pruning::PruningPredicate
 [`PhysicalExpr`]: crate::physical_plan::PhysicalExpr

 ## Execution

 ```text
            ExecutionPlan::execute             Calling next() on the
            produces a stream                  stream produces the data

 ┌────────────────┐      ┌─────────────────────────┐         ┌────────────┐
 │ProjectExec     │      │impl                     │    ┌───▶│RecordBatch │
 │  ...           │─────▶│SendableRecordBatchStream│────┤    └────────────┘
 │  DataSourceExec│      │                         │    │    ┌────────────┐
 └────────────────┘      └─────────────────────────┘    ├───▶│RecordBatch │
               ▲                                        │    └────────────┘
 ExecutionPlan │                                        │         ...
               │                                        │
               │                                        │    ┌────────────┐
             PhysicalOptimizerRules                     ├───▶│RecordBatch │
             request information                        │    └────────────┘
             such as partitioning                       │    ┌ ─ ─ ─ ─ ─ ─
                                                        └───▶ None        │
                                                             └ ─ ─ ─ ─ ─ ─
 ```

 [`ExecutionPlan`]s process data using the [Apache Arrow] memory
 format, making heavy use of functions from the [arrow]
 crate. Values are represented with [`ColumnarValue`], which are either
 [`ScalarValue`] (single constant values) or [`ArrayRef`] (Arrow
 Arrays).

 Calling [`execute`] produces 1 or more partitions of data,
 as a [`SendableRecordBatchStream`], which implements a pull based execution
 API. Calling [`next()`]`.await` will incrementally compute and return the next
 [`RecordBatch`]. Balanced parallelism is achieved using [Volcano style]
 "Exchange" operations implemented by [`RepartitionExec`].

 While some recent research such as [Morsel-Driven Parallelism] describes challenges
 with the pull style Volcano execution model on NUMA architectures, in practice DataFusion achieves
 similar scalability as systems that use push driven schedulers [such as DuckDB].
 See the [DataFusion paper in SIGMOD 2024] for more details.

 [`execute`]: physical_plan::ExecutionPlan::execute
 [`SendableRecordBatchStream`]: crate::physical_plan::SendableRecordBatchStream
 [`ColumnarValue`]: datafusion_expr::ColumnarValue
 [`ScalarValue`]: crate::scalar::ScalarValue
 [`ArrayRef`]: arrow::array::ArrayRef
 [`Stream`]: futures::stream::Stream

 See the [implementors of `ExecutionPlan`] for a list of physical operators available.

 [`RepartitionExec`]: https://docs.rs/datafusion/latest/datafusion/physical_plan/repartition/struct.RepartitionExec.html
 [Volcano style]: https://doi.org/10.1145/93605.98720
 [Morsel-Driven Parallelism]: https://db.in.tum.de/~leis/papers/morsels.pdf
 [DataFusion paper in SIGMOD 2024]: https://github.com/apache/datafusion/files/15149988/DataFusion_Query_Engine___SIGMOD_2024-FINAL-mk4.pdf
 [such as DuckDB]: https://github.com/duckdb/duckdb/issues/1583
 [implementors of `ExecutionPlan`]: https://docs.rs/datafusion/latest/datafusion/physical_plan/trait.ExecutionPlan.html#implementors

 ## Streaming Execution

 DataFusion is a "streaming" query engine which means `ExecutionPlan`s incrementally
 read from their input(s) and compute output one [`RecordBatch`] at a time
 by continually polling [`SendableRecordBatchStream`]s. Output and
 intermediate `RecordBatch`s each have approximately `batch_size` rows,
 which amortizes per-batch overhead of execution.

 Note that certain operations, sometimes called "pipeline breakers",
 (for example full sorts or hash aggregations) are fundamentally non streaming and
 must read their input fully before producing **any** output. As much as possible,
 other operators read a single [`RecordBatch`] from their input to produce a
 single `RecordBatch` as output.

 For example, given this SQL query:

 ```sql
 SELECT date_trunc('month', time) FROM data WHERE id IN (10,20,30);
 ```

 The diagram below shows the call sequence when a consumer calls [`next()`] to
 get the next `RecordBatch` of output. While it is possible that some
 steps run on different threads, typically tokio will use the same thread
 that called `next()` to read from the input, apply the filter, and
 return the results without interleaving any other operations. This results
 in excellent cache locality as the same CPU core that produces the data often
 consumes it immediately as well.

 ```text

 Step 3: FilterExec calls next()       Step 2: ProjectionExec calls
         on input Stream                  next() on input Stream
         ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─      ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
                            │                                               Step 1: Consumer
         ▼                        ▼                           │               calls next()
 ┏━━━━━━━━━━━━━━━━┓     ┏━━━━━┻━━━━━━━━━━━━━┓      ┏━━━━━━━━━━━━━━━━━━━━━━━━┓
 ┃                ┃     ┃                   ┃      ┃                        ◀ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
 ┃  DataSource    ┃     ┃                   ┃      ┃                        ┃
 ┃    (e.g.       ┃     ┃    FilterExec     ┃      ┃     ProjectionExec     ┃
 ┃ ParquetSource) ┃     ┃id IN (10, 20, 30) ┃      ┃date_bin('month', time) ┃
 ┃                ┃     ┃                   ┃      ┃                        ┣ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ▶
 ┃                ┃     ┃                   ┃      ┃                        ┃
 ┗━━━━━━━━━━━━━━━━┛     ┗━━━━━━━━━━━┳━━━━━━━┛      ┗━━━━━━━━━━━━━━━━━━━━━━━━┛
         │                  ▲                                 ▲          Step 6: ProjectionExec
                            │     │                           │        computes date_trunc into a
         └ ─ ─ ─ ─ ─ ─ ─ ─ ─       ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─          new RecordBatch returned
              ┌─────────────────────┐                ┌─────────────┐          from client
              │     RecordBatch     │                │ RecordBatch │
              └─────────────────────┘                └─────────────┘

           Step 4: DataSource returns a        Step 5: FilterExec returns a new
                single RecordBatch            RecordBatch with only matching rows
 ```

 [`next()`]: futures::StreamExt::next

 ## Thread Scheduling, CPU / IO Thread Pools, and [Tokio] [`Runtime`]s

 DataFusion automatically runs each plan with multiple CPU cores using
 a [Tokio] [`Runtime`] as a thread pool. While tokio is most commonly used
 for asynchronous network I/O, the combination of an efficient, work-stealing
 scheduler and first class compiler support for automatic continuation
 generation (`async`), also makes it a compelling choice for CPU intensive
 applications as explained in the [Using Rustlang’s Async Tokio
 Runtime for CPU-Bound Tasks] blog.

 The number of cores used is determined by the `target_partitions`
 configuration setting, which defaults to the number of CPU cores.
 While preparing for execution, DataFusion tries to create this many distinct
 `async` [`Stream`]s for each `ExecutionPlan`.
 The `Stream`s for certain `ExecutionPlans`, such as as [`RepartitionExec`]
 and [`CoalescePartitionsExec`], spawn [Tokio] [`task`]s, that are run by
 threads managed by the `Runtime`.
 Many DataFusion `Stream`s perform CPU intensive processing.

 Using `async` for CPU intensive tasks makes it easy for [`TableProvider`]s
 to perform network I/O using standard Rust `async` during execution.
 However, this design also makes it very easy to mix CPU intensive and latency
 sensitive I/O work on the same thread pool ([`Runtime`]).
 Using the same (default) `Runtime` is convenient, and often works well for
 initial development and processing local files, but it can lead to problems
 under load and/or when reading from network sources such as AWS S3.

 If your system does not fully utilize either the CPU or network bandwidth
 during execution, or you see significantly higher tail (e.g. p99) latencies
 responding to network requests, **it is likely you need to use a different
 `Runtime` for CPU intensive DataFusion plans**. This effect can be especially
 pronounced when running several queries concurrently.

 As shown in the following figure, using the same `Runtime` for both CPU
 intensive processing and network requests can introduce significant
 delays in responding to those network requests. Delays in processing network
 requests can and does lead network flow control to throttle the available
 bandwidth in response.

 ```text
                                                                          Legend

                                                                          ┏━━━━━━┓
                            Processing network request                    ┃      ┃  CPU bound work
                            is delayed due to processing                  ┗━━━━━━┛
                            CPU bound work                                ┌─┐
                                                                          │ │       Network request
                                         ││                               └─┘       processing

                                         ││
                                ─ ─ ─ ─ ─  ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
                               │                                            │

                               ▼                                            ▼
 ┌─────────────┐           ┌─┐┌─┐┏━━━━━━━━━━━━━━━━━━━┓┏━━━━━━━━━━━━━━━━━━━┓┌─┐
 │             │thread 1   │ ││ │┃     Decoding      ┃┃     Filtering     ┃│ │
 │             │           └─┘└─┘┗━━━━━━━━━━━━━━━━━━━┛┗━━━━━━━━━━━━━━━━━━━┛└─┘
 │             │           ┏━━━━━━━━━━━━━━┳━━━━━━━━━━━━━━━━━━━┳━━━━━━━━━━━━━━┓
 │Tokio Runtime│thread 2   ┃   Decoding   ┃     Filtering     ┃   Decoding   ┃       ...
 │(thread pool)│           ┗━━━━━━━━━━━━━━┻━━━━━━━━━━━━━━━━━━━┻━━━━━━━━━━━━━━┛
 │             │     ...                               ...
 │             │           ┏━━━━━━━━━━━━━━━━━━━┳━━━━━━━━━━━━━━━━━━━┓┌─┐ ┏━━━━━━━━━━━━━━┓
 │             │thread N   ┃     Decoding      ┃     Filtering     ┃│ │ ┃   Decoding   ┃
 └─────────────┘           ┗━━━━━━━━━━━━━━━━━━━┻━━━━━━━━━━━━━━━━━━━┛└─┘ ┗━━━━━━━━━━━━━━┛
                           ─────────────────────────────────────────────────────────────▶
                                                                                           time
 ```

 The bottleneck resulting from network throttling can be avoided
 by using separate [`Runtime`]s for the different types of work, as shown
 in the diagram below.

 ```text
                    A separate thread pool processes network       Legend
                    requests, reducing the latency for
                    processing each request                        ┏━━━━━━┓
                                                                   ┃      ┃  CPU bound work
                                         │                         ┗━━━━━━┛
                                          │                        ┌─┐
                               ┌ ─ ─ ─ ─ ┘                         │ │       Network request
                                  ┌ ─ ─ ─ ┘                        └─┘       processing
                               │
                               ▼  ▼
 ┌─────────────┐           ┌─┐┌─┐┌─┐
 │             │thread 1   │ ││ ││ │
 │             │           └─┘└─┘└─┘
 │Tokio Runtime│                                          ...
 │(thread pool)│thread 2
 │             │
 │"IO Runtime" │     ...
 │             │                                                   ┌─┐
 │             │thread N                                           │ │
 └─────────────┘                                                   └─┘
                           ─────────────────────────────────────────────────────────────▶
                                                                                           time

 ┌─────────────┐           ┏━━━━━━━━━━━━━━━━━━━┓┏━━━━━━━━━━━━━━━━━━━┓
 │             │thread 1   ┃     Decoding      ┃┃     Filtering     ┃
 │             │           ┗━━━━━━━━━━━━━━━━━━━┛┗━━━━━━━━━━━━━━━━━━━┛
 │Tokio Runtime│           ┏━━━━━━━━━━━━━━┳━━━━━━━━━━━━━━━━━━━┳━━━━━━━━━━━━━━┓
 │(thread pool)│thread 2   ┃   Decoding   ┃     Filtering     ┃   Decoding   ┃       ...
 │             │           ┗━━━━━━━━━━━━━━┻━━━━━━━━━━━━━━━━━━━┻━━━━━━━━━━━━━━┛
 │ CPU Runtime │     ...                               ...
 │             │           ┏━━━━━━━━━━━━━━━━━━━┳━━━━━━━━━━━━━━━━━━━┳━━━━━━━━━━━━━━┓
 │             │thread N   ┃     Decoding      ┃     Filtering     ┃   Decoding   ┃
 └─────────────┘           ┗━━━━━━━━━━━━━━━━━━━┻━━━━━━━━━━━━━━━━━━━┻━━━━━━━━━━━━━━┛
                          ─────────────────────────────────────────────────────────────▶
                                                                                           time
```

 Note that DataFusion does not use [`tokio::task::spawn_blocking`] for
 CPU-bounded work, because `spawn_blocking` is designed for blocking **IO**,
 not designed CPU bound tasks. Among other challenges, spawned blocking
 tasks can't yield waiting for input (can't call `await`) so they
 can't be used to limit the number of concurrent CPU bound tasks or
 keep the processing pipeline to the same core.

 [Tokio]:  https://tokio.rs
 [`Runtime`]: tokio::runtime::Runtime
 [`task`]: tokio::task
 [Using Rustlang’s Async Tokio Runtime for CPU-Bound Tasks]: https://thenewstack.io/using-rustlangs-async-tokio-runtime-for-cpu-bound-tasks/
 [`RepartitionExec`]: physical_plan::repartition::RepartitionExec
 [`CoalescePartitionsExec`]: physical_plan::coalesce_partitions::CoalescePartitionsExec

 ## State Management and Configuration

 [`ConfigOptions`] contain options to control DataFusion's
 execution.

 [`ConfigOptions`]: datafusion_common::config::ConfigOptions

 The state required to execute queries is managed by the following
 structures:

 1. [`SessionContext`]: State needed for create [`LogicalPlan`]s such
    as the table definitions, and the function registries.

 2. [`TaskContext`]: State needed for execution such as the
    [`MemoryPool`], [`DiskManager`], and [`ObjectStoreRegistry`].

 3. [`ExecutionProps`]: Per-execution properties and data (such as
    starting timestamps, etc).

 [`SessionContext`]: crate::execution::context::SessionContext
 [`TaskContext`]: crate::execution::context::TaskContext
 [`ExecutionProps`]: crate::execution::context::ExecutionProps

 ### Resource Management

 The amount of memory and temporary local disk space used by
 DataFusion when running a plan can be controlled using the
 [`MemoryPool`] and [`DiskManager`]. Other runtime options can be
 found on [`RuntimeEnv`].

 [`DiskManager`]: crate::execution::DiskManager
 [`MemoryPool`]: crate::execution::memory_pool::MemoryPool
 [`RuntimeEnv`]: crate::execution::runtime_env::RuntimeEnv
 [`ObjectStoreRegistry`]: crate::datasource::object_store::ObjectStoreRegistry

 ## Crate Organization

 Most users interact with DataFusion via this crate (`datafusion`), which re-exports
 all functionality needed to build and execute queries.

 There are three other crates that provide additional functionality that
 must be used directly:
 * [`datafusion_proto`]: Plan serialization and deserialization
 * [`datafusion_substrait`]: Support for the substrait plan serialization format
 * [`datafusion_sqllogictest`] : The DataFusion SQL logic test runner

 [`datafusion_proto`]: https://crates.io/crates/datafusion-proto
 [`datafusion_substrait`]: https://crates.io/crates/datafusion-substrait
 [`datafusion_sqllogictest`]: https://crates.io/crates/datafusion-sqllogictest

 DataFusion is internally split into multiple sub crates to
 enforce modularity and improve compilation times. See the
 [list of modules](#modules) for all available sub-crates. Major ones are

 * [datafusion_common]: Common traits and types
 * [datafusion_catalog]: Catalog APIs such as [`SchemaProvider`] and [`CatalogProvider`]
 * [datafusion_datasource]: File and Data IO such as [`FileSource`] and [`DataSink`]
 * [datafusion_session]: [`Session`] and related structures
 * [datafusion_execution]: State and structures needed for execution
 * [datafusion_expr]: [`LogicalPlan`], [`Expr`] and related logical planning structure
 * [datafusion_functions]: Scalar function packages
 * [datafusion_functions_aggregate]: Aggregate functions such as `MIN`, `MAX`, `SUM`, etc
 * [datafusion_functions_nested]: Scalar function packages for `ARRAY`s, `MAP`s and `STRUCT`s
 * [datafusion_functions_table]: Table Functions such as `GENERATE_SERIES`
 * [datafusion_functions_window]: Window functions such as `ROW_NUMBER`, `RANK`, etc
 * [datafusion_optimizer]: [`OptimizerRule`]s and [`AnalyzerRule`]s
 * [datafusion_physical_expr]: [`PhysicalExpr`] and related expressions
 * [datafusion_physical_plan]: [`ExecutionPlan`] and related expressions
 * [datafusion_physical_optimizer]: [`ExecutionPlan`] and related expressions
 * [datafusion_sql]: SQL planner ([`SqlToRel`])

 [`SchemaProvider`]: datafusion_catalog::SchemaProvider
 [`CatalogProvider`]: datafusion_catalog::CatalogProvider
 [`Session`]: datafusion_session::Session
 [`FileSource`]: datafusion_datasource::file::FileSource
 [`DataSink`]: datafusion_datasource::sink::DataSink

 ## Citing DataFusion in Academic Papers

 You can use the following citation to reference DataFusion in academic papers:

 ```text
 @inproceedings{lamb2024apache
   title={Apache Arrow DataFusion: A Fast, Embeddable, Modular Analytic Query Engine},
   author={Lamb, Andrew and Shen, Yijie and Heres, Dani{\"e}l and Chakraborty, Jayjeet and Kabak, Mehmet Ozan and Hsieh, Liang-Chi and Sun, Chao},
   booktitle={Companion of the 2024 International Conference on Management of Data},
   pages={5--17},
   year={2024}
 }
 ```

 [sqlparser]: https://docs.rs/sqlparser/latest/sqlparser
 [`SqlToRel`]: sql::planner::SqlToRel
 [`Expr`]: datafusion_expr::Expr
 [`LogicalPlan`]: datafusion_expr::LogicalPlan
 [`AnalyzerRule`]: datafusion_optimizer::analyzer::AnalyzerRule
 [`OptimizerRule`]: optimizer::optimizer::OptimizerRule
 [`ExecutionPlan`]: physical_plan::ExecutionPlan
 [`PhysicalPlanner`]: physical_planner::PhysicalPlanner
 [`PhysicalOptimizerRule`]: datafusion_physical_optimizer::PhysicalOptimizerRule
 [`Schema`]: arrow::datatypes::Schema
 [`PhysicalExpr`]: physical_plan::PhysicalExpr
 [`RecordBatch`]: arrow::array::RecordBatch
 [`RecordBatchReader`]: arrow::record_batch::RecordBatchReader
 [`Array`]: arrow::array::Array

## Modules

## Module `dataframe`

[`DataFrame`] API for building and executing query plans.

```rust
pub mod dataframe { /* ... */ }
```

### Types

#### Struct `DataFrameWriteOptions`

Contains options that control how data is
written out from a DataFrame

```rust
pub struct DataFrameWriteOptions {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  Create a new DataFrameWriteOptions with default values

- ```rust
  pub fn with_insert_operation(self: Self, insert_op: InsertOp) -> Self { /* ... */ }
  ```
  Set the insert operation

- ```rust
  pub fn with_single_file_output(self: Self, single_file_output: bool) -> Self { /* ... */ }
  ```
  Set the single_file_output value to true or false

- ```rust
  pub fn with_partition_by(self: Self, partition_by: Vec<String>) -> Self { /* ... */ }
  ```
  Sets the partition_by columns for output partitioning

- ```rust
  pub fn with_sort_by(self: Self, sort_by: Vec<SortExpr>) -> Self { /* ... */ }
  ```
  Sets the sort_by columns for output sorting

###### Trait Implementations

- **Unpin**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Sync**
- **IntoEither**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Same**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **MaybeSendSync**
- **ErasedDestructor**
- **Freeze**
#### Struct `DataFrame`

Represents a logical set of rows with the same named columns.

Similar to a [Pandas DataFrame] or [Spark DataFrame], a DataFusion DataFrame
represents a 2 dimensional table of rows and columns.

The typical workflow using DataFrames looks like

1. Create a DataFrame via methods on [SessionContext], such as [`read_csv`]
   and [`read_parquet`].

2. Build a desired calculation by calling methods such as [`filter`],
   [`select`], [`aggregate`], and [`limit`]

3. Execute into [`RecordBatch`]es by calling [`collect`]

A `DataFrame` is a wrapper around a [`LogicalPlan`] and the [`SessionState`]
   required for execution.

DataFrames are "lazy" in the sense that most methods do not actually compute
anything, they just build up a plan. Calling [`collect`] executes the plan
using the same DataFusion planning and execution process used to execute SQL
and other queries.

[Pandas DataFrame]: https://pandas.pydata.org/pandas-docs/stable/reference/api/pandas.DataFrame.html
[Spark DataFrame]: https://spark.apache.org/docs/latest/sql-programming-guide.html
[`read_csv`]: SessionContext::read_csv
[`read_parquet`]: SessionContext::read_parquet
[`filter`]: DataFrame::filter
[`select`]: DataFrame::select
[`aggregate`]: DataFrame::aggregate
[`limit`]: DataFrame::limit
[`collect`]: DataFrame::collect

# Example
```
# use datafusion::prelude::*;
# use datafusion::error::Result;
# use datafusion::functions_aggregate::expr_fn::min;
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
// Read the data from a csv file
let df = ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?;
// create a new dataframe that computes the equivalent of
// `SELECT a, MIN(b) FROM df WHERE a <= b GROUP BY a LIMIT 100;`
let df = df.filter(col("a").lt_eq(col("b")))?
           .aggregate(vec![col("a")], vec![min(col("b"))])?
           .limit(0, Some(100))?;
// Perform the actual computation
let results = df.collect();
# Ok(())
# }
```

```rust
pub struct DataFrame {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub async fn write_parquet(self: Self, path: &str, options: DataFrameWriteOptions, writer_options: Option<TableParquetOptions>) -> Result<Vec<RecordBatch>, DataFusionError> { /* ... */ }
  ```
  Execute the `DataFrame` and write the results to Parquet file(s).

- ```rust
  pub fn new(session_state: SessionState, plan: LogicalPlan) -> Self { /* ... */ }
  ```
  Create a new `DataFrame ` based on an existing `LogicalPlan`

- ```rust
  pub fn parse_sql_expr(self: &Self, sql: &str) -> Result<Expr> { /* ... */ }
  ```
  Creates logical expression from a SQL query text.

- ```rust
  pub async fn create_physical_plan(self: Self) -> Result<Arc<dyn ExecutionPlan>> { /* ... */ }
  ```
  Consume the DataFrame and produce a physical plan

- ```rust
  pub fn select_columns(self: Self, columns: &[&str]) -> Result<DataFrame> { /* ... */ }
  ```
  Filter the DataFrame by column. Returns a new DataFrame only containing the

- ```rust
  pub fn select_exprs(self: Self, exprs: &[&str]) -> Result<DataFrame> { /* ... */ }
  ```
  Project arbitrary list of expression strings into a new `DataFrame`.

- ```rust
  pub fn select</* synthetic */ impl Into<SelectExpr>: Into<SelectExpr>, /* synthetic */ impl IntoIterator<Item = impl Into<SelectExpr>>: IntoIterator<Item = impl Into<SelectExpr>>>(self: Self, expr_list: impl IntoIterator<Item = impl Into<SelectExpr>>) -> Result<DataFrame> { /* ... */ }
  ```
  Project arbitrary expressions (like SQL SELECT expressions) into a new

- ```rust
  pub fn drop_columns(self: Self, columns: &[&str]) -> Result<DataFrame> { /* ... */ }
  ```
  Returns a new DataFrame containing all columns except the specified columns.

- ```rust
  pub fn unnest_columns(self: Self, columns: &[&str]) -> Result<DataFrame> { /* ... */ }
  ```
  Expand multiple list/struct columns into a set of rows and new columns.

- ```rust
  pub fn unnest_columns_with_options(self: Self, columns: &[&str], options: UnnestOptions) -> Result<DataFrame> { /* ... */ }
  ```
  Expand multiple list columns into a set of rows, with

- ```rust
  pub fn filter(self: Self, predicate: Expr) -> Result<DataFrame> { /* ... */ }
  ```
  Return a DataFrame with only rows for which `predicate` evaluates to

- ```rust
  pub fn aggregate(self: Self, group_expr: Vec<Expr>, aggr_expr: Vec<Expr>) -> Result<DataFrame> { /* ... */ }
  ```
  Return a new `DataFrame` that aggregates the rows of the current

- ```rust
  pub fn window(self: Self, window_exprs: Vec<Expr>) -> Result<DataFrame> { /* ... */ }
  ```
  Return a new DataFrame that adds the result of evaluating one or more

- ```rust
  pub fn limit(self: Self, skip: usize, fetch: Option<usize>) -> Result<DataFrame> { /* ... */ }
  ```
  Returns a new `DataFrame` with a limited number of rows.

- ```rust
  pub fn union(self: Self, dataframe: DataFrame) -> Result<DataFrame> { /* ... */ }
  ```
  Calculate the union of two [`DataFrame`]s, preserving duplicate rows.

- ```rust
  pub fn union_by_name(self: Self, dataframe: DataFrame) -> Result<DataFrame> { /* ... */ }
  ```
  Calculate the union of two [`DataFrame`]s using column names, preserving duplicate rows.

- ```rust
  pub fn union_distinct(self: Self, dataframe: DataFrame) -> Result<DataFrame> { /* ... */ }
  ```
  Calculate the distinct union of two [`DataFrame`]s.

- ```rust
  pub fn union_by_name_distinct(self: Self, dataframe: DataFrame) -> Result<DataFrame> { /* ... */ }
  ```
  Calculate the union of two [`DataFrame`]s using column names with all duplicated rows removed.

- ```rust
  pub fn distinct(self: Self) -> Result<DataFrame> { /* ... */ }
  ```
  Return a new `DataFrame` with all duplicated rows removed.

- ```rust
  pub fn distinct_on(self: Self, on_expr: Vec<Expr>, select_expr: Vec<Expr>, sort_expr: Option<Vec<SortExpr>>) -> Result<DataFrame> { /* ... */ }
  ```
  Return a new `DataFrame` with duplicated rows removed as per the specified expression list

- ```rust
  pub async fn describe(self: Self) -> Result<Self> { /* ... */ }
  ```
  Return a new `DataFrame` that has statistics for a DataFrame.

- ```rust
  pub fn sort_by(self: Self, expr: Vec<Expr>) -> Result<DataFrame> { /* ... */ }
  ```
  Apply a sort by provided expressions with default direction

- ```rust
  pub fn sort(self: Self, expr: Vec<SortExpr>) -> Result<DataFrame> { /* ... */ }
  ```
  Sort the DataFrame by the specified sorting expressions.

- ```rust
  pub fn join(self: Self, right: DataFrame, join_type: JoinType, left_cols: &[&str], right_cols: &[&str], filter: Option<Expr>) -> Result<DataFrame> { /* ... */ }
  ```
  Join this `DataFrame` with another `DataFrame` using explicitly specified

- ```rust
  pub fn join_on</* synthetic */ impl IntoIterator<Item = Expr>: IntoIterator<Item = Expr>>(self: Self, right: DataFrame, join_type: JoinType, on_exprs: impl IntoIterator<Item = Expr>) -> Result<DataFrame> { /* ... */ }
  ```
  Join this `DataFrame` with another `DataFrame` using the specified

- ```rust
  pub fn repartition(self: Self, partitioning_scheme: Partitioning) -> Result<DataFrame> { /* ... */ }
  ```
  Repartition a DataFrame based on a logical partitioning scheme.

- ```rust
  pub async fn count(self: Self) -> Result<usize> { /* ... */ }
  ```
  Return the total number of rows in this `DataFrame`.

- ```rust
  pub async fn collect(self: Self) -> Result<Vec<RecordBatch>> { /* ... */ }
  ```
  Execute this `DataFrame` and buffer all resulting `RecordBatch`es  into memory.

- ```rust
  pub async fn show(self: Self) -> Result<()> { /* ... */ }
  ```
  Execute the `DataFrame` and print the results to the console.

- ```rust
  pub async fn show_limit(self: Self, num: usize) -> Result<()> { /* ... */ }
  ```
  Execute the `DataFrame` and print only the first `num` rows of the

- ```rust
  pub fn task_ctx(self: &Self) -> TaskContext { /* ... */ }
  ```
  Return a new [`TaskContext`] which would be used to execute this DataFrame

- ```rust
  pub async fn execute_stream(self: Self) -> Result<SendableRecordBatchStream> { /* ... */ }
  ```
  Executes this DataFrame and returns a stream over a single partition

- ```rust
  pub async fn collect_partitioned(self: Self) -> Result<Vec<Vec<RecordBatch>>> { /* ... */ }
  ```
  Executes this DataFrame and collects all results into a vector of vector of RecordBatch

- ```rust
  pub async fn execute_stream_partitioned(self: Self) -> Result<Vec<SendableRecordBatchStream>> { /* ... */ }
  ```
  Executes this DataFrame and returns one stream per partition.

- ```rust
  pub fn schema(self: &Self) -> &DFSchema { /* ... */ }
  ```
  Returns the `DFSchema` describing the output of this DataFrame.

- ```rust
  pub fn logical_plan(self: &Self) -> &LogicalPlan { /* ... */ }
  ```
  Return a reference to the unoptimized [`LogicalPlan`] that comprises

- ```rust
  pub fn into_parts(self: Self) -> (SessionState, LogicalPlan) { /* ... */ }
  ```
  Returns both the [`LogicalPlan`] and [`SessionState`] that comprise this [`DataFrame`]

- ```rust
  pub fn into_unoptimized_plan(self: Self) -> LogicalPlan { /* ... */ }
  ```
  Return the [`LogicalPlan`] represented by this DataFrame without running

- ```rust
  pub fn into_optimized_plan(self: Self) -> Result<LogicalPlan> { /* ... */ }
  ```
  Return the optimized [`LogicalPlan`] represented by this DataFrame.

- ```rust
  pub fn into_view(self: Self) -> Arc<dyn TableProvider> { /* ... */ }
  ```
  Converts this [`DataFrame`] into a [`TableProvider`] that can be registered

- ```rust
  pub fn explain(self: Self, verbose: bool, analyze: bool) -> Result<DataFrame> { /* ... */ }
  ```
  Return a DataFrame with the explanation of its plan so far.

- ```rust
  pub fn registry(self: &Self) -> &dyn FunctionRegistry { /* ... */ }
  ```
  Return a `FunctionRegistry` used to plan udf's calls

- ```rust
  pub fn intersect(self: Self, dataframe: DataFrame) -> Result<DataFrame> { /* ... */ }
  ```
  Calculate the intersection of two [`DataFrame`]s.  The two [`DataFrame`]s must have exactly the same schema

- ```rust
  pub fn except(self: Self, dataframe: DataFrame) -> Result<DataFrame> { /* ... */ }
  ```
  Calculate the exception of two [`DataFrame`]s.  The two [`DataFrame`]s must have exactly the same schema

- ```rust
  pub async fn write_table(self: Self, table_name: &str, write_options: DataFrameWriteOptions) -> Result<Vec<RecordBatch>, DataFusionError> { /* ... */ }
  ```
  Execute this `DataFrame` and write the results to `table_name`.

- ```rust
  pub async fn write_csv(self: Self, path: &str, options: DataFrameWriteOptions, writer_options: Option<CsvOptions>) -> Result<Vec<RecordBatch>, DataFusionError> { /* ... */ }
  ```
  Execute the `DataFrame` and write the results to CSV file(s).

- ```rust
  pub async fn write_json(self: Self, path: &str, options: DataFrameWriteOptions, writer_options: Option<JsonOptions>) -> Result<Vec<RecordBatch>, DataFusionError> { /* ... */ }
  ```
  Execute the `DataFrame` and write the results to JSON file(s).

- ```rust
  pub fn with_column(self: Self, name: &str, expr: Expr) -> Result<DataFrame> { /* ... */ }
  ```
  Add or replace a column in the DataFrame.

- ```rust
  pub fn with_column_renamed</* synthetic */ impl Into<String>: Into<String>>(self: Self, old_name: impl Into<String>, new_name: &str) -> Result<DataFrame> { /* ... */ }
  ```
  Rename one column by applying a new projection. This is a no-op if the column to be

- ```rust
  pub fn with_param_values</* synthetic */ impl Into<ParamValues>: Into<ParamValues>>(self: Self, query_values: impl Into<ParamValues>) -> Result<Self> { /* ... */ }
  ```
  Replace all parameters in logical plan with the specified

- ```rust
  pub async fn cache(self: Self) -> Result<DataFrame> { /* ... */ }
  ```
  Cache DataFrame as a memory table.

- ```rust
  pub fn alias(self: Self, alias: &str) -> Result<DataFrame> { /* ... */ }
  ```
  Apply an alias to the DataFrame.

- ```rust
  pub fn fill_null(self: &Self, value: ScalarValue, columns: Vec<String>) -> Result<DataFrame> { /* ... */ }
  ```
  Fill null values in specified columns with a given value

###### Trait Implementations

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Same**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DataFrame { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ErasedDestructor**
- **Send**
- **Unpin**
- **MaybeSendSync**
- **IntoEither**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

## Module `datasource`

DataFusion data sources: [`TableProvider`] and [`ListingTable`]

[`ListingTable`]: crate::datasource::listing::ListingTable

```rust
pub mod datasource { /* ... */ }
```

### Modules

## Module `dynamic_file`

dynamic_file_schema contains an [`UrlTableFactory`] implementation that
can create a [`ListingTable`] from the given url.

```rust
pub mod dynamic_file { /* ... */ }
```

### Types

#### Struct `DynamicListTableFactory`

[DynamicListTableFactory] is a factory that can create a [ListingTable] from the given url.

```rust
pub struct DynamicListTableFactory {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub fn new(session_store: SessionStore) -> Self { /* ... */ }
  ```
  Create a new [DynamicListTableFactory] with the given state store.

- ```rust
  pub fn session_store(self: &Self) -> &SessionStore { /* ... */ }
  ```
  Get the session store.

###### Trait Implementations

- **Freeze**
- **RefUnwindSafe**
- **Sync**
- **MaybeSendSync**
- **Default**
  - ```rust
    fn default() -> DynamicListTableFactory { /* ... */ }
    ```

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ErasedDestructor**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Same**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UrlTableFactory**
  - ```rust
    fn try_new<''life0, ''life1, ''async_trait>(self: &''life0 Self, url: &''life1 str) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Option<Arc<dyn TableProvider>>>> + ::core::marker::Send + ''async_trait>>
where
    Self: ''async_trait,
    ''life0: ''async_trait,
    ''life1: ''async_trait { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

## Module `empty`

[`EmptyTable`] useful for testing.

```rust
pub mod empty { /* ... */ }
```

### Types

#### Struct `EmptyTable`

An empty plan that is useful for testing and generating plans
without mapping them to actual data.

```rust
pub struct EmptyTable {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub fn new(schema: SchemaRef) -> Self { /* ... */ }
  ```
  Initialize a new `EmptyTable` from a schema.

- ```rust
  pub fn with_partitions(self: Self, partitions: usize) -> Self { /* ... */ }
  ```
  Creates a new EmptyTable with specified partition number.

###### Trait Implementations

- **Allocation**
- **UnwindSafe**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Same**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ErasedDestructor**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **MaybeSendSync**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **IntoEither**
- **TableProvider**
  - ```rust
    fn as_any(self: &Self) -> &dyn Any { /* ... */ }
    ```

  - ```rust
    fn schema(self: &Self) -> SchemaRef { /* ... */ }
    ```

  - ```rust
    fn table_type(self: &Self) -> TableType { /* ... */ }
    ```

  - ```rust
    fn scan<''life0, ''life1, ''life2, ''life3, ''async_trait>(self: &''life0 Self, _state: &''life1 dyn Session, projection: Option<&''life2 Vec<usize>>, _filters: &''life3 [Expr], _limit: Option<usize>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Arc<dyn ExecutionPlan>>> + ::core::marker::Send + ''async_trait>>
where
    Self: ''async_trait,
    ''life0: ''async_trait,
    ''life1: ''async_trait,
    ''life2: ''async_trait,
    ''life3: ''async_trait { /* ... */ }
    ```

- **Freeze**
## Module `file_format`

Module containing helper methods for the various file formats
See write.rs for write related helper methods

```rust
pub mod file_format { /* ... */ }
```

### Modules

## Module `arrow`

[`ArrowFormat`]: Apache Arrow [`FileFormat`] abstractions

Works with files following the [Arrow IPC format](https://arrow.apache.org/docs/format/Columnar.html#ipc-file-format)

```rust
pub mod arrow { /* ... */ }
```

### Types

#### Struct `ArrowFormatFactory`

Factory struct used to create [ArrowFormat]

```rust
pub struct ArrowFormatFactory;
```

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  Creates an instance of [ArrowFormatFactory]

###### Trait Implementations

- **Default**
  - ```rust
    fn default() -> ArrowFormatFactory { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **MaybeSendSync**
- **FileFormatFactory**
  - ```rust
    fn create(self: &Self, _state: &dyn Session, _format_options: &HashMap<String, String>) -> Result<Arc<dyn FileFormat>> { /* ... */ }
    ```

  - ```rust
    fn default(self: &Self) -> Arc<dyn FileFormat> { /* ... */ }
    ```

  - ```rust
    fn as_any(self: &Self) -> &dyn Any { /* ... */ }
    ```

- **GetExt**
  - ```rust
    fn get_ext(self: &Self) -> String { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **IntoEither**
- **Send**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **ErasedDestructor**
- **UnwindSafe**
- **Same**
- **Allocation**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `ArrowFormat`

Arrow `FileFormat` implementation.

```rust
pub struct ArrowFormat;
```

##### Implementations

###### Trait Implementations

- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **MaybeSendSync**
- **Unpin**
- **Default**
  - ```rust
    fn default() -> ArrowFormat { /* ... */ }
    ```

- **Same**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **IntoEither**
- **Freeze**
- **FileFormat**
  - ```rust
    fn as_any(self: &Self) -> &dyn Any { /* ... */ }
    ```

  - ```rust
    fn get_ext(self: &Self) -> String { /* ... */ }
    ```

  - ```rust
    fn get_ext_with_compression(self: &Self, file_compression_type: &FileCompressionType) -> Result<String> { /* ... */ }
    ```

  - ```rust
    fn infer_schema<''life0, ''life1, ''life2, ''life3, ''async_trait>(self: &''life0 Self, _state: &''life1 dyn Session, store: &''life2 Arc<dyn ObjectStore>, objects: &''life3 [ObjectMeta]) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<SchemaRef>> + ::core::marker::Send + ''async_trait>>
where
    Self: ''async_trait,
    ''life0: ''async_trait,
    ''life1: ''async_trait,
    ''life2: ''async_trait,
    ''life3: ''async_trait { /* ... */ }
    ```

  - ```rust
    fn infer_stats<''life0, ''life1, ''life2, ''life3, ''async_trait>(self: &''life0 Self, _state: &''life1 dyn Session, _store: &''life2 Arc<dyn ObjectStore>, table_schema: SchemaRef, _object: &''life3 ObjectMeta) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Statistics>> + ::core::marker::Send + ''async_trait>>
where
    Self: ''async_trait,
    ''life0: ''async_trait,
    ''life1: ''async_trait,
    ''life2: ''async_trait,
    ''life3: ''async_trait { /* ... */ }
    ```

  - ```rust
    fn create_physical_plan<''life0, ''life1, ''life2, ''async_trait>(self: &''life0 Self, _state: &''life1 dyn Session, conf: FileScanConfig, _filters: Option<&''life2 Arc<dyn PhysicalExpr>>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Arc<dyn ExecutionPlan>>> + ::core::marker::Send + ''async_trait>>
where
    Self: ''async_trait,
    ''life0: ''async_trait,
    ''life1: ''async_trait,
    ''life2: ''async_trait { /* ... */ }
    ```

  - ```rust
    fn create_writer_physical_plan<''life0, ''life1, ''async_trait>(self: &''life0 Self, input: Arc<dyn ExecutionPlan>, _state: &''life1 dyn Session, conf: FileSinkConfig, order_requirements: Option<LexRequirement>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Arc<dyn ExecutionPlan>>> + ::core::marker::Send + ''async_trait>>
where
    Self: ''async_trait,
    ''life0: ''async_trait,
    ''life1: ''async_trait { /* ... */ }
    ```

  - ```rust
    fn file_source(self: &Self) -> Arc<dyn FileSource> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Allocation**
- **ErasedDestructor**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

## Module `csv`

Re-exports the [`datafusion_datasource_csv::file_format`] module, and contains tests for it.

```rust
pub mod csv { /* ... */ }
```

### Re-exports

#### Re-export `datafusion_datasource_csv::file_format::*`

```rust
pub use datafusion_datasource_csv::file_format::*;
```

## Module `json`

Re-exports the [`datafusion_datasource_json::file_format`] module, and contains tests for it.

```rust
pub mod json { /* ... */ }
```

### Re-exports

#### Re-export `datafusion_datasource_json::file_format::*`

```rust
pub use datafusion_datasource_json::file_format::*;
```

## Module `parquet`

**Attributes:**

- `#[cfg(feature = "parquet")]`

Re-exports the [`datafusion_datasource_parquet::file_format`] module, and contains tests for it.

```rust
pub mod parquet { /* ... */ }
```

### Re-exports

#### Re-export `datafusion_datasource_parquet::file_format::*`

```rust
pub use datafusion_datasource_parquet::file_format::*;
```

## Module `options`

User facing options for the file formats readers

```rust
pub mod options { /* ... */ }
```

### Types

#### Struct `CsvReadOptions`

Options that control the reading of CSV files.

Note this structure is supplied when a datasource is created and
can not not vary from statement to statement. For settings that
can vary statement to statement see
[`ConfigOptions`](crate::config::ConfigOptions).

```rust
pub struct CsvReadOptions<''a> {
    pub has_header: bool,
    pub delimiter: u8,
    pub quote: u8,
    pub terminator: Option<u8>,
    pub escape: Option<u8>,
    pub comment: Option<u8>,
    pub newlines_in_values: bool,
    pub schema: Option<&''a arrow::datatypes::Schema>,
    pub schema_infer_max_records: usize,
    pub file_extension: &''a str,
    pub table_partition_cols: Vec<(String, arrow::datatypes::DataType)>,
    pub file_compression_type: crate::datasource::file_format::file_compression_type::FileCompressionType,
    pub file_sort_order: Vec<Vec<datafusion_expr::SortExpr>>,
    pub null_regex: Option<String>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `has_header` | `bool` | Does the CSV file have a header?<br><br>If schema inference is run on a file with no headers, default column names<br>are created. |
| `delimiter` | `u8` | An optional column delimiter. Defaults to `b','`. |
| `quote` | `u8` | An optional quote character. Defaults to `b'"'`. |
| `terminator` | `Option<u8>` | An optional terminator character. Defaults to None (CRLF). |
| `escape` | `Option<u8>` | An optional escape character. Defaults to None. |
| `comment` | `Option<u8>` | If enabled, lines beginning with this byte are ignored. |
| `newlines_in_values` | `bool` | Specifies whether newlines in (quoted) values are supported.<br><br>Parsing newlines in quoted values may be affected by execution behaviour such as<br>parallel file scanning. Setting this to `true` ensures that newlines in values are<br>parsed successfully, which may reduce performance.<br><br>The default behaviour depends on the `datafusion.catalog.newlines_in_values` setting. |
| `schema` | `Option<&''a arrow::datatypes::Schema>` | An optional schema representing the CSV files. If None, CSV reader will try to infer it<br>based on data in file. |
| `schema_infer_max_records` | `usize` | Max number of rows to read from CSV files for schema inference if needed. Defaults to `DEFAULT_SCHEMA_INFER_MAX_RECORD`. |
| `file_extension` | `&''a str` | File extension; only files with this extension are selected for data input.<br>Defaults to `FileType::CSV.get_ext().as_str()`. |
| `table_partition_cols` | `Vec<(String, arrow::datatypes::DataType)>` | Partition Columns |
| `file_compression_type` | `crate::datasource::file_format::file_compression_type::FileCompressionType` | File compression type |
| `file_sort_order` | `Vec<Vec<datafusion_expr::SortExpr>>` | Indicates how the file is sorted |
| `null_regex` | `Option<String>` | Optional regex to match null values |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  Create a CSV read option with default presets

- ```rust
  pub fn has_header(self: Self, has_header: bool) -> Self { /* ... */ }
  ```
  Configure has_header setting

- ```rust
  pub fn comment(self: Self, comment: u8) -> Self { /* ... */ }
  ```
  Specify comment char to use for CSV read

- ```rust
  pub fn delimiter(self: Self, delimiter: u8) -> Self { /* ... */ }
  ```
  Specify delimiter to use for CSV read

- ```rust
  pub fn quote(self: Self, quote: u8) -> Self { /* ... */ }
  ```
  Specify quote to use for CSV read

- ```rust
  pub fn terminator(self: Self, terminator: Option<u8>) -> Self { /* ... */ }
  ```
  Specify terminator to use for CSV read

- ```rust
  pub fn escape(self: Self, escape: u8) -> Self { /* ... */ }
  ```
  Specify delimiter to use for CSV read

- ```rust
  pub fn newlines_in_values(self: Self, newlines_in_values: bool) -> Self { /* ... */ }
  ```
  Specifies whether newlines in (quoted) values are supported.

- ```rust
  pub fn file_extension(self: Self, file_extension: &''a str) -> Self { /* ... */ }
  ```
  Specify the file extension for CSV file selection

- ```rust
  pub fn delimiter_option(self: Self, delimiter: Option<u8>) -> Self { /* ... */ }
  ```
  Configure delimiter setting with Option, None value will be ignored

- ```rust
  pub fn schema(self: Self, schema: &''a Schema) -> Self { /* ... */ }
  ```
  Specify schema to use for CSV read

- ```rust
  pub fn table_partition_cols(self: Self, table_partition_cols: Vec<(String, DataType)>) -> Self { /* ... */ }
  ```
  Specify table_partition_cols for partition pruning

- ```rust
  pub fn schema_infer_max_records(self: Self, max_records: usize) -> Self { /* ... */ }
  ```
  Configure number of max records to read for schema inference

- ```rust
  pub fn file_compression_type(self: Self, file_compression_type: FileCompressionType) -> Self { /* ... */ }
  ```
  Configure file compression type

- ```rust
  pub fn file_sort_order(self: Self, file_sort_order: Vec<Vec<SortExpr>>) -> Self { /* ... */ }
  ```
  Configure if file has known sort order

- ```rust
  pub fn null_regex(self: Self, null_regex: Option<String>) -> Self { /* ... */ }
  ```
  Configure the null parsing regex.

###### Trait Implementations

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **ReadOptions**
  - ```rust
    fn to_listing_options(self: &Self, config: &SessionConfig, table_options: TableOptions) -> ListingOptions { /* ... */ }
    ```

  - ```rust
    fn get_resolved_schema<''life0, ''life1, ''async_trait>(self: &''life0 Self, config: &''life1 SessionConfig, state: SessionState, table_path: ListingTableUrl) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<SchemaRef>> + ::core::marker::Send + ''async_trait>>
where
    Self: ''async_trait,
    ''life0: ''async_trait,
    ''life1: ''async_trait { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **IntoEither**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **MaybeSendSync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> CsvReadOptions<''a> { /* ... */ }
    ```

- **Unpin**
- **ErasedDestructor**
- **Same**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `ParquetReadOptions`

Options that control the reading of Parquet files.

Note this structure is supplied when a datasource is created and
can not not vary from statement to statement. For settings that
can vary statement to statement see
[`ConfigOptions`](crate::config::ConfigOptions).

```rust
pub struct ParquetReadOptions<''a> {
    pub file_extension: &''a str,
    pub table_partition_cols: Vec<(String, arrow::datatypes::DataType)>,
    pub parquet_pruning: Option<bool>,
    pub skip_metadata: Option<bool>,
    pub schema: Option<&''a arrow::datatypes::Schema>,
    pub file_sort_order: Vec<Vec<datafusion_expr::SortExpr>>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `file_extension` | `&''a str` | File extension; only files with this extension are selected for data input.<br>Defaults to ".parquet". |
| `table_partition_cols` | `Vec<(String, arrow::datatypes::DataType)>` | Partition Columns |
| `parquet_pruning` | `Option<bool>` | Should the parquet reader use the predicate to prune row groups?<br>If None, uses value in SessionConfig |
| `skip_metadata` | `Option<bool>` | Should the parquet reader to skip any metadata that may be in<br>the file Schema? This can help avoid schema conflicts due to<br>metadata.<br><br>If None specified, uses value in SessionConfig |
| `schema` | `Option<&''a arrow::datatypes::Schema>` | An optional schema representing the parquet files. If None, parquet reader will try to infer it<br>based on data in file. |
| `file_sort_order` | `Vec<Vec<datafusion_expr::SortExpr>>` | Indicates how the file is sorted |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  Create a new ParquetReadOptions with default values

- ```rust
  pub fn file_extension(self: Self, file_extension: &''a str) -> Self { /* ... */ }
  ```
  Specify file_extension

- ```rust
  pub fn parquet_pruning(self: Self, parquet_pruning: bool) -> Self { /* ... */ }
  ```
  Specify parquet_pruning

- ```rust
  pub fn skip_metadata(self: Self, skip_metadata: bool) -> Self { /* ... */ }
  ```
  Tell the parquet reader to skip any metadata that may be in

- ```rust
  pub fn schema(self: Self, schema: &''a Schema) -> Self { /* ... */ }
  ```
  Specify schema to use for parquet read

- ```rust
  pub fn table_partition_cols(self: Self, table_partition_cols: Vec<(String, DataType)>) -> Self { /* ... */ }
  ```
  Specify table_partition_cols for partition pruning

- ```rust
  pub fn file_sort_order(self: Self, file_sort_order: Vec<Vec<SortExpr>>) -> Self { /* ... */ }
  ```
  Configure if file has known sort order

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ParquetReadOptions<''a> { /* ... */ }
    ```

- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Same**
- **ReadOptions**
  - ```rust
    fn to_listing_options(self: &Self, config: &SessionConfig, table_options: TableOptions) -> ListingOptions { /* ... */ }
    ```

  - ```rust
    fn get_resolved_schema<''life0, ''life1, ''async_trait>(self: &''life0 Self, config: &''life1 SessionConfig, state: SessionState, table_path: ListingTableUrl) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<SchemaRef>> + ::core::marker::Send + ''async_trait>>
where
    Self: ''async_trait,
    ''life0: ''async_trait,
    ''life1: ''async_trait { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **MaybeSendSync**
- **IntoEither**
- **ErasedDestructor**
- **Freeze**
- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `ArrowReadOptions`

Options that control the reading of ARROW files.

Note this structure is supplied when a datasource is created and
can not not vary from statement to statement. For settings that
can vary statement to statement see
[`ConfigOptions`](crate::config::ConfigOptions).

```rust
pub struct ArrowReadOptions<''a> {
    pub schema: Option<&''a arrow::datatypes::Schema>,
    pub file_extension: &''a str,
    pub table_partition_cols: Vec<(String, arrow::datatypes::DataType)>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `schema` | `Option<&''a arrow::datatypes::Schema>` | The data source schema. |
| `file_extension` | `&''a str` | File extension; only files with this extension are selected for data input.<br>Defaults to `FileType::ARROW.get_ext().as_str()`. |
| `table_partition_cols` | `Vec<(String, arrow::datatypes::DataType)>` | Partition Columns |

##### Implementations

###### Methods

- ```rust
  pub fn table_partition_cols(self: Self, table_partition_cols: Vec<(String, DataType)>) -> Self { /* ... */ }
  ```
  Specify table_partition_cols for partition pruning

- ```rust
  pub fn schema(self: Self, schema: &''a Schema) -> Self { /* ... */ }
  ```
  Specify schema to use for AVRO read

###### Trait Implementations

- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ArrowReadOptions<''a> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Send**
- **Same**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **UnwindSafe**
- **MaybeSendSync**
- **ReadOptions**
  - ```rust
    fn to_listing_options(self: &Self, config: &SessionConfig, _table_options: TableOptions) -> ListingOptions { /* ... */ }
    ```

  - ```rust
    fn get_resolved_schema<''life0, ''life1, ''async_trait>(self: &''life0 Self, config: &''life1 SessionConfig, state: SessionState, table_path: ListingTableUrl) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<SchemaRef>> + ::core::marker::Send + ''async_trait>>
where
    Self: ''async_trait,
    ''life0: ''async_trait,
    ''life1: ''async_trait { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Allocation**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ErasedDestructor**
#### Struct `AvroReadOptions`

Options that control the reading of AVRO files.

Note this structure is supplied when a datasource is created and
can not not vary from statement to statement. For settings that
can vary statement to statement see
[`ConfigOptions`](crate::config::ConfigOptions).

```rust
pub struct AvroReadOptions<''a> {
    pub schema: Option<&''a arrow::datatypes::Schema>,
    pub file_extension: &''a str,
    pub table_partition_cols: Vec<(String, arrow::datatypes::DataType)>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `schema` | `Option<&''a arrow::datatypes::Schema>` | The data source schema. |
| `file_extension` | `&''a str` | File extension; only files with this extension are selected for data input.<br>Defaults to `FileType::AVRO.get_ext().as_str()`. |
| `table_partition_cols` | `Vec<(String, arrow::datatypes::DataType)>` | Partition Columns |

##### Implementations

###### Methods

- ```rust
  pub fn table_partition_cols(self: Self, table_partition_cols: Vec<(String, DataType)>) -> Self { /* ... */ }
  ```
  Specify table_partition_cols for partition pruning

- ```rust
  pub fn schema(self: Self, schema: &''a Schema) -> Self { /* ... */ }
  ```
  Specify schema to use for AVRO read

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Freeze**
- **Sync**
- **RefUnwindSafe**
- **IntoEither**
- **Allocation**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> AvroReadOptions<''a> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **MaybeSendSync**
- **Same**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ErasedDestructor**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

#### Struct `NdJsonReadOptions`

Options that control the reading of Line-delimited JSON files (NDJson)

Note this structure is supplied when a datasource is created and
can not not vary from statement to statement. For settings that
can vary statement to statement see
[`ConfigOptions`](crate::config::ConfigOptions).

```rust
pub struct NdJsonReadOptions<''a> {
    pub schema: Option<&''a arrow::datatypes::Schema>,
    pub schema_infer_max_records: usize,
    pub file_extension: &''a str,
    pub table_partition_cols: Vec<(String, arrow::datatypes::DataType)>,
    pub file_compression_type: crate::datasource::file_format::file_compression_type::FileCompressionType,
    pub infinite: bool,
    pub file_sort_order: Vec<Vec<datafusion_expr::SortExpr>>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `schema` | `Option<&''a arrow::datatypes::Schema>` | The data source schema. |
| `schema_infer_max_records` | `usize` | Max number of rows to read from JSON files for schema inference if needed. Defaults to `DEFAULT_SCHEMA_INFER_MAX_RECORD`. |
| `file_extension` | `&''a str` | File extension; only files with this extension are selected for data input.<br>Defaults to `FileType::JSON.get_ext().as_str()`. |
| `table_partition_cols` | `Vec<(String, arrow::datatypes::DataType)>` | Partition Columns |
| `file_compression_type` | `crate::datasource::file_format::file_compression_type::FileCompressionType` | File compression type |
| `infinite` | `bool` | Flag indicating whether this file may be unbounded (as in a FIFO file). |
| `file_sort_order` | `Vec<Vec<datafusion_expr::SortExpr>>` | Indicates how the file is sorted |

##### Implementations

###### Methods

- ```rust
  pub fn table_partition_cols(self: Self, table_partition_cols: Vec<(String, DataType)>) -> Self { /* ... */ }
  ```
  Specify table_partition_cols for partition pruning

- ```rust
  pub fn file_extension(self: Self, file_extension: &''a str) -> Self { /* ... */ }
  ```
  Specify file_extension

- ```rust
  pub fn mark_infinite(self: Self, infinite: bool) -> Self { /* ... */ }
  ```
  Configure mark_infinite setting

- ```rust
  pub fn file_compression_type(self: Self, file_compression_type: FileCompressionType) -> Self { /* ... */ }
  ```
  Specify file_compression_type

- ```rust
  pub fn schema(self: Self, schema: &''a Schema) -> Self { /* ... */ }
  ```
  Specify schema to use for NdJson read

- ```rust
  pub fn file_sort_order(self: Self, file_sort_order: Vec<Vec<SortExpr>>) -> Self { /* ... */ }
  ```
  Configure if file has known sort order

###### Trait Implementations

- **Send**
- **ErasedDestructor**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **IntoEither**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Same**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> NdJsonReadOptions<''a> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ReadOptions**
  - ```rust
    fn to_listing_options(self: &Self, config: &SessionConfig, table_options: TableOptions) -> ListingOptions { /* ... */ }
    ```

  - ```rust
    fn get_resolved_schema<''life0, ''life1, ''async_trait>(self: &''life0 Self, config: &''life1 SessionConfig, state: SessionState, table_path: ListingTableUrl) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<SchemaRef>> + ::core::marker::Send + ''async_trait>>
where
    Self: ''async_trait,
    ''life0: ''async_trait,
    ''life1: ''async_trait { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **MaybeSendSync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **Freeze**
### Traits

#### Trait `ReadOptions`

['ReadOptions'] is implemented by Options like ['CsvReadOptions'] that control the reading of respective files/sources.

```rust
pub trait ReadOptions<''a> {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `to_listing_options`: Helper to convert these user facing options to `ListingTable` options
- `get_resolved_schema`: Infer and resolve the schema from the files/sources provided.

##### Provided Methods

- ```rust
  fn _get_resolved_schema<''life0, ''async_trait>(self: &''a Self, config: &''life0 SessionConfig, state: SessionState, table_path: ListingTableUrl, schema: Option<&''a Schema>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<SchemaRef>> + ::core::marker::Send + ''async_trait>>
where
    Self: ::core::marker::Sync + ''async_trait,
    ''a: ''async_trait,
    ''life0: ''async_trait { /* ... */ }
  ```
  helper function to reduce repetitive code. Infers the schema from sources if not provided. Infinite data sources not supported through this function.

##### Implementations

This trait is implemented for the following types:

- `CsvReadOptions<''_>`
- `ParquetReadOptions<''_>`
- `NdJsonReadOptions<''_>`
- `ArrowReadOptions<''_>`

### Re-exports

#### Re-export `file_compression_type`

```rust
pub use datafusion_datasource::file_compression_type;
```

#### Re-export `write`

```rust
pub use datafusion_datasource::write;
```

#### Re-export `datafusion_datasource::file_format::*`

```rust
pub use datafusion_datasource::file_format::*;
```

## Module `listing`

A table that uses the `ObjectStore` listing capability
to get the list of files to process.

```rust
pub mod listing { /* ... */ }
```

### Re-exports

#### Re-export `helpers`

```rust
pub use datafusion_catalog_listing::helpers;
```

#### Re-export `FileRange`

```rust
pub use datafusion_datasource::FileRange;
```

#### Re-export `ListingTableUrl`

```rust
pub use datafusion_datasource::ListingTableUrl;
```

#### Re-export `PartitionedFile`

```rust
pub use datafusion_datasource::PartitionedFile;
```

#### Re-export `PartitionedFileStream`

```rust
pub use datafusion_datasource::PartitionedFileStream;
```

#### Re-export `ListingOptions`

```rust
pub use table::ListingOptions;
```

#### Re-export `ListingTable`

```rust
pub use table::ListingTable;
```

#### Re-export `ListingTableConfig`

```rust
pub use table::ListingTableConfig;
```

## Module `listing_table_factory`

Factory for creating ListingTables with default options

```rust
pub mod listing_table_factory { /* ... */ }
```

### Types

#### Struct `ListingTableFactory`

A `TableProviderFactory` capable of creating new `ListingTable`s

```rust
pub struct ListingTableFactory {
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  Creates a new `ListingTableFactory`

###### Trait Implementations

- **IntoEither**
- **Same**
- **Unpin**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **TableProviderFactory**
  - ```rust
    fn create<''life0, ''life1, ''life2, ''async_trait>(self: &''life0 Self, state: &''life1 dyn Session, cmd: &''life2 CreateExternalTable) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Arc<dyn TableProvider>>> + ::core::marker::Send + ''async_trait>>
where
    Self: ''async_trait,
    ''life0: ''async_trait,
    ''life1: ''async_trait,
    ''life2: ''async_trait { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> ListingTableFactory { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Allocation**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **MaybeSendSync**
## Module `physical_plan`

Execution plans that read file formats

```rust
pub mod physical_plan { /* ... */ }
```

### Modules

## Module `csv`

Reexports the [`datafusion_datasource_json::source`] module, containing CSV based [`FileSource`].

[`FileSource`]: datafusion_datasource::file::FileSource

```rust
pub mod csv { /* ... */ }
```

### Re-exports

#### Re-export `datafusion_datasource_csv::source::*`

```rust
pub use datafusion_datasource_csv::source::*;
```

## Module `json`

Reexports the [`datafusion_datasource_json::source`] module, containing JSON based [`FileSource`].

[`FileSource`]: datafusion_datasource::file::FileSource

```rust
pub mod json { /* ... */ }
```

### Re-exports

#### Re-export `datafusion_datasource_json::source::*`

**Attributes:**

- `#[allow(deprecated)]`

```rust
pub use datafusion_datasource_json::source::*;
```

## Module `parquet`

**Attributes:**

- `#[cfg(feature = "parquet")]`

Reexports the [`datafusion_datasource_parquet`] crate, containing Parquet based [`FileSource`].

[`FileSource`]: datafusion_datasource::file::FileSource

```rust
pub mod parquet { /* ... */ }
```

### Re-exports

#### Re-export `datafusion_datasource_parquet::*`

```rust
pub use datafusion_datasource_parquet::*;
```

### Re-exports

#### Re-export `ParquetSource`

**Attributes:**

- `#[cfg(feature = "parquet")]`

```rust
pub use datafusion_datasource_parquet::source::ParquetSource;
```

#### Re-export `ParquetExec`

**Attributes:**

- `#[cfg(feature = "parquet")]`
- `#[allow(deprecated)]`

```rust
pub use datafusion_datasource_parquet::ParquetExec;
```

#### Re-export `ParquetExecBuilder`

**Attributes:**

- `#[cfg(feature = "parquet")]`
- `#[allow(deprecated)]`

```rust
pub use datafusion_datasource_parquet::ParquetExecBuilder;
```

#### Re-export `ParquetFileMetrics`

**Attributes:**

- `#[cfg(feature = "parquet")]`
- `#[allow(deprecated)]`

```rust
pub use datafusion_datasource_parquet::ParquetFileMetrics;
```

#### Re-export `ParquetFileReaderFactory`

**Attributes:**

- `#[cfg(feature = "parquet")]`
- `#[allow(deprecated)]`

```rust
pub use datafusion_datasource_parquet::ParquetFileReaderFactory;
```

#### Re-export `ArrowExec`

**Attributes:**

- `#[allow(deprecated)]`

```rust
pub use arrow_file::ArrowExec;
```

#### Re-export `ArrowSource`

```rust
pub use arrow_file::ArrowSource;
```

#### Re-export `NdJsonExec`

**Attributes:**

- `#[allow(deprecated)]`

```rust
pub use json::NdJsonExec;
```

#### Re-export `JsonOpener`

```rust
pub use json::JsonOpener;
```

#### Re-export `JsonSource`

```rust
pub use json::JsonSource;
```

#### Re-export `CsvExec`

**Attributes:**

- `#[allow(deprecated)]`

```rust
pub use csv::CsvExec;
```

#### Re-export `CsvExecBuilder`

**Attributes:**

- `#[allow(deprecated)]`

```rust
pub use csv::CsvExecBuilder;
```

#### Re-export `CsvOpener`

```rust
pub use csv::CsvOpener;
```

#### Re-export `CsvSource`

```rust
pub use csv::CsvSource;
```

#### Re-export `FileSource`

```rust
pub use datafusion_datasource::file::FileSource;
```

#### Re-export `FileGroup`

```rust
pub use datafusion_datasource::file_groups::FileGroup;
```

#### Re-export `FileGroupPartitioner`

```rust
pub use datafusion_datasource::file_groups::FileGroupPartitioner;
```

#### Re-export `FileMeta`

```rust
pub use datafusion_datasource::file_meta::FileMeta;
```

#### Re-export `wrap_partition_type_in_dict`

```rust
pub use datafusion_datasource::file_scan_config::wrap_partition_type_in_dict;
```

#### Re-export `wrap_partition_value_in_dict`

```rust
pub use datafusion_datasource::file_scan_config::wrap_partition_value_in_dict;
```

#### Re-export `FileScanConfig`

```rust
pub use datafusion_datasource::file_scan_config::FileScanConfig;
```

#### Re-export `FileScanConfigBuilder`

```rust
pub use datafusion_datasource::file_scan_config::FileScanConfigBuilder;
```

#### Re-export `FileOpenFuture`

```rust
pub use datafusion_datasource::file_stream::FileOpenFuture;
```

#### Re-export `FileOpener`

```rust
pub use datafusion_datasource::file_stream::FileOpener;
```

#### Re-export `FileStream`

```rust
pub use datafusion_datasource::file_stream::FileStream;
```

#### Re-export `OnError`

```rust
pub use datafusion_datasource::file_stream::OnError;
```

#### Re-export `datafusion_datasource::file_sink_config::*`

```rust
pub use datafusion_datasource::file_sink_config::*;
```

## Module `provider`

Data source traits

```rust
pub mod provider { /* ... */ }
```

### Types

#### Struct `DefaultTableFactory`

The default [`TableProviderFactory`]

If [`CreateExternalTable`] is unbounded calls [`StreamTableFactory::create`],
otherwise calls [`ListingTableFactory::create`]

```rust
pub struct DefaultTableFactory {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  Creates a new [`DefaultTableFactory`]

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Allocation**
- **Same**
- **Default**
  - ```rust
    fn default() -> DefaultTableFactory { /* ... */ }
    ```

- **Send**
- **ErasedDestructor**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **IntoEither**
- **TableProviderFactory**
  - ```rust
    fn create<''life0, ''life1, ''life2, ''async_trait>(self: &''life0 Self, state: &''life1 dyn Session, cmd: &''life2 CreateExternalTable) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Arc<dyn TableProvider>>> + ::core::marker::Send + ''async_trait>>
where
    Self: ''async_trait,
    ''life0: ''async_trait,
    ''life1: ''async_trait,
    ''life2: ''async_trait { /* ... */ }
    ```

- **MaybeSendSync**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

### Re-exports

#### Re-export `TableProviderFilterPushDown`

```rust
pub use datafusion_expr::TableProviderFilterPushDown;
```

#### Re-export `TableType`

```rust
pub use datafusion_expr::TableType;
```

### Re-exports

#### Re-export `provider_as_source`

```rust
pub use self::default_table_source::provider_as_source;
```

#### Re-export `source_as_provider`

```rust
pub use self::default_table_source::source_as_provider;
```

#### Re-export `DefaultTableSource`

```rust
pub use self::default_table_source::DefaultTableSource;
```

#### Re-export `MemTable`

```rust
pub use self::memory::MemTable;
```

#### Re-export `ViewTable`

```rust
pub use self::view::ViewTable;
```

#### Re-export `TableProvider`

```rust
pub use crate::catalog::TableProvider;
```

#### Re-export `TableType`

```rust
pub use crate::logical_expr::TableType;
```

#### Re-export `cte_worktable`

```rust
pub use datafusion_catalog::cte_worktable;
```

#### Re-export `default_table_source`

```rust
pub use datafusion_catalog::default_table_source;
```

#### Re-export `memory`

```rust
pub use datafusion_catalog::memory;
```

#### Re-export `stream`

```rust
pub use datafusion_catalog::stream;
```

#### Re-export `view`

```rust
pub use datafusion_catalog::view;
```

#### Re-export `schema_adapter`

```rust
pub use datafusion_datasource::schema_adapter;
```

#### Re-export `sink`

```rust
pub use datafusion_datasource::sink;
```

#### Re-export `source`

```rust
pub use datafusion_datasource::source;
```

#### Re-export `object_store`

```rust
pub use datafusion_execution::object_store;
```

#### Re-export `create_ordering`

```rust
pub use datafusion_physical_expr::create_ordering;
```

## Module `error`

DataFusion error type [`DataFusionError`] and [`Result`].

```rust
pub mod error { /* ... */ }
```

### Re-exports

#### Re-export `DataFusionError`

```rust
pub use datafusion_common::DataFusionError;
```

#### Re-export `Result`

```rust
pub use datafusion_common::Result;
```

#### Re-export `SharedResult`

```rust
pub use datafusion_common::SharedResult;
```

## Module `execution`

Shared state for query planning and execution.

```rust
pub mod execution { /* ... */ }
```

### Modules

## Module `context`

[`SessionContext`] API for registering data sources and executing queries

```rust
pub mod context { /* ... */ }
```

### Types

#### Struct `SessionContext`

Main interface for executing queries with DataFusion. Maintains
the state of the connection between a user and an instance of the
DataFusion engine.

See examples below for how to use the `SessionContext` to execute queries
and how to configure the session.

# Overview

[`SessionContext`] provides the following functionality:

* Create a [`DataFrame`] from a CSV or Parquet data source.
* Register a CSV or Parquet data source as a table that can be referenced from a SQL query.
* Register a custom data source that can be referenced from a SQL query.
* Execution a SQL query

# Example: DataFrame API

The following example demonstrates how to use the context to execute a query against a CSV
data source using the [`DataFrame`] API:

```
use datafusion::prelude::*;
# use datafusion::functions_aggregate::expr_fn::min;
# use datafusion::{error::Result, assert_batches_eq};
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
let df = ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?;
let df = df.filter(col("a").lt_eq(col("b")))?
           .aggregate(vec![col("a")], vec![min(col("b"))])?
           .limit(0, Some(100))?;
let results = df
  .collect()
  .await?;
assert_batches_eq!(
 &[
   "+---+----------------+",
   "| a | min(?table?.b) |",
   "+---+----------------+",
   "| 1 | 2              |",
   "+---+----------------+",
 ],
 &results
);
# Ok(())
# }
```

# Example: SQL API

The following example demonstrates how to execute the same query using SQL:

```
use datafusion::prelude::*;
# use datafusion::{error::Result, assert_batches_eq};
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
ctx.register_csv("example", "tests/data/example.csv", CsvReadOptions::new()).await?;
let results = ctx
  .sql("SELECT a, min(b) FROM example GROUP BY a LIMIT 100")
  .await?
  .collect()
  .await?;
assert_batches_eq!(
 &[
   "+---+----------------+",
   "| a | min(example.b) |",
   "+---+----------------+",
   "| 1 | 2              |",
   "+---+----------------+",
 ],
 &results
);
# Ok(())
# }
```

# Example: Configuring `SessionContext`

The `SessionContext` can be configured by creating a [`SessionState`] using
[`SessionStateBuilder`]:

```
# use std::sync::Arc;
# use datafusion::prelude::*;
# use datafusion::execution::SessionStateBuilder;
# use datafusion_execution::runtime_env::RuntimeEnvBuilder;
// Configure a 4k batch size
let config = SessionConfig::new() .with_batch_size(4 * 1024);

// configure a memory limit of 1GB with 20%  slop
 let runtime_env = RuntimeEnvBuilder::new()
    .with_memory_limit(1024 * 1024 * 1024, 0.80)
    .build_arc()
    .unwrap();

// Create a SessionState using the config and runtime_env
let state = SessionStateBuilder::new()
  .with_config(config)
  .with_runtime_env(runtime_env)
  // include support for built in functions and configurations
  .with_default_features()
  .build();

// Create a SessionContext
let ctx = SessionContext::from(state);
```

# Relationship between `SessionContext`, `SessionState`, and `TaskContext`

The state required to optimize, and evaluate queries is
broken into three levels to allow tailoring

The objects are:

1. [`SessionContext`]: Most users should use a `SessionContext`. It contains
   all information required to execute queries including  high level APIs such
   as [`SessionContext::sql`]. All queries run with the same `SessionContext`
   share the same configuration and resources (e.g. memory limits).

2. [`SessionState`]: contains information required to plan and execute an
   individual query (e.g. creating a [`LogicalPlan`] or [`ExecutionPlan`]).
   Each query is planned and executed using its own `SessionState`, which can
   be created with [`SessionContext::state`]. `SessionState` allows finer
   grained control over query execution, for example disallowing DDL operations
   such as `CREATE TABLE`.

3. [`TaskContext`] contains the state required for query execution (e.g.
   [`ExecutionPlan::execute`]). It contains a subset of information in
   [`SessionState`]. `TaskContext` allows executing [`ExecutionPlan`]s
   [`PhysicalExpr`]s without requiring a full [`SessionState`].

[`PhysicalExpr`]: crate::physical_expr::PhysicalExpr

```rust
pub struct SessionContext {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub async fn read_csv<P: DataFilePaths>(self: &Self, table_paths: P, options: CsvReadOptions<''_>) -> Result<DataFrame> { /* ... */ }
  ```
  Creates a [`DataFrame`] for reading a CSV data source.

- ```rust
  pub async fn register_csv</* synthetic */ impl Into<TableReference>: Into<TableReference>, /* synthetic */ impl AsRef<str>: AsRef<str>>(self: &Self, table_ref: impl Into<TableReference>, table_path: impl AsRef<str>, options: CsvReadOptions<''_>) -> Result<()> { /* ... */ }
  ```
  Registers a CSV file as a table which can referenced from SQL

- ```rust
  pub async fn write_csv</* synthetic */ impl AsRef<str>: AsRef<str>>(self: &Self, plan: Arc<dyn ExecutionPlan>, path: impl AsRef<str>) -> Result<()> { /* ... */ }
  ```
  Executes a query and writes the results to a partitioned CSV file.

- ```rust
  pub async fn read_json<P: DataFilePaths>(self: &Self, table_paths: P, options: NdJsonReadOptions<''_>) -> Result<DataFrame> { /* ... */ }
  ```
  Creates a [`DataFrame`] for reading an JSON data source.

- ```rust
  pub async fn register_json</* synthetic */ impl Into<TableReference>: Into<TableReference>, /* synthetic */ impl AsRef<str>: AsRef<str>>(self: &Self, table_ref: impl Into<TableReference>, table_path: impl AsRef<str>, options: NdJsonReadOptions<''_>) -> Result<()> { /* ... */ }
  ```
  Registers a JSON file as a table that it can be referenced

- ```rust
  pub async fn write_json</* synthetic */ impl AsRef<str>: AsRef<str>>(self: &Self, plan: Arc<dyn ExecutionPlan>, path: impl AsRef<str>) -> Result<()> { /* ... */ }
  ```
  Executes a query and writes the results to a partitioned JSON file.

- ```rust
  pub async fn read_parquet<P: DataFilePaths>(self: &Self, table_paths: P, options: ParquetReadOptions<''_>) -> Result<DataFrame> { /* ... */ }
  ```
  Creates a [`DataFrame`] for reading a Parquet data source.

- ```rust
  pub async fn register_parquet</* synthetic */ impl Into<TableReference>: Into<TableReference>, /* synthetic */ impl AsRef<str>: AsRef<str>>(self: &Self, table_ref: impl Into<TableReference>, table_path: impl AsRef<str>, options: ParquetReadOptions<''_>) -> Result<()> { /* ... */ }
  ```
  Registers a Parquet file as a table that can be referenced from SQL

- ```rust
  pub async fn write_parquet</* synthetic */ impl AsRef<str>: AsRef<str>>(self: &Self, plan: Arc<dyn ExecutionPlan>, path: impl AsRef<str>, writer_properties: Option<WriterProperties>) -> Result<()> { /* ... */ }
  ```
  Executes a query and writes the results to a partitioned Parquet file.

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  Creates a new `SessionContext` using the default [`SessionConfig`].

- ```rust
  pub async fn refresh_catalogs(self: &Self) -> Result<()> { /* ... */ }
  ```
  Finds any [`ListingSchemaProvider`]s and instructs them to reload tables from "disk"

- ```rust
  pub fn new_with_config(config: SessionConfig) -> Self { /* ... */ }
  ```
  Creates a new `SessionContext` using the provided

- ```rust
  pub fn new_with_config_rt(config: SessionConfig, runtime: Arc<RuntimeEnv>) -> Self { /* ... */ }
  ```
  Creates a new `SessionContext` using the provided

- ```rust
  pub fn new_with_state(state: SessionState) -> Self { /* ... */ }
  ```
  Creates a new `SessionContext` using the provided [`SessionState`]

- ```rust
  pub fn enable_url_table(self: Self) -> Self { /* ... */ }
  ```
  Enable querying local files as tables.

- ```rust
  pub fn into_state_builder(self: Self) -> SessionStateBuilder { /* ... */ }
  ```
  Convert the current `SessionContext` into a [`SessionStateBuilder`]

- ```rust
  pub fn session_start_time(self: &Self) -> DateTime<Utc> { /* ... */ }
  ```
  Returns the time this `SessionContext` was created

- ```rust
  pub fn with_function_factory(self: Self, function_factory: Arc<dyn FunctionFactory>) -> Self { /* ... */ }
  ```
  Registers a [`FunctionFactory`] to handle `CREATE FUNCTION` statements

- ```rust
  pub fn add_optimizer_rule(self: &Self, optimizer_rule: Arc<dyn OptimizerRule + Send + Sync>) { /* ... */ }
  ```
  Adds an optimizer rule to the end of the existing rules.

- ```rust
  pub fn add_analyzer_rule(self: &Self, analyzer_rule: Arc<dyn AnalyzerRule + Send + Sync>) { /* ... */ }
  ```
  Adds an analyzer rule to the end of the existing rules.

- ```rust
  pub fn register_object_store(self: &Self, url: &Url, object_store: Arc<dyn ObjectStore>) -> Option<Arc<dyn ObjectStore>> { /* ... */ }
  ```
  Registers an [`ObjectStore`] to be used with a specific URL prefix.

- ```rust
  pub fn register_batch(self: &Self, table_name: &str, batch: RecordBatch) -> Result<Option<Arc<dyn TableProvider>>> { /* ... */ }
  ```
  Registers the [`RecordBatch`] as the specified table name

- ```rust
  pub fn runtime_env(self: &Self) -> Arc<RuntimeEnv> { /* ... */ }
  ```
  Return the [RuntimeEnv] used to run queries with this `SessionContext`

- ```rust
  pub fn session_id(self: &Self) -> String { /* ... */ }
  ```
  Returns an id that uniquely identifies this `SessionContext`.

- ```rust
  pub fn table_factory(self: &Self, file_type: &str) -> Option<Arc<dyn TableProviderFactory>> { /* ... */ }
  ```
  Return the [`TableProviderFactory`] that is registered for the

- ```rust
  pub fn enable_ident_normalization(self: &Self) -> bool { /* ... */ }
  ```
  Return the `enable_ident_normalization` of this Session

- ```rust
  pub fn copied_config(self: &Self) -> SessionConfig { /* ... */ }
  ```
  Return a copied version of config for this Session

- ```rust
  pub fn copied_table_options(self: &Self) -> TableOptions { /* ... */ }
  ```
  Return a copied version of table options for this Session

- ```rust
  pub async fn sql(self: &Self, sql: &str) -> Result<DataFrame> { /* ... */ }
  ```
  Creates a [`DataFrame`] from SQL query text.

- ```rust
  pub async fn sql_with_options(self: &Self, sql: &str, options: SQLOptions) -> Result<DataFrame> { /* ... */ }
  ```
  Creates a [`DataFrame`] from SQL query text, first validating

- ```rust
  pub fn parse_sql_expr(self: &Self, sql: &str, df_schema: &DFSchema) -> Result<Expr> { /* ... */ }
  ```
  Creates logical expressions from SQL query text.

- ```rust
  pub async fn execute_logical_plan(self: &Self, plan: LogicalPlan) -> Result<DataFrame> { /* ... */ }
  ```
  Execute the [`LogicalPlan`], return a [`DataFrame`]. This API

- ```rust
  pub fn create_physical_expr(self: &Self, expr: Expr, df_schema: &DFSchema) -> Result<Arc<dyn PhysicalExpr>> { /* ... */ }
  ```
  Create a [`PhysicalExpr`] from an [`Expr`] after applying type

- ```rust
  pub fn register_variable(self: &Self, variable_type: VarType, provider: Arc<dyn VarProvider + Send + Sync>) { /* ... */ }
  ```
  Registers a variable provider within this context.

- ```rust
  pub fn register_udtf(self: &Self, name: &str, fun: Arc<dyn TableFunctionImpl>) { /* ... */ }
  ```
  Register a table UDF with this context

- ```rust
  pub fn register_udf(self: &Self, f: ScalarUDF) { /* ... */ }
  ```
  Registers a scalar UDF within this context.

- ```rust
  pub fn register_udaf(self: &Self, f: AggregateUDF) { /* ... */ }
  ```
  Registers an aggregate UDF within this context.

- ```rust
  pub fn register_udwf(self: &Self, f: WindowUDF) { /* ... */ }
  ```
  Registers a window UDF within this context.

- ```rust
  pub fn deregister_udf(self: &Self, name: &str) { /* ... */ }
  ```
  Deregisters a UDF within this context.

- ```rust
  pub fn deregister_udaf(self: &Self, name: &str) { /* ... */ }
  ```
  Deregisters a UDAF within this context.

- ```rust
  pub fn deregister_udwf(self: &Self, name: &str) { /* ... */ }
  ```
  Deregisters a UDWF within this context.

- ```rust
  pub fn deregister_udtf(self: &Self, name: &str) { /* ... */ }
  ```
  Deregisters a UDTF within this context.

- ```rust
  pub async fn read_arrow<P: DataFilePaths>(self: &Self, table_paths: P, options: ArrowReadOptions<''_>) -> Result<DataFrame> { /* ... */ }
  ```
  Creates a [`DataFrame`] for reading an Arrow data source.

- ```rust
  pub fn read_empty(self: &Self) -> Result<DataFrame> { /* ... */ }
  ```
  Creates an empty DataFrame.

- ```rust
  pub fn read_table(self: &Self, provider: Arc<dyn TableProvider>) -> Result<DataFrame> { /* ... */ }
  ```
  Creates a [`DataFrame`] for a [`TableProvider`] such as a

- ```rust
  pub fn read_batch(self: &Self, batch: RecordBatch) -> Result<DataFrame> { /* ... */ }
  ```
  Creates a [`DataFrame`] for reading a [`RecordBatch`]

- ```rust
  pub fn read_batches</* synthetic */ impl IntoIterator<Item = RecordBatch>: IntoIterator<Item = RecordBatch>>(self: &Self, batches: impl IntoIterator<Item = RecordBatch>) -> Result<DataFrame> { /* ... */ }
  ```
  Create a [`DataFrame`] for reading a [`Vec[`RecordBatch`]`]

- ```rust
  pub async fn register_listing_table</* synthetic */ impl Into<TableReference>: Into<TableReference>, /* synthetic */ impl AsRef<str>: AsRef<str>>(self: &Self, table_ref: impl Into<TableReference>, table_path: impl AsRef<str>, options: ListingOptions, provided_schema: Option<SchemaRef>, sql_definition: Option<String>) -> Result<()> { /* ... */ }
  ```
  Registers a [`ListingTable`] that can assemble multiple files

- ```rust
  pub async fn register_arrow(self: &Self, name: &str, table_path: &str, options: ArrowReadOptions<''_>) -> Result<()> { /* ... */ }
  ```
  Registers an Arrow file as a table that can be referenced from

- ```rust
  pub fn register_catalog</* synthetic */ impl Into<String>: Into<String>>(self: &Self, name: impl Into<String>, catalog: Arc<dyn CatalogProvider>) -> Option<Arc<dyn CatalogProvider>> { /* ... */ }
  ```
  Registers a named catalog using a custom `CatalogProvider` so that

- ```rust
  pub fn catalog_names(self: &Self) -> Vec<String> { /* ... */ }
  ```
  Retrieves the list of available catalog names.

- ```rust
  pub fn catalog(self: &Self, name: &str) -> Option<Arc<dyn CatalogProvider>> { /* ... */ }
  ```
  Retrieves a [`CatalogProvider`] instance by name

- ```rust
  pub fn register_table</* synthetic */ impl Into<TableReference>: Into<TableReference>>(self: &Self, table_ref: impl Into<TableReference>, provider: Arc<dyn TableProvider>) -> Result<Option<Arc<dyn TableProvider>>> { /* ... */ }
  ```
  Registers a [`TableProvider`] as a table that can be

- ```rust
  pub fn deregister_table</* synthetic */ impl Into<TableReference>: Into<TableReference>>(self: &Self, table_ref: impl Into<TableReference>) -> Result<Option<Arc<dyn TableProvider>>> { /* ... */ }
  ```
  Deregisters the given table.

- ```rust
  pub fn table_exist</* synthetic */ impl Into<TableReference>: Into<TableReference>>(self: &Self, table_ref: impl Into<TableReference>) -> Result<bool> { /* ... */ }
  ```
  Return `true` if the specified table exists in the schema provider.

- ```rust
  pub async fn table</* synthetic */ impl Into<TableReference>: Into<TableReference>>(self: &Self, table_ref: impl Into<TableReference>) -> Result<DataFrame> { /* ... */ }
  ```
  Retrieves a [`DataFrame`] representing a table previously

- ```rust
  pub fn table_function(self: &Self, name: &str) -> Result<Arc<TableFunction>> { /* ... */ }
  ```
  Retrieves a [`TableFunction`] reference by name.

- ```rust
  pub async fn table_provider</* synthetic */ impl Into<TableReference>: Into<TableReference>>(self: &Self, table_ref: impl Into<TableReference>) -> Result<Arc<dyn TableProvider>> { /* ... */ }
  ```
  Return a [`TableProvider`] for the specified table.

- ```rust
  pub fn task_ctx(self: &Self) -> Arc<TaskContext> { /* ... */ }
  ```
  Get a new TaskContext to run in this session

- ```rust
  pub fn state(self: &Self) -> SessionState { /* ... */ }
  ```
  Return a new  [`SessionState`] suitable for executing a single query.

- ```rust
  pub fn state_ref(self: &Self) -> Arc<RwLock<SessionState>> { /* ... */ }
  ```
  Get reference to [`SessionState`]

- ```rust
  pub fn state_weak_ref(self: &Self) -> Weak<RwLock<SessionState>> { /* ... */ }
  ```
  Get weak reference to [`SessionState`]

- ```rust
  pub fn register_catalog_list(self: &Self, catalog_list: Arc<dyn CatalogProviderList>) { /* ... */ }
  ```
  Register [`CatalogProviderList`] in [`SessionState`]

- ```rust
  pub fn register_table_options_extension<T: ConfigExtension>(self: &Self, extension: T) { /* ... */ }
  ```
  Registers a [`ConfigExtension`] as a table option extension that can be

###### Trait Implementations

- **Unpin**
- **MaybeSendSync**
- **FunctionRegistry**
  - ```rust
    fn udfs(self: &Self) -> HashSet<String> { /* ... */ }
    ```

  - ```rust
    fn udf(self: &Self, name: &str) -> Result<Arc<ScalarUDF>> { /* ... */ }
    ```

  - ```rust
    fn udaf(self: &Self, name: &str) -> Result<Arc<AggregateUDF>> { /* ... */ }
    ```

  - ```rust
    fn udwf(self: &Self, name: &str) -> Result<Arc<WindowUDF>> { /* ... */ }
    ```

  - ```rust
    fn register_udf(self: &mut Self, udf: Arc<ScalarUDF>) -> Result<Option<Arc<ScalarUDF>>> { /* ... */ }
    ```

  - ```rust
    fn register_udaf(self: &mut Self, udaf: Arc<AggregateUDF>) -> Result<Option<Arc<AggregateUDF>>> { /* ... */ }
    ```

  - ```rust
    fn register_udwf(self: &mut Self, udwf: Arc<WindowUDF>) -> Result<Option<Arc<WindowUDF>>> { /* ... */ }
    ```

  - ```rust
    fn register_function_rewrite(self: &mut Self, rewrite: Arc<dyn FunctionRewrite + Send + Sync>) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn expr_planners(self: &Self) -> Vec<Arc<dyn ExprPlanner>> { /* ... */ }
    ```

  - ```rust
    fn register_expr_planner(self: &mut Self, expr_planner: Arc<dyn ExprPlanner>) -> Result<()> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Same**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoEither**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(session: &SessionContext) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(state: SessionState) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(session: SessionContext) -> Self { /* ... */ }
    ```

- **Send**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ErasedDestructor**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> SessionContext { /* ... */ }
    ```

- **Sync**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Enum `RegisterFunction`

Type of function to create

```rust
pub enum RegisterFunction {
    Scalar(std::sync::Arc<crate::logical_expr::ScalarUDF>),
    Aggregate(std::sync::Arc<crate::logical_expr::AggregateUDF>),
    Window(std::sync::Arc<datafusion_expr::WindowUDF>),
    Table(String, std::sync::Arc<dyn TableFunctionImpl>),
}
```

##### Variants

###### `Scalar`

Scalar user defined function

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `std::sync::Arc<crate::logical_expr::ScalarUDF>` |  |

###### `Aggregate`

Aggregate user defined function

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `std::sync::Arc<crate::logical_expr::AggregateUDF>` |  |

###### `Window`

Window user defined function

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `std::sync::Arc<datafusion_expr::WindowUDF>` |  |

###### `Table`

Table user defined function

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |
| 1 | `std::sync::Arc<dyn TableFunctionImpl>` |  |

##### Implementations

###### Trait Implementations

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Freeze**
- **MaybeSendSync**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **ErasedDestructor**
- **Send**
- **Same**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoEither**
#### Struct `EmptySerializerRegistry`

Default implementation of [SerializerRegistry] that throws unimplemented error
for all requests.

```rust
pub struct EmptySerializerRegistry;
```

##### Implementations

###### Trait Implementations

- **Freeze**
- **Unpin**
- **RefUnwindSafe**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **IntoEither**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Same**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **SerializerRegistry**
  - ```rust
    fn serialize_logical_plan(self: &Self, node: &dyn UserDefinedLogicalNode) -> Result<Vec<u8>> { /* ... */ }
    ```

  - ```rust
    fn deserialize_logical_plan(self: &Self, name: &str, _bytes: &[u8]) -> Result<Arc<dyn UserDefinedLogicalNode>> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Allocation**
- **ErasedDestructor**
#### Struct `SQLOptions`

Describes which SQL statements can be run.

See [`SessionContext::sql_with_options`] for more details.

```rust
pub struct SQLOptions {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  Create a new `SQLOptions` with default values

- ```rust
  pub fn with_allow_ddl(self: Self, allow: bool) -> Self { /* ... */ }
  ```
  Should DDL data definition commands  (e.g. `CREATE TABLE`) be run? Defaults to `true`.

- ```rust
  pub fn with_allow_dml(self: Self, allow: bool) -> Self { /* ... */ }
  ```
  Should DML data modification commands (e.g. `INSERT` and `COPY`) be run? Defaults to `true`

- ```rust
  pub fn with_allow_statements(self: Self, allow: bool) -> Self { /* ... */ }
  ```
  Should Statements such as (e.g. `SET VARIABLE and `BEGIN TRANSACTION` ...`) be run?. Defaults to `true`

- ```rust
  pub fn verify_plan(self: &Self, plan: &LogicalPlan) -> Result<()> { /* ... */ }
  ```
  Return an error if the [`LogicalPlan`] has any nodes that are

###### Trait Implementations

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **UnwindSafe**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ErasedDestructor**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Same**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> SQLOptions { /* ... */ }
    ```

- **IntoEither**
- **Copy**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **RefUnwindSafe**
- **Allocation**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

### Traits

#### Trait `DataFilePaths`

DataFilePaths adds a method to convert strings and vector of strings to vector of [`ListingTableUrl`] URLs.
This allows methods such [`SessionContext::read_csv`] and [`SessionContext::read_avro`]
to take either a single file or multiple files.

```rust
pub trait DataFilePaths {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `to_urls`: Parse to a vector of [`ListingTableUrl`] URLs.

##### Implementations

This trait is implemented for the following types:

- `&str`
- `String`
- `&String`
- `Vec<P>` with <P>

#### Trait `QueryPlanner`

A planner used to add extensions to DataFusion logical and physical plans.

```rust
pub trait QueryPlanner: Debug {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `create_physical_plan`: Given a `LogicalPlan`, create an [`ExecutionPlan`] suitable for execution

#### Trait `FunctionFactory`

A pluggable interface to handle `CREATE FUNCTION` statements
and interact with [SessionState] to registers new udf, udaf or udwf.

```rust
pub trait FunctionFactory: Debug + Sync + Send {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `create`: Handles creation of user defined function specified in [CreateFunction] statement

### Re-exports

#### Re-export `SessionState`

```rust
pub use crate::execution::session_state::SessionState;
```

#### Re-export `SessionConfig`

```rust
pub use datafusion_execution::config::SessionConfig;
```

#### Re-export `TaskContext`

```rust
pub use datafusion_execution::TaskContext;
```

#### Re-export `ExecutionProps`

```rust
pub use datafusion_expr::execution_props::ExecutionProps;
```

## Module `session_state`

[`SessionState`]: information required to run queries in a session

```rust
pub mod session_state { /* ... */ }
```

### Types

#### Struct `SessionState`

`SessionState` contains all the necessary state to plan and execute queries,
such as configuration, functions, and runtime environment. Please see the
documentation on [`SessionContext`] for more information.


# Example: `SessionState` from a [`SessionContext`]

```
use datafusion::prelude::*;
let ctx = SessionContext::new();
let state = ctx.state();
```

# Example: `SessionState` via [`SessionStateBuilder`]

You can also use [`SessionStateBuilder`] to build a `SessionState` object
directly:

```
use datafusion::prelude::*;
# use datafusion::{error::Result, assert_batches_eq};
# use datafusion::execution::session_state::SessionStateBuilder;
# use datafusion_execution::runtime_env::RuntimeEnv;
# use std::sync::Arc;
# #[tokio::main]
# async fn main() -> Result<()> {
    let state = SessionStateBuilder::new()
        .with_config(SessionConfig::new())
        .with_runtime_env(Arc::new(RuntimeEnv::default()))
        .with_default_features()
        .build();
    Ok(())
# }
```

Note that there is no `Default` or `new()` for SessionState,
to avoid accidentally running queries or other operations without passing through
the [`SessionConfig`] or [`RuntimeEnv`]. See [`SessionStateBuilder`] and
[`SessionContext`].

[`SessionContext`]: crate::execution::context::SessionContext

```rust
pub struct SessionState {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub fn new_with_config_rt(config: SessionConfig, runtime: Arc<RuntimeEnv>) -> Self { /* ... */ }
  ```
  Returns new [`SessionState`] using the provided

- ```rust
  pub fn schema_for_ref</* synthetic */ impl Into<TableReference>: Into<TableReference>>(self: &Self, table_ref: impl Into<TableReference>) -> datafusion_common::Result<Arc<dyn SchemaProvider>> { /* ... */ }
  ```
  Retrieve the [`SchemaProvider`] for a specific [`TableReference`], if it

- ```rust
  pub fn add_analyzer_rule(self: &mut Self, analyzer_rule: Arc<dyn AnalyzerRule + Send + Sync>) -> &Self { /* ... */ }
  ```
  Add `analyzer_rule` to the end of the list of

- ```rust
  pub fn set_function_factory(self: &mut Self, function_factory: Arc<dyn FunctionFactory>) { /* ... */ }
  ```
  Registers a [`FunctionFactory`] to handle `CREATE FUNCTION` statements

- ```rust
  pub fn function_factory(self: &Self) -> Option<&Arc<dyn FunctionFactory>> { /* ... */ }
  ```
  Get the function factory

- ```rust
  pub fn table_factories(self: &Self) -> &HashMap<String, Arc<dyn TableProviderFactory>> { /* ... */ }
  ```
  Get the table factories

- ```rust
  pub fn table_factories_mut(self: &mut Self) -> &mut HashMap<String, Arc<dyn TableProviderFactory>> { /* ... */ }
  ```
  Get the table factories

- ```rust
  pub fn sql_to_statement(self: &Self, sql: &str, dialect: &str) -> datafusion_common::Result<Statement> { /* ... */ }
  ```
  Parse an SQL string into an DataFusion specific AST

- ```rust
  pub fn sql_to_expr(self: &Self, sql: &str, dialect: &str) -> datafusion_common::Result<SQLExpr> { /* ... */ }
  ```
  parse a sql string into a sqlparser-rs AST [`SQLExpr`].

- ```rust
  pub fn sql_to_expr_with_alias(self: &Self, sql: &str, dialect: &str) -> datafusion_common::Result<SQLExprWithAlias> { /* ... */ }
  ```
  parse a sql string into a sqlparser-rs AST [`SQLExprWithAlias`].

- ```rust
  pub fn resolve_table_references(self: &Self, statement: &Statement) -> datafusion_common::Result<Vec<TableReference>> { /* ... */ }
  ```
  Resolve all table references in the SQL statement. Does not include CTE references.

- ```rust
  pub async fn statement_to_plan(self: &Self, statement: Statement) -> datafusion_common::Result<LogicalPlan> { /* ... */ }
  ```
  Convert an AST Statement into a LogicalPlan

- ```rust
  pub async fn create_logical_plan(self: &Self, sql: &str) -> datafusion_common::Result<LogicalPlan> { /* ... */ }
  ```
  Creates a [`LogicalPlan`] from the provided SQL string. This

- ```rust
  pub fn create_logical_expr(self: &Self, sql: &str, df_schema: &DFSchema) -> datafusion_common::Result<Expr> { /* ... */ }
  ```
  Creates a datafusion style AST [`Expr`] from a SQL string.

- ```rust
  pub fn analyzer(self: &Self) -> &Analyzer { /* ... */ }
  ```
  Returns the [`Analyzer`] for this session

- ```rust
  pub fn optimizer(self: &Self) -> &Optimizer { /* ... */ }
  ```
  Returns the [`Optimizer`] for this session

- ```rust
  pub fn query_planner(self: &Self) -> &Arc<dyn QueryPlanner + Send + Sync> { /* ... */ }
  ```
  Returns the [`QueryPlanner`] for this session

- ```rust
  pub fn optimize(self: &Self, plan: &LogicalPlan) -> datafusion_common::Result<LogicalPlan> { /* ... */ }
  ```
  Optimizes the logical plan by applying optimizer rules.

- ```rust
  pub async fn create_physical_plan(self: &Self, logical_plan: &LogicalPlan) -> datafusion_common::Result<Arc<dyn ExecutionPlan>> { /* ... */ }
  ```
  Creates a physical [`ExecutionPlan`] plan from a [`LogicalPlan`].

- ```rust
  pub fn create_physical_expr(self: &Self, expr: Expr, df_schema: &DFSchema) -> datafusion_common::Result<Arc<dyn PhysicalExpr>> { /* ... */ }
  ```
  Create a [`PhysicalExpr`] from an [`Expr`] after applying type

- ```rust
  pub fn session_id(self: &Self) -> &str { /* ... */ }
  ```
  Return the session ID

- ```rust
  pub fn runtime_env(self: &Self) -> &Arc<RuntimeEnv> { /* ... */ }
  ```
  Return the runtime env

- ```rust
  pub fn execution_props(self: &Self) -> &ExecutionProps { /* ... */ }
  ```
  Return the execution properties

- ```rust
  pub fn execution_props_mut(self: &mut Self) -> &mut ExecutionProps { /* ... */ }
  ```
  Return mutable execution properties

- ```rust
  pub fn config(self: &Self) -> &SessionConfig { /* ... */ }
  ```
  Return the [`SessionConfig`]

- ```rust
  pub fn config_mut(self: &mut Self) -> &mut SessionConfig { /* ... */ }
  ```
  Return the mutable [`SessionConfig`].

- ```rust
  pub fn optimizers(self: &Self) -> &[Arc<dyn OptimizerRule + Send + Sync>] { /* ... */ }
  ```
  Return the logical optimizers

- ```rust
  pub fn physical_optimizers(self: &Self) -> &[Arc<dyn PhysicalOptimizerRule + Send + Sync>] { /* ... */ }
  ```
  Return the physical optimizers

- ```rust
  pub fn config_options(self: &Self) -> &ConfigOptions { /* ... */ }
  ```
  return the configuration options

- ```rust
  pub fn table_options(self: &Self) -> &TableOptions { /* ... */ }
  ```
  Return the table options

- ```rust
  pub fn default_table_options(self: &Self) -> TableOptions { /* ... */ }
  ```
  return the TableOptions options with its extensions

- ```rust
  pub fn table_options_mut(self: &mut Self) -> &mut TableOptions { /* ... */ }
  ```
  Returns a mutable reference to [`TableOptions`]

- ```rust
  pub fn register_table_options_extension<T: ConfigExtension>(self: &mut Self, extension: T) { /* ... */ }
  ```
  Registers a [`ConfigExtension`] as a table option extension that can be

- ```rust
  pub fn register_file_format(self: &mut Self, file_format: Arc<dyn FileFormatFactory>, overwrite: bool) -> Result<(), DataFusionError> { /* ... */ }
  ```
  Adds or updates a [FileFormatFactory] which can be used with COPY TO or

- ```rust
  pub fn get_file_format_factory(self: &Self, ext: &str) -> Option<Arc<dyn FileFormatFactory>> { /* ... */ }
  ```
  Retrieves a [FileFormatFactory] based on file extension which has been registered

- ```rust
  pub fn task_ctx(self: &Self) -> Arc<TaskContext> { /* ... */ }
  ```
  Get a new TaskContext to run in this session

- ```rust
  pub fn catalog_list(self: &Self) -> &Arc<dyn CatalogProviderList> { /* ... */ }
  ```
  Return catalog list

- ```rust
  pub fn scalar_functions(self: &Self) -> &HashMap<String, Arc<ScalarUDF>> { /* ... */ }
  ```
  Return reference to scalar_functions

- ```rust
  pub fn aggregate_functions(self: &Self) -> &HashMap<String, Arc<AggregateUDF>> { /* ... */ }
  ```
  Return reference to aggregate_functions

- ```rust
  pub fn window_functions(self: &Self) -> &HashMap<String, Arc<WindowUDF>> { /* ... */ }
  ```
  Return reference to window functions

- ```rust
  pub fn table_functions(self: &Self) -> &HashMap<String, Arc<TableFunction>> { /* ... */ }
  ```
  Return reference to table_functions

- ```rust
  pub fn serializer_registry(self: &Self) -> &Arc<dyn SerializerRegistry> { /* ... */ }
  ```
  Return [SerializerRegistry] for extensions

- ```rust
  pub fn version(self: &Self) -> &str { /* ... */ }
  ```
  Return version of the cargo package that produced this query

- ```rust
  pub fn register_udtf(self: &mut Self, name: &str, fun: Arc<dyn TableFunctionImpl>) { /* ... */ }
  ```
  Register a user defined table function

- ```rust
  pub fn deregister_udtf(self: &mut Self, name: &str) -> datafusion_common::Result<Option<Arc<dyn TableFunctionImpl>>> { /* ... */ }
  ```
  Deregister a user defined table function

###### Trait Implementations

- **RefUnwindSafe**
- **Sync**
- **Unpin**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **MaybeSendSync**
- **ErasedDestructor**
- **Session**
  - ```rust
    fn session_id(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn config(self: &Self) -> &SessionConfig { /* ... */ }
    ```

  - ```rust
    fn create_physical_plan<''life0, ''life1, ''async_trait>(self: &''life0 Self, logical_plan: &''life1 LogicalPlan) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = datafusion_common::Result<Arc<dyn ExecutionPlan>>> + ::core::marker::Send + ''async_trait>>
where
    Self: ''async_trait,
    ''life0: ''async_trait,
    ''life1: ''async_trait { /* ... */ }
    ```

  - ```rust
    fn create_physical_expr(self: &Self, expr: Expr, df_schema: &DFSchema) -> datafusion_common::Result<Arc<dyn PhysicalExpr>> { /* ... */ }
    ```

  - ```rust
    fn scalar_functions(self: &Self) -> &HashMap<String, Arc<ScalarUDF>> { /* ... */ }
    ```

  - ```rust
    fn aggregate_functions(self: &Self) -> &HashMap<String, Arc<AggregateUDF>> { /* ... */ }
    ```

  - ```rust
    fn window_functions(self: &Self) -> &HashMap<String, Arc<WindowUDF>> { /* ... */ }
    ```

  - ```rust
    fn runtime_env(self: &Self) -> &Arc<RuntimeEnv> { /* ... */ }
    ```

  - ```rust
    fn execution_props(self: &Self) -> &ExecutionProps { /* ... */ }
    ```

  - ```rust
    fn as_any(self: &Self) -> &dyn Any { /* ... */ }
    ```

  - ```rust
    fn table_options(self: &Self) -> &TableOptions { /* ... */ }
    ```

  - ```rust
    fn table_options_mut(self: &mut Self) -> &mut TableOptions { /* ... */ }
    ```

  - ```rust
    fn task_ctx(self: &Self) -> Arc<TaskContext> { /* ... */ }
    ```

- **OptimizerConfig**
  - ```rust
    fn query_execution_start_time(self: &Self) -> DateTime<Utc> { /* ... */ }
    ```

  - ```rust
    fn alias_generator(self: &Self) -> &Arc<AliasGenerator> { /* ... */ }
    ```

  - ```rust
    fn options(self: &Self) -> &ConfigOptions { /* ... */ }
    ```

  - ```rust
    fn function_registry(self: &Self) -> Option<&dyn FunctionRegistry> { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **IntoEither**
- **Same**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **FunctionRegistry**
  - ```rust
    fn udfs(self: &Self) -> HashSet<String> { /* ... */ }
    ```

  - ```rust
    fn udf(self: &Self, name: &str) -> datafusion_common::Result<Arc<ScalarUDF>> { /* ... */ }
    ```

  - ```rust
    fn udaf(self: &Self, name: &str) -> datafusion_common::Result<Arc<AggregateUDF>> { /* ... */ }
    ```

  - ```rust
    fn udwf(self: &Self, name: &str) -> datafusion_common::Result<Arc<WindowUDF>> { /* ... */ }
    ```

  - ```rust
    fn register_udf(self: &mut Self, udf: Arc<ScalarUDF>) -> datafusion_common::Result<Option<Arc<ScalarUDF>>> { /* ... */ }
    ```

  - ```rust
    fn register_udaf(self: &mut Self, udaf: Arc<AggregateUDF>) -> datafusion_common::Result<Option<Arc<AggregateUDF>>> { /* ... */ }
    ```

  - ```rust
    fn register_udwf(self: &mut Self, udwf: Arc<WindowUDF>) -> datafusion_common::Result<Option<Arc<WindowUDF>>> { /* ... */ }
    ```

  - ```rust
    fn deregister_udf(self: &mut Self, name: &str) -> datafusion_common::Result<Option<Arc<ScalarUDF>>> { /* ... */ }
    ```

  - ```rust
    fn deregister_udaf(self: &mut Self, name: &str) -> datafusion_common::Result<Option<Arc<AggregateUDF>>> { /* ... */ }
    ```

  - ```rust
    fn deregister_udwf(self: &mut Self, name: &str) -> datafusion_common::Result<Option<Arc<WindowUDF>>> { /* ... */ }
    ```

  - ```rust
    fn register_function_rewrite(self: &mut Self, rewrite: Arc<dyn FunctionRewrite + Send + Sync>) -> datafusion_common::Result<()> { /* ... */ }
    ```

  - ```rust
    fn expr_planners(self: &Self) -> Vec<Arc<dyn ExprPlanner>> { /* ... */ }
    ```

  - ```rust
    fn register_expr_planner(self: &mut Self, expr_planner: Arc<dyn ExprPlanner>) -> datafusion_common::Result<()> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> SessionState { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(state: SessionState) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(state: SessionState) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(state: &SessionState) -> Self { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```
    Prefer having short fields at the top and long vector fields near the end

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

#### Struct `SessionStateBuilder`

A builder to be used for building [`SessionState`]'s. Defaults will
be used for all values unless explicitly provided.

See example on [`SessionState`]

```rust
pub struct SessionStateBuilder {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  Returns a new empty [`SessionStateBuilder`].

- ```rust
  pub fn new_from_existing(existing: SessionState) -> Self { /* ... */ }
  ```
  Returns a new [SessionStateBuilder] based on an existing [SessionState].

- ```rust
  pub fn with_default_features(self: Self) -> Self { /* ... */ }
  ```
  Adds defaults for table_factories, file formats, expr_planners and builtin

- ```rust
  pub fn new_with_default_features() -> Self { /* ... */ }
  ```
  Returns a new [`SessionStateBuilder`] with default features.

- ```rust
  pub fn with_session_id(self: Self, session_id: String) -> Self { /* ... */ }
  ```
  Set the session id.

- ```rust
  pub fn with_analyzer_rules(self: Self, rules: Vec<Arc<dyn AnalyzerRule + Send + Sync>>) -> Self { /* ... */ }
  ```
  Set the [`AnalyzerRule`]s optimizer plan rules.

- ```rust
  pub fn with_analyzer_rule(self: Self, analyzer_rule: Arc<dyn AnalyzerRule + Send + Sync>) -> Self { /* ... */ }
  ```
  Add `analyzer_rule` to the end of the list of

- ```rust
  pub fn with_optimizer_rules(self: Self, rules: Vec<Arc<dyn OptimizerRule + Send + Sync>>) -> Self { /* ... */ }
  ```
  Set the [`OptimizerRule`]s used to optimize plans.

- ```rust
  pub fn with_optimizer_rule(self: Self, optimizer_rule: Arc<dyn OptimizerRule + Send + Sync>) -> Self { /* ... */ }
  ```
  Add `optimizer_rule` to the end of the list of

- ```rust
  pub fn with_expr_planners(self: Self, expr_planners: Vec<Arc<dyn ExprPlanner>>) -> Self { /* ... */ }
  ```
  Set the [`ExprPlanner`]s used to customize the behavior of the SQL planner.

- ```rust
  pub fn with_type_planner(self: Self, type_planner: Arc<dyn TypePlanner>) -> Self { /* ... */ }
  ```
  Set the [`TypePlanner`] used to customize the behavior of the SQL planner.

- ```rust
  pub fn with_physical_optimizer_rules(self: Self, physical_optimizers: Vec<Arc<dyn PhysicalOptimizerRule + Send + Sync>>) -> Self { /* ... */ }
  ```
  Set the [`PhysicalOptimizerRule`]s used to optimize plans.

- ```rust
  pub fn with_physical_optimizer_rule(self: Self, physical_optimizer_rule: Arc<dyn PhysicalOptimizerRule + Send + Sync>) -> Self { /* ... */ }
  ```
  Add `physical_optimizer_rule` to the end of the list of

- ```rust
  pub fn with_query_planner(self: Self, query_planner: Arc<dyn QueryPlanner + Send + Sync>) -> Self { /* ... */ }
  ```
  Set the [`QueryPlanner`]

- ```rust
  pub fn with_catalog_list(self: Self, catalog_list: Arc<dyn CatalogProviderList>) -> Self { /* ... */ }
  ```
  Set the [`CatalogProviderList`]

- ```rust
  pub fn with_table_functions(self: Self, table_functions: HashMap<String, Arc<TableFunction>>) -> Self { /* ... */ }
  ```
  Set the map of [`TableFunction`]s

- ```rust
  pub fn with_table_function_list(self: Self, table_functions: Vec<Arc<TableFunction>>) -> Self { /* ... */ }
  ```
  Set the list of [`TableFunction`]s

- ```rust
  pub fn with_scalar_functions(self: Self, scalar_functions: Vec<Arc<ScalarUDF>>) -> Self { /* ... */ }
  ```
  Set the map of [`ScalarUDF`]s

- ```rust
  pub fn with_aggregate_functions(self: Self, aggregate_functions: Vec<Arc<AggregateUDF>>) -> Self { /* ... */ }
  ```
  Set the map of [`AggregateUDF`]s

- ```rust
  pub fn with_window_functions(self: Self, window_functions: Vec<Arc<WindowUDF>>) -> Self { /* ... */ }
  ```
  Set the map of [`WindowUDF`]s

- ```rust
  pub fn with_serializer_registry(self: Self, serializer_registry: Arc<dyn SerializerRegistry>) -> Self { /* ... */ }
  ```
  Set the [`SerializerRegistry`]

- ```rust
  pub fn with_file_formats(self: Self, file_formats: Vec<Arc<dyn FileFormatFactory>>) -> Self { /* ... */ }
  ```
  Set the map of [`FileFormatFactory`]s

- ```rust
  pub fn with_config(self: Self, config: SessionConfig) -> Self { /* ... */ }
  ```
  Set the [`SessionConfig`]

- ```rust
  pub fn with_table_options(self: Self, table_options: TableOptions) -> Self { /* ... */ }
  ```
  Set the [`TableOptions`]

- ```rust
  pub fn with_execution_props(self: Self, execution_props: ExecutionProps) -> Self { /* ... */ }
  ```
  Set the [`ExecutionProps`]

- ```rust
  pub fn with_table_factory(self: Self, key: String, table_factory: Arc<dyn TableProviderFactory>) -> Self { /* ... */ }
  ```
  Add a [`TableProviderFactory`] to the map of factories

- ```rust
  pub fn with_table_factories(self: Self, table_factories: HashMap<String, Arc<dyn TableProviderFactory>>) -> Self { /* ... */ }
  ```
  Set the map of [`TableProviderFactory`]s

- ```rust
  pub fn with_runtime_env(self: Self, runtime_env: Arc<RuntimeEnv>) -> Self { /* ... */ }
  ```
  Set the [`RuntimeEnv`]

- ```rust
  pub fn with_function_factory(self: Self, function_factory: Option<Arc<dyn FunctionFactory>>) -> Self { /* ... */ }
  ```
  Set a [`FunctionFactory`] to handle `CREATE FUNCTION` statements

- ```rust
  pub fn with_object_store(self: Self, url: &Url, object_store: Arc<dyn ObjectStore>) -> Self { /* ... */ }
  ```
  Register an `ObjectStore` to the [`RuntimeEnv`]. See [`RuntimeEnv::register_object_store`]

- ```rust
  pub fn build(self: Self) -> SessionState { /* ... */ }
  ```
  Builds a [`SessionState`] with the current configuration.

- ```rust
  pub fn session_id(self: &Self) -> &Option<String> { /* ... */ }
  ```
  Returns the current session_id value

- ```rust
  pub fn analyzer(self: &mut Self) -> &mut Option<Analyzer> { /* ... */ }
  ```
  Returns the current analyzer value

- ```rust
  pub fn expr_planners(self: &mut Self) -> &mut Option<Vec<Arc<dyn ExprPlanner>>> { /* ... */ }
  ```
  Returns the current expr_planners value

- ```rust
  pub fn type_planner(self: &mut Self) -> &mut Option<Arc<dyn TypePlanner>> { /* ... */ }
  ```
  Returns the current type_planner value

- ```rust
  pub fn optimizer(self: &mut Self) -> &mut Option<Optimizer> { /* ... */ }
  ```
  Returns the current optimizer value

- ```rust
  pub fn physical_optimizers(self: &mut Self) -> &mut Option<PhysicalOptimizer> { /* ... */ }
  ```
  Returns the current physical_optimizers value

- ```rust
  pub fn query_planner(self: &mut Self) -> &mut Option<Arc<dyn QueryPlanner + Send + Sync>> { /* ... */ }
  ```
  Returns the current query_planner value

- ```rust
  pub fn catalog_list(self: &mut Self) -> &mut Option<Arc<dyn CatalogProviderList>> { /* ... */ }
  ```
  Returns the current catalog_list value

- ```rust
  pub fn table_functions(self: &mut Self) -> &mut Option<HashMap<String, Arc<TableFunction>>> { /* ... */ }
  ```
  Returns the current table_functions value

- ```rust
  pub fn scalar_functions(self: &mut Self) -> &mut Option<Vec<Arc<ScalarUDF>>> { /* ... */ }
  ```
  Returns the current scalar_functions value

- ```rust
  pub fn aggregate_functions(self: &mut Self) -> &mut Option<Vec<Arc<AggregateUDF>>> { /* ... */ }
  ```
  Returns the current aggregate_functions value

- ```rust
  pub fn window_functions(self: &mut Self) -> &mut Option<Vec<Arc<WindowUDF>>> { /* ... */ }
  ```
  Returns the current window_functions value

- ```rust
  pub fn serializer_registry(self: &mut Self) -> &mut Option<Arc<dyn SerializerRegistry>> { /* ... */ }
  ```
  Returns the current serializer_registry value

- ```rust
  pub fn file_formats(self: &mut Self) -> &mut Option<Vec<Arc<dyn FileFormatFactory>>> { /* ... */ }
  ```
  Returns the current file_formats value

- ```rust
  pub fn config(self: &mut Self) -> &mut Option<SessionConfig> { /* ... */ }
  ```
  Returns the current session_config value

- ```rust
  pub fn table_options(self: &mut Self) -> &mut Option<TableOptions> { /* ... */ }
  ```
  Returns the current table_options value

- ```rust
  pub fn execution_props(self: &mut Self) -> &mut Option<ExecutionProps> { /* ... */ }
  ```
  Returns the current execution_props value

- ```rust
  pub fn table_factories(self: &mut Self) -> &mut Option<HashMap<String, Arc<dyn TableProviderFactory>>> { /* ... */ }
  ```
  Returns the current table_factories value

- ```rust
  pub fn runtime_env(self: &mut Self) -> &mut Option<Arc<RuntimeEnv>> { /* ... */ }
  ```
  Returns the current runtime_env value

- ```rust
  pub fn function_factory(self: &mut Self) -> &mut Option<Arc<dyn FunctionFactory>> { /* ... */ }
  ```
  Returns the current function_factory value

- ```rust
  pub fn analyzer_rules(self: &mut Self) -> &mut Option<Vec<Arc<dyn AnalyzerRule + Send + Sync>>> { /* ... */ }
  ```
  Returns the current analyzer_rules value

- ```rust
  pub fn optimizer_rules(self: &mut Self) -> &mut Option<Vec<Arc<dyn OptimizerRule + Send + Sync>>> { /* ... */ }
  ```
  Returns the current optimizer_rules value

- ```rust
  pub fn physical_optimizer_rules(self: &mut Self) -> &mut Option<Vec<Arc<dyn PhysicalOptimizerRule + Send + Sync>>> { /* ... */ }
  ```
  Returns the current physical_optimizer_rules value

###### Trait Implementations

- **RefUnwindSafe**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ErasedDestructor**
- **Same**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(session: SessionContext) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(state: SessionState) -> Self { /* ... */ }
    ```

- **Send**
- **Unpin**
- **MaybeSendSync**
- **Sync**
- **Freeze**
- **IntoEither**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```
    Prefer having short fields at the top and long vector fields near the end

### Re-exports

#### Re-export `SessionState`

```rust
pub use session_state::SessionState;
```

#### Re-export `SessionStateBuilder`

```rust
pub use session_state::SessionStateBuilder;
```

#### Re-export `SessionStateDefaults`

```rust
pub use session_state_defaults::SessionStateDefaults;
```

#### Re-export `options`

```rust
pub use crate::datasource::file_format::options;
```

#### Re-export `datafusion_execution::*`

```rust
pub use datafusion_execution::*;
```

## Module `physical_planner`

Planner for [`LogicalPlan`] to [`ExecutionPlan`]

```rust
pub mod physical_planner { /* ... */ }
```

### Types

#### Struct `DefaultPhysicalPlanner`

Default single node physical query planner that converts a
`LogicalPlan` to an `ExecutionPlan` suitable for execution.

This planner will first flatten the `LogicalPlan` tree via a
depth first approach, which allows it to identify the leaves
of the tree.

Tasks are spawned from these leaves and traverse back up the
tree towards the root, converting each `LogicalPlan` node it
reaches into their equivalent `ExecutionPlan` node. When these
tasks reach a common node, they will terminate until the last
task reaches the node which will then continue building up the
tree.

Up to [`planning_concurrency`] tasks are buffered at once to
execute concurrently.

[`planning_concurrency`]: crate::config::ExecutionOptions::planning_concurrency

```rust
pub struct DefaultPhysicalPlanner {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub fn with_extension_planners(extension_planners: Vec<Arc<dyn ExtensionPlanner + Send + Sync>>) -> Self { /* ... */ }
  ```
  Create a physical planner that uses `extension_planners` to

- ```rust
  pub fn optimize_physical_plan<F>(self: &Self, plan: Arc<dyn ExecutionPlan>, session_state: &SessionState, observer: F) -> Result<Arc<dyn ExecutionPlan>>
where
    F: FnMut(&dyn ExecutionPlan, &dyn PhysicalOptimizerRule) { /* ... */ }
  ```
  Optimize a physical plan by applying each physical optimizer,

###### Trait Implementations

- **PhysicalPlanner**
  - ```rust
    fn create_physical_plan<''life0, ''life1, ''life2, ''async_trait>(self: &''life0 Self, logical_plan: &''life1 LogicalPlan, session_state: &''life2 SessionState) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Arc<dyn ExecutionPlan>>> + ::core::marker::Send + ''async_trait>>
where
    Self: ''async_trait,
    ''life0: ''async_trait,
    ''life1: ''async_trait,
    ''life2: ''async_trait { /* ... */ }
    ```
    Create a physical plan from a logical plan

  - ```rust
    fn create_physical_expr(self: &Self, expr: &Expr, input_dfschema: &DFSchema, session_state: &SessionState) -> Result<Arc<dyn PhysicalExpr>> { /* ... */ }
    ```
    Create a physical expression from a logical expression

- **IntoEither**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Same**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> DefaultPhysicalPlanner { /* ... */ }
    ```

- **ErasedDestructor**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **Send**
### Traits

#### Trait `PhysicalPlanner`

Physical query planner that converts a `LogicalPlan` to an
`ExecutionPlan` suitable for execution.

```rust
pub trait PhysicalPlanner: Send + Sync {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `create_physical_plan`: Create a physical plan from a logical plan
- `create_physical_expr`: Create a physical expression from a logical expression

##### Implementations

This trait is implemented for the following types:

- `DefaultPhysicalPlanner`

#### Trait `ExtensionPlanner`

This trait exposes the ability to plan an [`ExecutionPlan`] out of a [`LogicalPlan`].

```rust
pub trait ExtensionPlanner {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `plan_extension`: Create a physical plan for a [`UserDefinedLogicalNode`].

### Functions

#### Function `is_window_frame_bound_valid`

Check if window bounds are valid after schema information is available, and
window_frame bounds are casted to the corresponding column type.
queries like:
OVER (ORDER BY a RANGES BETWEEN 3 PRECEDING AND 5 PRECEDING)
OVER (ORDER BY a RANGES BETWEEN INTERVAL '3 DAY' PRECEDING AND '5 DAY' PRECEDING)  are rejected

```rust
pub fn is_window_frame_bound_valid(window_frame: &datafusion_expr::WindowFrame) -> bool { /* ... */ }
```

#### Function `create_window_expr_with_name`

Create a window expression with a name from a logical expression

```rust
pub fn create_window_expr_with_name</* synthetic */ impl Into<String>: Into<String>>(e: &crate::logical_expr::Expr, name: impl Into<String>, logical_schema: &datafusion_common::DFSchema, execution_props: &crate::execution::context::ExecutionProps) -> crate::error::Result<std::sync::Arc<dyn WindowExpr>> { /* ... */ }
```

#### Function `create_window_expr`

Create a window expression from a logical expression or an alias

```rust
pub fn create_window_expr(e: &crate::logical_expr::Expr, logical_schema: &datafusion_common::DFSchema, execution_props: &crate::execution::context::ExecutionProps) -> crate::error::Result<std::sync::Arc<dyn WindowExpr>> { /* ... */ }
```

#### Function `create_aggregate_expr_with_name_and_maybe_filter`

Create an aggregate expression with a name from a logical expression

```rust
pub fn create_aggregate_expr_with_name_and_maybe_filter(e: &crate::logical_expr::Expr, name: Option<String>, human_displan: String, logical_input_schema: &datafusion_common::DFSchema, physical_input_schema: &arrow::datatypes::Schema, execution_props: &crate::execution::context::ExecutionProps) -> crate::error::Result<(std::sync::Arc<datafusion_physical_expr::aggregate::AggregateFunctionExpr>, Option<std::sync::Arc<dyn PhysicalExpr>>, Option<datafusion_physical_expr::LexOrdering>)> { /* ... */ }
```

#### Function `create_aggregate_expr_and_maybe_filter`

Create an aggregate expression from a logical expression or an alias

```rust
pub fn create_aggregate_expr_and_maybe_filter(e: &crate::logical_expr::Expr, logical_input_schema: &datafusion_common::DFSchema, physical_input_schema: &arrow::datatypes::Schema, execution_props: &crate::execution::context::ExecutionProps) -> crate::error::Result<(std::sync::Arc<datafusion_physical_expr::aggregate::AggregateFunctionExpr>, Option<std::sync::Arc<dyn PhysicalExpr>>, Option<datafusion_physical_expr::LexOrdering>)> { /* ... */ }
```

### Re-exports

#### Re-export `create_physical_sort_expr`

**Attributes:**

- `#[deprecated(since = "47.0.0", note =
"use datafusion::{create_physical_sort_expr, create_physical_sort_exprs}")]`

**⚠️ Deprecated since 47.0.0**: use datafusion::{create_physical_sort_expr, create_physical_sort_exprs}

```rust
pub use datafusion_physical_expr::create_physical_sort_expr;
```

#### Re-export `create_physical_sort_exprs`

**Attributes:**

- `#[deprecated(since = "47.0.0", note =
"use datafusion::{create_physical_sort_expr, create_physical_sort_exprs}")]`

**⚠️ Deprecated since 47.0.0**: use datafusion::{create_physical_sort_expr, create_physical_sort_exprs}

```rust
pub use datafusion_physical_expr::create_physical_sort_exprs;
```

## Module `prelude`

DataFusion "prelude" to simplify importing common types.

Like the standard library's prelude, this module simplifies importing of
common items. Unlike the standard prelude, the contents of this module must
be imported manually:

```
use datafusion::prelude::*;
```

```rust
pub mod prelude { /* ... */ }
```

### Re-exports

#### Re-export `DataFrame`

```rust
pub use crate::dataframe::DataFrame;
```

#### Re-export `SQLOptions`

```rust
pub use crate::execution::context::SQLOptions;
```

#### Re-export `SessionConfig`

```rust
pub use crate::execution::context::SessionConfig;
```

#### Re-export `SessionContext`

```rust
pub use crate::execution::context::SessionContext;
```

#### Re-export `AvroReadOptions`

```rust
pub use crate::execution::options::AvroReadOptions;
```

#### Re-export `CsvReadOptions`

```rust
pub use crate::execution::options::CsvReadOptions;
```

#### Re-export `NdJsonReadOptions`

```rust
pub use crate::execution::options::NdJsonReadOptions;
```

#### Re-export `ParquetReadOptions`

```rust
pub use crate::execution::options::ParquetReadOptions;
```

#### Re-export `Column`

```rust
pub use datafusion_common::Column;
```

#### Re-export `lit`

```rust
pub use datafusion_expr::lit;
```

#### Re-export `lit_timestamp_nano`

```rust
pub use datafusion_expr::lit_timestamp_nano;
```

#### Re-export `JoinType`

```rust
pub use datafusion_expr::logical_plan::JoinType;
```

#### Re-export `Partitioning`

```rust
pub use datafusion_expr::logical_plan::Partitioning;
```

#### Re-export `Expr`

```rust
pub use datafusion_expr::Expr;
```

#### Re-export `Not`

```rust
pub use std::ops::Not;
```

#### Re-export `Add`

```rust
pub use std::ops::Add;
```

#### Re-export `Div`

```rust
pub use std::ops::Div;
```

#### Re-export `Mul`

```rust
pub use std::ops::Mul;
```

#### Re-export `Neg`

```rust
pub use std::ops::Neg;
```

#### Re-export `Rem`

```rust
pub use std::ops::Rem;
```

#### Re-export `Sub`

```rust
pub use std::ops::Sub;
```

#### Re-export `BitAnd`

```rust
pub use std::ops::BitAnd;
```

#### Re-export `BitOr`

```rust
pub use std::ops::BitOr;
```

#### Re-export `BitXor`

```rust
pub use std::ops::BitXor;
```

#### Re-export `Shl`

```rust
pub use std::ops::Shl;
```

#### Re-export `Shr`

```rust
pub use std::ops::Shr;
```

#### Re-export `datafusion_expr::expr_fn::*`

```rust
pub use datafusion_expr::expr_fn::*;
```

#### Re-export `datafusion_functions::expr_fn::*`

```rust
pub use datafusion_functions::expr_fn::*;
```

#### Re-export `datafusion_functions_nested::expr_fn::*`

**Attributes:**

- `#[cfg(feature = "nested_expressions")]`

```rust
pub use datafusion_functions_nested::expr_fn::*;
```

## Module `scalar`

[`ScalarValue`] single value representation.

Note this is reimported from the datafusion-common crate for easy
migration when datafusion was split into several different crates

```rust
pub mod scalar { /* ... */ }
```

### Re-exports

#### Re-export `ScalarType`

```rust
pub use datafusion_common::ScalarType;
```

#### Re-export `ScalarValue`

```rust
pub use datafusion_common::ScalarValue;
```

## Module `common`

re-export of [`datafusion_common`] crate

```rust
pub mod common { /* ... */ }
```

### Modules

## Module `runtime`

re-export of [`datafusion_common_runtime`] crate

```rust
pub mod runtime { /* ... */ }
```

### Re-exports

#### Re-export `datafusion_common_runtime::*`

```rust
pub use datafusion_common_runtime::*;
```

### Re-exports

#### Re-export `datafusion_common::*`

```rust
pub use datafusion_common::*;
```

## Module `catalog`

re-export of [`datafusion_catalog`] crate

```rust
pub mod catalog { /* ... */ }
```

### Re-exports

#### Re-export `datafusion_catalog::*`

```rust
pub use datafusion_catalog::*;
```

## Module `logical_expr`

re-export of [`datafusion_expr`] crate

```rust
pub mod logical_expr { /* ... */ }
```

### Re-exports

#### Re-export `datafusion_expr::*`

```rust
pub use datafusion_expr::*;
```

## Module `logical_expr_common`

re-export of [`datafusion_expr_common`] crate

```rust
pub mod logical_expr_common { /* ... */ }
```

### Re-exports

#### Re-export `datafusion_expr_common::*`

```rust
pub use datafusion_expr_common::*;
```

## Module `optimizer`

re-export of [`datafusion_optimizer`] crate

```rust
pub mod optimizer { /* ... */ }
```

### Re-exports

#### Re-export `datafusion_optimizer::*`

```rust
pub use datafusion_optimizer::*;
```

## Module `physical_optimizer`

re-export of [`datafusion_physical_optimizer`] crate

```rust
pub mod physical_optimizer { /* ... */ }
```

### Re-exports

#### Re-export `datafusion_physical_optimizer::*`

```rust
pub use datafusion_physical_optimizer::*;
```

## Module `physical_expr_common`

re-export of [`datafusion_physical_expr`] crate

```rust
pub mod physical_expr_common { /* ... */ }
```

### Re-exports

#### Re-export `datafusion_physical_expr_common::*`

```rust
pub use datafusion_physical_expr_common::*;
```

## Module `physical_expr`

re-export of [`datafusion_physical_expr`] crate

```rust
pub mod physical_expr { /* ... */ }
```

### Re-exports

#### Re-export `datafusion_physical_expr::*`

```rust
pub use datafusion_physical_expr::*;
```

## Module `physical_plan`

re-export of [`datafusion_physical_plan`] crate

```rust
pub mod physical_plan { /* ... */ }
```

### Re-exports

#### Re-export `datafusion_physical_plan::*`

```rust
pub use datafusion_physical_plan::*;
```

## Module `sql`

re-export of [`datafusion_sql`] crate

```rust
pub mod sql { /* ... */ }
```

### Re-exports

#### Re-export `datafusion_sql::*`

```rust
pub use datafusion_sql::*;
```

## Module `functions`

re-export of [`datafusion_functions`] crate

```rust
pub mod functions { /* ... */ }
```

### Re-exports

#### Re-export `datafusion_functions::*`

```rust
pub use datafusion_functions::*;
```

## Module `functions_nested`

re-export of [`datafusion_functions_nested`] crate, if "nested_expressions" feature is enabled

```rust
pub mod functions_nested { /* ... */ }
```

### Re-exports

#### Re-export `datafusion_functions_nested::*`

**Attributes:**

- `#[cfg(feature = "nested_expressions")]`

```rust
pub use datafusion_functions_nested::*;
```

## Module `functions_array`

**Attributes:**

- `#[deprecated(since = "41.0.0", note =
"use datafusion-functions-nested instead")]`

**⚠️ Deprecated since 41.0.0**: use datafusion-functions-nested instead

re-export of [`datafusion_functions_nested`] crate as [`functions_array`] for backward compatibility, if "nested_expressions" feature is enabled

```rust
pub mod functions_array { /* ... */ }
```

### Re-exports

#### Re-export `datafusion_functions_nested::*`

**Attributes:**

- `#[cfg(feature = "nested_expressions")]`

**⚠️ Deprecated since 41.0.0**: use datafusion-functions-nested instead

```rust
pub use datafusion_functions_nested::*;
```

## Module `functions_aggregate`

re-export of [`datafusion_functions_aggregate`] crate

```rust
pub mod functions_aggregate { /* ... */ }
```

### Re-exports

#### Re-export `datafusion_functions_aggregate::*`

```rust
pub use datafusion_functions_aggregate::*;
```

## Module `functions_window`

re-export of [`datafusion_functions_window`] crate

```rust
pub mod functions_window { /* ... */ }
```

### Re-exports

#### Re-export `datafusion_functions_window::*`

```rust
pub use datafusion_functions_window::*;
```

## Module `functions_table`

re-export of [`datafusion_functions_table`] crate

```rust
pub mod functions_table { /* ... */ }
```

### Re-exports

#### Re-export `datafusion_functions_table::*`

```rust
pub use datafusion_functions_table::*;
```

## Module `variable`

re-export of variable provider for `@name` and `@@name` style runtime values.

```rust
pub mod variable { /* ... */ }
```

### Re-exports

#### Re-export `VarProvider`

```rust
pub use datafusion_expr::var_provider::VarProvider;
```

#### Re-export `VarType`

```rust
pub use datafusion_expr::var_provider::VarType;
```

## Module `test`

**Attributes:**

- `#[cfg(not(target_arch = "wasm32"))]`
- `#![allow(missing_docs)]`

Common unit test utility methods

```rust
pub mod test { /* ... */ }
```

### Modules

## Module `object_store`

Object store implementation used for testing

```rust
pub mod object_store { /* ... */ }
```

### Functions

#### Function `register_test_store`

Registers a test object store with the provided `ctx`

```rust
pub fn register_test_store(ctx: &crate::prelude::SessionContext, files: &[(&str, u64)]) { /* ... */ }
```

#### Function `make_test_store_and_state`

Create a test object store with the provided files

```rust
pub fn make_test_store_and_state(files: &[(&str, u64)]) -> (std::sync::Arc<object_store::memory::InMemory>, crate::execution::context::SessionState) { /* ... */ }
```

#### Function `local_unpartitioned_file`

Helper method to fetch the file size and date at given path and create a `ObjectMeta`

```rust
pub fn local_unpartitioned_file</* synthetic */ impl AsRef<std::path::Path>: AsRef<std::path::Path>>(path: impl AsRef<std::path::Path>) -> object_store::ObjectMeta { /* ... */ }
```

#### Function `ensure_head_concurrency`

Blocks the object_store `head` call until `concurrency` number of calls are pending.

```rust
pub fn ensure_head_concurrency(object_store: std::sync::Arc<dyn ObjectStore>, concurrency: usize) -> std::sync::Arc<dyn ObjectStore> { /* ... */ }
```

## Module `variable`

System variable provider

```rust
pub mod variable { /* ... */ }
```

### Types

#### Struct `SystemVar`

System variable

```rust
pub struct SystemVar {
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  new system variable

###### Trait Implementations

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **MaybeSendSync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Same**
- **VarProvider**
  - ```rust
    fn get_value(self: &Self, var_names: Vec<String>) -> Result<ScalarValue> { /* ... */ }
    ```
    get system variable value

  - ```rust
    fn get_type(self: &Self, _: &[String]) -> Option<DataType> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **Default**
  - ```rust
    fn default() -> SystemVar { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Allocation**
- **RefUnwindSafe**
- **Sync**
- **Freeze**
#### Struct `UserDefinedVar`

user defined variable

```rust
pub struct UserDefinedVar {
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  new user defined variable

###### Trait Implementations

- **Same**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Allocation**
- **MaybeSendSync**
- **IntoEither**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Default**
  - ```rust
    fn default() -> UserDefinedVar { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **ErasedDestructor**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **VarProvider**
  - ```rust
    fn get_value(self: &Self, var_names: Vec<String>) -> Result<ScalarValue> { /* ... */ }
    ```
    Get user defined variable value

  - ```rust
    fn get_type(self: &Self, var_names: &[String]) -> Option<DataType> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Functions

#### Function `create_table_dual`

```rust
pub fn create_table_dual() -> std::sync::Arc<dyn TableProvider> { /* ... */ }
```

#### Function `scan_partitioned_csv`

Returns a [`DataSourceExec`] that scans "aggregate_test_100.csv" with `partitions` partitions

```rust
pub fn scan_partitioned_csv(partitions: usize, work_dir: &std::path::Path) -> crate::error::Result<std::sync::Arc<datafusion_datasource::source::DataSourceExec>> { /* ... */ }
```

#### Function `partitioned_file_groups`

Returns file groups [`Vec<FileGroup>`] for scanning `partitions` of `filename`

```rust
pub fn partitioned_file_groups(path: &str, filename: &str, partitions: usize, file_format: std::sync::Arc<dyn FileFormat>, file_compression_type: crate::datasource::file_format::file_compression_type::FileCompressionType, work_dir: &std::path::Path) -> crate::error::Result<Vec<datafusion_datasource::file_groups::FileGroup>> { /* ... */ }
```

#### Function `assert_fields_eq`

```rust
pub fn assert_fields_eq(plan: &crate::logical_expr::LogicalPlan, expected: Vec<&str>) { /* ... */ }
```

#### Function `columns`

Returns the column names on the schema

```rust
pub fn columns(schema: &arrow::datatypes::Schema) -> Vec<String> { /* ... */ }
```

#### Function `table_with_sequence`

Return a new table provider that has a single Int32 column with
values between `seq_start` and `seq_end`

```rust
pub fn table_with_sequence(seq_start: i32, seq_end: i32) -> crate::error::Result<std::sync::Arc<dyn TableProvider>> { /* ... */ }
```

#### Function `make_partition`

Return a RecordBatch with a single Int32 array with values (0..sz)

```rust
pub fn make_partition(sz: i32) -> arrow::record_batch::RecordBatch { /* ... */ }
```

#### Function `table_with_decimal`

Return a new table which provide this decimal column

```rust
pub fn table_with_decimal() -> std::sync::Arc<dyn TableProvider> { /* ... */ }
```

## Module `test_util`

Utility functions to make testing DataFusion based crates easier

```rust
pub mod test_util { /* ... */ }
```

### Modules

## Module `parquet`

**Attributes:**

- `#[cfg(feature = "parquet")]`

Helpers for writing parquet files and reading them back

```rust
pub mod parquet { /* ... */ }
```

### Types

#### Struct `TestParquetFile`

a ParquetFile that has been created for testing.

```rust
pub struct TestParquetFile {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub fn try_new</* synthetic */ impl IntoIterator<Item = RecordBatch>: IntoIterator<Item = RecordBatch>>(path: PathBuf, props: WriterProperties, batches: impl IntoIterator<Item = RecordBatch>) -> Result<Self> { /* ... */ }
  ```
  Creates a new parquet file at the specified location with the

- ```rust
  pub async fn create_scan(self: &Self, ctx: &SessionContext, maybe_filter: Option<Expr>) -> Result<Arc<dyn ExecutionPlan>> { /* ... */ }
  ```
  Return a `DataSourceExec` with the specified options.

- ```rust
  pub fn parquet_metrics(plan: &Arc<dyn ExecutionPlan>) -> Option<MetricsSet> { /* ... */ }
  ```
  Retrieve metrics from the parquet exec returned from `create_scan`

- ```rust
  pub fn schema(self: &Self) -> SchemaRef { /* ... */ }
  ```
  The schema of this parquet file

- ```rust
  pub fn path(self: &Self) -> &std::path::Path { /* ... */ }
  ```
  The path to the parquet file

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Allocation**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **MaybeSendSync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **Same**
- **UnwindSafe**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **ErasedDestructor**
- **Freeze**
#### Struct `ParquetScanOptions`

Options for how to create the parquet scan

```rust
pub struct ParquetScanOptions {
    pub pushdown_filters: bool,
    pub reorder_filters: bool,
    pub enable_page_index: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `pushdown_filters` | `bool` | Enable pushdown filters |
| `reorder_filters` | `bool` | enable reordering filters |
| `enable_page_index` | `bool` | enable page index |

##### Implementations

###### Methods

- ```rust
  pub fn config(self: &Self) -> SessionConfig { /* ... */ }
  ```
  Returns a [`SessionConfig`] with the given options

###### Trait Implementations

- **ErasedDestructor**
- **Copy**
- **Allocation**
- **Same**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **MaybeSendSync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ParquetScanOptions { /* ... */ }
    ```

- **Unpin**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

## Module `csv`

Helpers for writing csv files and reading them back

```rust
pub mod csv { /* ... */ }
```

### Types

#### Struct `TestCsvFile`

a CSV file that has been created for testing.

```rust
pub struct TestCsvFile {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub fn try_new</* synthetic */ impl IntoIterator<Item = RecordBatch>: IntoIterator<Item = RecordBatch>>(path: PathBuf, batches: impl IntoIterator<Item = RecordBatch>) -> Result<Self> { /* ... */ }
  ```
  Creates a new csv file at the specified location

- ```rust
  pub fn schema(self: &Self) -> SchemaRef { /* ... */ }
  ```
  The schema of this csv file

- ```rust
  pub fn path(self: &Self) -> &std::path::Path { /* ... */ }
  ```
  The path to the csv file

###### Trait Implementations

- **Send**
- **Allocation**
- **Freeze**
- **Same**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
- **UnwindSafe**
- **IntoEither**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ErasedDestructor**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Types

#### Struct `TestTableFactory`

TableFactory for tests

```rust
pub struct TestTableFactory {
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|

##### Implementations

###### Trait Implementations

- **ErasedDestructor**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Same**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TableProviderFactory**
  - ```rust
    fn create<''life0, ''life1, ''life2, ''async_trait>(self: &''life0 Self, _: &''life1 dyn Session, cmd: &''life2 CreateExternalTable) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Arc<dyn TableProvider>>> + ::core::marker::Send + ''async_trait>>
where
    Self: ''async_trait,
    ''life0: ''async_trait,
    ''life1: ''async_trait,
    ''life2: ''async_trait { /* ... */ }
    ```

- **Allocation**
- **IntoEither**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **MaybeSendSync**
- **Unpin**
- **Send**
- **Sync**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> TestTableFactory { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `TestTableProvider`

TableProvider for testing purposes

```rust
pub struct TestTableProvider {
    pub url: String,
    pub schema: arrow::datatypes::SchemaRef,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `url` | `String` | URL of table files or folder |
| `schema` | `arrow::datatypes::SchemaRef` | test table schema |

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **IntoEither**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Same**
- **Sync**
- **MaybeSendSync**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **ErasedDestructor**
- **Allocation**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TableProvider**
  - ```rust
    fn as_any(self: &Self) -> &dyn Any { /* ... */ }
    ```

  - ```rust
    fn schema(self: &Self) -> SchemaRef { /* ... */ }
    ```

  - ```rust
    fn table_type(self: &Self) -> TableType { /* ... */ }
    ```

  - ```rust
    fn scan<''life0, ''life1, ''life2, ''life3, ''async_trait>(self: &''life0 Self, _state: &''life1 dyn Session, _projection: Option<&''life2 Vec<usize>>, _filters: &''life3 [Expr], _limit: Option<usize>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Arc<dyn ExecutionPlan>>> + ::core::marker::Send + ''async_trait>>
where
    Self: ''async_trait,
    ''life0: ''async_trait,
    ''life1: ''async_trait,
    ''life2: ''async_trait,
    ''life3: ''async_trait { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

### Functions

#### Function `scan_empty`

Scan an empty data source, mainly used in tests

```rust
pub fn scan_empty(name: Option<&str>, table_schema: &arrow::datatypes::Schema, projection: Option<Vec<usize>>) -> crate::error::Result<crate::logical_expr::LogicalPlanBuilder> { /* ... */ }
```

#### Function `scan_empty_with_partitions`

Scan an empty data source with configured partition, mainly used in tests.

```rust
pub fn scan_empty_with_partitions(name: Option<&str>, table_schema: &arrow::datatypes::Schema, projection: Option<Vec<usize>>, partitions: usize) -> crate::error::Result<crate::logical_expr::LogicalPlanBuilder> { /* ... */ }
```

#### Function `aggr_test_schema`

Get the schema for the aggregate_test_* csv files

```rust
pub fn aggr_test_schema() -> arrow::datatypes::SchemaRef { /* ... */ }
```

#### Function `register_aggregate_csv`

Register session context for the aggregate_test_100.csv file

```rust
pub async fn register_aggregate_csv(ctx: &crate::prelude::SessionContext, table_name: &str) -> crate::error::Result<()> { /* ... */ }
```

#### Function `test_table_with_name`

Create a table from the aggregate_test_100.csv file with the specified name

```rust
pub async fn test_table_with_name(name: &str) -> crate::error::Result<crate::dataframe::DataFrame> { /* ... */ }
```

#### Function `test_table`

Create a table from the aggregate_test_100.csv file with the name "aggregate_test_100"

```rust
pub async fn test_table() -> crate::error::Result<crate::dataframe::DataFrame> { /* ... */ }
```

#### Function `plan_and_collect`

Execute SQL and return results

```rust
pub async fn plan_and_collect(ctx: &crate::prelude::SessionContext, sql: &str) -> crate::error::Result<Vec<arrow::record_batch::RecordBatch>> { /* ... */ }
```

#### Function `populate_csv_partitions`

Generate CSV partitions within the supplied directory

```rust
pub fn populate_csv_partitions(tmp_dir: &tempfile::TempDir, partition_count: usize, file_extension: &str) -> crate::error::Result<arrow::datatypes::SchemaRef> { /* ... */ }
```

#### Function `register_unbounded_file_with_ordering`

This function creates an unbounded sorted file for testing purposes.

```rust
pub fn register_unbounded_file_with_ordering(ctx: &crate::prelude::SessionContext, schema: arrow::datatypes::SchemaRef, file_path: &std::path::Path, table_name: &str, file_sort_order: Vec<Vec<datafusion_expr::SortExpr>>) -> crate::error::Result<()> { /* ... */ }
```

### Re-exports

#### Re-export `parquet_test_data`

**Attributes:**

- `#[cfg(feature = "parquet")]`

```rust
pub use datafusion_common::test_util::parquet_test_data;
```

#### Re-export `arrow_test_data`

```rust
pub use datafusion_common::test_util::arrow_test_data;
```

#### Re-export `get_data_dir`

```rust
pub use datafusion_common::test_util::get_data_dir;
```

## Constants and Statics

### Constant `DATAFUSION_VERSION`

DataFusion crate version

```rust
pub const DATAFUSION_VERSION: &str = "47.0.0";
```

## Re-exports

### Re-export `arrow`

```rust
pub use arrow;
```

### Re-export `parquet`

**Attributes:**

- `#[cfg(feature = "parquet")]`

```rust
pub use parquet;
```

### Re-export `config`

```rust
pub use common::config;
```

### Re-export `assert_batches_eq`

```rust
pub use datafusion_common::assert_batches_eq;
```

### Re-export `assert_batches_sorted_eq`

```rust
pub use datafusion_common::assert_batches_sorted_eq;
```
