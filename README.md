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


