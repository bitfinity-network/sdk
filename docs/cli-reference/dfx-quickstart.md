# dfx quickstart

Use the `dfx quickstart` command to perform initial one time setup for your identity and/or wallet. This command
can be run anytime to repeat the setup process or to be used as an informational command, printing
information about
your ICP balance, current ICP to XDR conversion rate, and more.

## Basic usage

``` bash
dfx quickstart
```

## Flags

You can use the following optional flags with the `dfx quickstart` command.

| Flag              | Description                                                                                                                            |
|-------------------|----------------------------------------------------------------------------------------------------------------------------------------|
| `-h`, `--help`    | Displays usage information.                                                                                                            |
| `-q`, `--quiet`   | Suppresses informational messages. -qq limits to errors only; -qqqq disables them all.                                                 |
| `-v`, `--verbose` | Displays detailed information about operations. -vv will generate a very large number of messages and can affect performance.          |
| `--identity`      | The user identity to run this command as. It contains your principal as well as some things DFX associates with it such as the wallet. |

## Options

You can specify the following options for the `dfx quickstart` command.

| Option                | Description                                                                                                             |
|-----------------------|-------------------------------------------------------------------------------------------------------------------------|
| `--log <LOGMODE>`     | The logging mode to use. You can log to stderr, a file, or both [default: stderr] [possible values: stderr, tee, file]. |
| `--logfile <LOGFILE>` | The file to log to, if logging to a file (see --logmode).                                                               |

## Examples

To run a guided setup, run `dfx quickstart`

``` bash
dfx quickstart
```
```shell
Your DFX user principal: xxxxx-xxxxx-xxxxx-xxxxx-xxxxx-xxxxx-xxxxx-xxxxx-xxxxx-xxxxx-xxx
Your ledger account address: xxxxx
Your ICP balance: 1234.567 ICP
Conversion rate: 1 ICP <> 3.7950 XDR
Import an existing wallet? no
Spend 2.63504611 ICP to create a new wallet with 10 TC? no
Run this command again at any time to continue from here.
```
