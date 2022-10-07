The `pg_enquo` `enquo_text` data type is intended to be an encrypted replacement for the PostgreSQL [character types](https://www.postgresql.org/docs/current/datatype-character.html), such as `varchar` and `text`.
It stores arbitrary-length UTF-8 text.


# Operations

At the [default security level](https://enquo.org/about/threat-models#snapshot-security), and with default storage options, you can do the following with an `enquo_text` value:

* Store and retrieve the encrypted string.

* Perform queries using the equals/not-equals operators (`=` / `<>`), and determine whether the value is `IS NULL` / `IS NOT NULL`.


## Reduced Security Operations

Some additional operations are available on a column if you are willing to trade-off some security properties for that column.
Enabling these additional operations requires storing more data on disk, which can be used by an attacker to identify ciphertexts which are identical, with a high degree of confidence (although they will not be able to determine the actual value of those ciphertexts).

If you are willing to accept this trade-off, you can use the `enable_reduced_security_operations` storage option for a given column (see your Enquo client for details), which will then allow you to:

* Use the column in a Hash index.


# Storage Requirements

An `enquo_text` that stores a zero-length (ie empty) text value requires around 110 bytes on disk.  Each 16 bytes (UTF-8 characters can be multiple bytes in length) increases the size of the ciphertext by 16 bytes.

If the values in a column only need to be read and written, but never queried, enabling the `no_query` storage option (see your Enquo client for details) reduces the storage requirement for an empty string to around 55 bytes per value.

If the column is in "reduced security operations" mode, then the zero-length value will use about FIXME bytes on disk per value.
