# \CashoutService

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cash_out_orange_money_cashout_orange_money_post**](CashoutService.md#cash_out_orange_money_cashout_orange_money_post) | **POST** /cashout/orange-money | Cash Out Orange Money
[**cash_out_wave_cashout_wave_post**](CashoutService.md#cash_out_wave_cashout_wave_post) | **POST** /cashout/wave | Cash Out Wave



## cash_out_orange_money_cashout_orange_money_post

> models::CashOutResponse cash_out_orange_money_cashout_orange_money_post(cash_out_orange_request)
Cash Out Orange Money

This endpoint enables a user to withdraw funds from their NabooPay account to their Orange Money account. Authentication is required, and users with the 'dev' role are not permitted to perform this operation. The amount to be withdrawn must be positive and within the user's available balance. The user's account must be active and registered in the system. After validation, the user's account balance is adjusted accordingly, and the transaction is logged for record-keeping.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cash_out_orange_request** | [**CashOutOrangeRequest**](CashOutOrangeRequest.md) |  | [required] |

### Return type

[**models::CashOutResponse**](CashOutResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cash_out_wave_cashout_wave_post

> models::CashOutResponse cash_out_wave_cashout_wave_post(cash_out_wave_request)
Cash Out Wave

This endpoint allows a user to withdraw funds from their NabooPay account to their Wave account. The user must be authenticated and not possess the 'dev' role. The withdrawal amount must be greater than 10 and not exceed the user's account balance. The user's account must exist and be active. Upon successful validation, the system updates the user's account balance and records the transaction for tracking purposes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cash_out_wave_request** | [**CashOutWaveRequest**](CashOutWaveRequest.md) |  | [required] |

### Return type

[**models::CashOutResponse**](CashOutResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

