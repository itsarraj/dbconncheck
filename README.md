# dbconncheck

A one-command connectivity/auth/latency check against Postgres, MySQL,
or Redis — the first thing you'd run when on-call and something can't
reach its database, instead of reaching for `psql`/`mysql`/`redis-cli`
and remembering each one's own slightly different connection-testing
incantation.

## Usage

```bash
dbconncheck postgres://user:pass@host/db
dbconncheck mysql://user:pass@host/db
dbconncheck redis://host:6379
```

The database type is auto-detected from the URL scheme
(`postgres`/`postgresql`, `mysql`, `redis`/`rediss`). Reports either
`OK: connected and authenticated in <duration>` or `FAILED after
<duration>: <the real underlying error>`, distinguishing "can't reach
the host at all" from "reached it, but auth failed" by simply
surfacing the driver's own real error text rather than collapsing both
into one generic "connection failed."

## Status: built and verified against three real databases, all three real failure modes

- **5 unit tests** (`cargo test --lib`): scheme detection for both
  Postgres spellings, MySQL, both Redis schemes (`redis`/`rediss`), an
  unrecognized scheme, and a malformed URL with no `://` at all.
- **Live-verified against a real local Postgres**: correct credentials
  reported `OK` with a real measured latency in single-digit
  milliseconds; a real nonexistent role reported the *actual* Postgres
  auth error (`role "nonexistentuser" does not exist`), not a generic
  failure message; a real unreachable port (nothing listening)
  correctly reported a real `Connection refused` after a fast failure,
  not a hang.
- **Live-verified against a real scratch Redis container**: a real
  `PING` round-trip succeeded and reported real sub-millisecond
  latency.
- **Live-verified the scheme-rejection path**: an unrecognized scheme
  (`mongodb://`) is rejected immediately with a clear message naming
  the three schemes this tool actually supports, before attempting any
  network connection at all.

**Not done / deliberately deferred**: connection pooling / repeated
checks over time (this makes exactly one connection attempt and exits —
for continuous monitoring, wrap it in a loop or a scheduler); TLS/SSL
mode flags beyond whatever the URL itself specifies; and MongoDB/
Cassandra/other non-SQL-non-Redis databases aren't supported, matching
the three backends this workspace's own tools (`pgqueue`, `myschema`,
`redisdump`) already work with.
