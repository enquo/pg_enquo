Listed below are the data types supported by `pg_enquo`.
They are named for the corresponding PostgresSQL data types that they emulate.
However, it is important to bear in mind that, due to the limitations of cryptographic research, they typically only support a subset of the features of the native data type.
Please read the documentation for the data type for full details of what is and is not supported, and any security caveats surrounding them.

* [`enquo_bigint`](bigint.md) -- 64-bit signed integer, capable of storing values from `-2^63` to `2^63-1`.
* [`enquo_date`](date.md) -- a calendar date.


# A Note on "Reduced Security Operations"

In our [default security model](https://enquo.org/about/threat-models#snapshot-security), Enquo-encrypted values are strongly encrypted and intended to be safe from cryptographic attack.
However, some common and useful operations are not possible at this security level, and so we provide the *option* to trade reduced security against more features.
It can be difficult for non-cryptographers (heck, even expert cryptographers!) to decide exactly what the "cost" is of the security reduction is.

The rule of thumb we recommend you adopt is this:

1. Use the "default security" mode wherever possible, and spend a bit of time thinking about how you can "work around" the limitations.

2. If you find that you just *can't* work with the default security mode for a given column, and the options are "use reduced security mode" or "leave the column in plaintext",
   then use reduced security mode.
   Yes, reduced security mode provides *less* security, but it's still better than leaving the data unencrypted.
