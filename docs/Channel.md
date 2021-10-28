# Channel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address** | Option<**String**> | The address where notifications are delivered for this channel. | [optional]
**expiration** | Option<**String**> | Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional. | [optional]
**id** | Option<**String**> | A UUID or similar unique string that identifies this channel. | [optional]
**kind** | Option<**String**> | Identifies this as a notification channel used to watch for changes to a resource, which is \"api#channel\". | [optional][default to api#channel]
**params** | Option<**::std::collections::HashMap<String, String>**> | Additional parameters controlling delivery channel behavior. Optional. | [optional]
**payload** | Option<**bool**> | A Boolean value to indicate whether payload is wanted. Optional. | [optional]
**resource_id** | Option<**String**> | An opaque ID that identifies the resource being watched on this channel. Stable across different API versions. | [optional]
**resource_uri** | Option<**String**> | A version-specific identifier for the watched resource. | [optional]
**token** | Option<**String**> | An arbitrary string delivered to the target address with each notification delivered over this channel. Optional. | [optional]
**_type** | Option<**String**> | The type of delivery mechanism used for this channel. Valid values are \"web_hook\" (or \"webhook\"). Both values refer to a channel where Http requests are used to deliver messages. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


