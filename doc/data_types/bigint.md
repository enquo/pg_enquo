The `pg_enquo` `enquo_bigint` data type is intended to be an encrypted replacement for the PostgreSQL [`bigint` numeric type](https://www.postgresql.org/docs/current/datatype-numeric.html#DATATYPE-INT).
It stores values in the same range as the PostgreSQL `bigint`, from `-2^63` to `2^63-1`, inclusive.


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

* Use the column in a B-Tree or Hash index; and

* Sort query results with `ORDER BY <column>`.


# Storage Requirements

A default `enquo_bigint` value requires around 500 bytes on disk.

If the values in a column only need to be read and written, but never queried, enabling the `no_query` storage option (see your Enquo client for details) reduces the storage requirement to around 50 bytes per value.

If the column is in "reduced security operations" mode, then expect to use about 640 bytes on disk per value.
