# FileCapabilities

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can_add_children** | Option<**bool**> | Whether the current user can add children to this folder. This is always false when the item is not a folder. | [optional]
**can_add_folder_from_another_drive** | Option<**bool**> | Whether the current user can add a folder from another drive (different shared drive or My Drive) to this folder. This is false when the item is not a folder. Only populated for items in shared drives. | [optional]
**can_add_my_drive_parent** | Option<**bool**> | Whether the current user can add a parent for the item without removing an existing parent in the same request. Not populated for shared drive files. | [optional]
**can_change_copy_requires_writer_permission** | Option<**bool**> | Whether the current user can change the copyRequiresWriterPermission restriction of this file. | [optional]
**can_change_security_update_enabled** | Option<**bool**> | Whether the current user can change the securityUpdateEnabled field on link share metadata. | [optional]
**can_change_viewers_can_copy_content** | Option<**bool**> | Deprecated | [optional]
**can_comment** | Option<**bool**> | Whether the current user can comment on this file. | [optional]
**can_copy** | Option<**bool**> | Whether the current user can copy this file. For an item in a shared drive, whether the current user can copy non-folder descendants of this item, or this item itself if it is not a folder. | [optional]
**can_delete** | Option<**bool**> | Whether the current user can delete this file. | [optional]
**can_delete_children** | Option<**bool**> | Whether the current user can delete children of this folder. This is false when the item is not a folder. Only populated for items in shared drives. | [optional]
**can_download** | Option<**bool**> | Whether the current user can download this file. | [optional]
**can_edit** | Option<**bool**> | Whether the current user can edit this file. Other factors may limit the type of changes a user can make to a file. For example, see canChangeCopyRequiresWriterPermission or canModifyContent. | [optional]
**can_list_children** | Option<**bool**> | Whether the current user can list the children of this folder. This is always false when the item is not a folder. | [optional]
**can_modify_content** | Option<**bool**> | Whether the current user can modify the content of this file. | [optional]
**can_modify_content_restriction** | Option<**bool**> | Whether the current user can modify restrictions on content of this file. | [optional]
**can_move_children_out_of_drive** | Option<**bool**> | Whether the current user can move children of this folder outside of the shared drive. This is false when the item is not a folder. Only populated for items in shared drives. | [optional]
**can_move_children_out_of_team_drive** | Option<**bool**> | Deprecated - use canMoveChildrenOutOfDrive instead. | [optional]
**can_move_children_within_drive** | Option<**bool**> | Whether the current user can move children of this folder within this drive. This is false when the item is not a folder. Note that a request to move the child may still fail depending on the current user's access to the child and to the destination folder. | [optional]
**can_move_children_within_team_drive** | Option<**bool**> | Deprecated - use canMoveChildrenWithinDrive instead. | [optional]
**can_move_item_into_team_drive** | Option<**bool**> | Deprecated - use canMoveItemOutOfDrive instead. | [optional]
**can_move_item_out_of_drive** | Option<**bool**> | Whether the current user can move this item outside of this drive by changing its parent. Note that a request to change the parent of the item may still fail depending on the new parent that is being added. | [optional]
**can_move_item_out_of_team_drive** | Option<**bool**> | Deprecated - use canMoveItemOutOfDrive instead. | [optional]
**can_move_item_within_drive** | Option<**bool**> | Whether the current user can move this item within this drive. Note that a request to change the parent of the item may still fail depending on the new parent that is being added and the parent that is being removed. | [optional]
**can_move_item_within_team_drive** | Option<**bool**> | Deprecated - use canMoveItemWithinDrive instead. | [optional]
**can_move_team_drive_item** | Option<**bool**> | Deprecated - use canMoveItemWithinDrive or canMoveItemOutOfDrive instead. | [optional]
**can_read_drive** | Option<**bool**> | Whether the current user can read the shared drive to which this file belongs. Only populated for items in shared drives. | [optional]
**can_read_revisions** | Option<**bool**> | Whether the current user can read the revisions resource of this file. For a shared drive item, whether revisions of non-folder descendants of this item, or this item itself if it is not a folder, can be read. | [optional]
**can_read_team_drive** | Option<**bool**> | Deprecated - use canReadDrive instead. | [optional]
**can_remove_children** | Option<**bool**> | Whether the current user can remove children from this folder. This is always false when the item is not a folder. For a folder in a shared drive, use canDeleteChildren or canTrashChildren instead. | [optional]
**can_remove_my_drive_parent** | Option<**bool**> | Whether the current user can remove a parent from the item without adding another parent in the same request. Not populated for shared drive files. | [optional]
**can_rename** | Option<**bool**> | Whether the current user can rename this file. | [optional]
**can_share** | Option<**bool**> | Whether the current user can modify the sharing settings for this file. | [optional]
**can_trash** | Option<**bool**> | Whether the current user can move this file to trash. | [optional]
**can_trash_children** | Option<**bool**> | Whether the current user can trash children of this folder. This is false when the item is not a folder. Only populated for items in shared drives. | [optional]
**can_untrash** | Option<**bool**> | Whether the current user can restore this file from trash. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


