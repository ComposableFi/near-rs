# PoC: Demo of sp_io crypto functions

This folder contains a demo of minimal Substrate environment to call `sp_io` crypto functions.
It constructs a minimal environment with additional crypto keys storage because `sp_io` library
don't interact directly with private keys and delegates all their management to an underlying storage.
