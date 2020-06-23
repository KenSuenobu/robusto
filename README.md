# robusto

Robusto is a Rust Distributed Job Server that uses the `etcd` server as a means
to communicate between scheduled jobs.

## Philosophy

Robusto allows applications to build [DAGs](https://en.wikipedia.org/wiki/Directed_acyclic_graph)
of tasks that can be triggered in a controlled fashion.

This comes from the design from the [Scattersphere](https://www.github.com/KenSuenobu/scattersphere/)
project that was originally designed for Scala, using the `CompletableFuture` library, but 
it was found to be too limiting and difficult to use.  Communication using a triggering
mechanism rather than a programmatic solution tends to be more flexible.

This allows applications to run on servers big and small, with no requirements or restrictions
on the number of cores, amount of memory, or amount of disk space a particular job might take.

## Jobs and Rust

Since the `etcd` server is used to control job state, this means that Robusto servers can
be started as needed, and shut down when jobs complete.  It also means that jobs can be
run from other languages, not specifically requiring Robusto to run.

A good example could be that a job triggering a task in Rust, then triggering another
job to be run using [Ballista](https://github.com/ballista-compute/ballista), or kicking
off a job in [Apache Spark](https://spark.apache.org/) using a GPU for compute, or other
servers.

The list goes on and on.  Robusto is simply a coordinating step that allows jobs to run
using a DAG with Rust.  Jobs with Robusto are written using Rust, and the Robusto server
is simply a lightweight library that runs over top of the job, allowing it to be triggered
when the right set of circumstances occurs.

