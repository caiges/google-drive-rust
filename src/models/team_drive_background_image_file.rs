/*
 * Drive API
 *
 * Manages files in Drive including uploading, downloading, searching, detecting changes, and updating sharing permissions.
 *
 * The version of the OpenAPI document: v3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TeamDriveBackgroundImageFile : An image file and cropping parameters from which a background image for this Team Drive is set. This is a write only field; it can only be set on drive.teamdrives.update requests that don't set themeId. When specified, all fields of the backgroundImageFile must be set.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TeamDriveBackgroundImageFile {
    /// The ID of an image file in Drive to use for the background image.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The width of the cropped image in the closed range of 0 to 1. This value represents the width of the cropped image divided by the width of the entire image. The height is computed by applying a width to height aspect ratio of 80 to 9. The resulting image must be at least 1280 pixels wide and 144 pixels high.
    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
    pub width: Option<f32>,
    /// The X coordinate of the upper left corner of the cropping area in the background image. This is a value in the closed range of 0 to 1. This value represents the horizontal distance from the left side of the entire image to the left side of the cropping area divided by the width of the entire image.
    #[serde(rename = "xCoordinate", skip_serializing_if = "Option::is_none")]
    pub x_coordinate: Option<f32>,
    /// The Y coordinate of the upper left corner of the cropping area in the background image. This is a value in the closed range of 0 to 1. This value represents the vertical distance from the top side of the entire image to the top side of the cropping area divided by the height of the entire image.
    #[serde(rename = "yCoordinate", skip_serializing_if = "Option::is_none")]
    pub y_coordinate: Option<f32>,
}

impl TeamDriveBackgroundImageFile {
    /// An image file and cropping parameters from which a background image for this Team Drive is set. This is a write only field; it can only be set on drive.teamdrives.update requests that don't set themeId. When specified, all fields of the backgroundImageFile must be set.
    pub fn new() -> TeamDriveBackgroundImageFile {
        TeamDriveBackgroundImageFile {
            id: None,
            width: None,
            x_coordinate: None,
            y_coordinate: None,
        }
    }
}


