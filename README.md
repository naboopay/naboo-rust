# Rust API client for naboo

Here you have the first version of the naboo api to create checkout automatically


<!-- - API version: 0.1.0 -->

## Installation

Put the package under your project folder in a directory named `naboo` and add the following to `Cargo.toml` under `[dependencies]`:

```
naboo = { path = "./naboo" }
```

## Documentation for API Endpoints

All URIs are relative to */api/v1*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AccountService* | [**get_account_information_account_get**](docs/AccountService.md#get_account_information_account_get) | **GET** /account/ | Get Account Information
*CashoutService* | [**cash_out_orange_money_cashout_orange_money_post**](docs/CashoutService.md#cash_out_orange_money_cashout_orange_money_post) | **POST** /cashout/orange-money | Cash Out Orange Money
*CashoutService* | [**cash_out_wave_cashout_wave_post**](docs/CashoutService.md#cash_out_wave_cashout_wave_post) | **POST** /cashout/wave | Cash Out Wave
*TransactionService* | [**create_transaction_transaction_create_transaction_post**](docs/TransactionService.md#create_transaction_transaction_create_transaction_post) | **PUT** /transaction/create-transaction | Create Transaction
*TransactionService* | [**delete_transaction_transaction_delete_transaction_delete**](docs/TransactionService.md#delete_transaction_transaction_delete_transaction_delete) | **DELETE** /transaction/delete-transaction | Delete Transaction
*TransactionService* | [**get_one_transaction_transaction_get_one_transaction_get**](docs/TransactionService.md#get_one_transaction_transaction_get_one_transaction_get) | **GET** /transaction/get-one-transaction | Get One Transaction
*TransactionService* | [**get_transactions_transaction_get_transactions_get**](docs/TransactionService.md#get_transactions_transaction_get_transactions_get) | **GET** /transaction/get-transactions | Get Transactions


## Documentation For Models

 - [CashOutOrangeRequest](docs/CashOutOrangeRequest.md)
 - [CashOutResponse](docs/CashOutResponse.md)
 - [CashOutWaveRequest](docs/CashOutWaveRequest.md)
 - [DeleteTransactionRequest](docs/DeleteTransactionRequest.md)
 - [DeleteTransactionResponse](docs/DeleteTransactionResponse.md)
 - [GetAccountResponse](docs/GetAccountResponse.md)
 - [GetAllTransaction](docs/GetAllTransaction.md)
 - [GetOneTransaction](docs/GetOneTransaction.md)
 - [HttpValidationError](docs/HttpValidationError.md)
 - [ProductModel](docs/ProductModel.md)
 - [TransactionRequest](docs/TransactionRequest.md)
 - [TransactionResponse](docs/TransactionResponse.md)
 - [TransactionStatusEnum](docs/TransactionStatusEnum.md)
 - [ValidationError](docs/ValidationError.md)
 - [ValidationErrorLocInner](docs/ValidationErrorLocInner.md)
 - [Wallet](docs/Wallet.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



