# \TransactionService

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_transaction_transaction_create_transaction_post**](TransactionService.md#create_transaction_transaction_create_transaction_post) | **PUT** /transaction/create-transaction | Create Transaction
[**delete_transaction_transaction_delete_transaction_delete**](TransactionService.md#delete_transaction_transaction_delete_transaction_delete) | **DELETE** /transaction/delete-transaction | Delete Transaction
[**get_one_transaction_transaction_get_one_transaction_get**](TransactionService.md#get_one_transaction_transaction_get_one_transaction_get) | **GET** /transaction/get-one-transaction | Get One Transaction
[**get_transactions_transaction_get_transactions_get**](TransactionService.md#get_transactions_transaction_get_transactions_get) | **GET** /transaction/get-transactions | Get Transactions



## create_transaction_transaction_create_transaction_post

> models::TransactionResponse create_transaction_transaction_create_transaction_post(transaction_request)
Create Transaction

This endpoint allows authenticated users to create a transaction by submitting a request with the necessary details. The endpoint ensures that the user's access level permits transaction creation and enforces a rate limit of 30 requests. It checks that the number of products is within the allowed range (1-20) and calculates the total transaction amount, applying a 20% charge for escrow transactions. It also verifies that the total amount does not exceed a specific threshold (2,000,000). The user's IP address and browser information are logged, and the transaction details are saved with relevant metadata, including a unique order ID and the associated account state. If any conditions are not met, appropriate error messages are returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_request** | [**TransactionRequest**](TransactionRequest.md) |  | [required] |

### Return type

[**models::TransactionResponse**](TransactionResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_transaction_transaction_delete_transaction_delete

> models::DeleteTransactionResponse delete_transaction_transaction_delete_transaction_delete(delete_transaction_request)
Delete Transaction

This endpoint allows authenticated users to delete a transaction by submitting a request with the necessary details. The endpoint checks the user's access level to ensure they have permission to delete transactions and enforces a rate limit of 30 requests. It verifies that the transaction belongs to the user and has not already been deleted or withdrawn. If the transaction is paid and has a payment reference, it processes refunds and updates the user's account balance accordingly.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_transaction_request** | [**DeleteTransactionRequest**](DeleteTransactionRequest.md) |  | [required] |

### Return type

[**models::DeleteTransactionResponse**](DeleteTransactionResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_one_transaction_transaction_get_one_transaction_get

> models::GetOneTransaction get_one_transaction_transaction_get_one_transaction_get(order_id)
Get One Transaction

This endpoint allows authenticated users to retrieve the details of a specific transaction using the order ID. The endpoint ensures the user has read access and enforces a rate limit of 30 requests. It checks if the transaction exists in the database and if the user has the appropriate access rights,then provides a checkout URL for the transaction and others informations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_id** | **String** |  | [required] |

### Return type

[**models::GetOneTransaction**](GetOneTransaction.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transactions_transaction_get_transactions_get

> models::GetAllTransaction get_transactions_transaction_get_transactions_get()
Get Transactions

This endpoint allows authenticated users to retrieve a list of their visible transactions, up to a maximum of 50. The endpoint enforces a rate limit of 30 requests and ensures the user has read access.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetAllTransaction**](GetAllTransaction.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

