The `pg_enquo` `enquo_date` data type is intended to be an encrypted replacement for the PostgreSQL [`date` type](https://www.postgresql.org/docs/current/datatype-datetime.html).
It stores a calendar date as the combination of year, month, and day.  The range of supported years is from -32,768 to 32,767.


# Operations

At the [default security level](https://enquo.org/threat-models#snapshot-security), and with default storage options, you can do the following with an `enquo_bigint`:

* Store an encrypted value.

* Query the values stored in the column using any of the [standard comparison operators](https://www.postgresql.org/docs/current/functions-comparison.html#FUNCTIONS-COMPARISON-OP-TABLE) and `IS NULL` / `IS NOT NULL`.


## Reduced Security Operations

Some additional operations are available on a column if you are willing to trade-off some security properties for that column.
Enabling these additional operations requires storing more data on disk, which can be used by an attacker to:

1. Determine all values which are the same (although they cannot directly determine the numeric values themselves); and

2. Order the entire set of values in the column, which enables inference attacks against the data set, which may reveal some or all of the exact numeric values in the column.

If you are willing to accept these trade-offs, you can use the `enable_reduced_security_operations` storage option for a given column (see your Enquo client for details), which will then allow you to:

* Use the column in a B-Tree index; and

* Sort query results with `ORDER BY <column>`.


# Storage Requirements

A default `enquo_date` value requires around 250 bytes on disk.

If the values in a column only need to be read and written, but never queried, enabling the `no_query` storage option (see your Enquo client for details) reduces the storage requirement to around 50 bytes per value.

If the column is in "reduced security operations" mode, then expect to use about 300 bytes on disk per value.
