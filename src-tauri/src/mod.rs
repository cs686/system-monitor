// pub mod abi;
// pub use abi::*;
// use anyhow::Ok;
// use base64::{Engine as _, engine::general_purpose};
// use prost::Message;
// // use photon_rs::transform::SamplingFilter;



// impl ImageSpec {
//     fn new(specs: Vec<Spec>) -> Self {
//         Self { specs }
//     }
// }

// impl From<&ImageSpec> for String {
//     fn from(image_spec: &ImageSpec) -> Self {
//         let data = image_spec.encode_to_vec();
//         general_purpose::URL_SAFE_NO_PAD.encode(data)
//     }
// }

// impl TryFrom<&str> for ImageSpec {
//     type Error = anyhow::Error;

//     fn try_from(value: &str) -> Result<Self, Self::Error> {
//         let data = general_purpose::URL_SAFE_NO_PAD.decode(value)?;
//         Ok(ImageSpec::decode(&data[..])?)
//     }
// }

// impl filter::Filter {
//     // trans filter enum to str
//     pub fn to_str(&self) -> Option<&str> {
//         match self {
//             filter::Filter::Unspecified => None,
//             filter::Filter::Islands => Some("Islands"),
//             filter::Filter::Marine => Some("marine"),
//             filter::Filter::Oceanic => Some("oceanic"),
//         }
//     }
// }

// // impl From<SampleFilter> for SamplingFilter {
// //     fn from(value: SampleFilter) -> Self {
// //         match value {
// //         }
// //     }
// // }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_spec () {
//         // test combinations of supported image formats
//         let image_spec = ImageSpec::new(vec![
//         ]);

//     }
// }