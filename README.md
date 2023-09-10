# transfer-sol

In this program, i will be transferring sol between 2 system acocunts. We can transfer sol between many types of accounts, not just system accounts.

The act of transferring SOL to the new keypair's account will initialize it as a default system account - hence the ///CHECK comment / DOC.

Here, we are using a CPI (Cross Program Invocation) to invoke the Solana System Program to handle the transfer of funds from one account to another.
Using built-in programs through CPIs helps in reusing these functionalities in a modular way, which in turn helps in reducing the complexities of our programs.

In this program, we try transferring funds using 2 methods:
1. Using the `transfer` method of the `SystemProgram` class. This is the recommended way of transferring funds, using CPIs.
2. Using the risky add balance to recipient and subtract balance from sender method.
