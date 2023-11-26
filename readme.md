# Blockchain Simulation in Rust

Toy blockchain called ‘B’.

Like other blockchains, B creates new blocks. Therefore, when we send a transaction command, it takes a few seconds to be confirmed because the transaction needs to be included in a new block. As on some real blockchains, B creates new blocks at regular time intervals of 10 seconds. So, let’s say blocks are minted at T=10, T=20, T=30, etc. If we send a transaction a T=7, we will wait 3 seconds for its confirmation. If we send one at T=12, we will wait 8 seconds for the transaction to be confirmed in a new block.

There are two types of transfers on B, one for creating accounts and the other for transferring funds.

There is also a read command for viewing an account balance. However, it is a read command, not a transaction. So the balance command should instantaneously show the result.

Here are its desired features:

### Server

```
b start-node
```

> The `start-node` command starts a local, new B blockchain server. Keep it running in a separate terminal. It should stop with Ctrl-C. 

### Client

```
b create-account <id-of-account> <starting-balance>
```
> The `create-account` transaction should create an account on B.

```
b transfer <from-account> <to-account> <amount>
```
> The `transfer` transaction should send funds from one account to another on B.
```
b balance <account>
```
>The `balance` command should display the funds of a B account. Remember, this is a read command.


### Miscellaneous:

Display meaningful error messages only if the user misuses a command. You do not have to handle other errors.

The B simulation is a local, single-threaded CLI. There is no need for cryptography! Account information is not permanently stored, as the `start-node` command will start a new blockchain.

As long as the four commands work as expected, there is no single “right” way of doing this simulation project 🙂