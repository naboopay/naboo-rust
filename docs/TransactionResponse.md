# TransactionResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**order_id** | **String** |  | 
**method_of_payment** | **Vec<String>** |  | 
**amount** | Option<**i32**> |  | [optional][default to 0]
**amount_to_pay** | Option<**i32**> |  | [optional][default to 0]
**currency** | **String** |  | 
**created_at** | **String** |  | 
**transaction_status** | Option<**String**> |  | [optional][default to pending]
**is_escrow** | Option<**bool**> |  | [optional][default to false]
**is_merchant** | Option<**bool**> |  | [optional][default to false]
**checkout_url** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


