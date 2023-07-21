// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.

#![allow(trivial_numeric_casts)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

/// A 3D point cloud with positions and optional colors, radii, labels, etc.
///
/// ## Example
///
/// ```ignore
/// //! Log some very simple points.
/// use rerun::{experimental::archetypes::Points3D, MsgSender, RecordingStreamBuilder};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///    let (rec_stream, storage) = RecordingStreamBuilder::new("points").memory()?;
///
///    MsgSender::from_archetype("points", &Points3D::new([(0.0, 0.0, 0.0), (1.0, 1.0, 1.0)]))?
///        .send(&rec_stream)?;
///
///    rerun::native_viewer::show(storage.take())?;
///    Ok(())
/// }
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Points3D {
    /// All the actual 3D points that make up the point cloud.
    pub points: Vec<crate::components::Point3D>,

    /// Optional radii for the points, effectively turning them into circles.
    pub radii: Option<Vec<crate::components::Radius>>,

    /// Optional colors for the points.
    pub colors: Option<Vec<crate::components::Color>>,

    /// Optional text labels for the points.
    pub labels: Option<Vec<crate::components::Label>>,

    /// An optional floating point value that specifies the 3D drawing order.
    /// Objects with higher values are drawn on top of those with lower values.
    ///
    /// The default for 3D points is 30.0.
    pub draw_order: Option<crate::components::DrawOrder>,

    /// Optional class Ids for the points.
    ///
    /// The class ID provides colors and labels if not specified explicitly.
    pub class_ids: Option<Vec<crate::components::ClassId>>,

    /// Optional keypoint IDs for the points, identifying them within a class.
    ///
    /// If keypoint IDs are passed in but no class IDs were specified, the class ID will
    /// default to 0.
    /// This is useful to identify points within a single classification (which is identified
    /// with `class_id`).
    /// E.g. the classification might be 'Person' and the keypoints refer to joints on a
    /// detected skeleton.
    pub keypoint_ids: Option<Vec<crate::components::KeypointId>>,

    /// Unique identifiers for each individual point in the batch.
    pub instance_keys: Option<Vec<crate::components::InstanceKey>>,
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[crate::ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.point3d".into()]);
static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[crate::ComponentName; 2usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.radius".into(), "rerun.colorrgba".into()]);
static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[crate::ComponentName; 5usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.label".into(),
            "rerun.draw_order".into(),
            "rerun.class_id".into(),
            "rerun.keypoint_id".into(),
            "rerun.instance_key".into(),
        ]
    });
static ALL_COMPONENTS: once_cell::sync::Lazy<[crate::ComponentName; 8usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.point3d".into(),
            "rerun.radius".into(),
            "rerun.colorrgba".into(),
            "rerun.label".into(),
            "rerun.draw_order".into(),
            "rerun.class_id".into(),
            "rerun.keypoint_id".into(),
            "rerun.instance_key".into(),
        ]
    });

impl Points3D {
    pub const NUM_COMPONENTS: usize = 8usize;
}

impl crate::Archetype for Points3D {
    #[inline]
    fn name() -> crate::ArchetypeName {
        crate::ArchetypeName::Borrowed("rerun.archetypes.Points3D")
    }

    #[inline]
    fn required_components() -> &'static [crate::ComponentName] {
        REQUIRED_COMPONENTS.as_slice()
    }

    #[inline]
    fn recommended_components() -> &'static [crate::ComponentName] {
        RECOMMENDED_COMPONENTS.as_slice()
    }

    #[inline]
    fn optional_components() -> &'static [crate::ComponentName] {
        OPTIONAL_COMPONENTS.as_slice()
    }

    #[inline]
    fn all_components() -> &'static [crate::ComponentName] {
        ALL_COMPONENTS.as_slice()
    }

    #[inline]
    fn try_to_arrow(
        &self,
    ) -> crate::SerializationResult<
        Vec<(::arrow2::datatypes::Field, Box<dyn ::arrow2::array::Array>)>,
    > {
        use crate::Loggable as _;
        Ok([
            {
                Some({
                    let array =
                        <crate::components::Point3D>::try_to_arrow(self.points.iter(), None);
                    array.map(|array| {
                        let datatype = ::arrow2::datatypes::DataType::Extension(
                            "rerun.components.Point3D".into(),
                            Box::new(array.data_type().clone()),
                            Some("rerun.point3d".into()),
                        );
                        (
                            ::arrow2::datatypes::Field::new("points", datatype, false),
                            array,
                        )
                    })
                })
                .transpose()
                .map_err(|err| crate::SerializationError::Context {
                    location: "rerun.archetypes.Points3D#points".into(),
                    source: Box::new(err),
                })?
            },
            {
                self.radii
                    .as_ref()
                    .map(|many| {
                        let array = <crate::components::Radius>::try_to_arrow(many.iter(), None);
                        array.map(|array| {
                            let datatype = ::arrow2::datatypes::DataType::Extension(
                                "rerun.components.Radius".into(),
                                Box::new(array.data_type().clone()),
                                Some("rerun.radius".into()),
                            );
                            (
                                ::arrow2::datatypes::Field::new("radii", datatype, false),
                                array,
                            )
                        })
                    })
                    .transpose()
                    .map_err(|err| crate::SerializationError::Context {
                        location: "rerun.archetypes.Points3D#radii".into(),
                        source: Box::new(err),
                    })?
            },
            {
                self.colors
                    .as_ref()
                    .map(|many| {
                        let array = <crate::components::Color>::try_to_arrow(many.iter(), None);
                        array.map(|array| {
                            let datatype = ::arrow2::datatypes::DataType::Extension(
                                "rerun.components.Color".into(),
                                Box::new(array.data_type().clone()),
                                Some("rerun.colorrgba".into()),
                            );
                            (
                                ::arrow2::datatypes::Field::new("colors", datatype, false),
                                array,
                            )
                        })
                    })
                    .transpose()
                    .map_err(|err| crate::SerializationError::Context {
                        location: "rerun.archetypes.Points3D#colors".into(),
                        source: Box::new(err),
                    })?
            },
            {
                self.labels
                    .as_ref()
                    .map(|many| {
                        let array = <crate::components::Label>::try_to_arrow(many.iter(), None);
                        array.map(|array| {
                            let datatype = ::arrow2::datatypes::DataType::Extension(
                                "rerun.components.Label".into(),
                                Box::new(array.data_type().clone()),
                                Some("rerun.label".into()),
                            );
                            (
                                ::arrow2::datatypes::Field::new("labels", datatype, false),
                                array,
                            )
                        })
                    })
                    .transpose()
                    .map_err(|err| crate::SerializationError::Context {
                        location: "rerun.archetypes.Points3D#labels".into(),
                        source: Box::new(err),
                    })?
            },
            {
                self.draw_order
                    .as_ref()
                    .map(|single| {
                        let array = <crate::components::DrawOrder>::try_to_arrow([single], None);
                        array.map(|array| {
                            let datatype = ::arrow2::datatypes::DataType::Extension(
                                "rerun.components.DrawOrder".into(),
                                Box::new(array.data_type().clone()),
                                Some("rerun.draw_order".into()),
                            );
                            (
                                ::arrow2::datatypes::Field::new("draw_order", datatype, false),
                                array,
                            )
                        })
                    })
                    .transpose()
                    .map_err(|err| crate::SerializationError::Context {
                        location: "rerun.archetypes.Points3D#draw_order".into(),
                        source: Box::new(err),
                    })?
            },
            {
                self.class_ids
                    .as_ref()
                    .map(|many| {
                        let array = <crate::components::ClassId>::try_to_arrow(many.iter(), None);
                        array.map(|array| {
                            let datatype = ::arrow2::datatypes::DataType::Extension(
                                "rerun.components.ClassId".into(),
                                Box::new(array.data_type().clone()),
                                Some("rerun.class_id".into()),
                            );
                            (
                                ::arrow2::datatypes::Field::new("class_ids", datatype, false),
                                array,
                            )
                        })
                    })
                    .transpose()
                    .map_err(|err| crate::SerializationError::Context {
                        location: "rerun.archetypes.Points3D#class_ids".into(),
                        source: Box::new(err),
                    })?
            },
            {
                self.keypoint_ids
                    .as_ref()
                    .map(|many| {
                        let array =
                            <crate::components::KeypointId>::try_to_arrow(many.iter(), None);
                        array.map(|array| {
                            let datatype = ::arrow2::datatypes::DataType::Extension(
                                "rerun.components.KeypointId".into(),
                                Box::new(array.data_type().clone()),
                                Some("rerun.keypoint_id".into()),
                            );
                            (
                                ::arrow2::datatypes::Field::new("keypoint_ids", datatype, false),
                                array,
                            )
                        })
                    })
                    .transpose()
                    .map_err(|err| crate::SerializationError::Context {
                        location: "rerun.archetypes.Points3D#keypoint_ids".into(),
                        source: Box::new(err),
                    })?
            },
            {
                self.instance_keys
                    .as_ref()
                    .map(|many| {
                        let array =
                            <crate::components::InstanceKey>::try_to_arrow(many.iter(), None);
                        array.map(|array| {
                            let datatype = ::arrow2::datatypes::DataType::Extension(
                                "rerun.components.InstanceKey".into(),
                                Box::new(array.data_type().clone()),
                                Some("rerun.instance_key".into()),
                            );
                            (
                                ::arrow2::datatypes::Field::new("instance_keys", datatype, false),
                                array,
                            )
                        })
                    })
                    .transpose()
                    .map_err(|err| crate::SerializationError::Context {
                        location: "rerun.archetypes.Points3D#instance_keys".into(),
                        source: Box::new(err),
                    })?
            },
        ]
        .into_iter()
        .flatten()
        .collect())
    }

    #[inline]
    fn try_from_arrow(
        data: impl IntoIterator<Item = (::arrow2::datatypes::Field, Box<dyn ::arrow2::array::Array>)>,
    ) -> crate::DeserializationResult<Self> {
        use crate::Loggable as _;
        let arrays_by_name: ::std::collections::HashMap<_, _> = data
            .into_iter()
            .map(|(field, array)| (field.name, array))
            .collect();
        let points = {
            let array = arrays_by_name
                .get("points")
                .ok_or_else(|| crate::DeserializationError::MissingData {
                    backtrace: ::backtrace::Backtrace::new_unresolved(),
                })
                .map_err(|err| crate::DeserializationError::Context {
                    location: "rerun.archetypes.Points3D#points".into(),
                    source: Box::new(err),
                })?;
            <crate::components::Point3D>::try_from_arrow_opt(&**array)
                .map_err(|err| crate::DeserializationError::Context {
                    location: "rerun.archetypes.Points3D#points".into(),
                    source: Box::new(err),
                })?
                .into_iter()
                .map(|v| {
                    v.ok_or_else(|| crate::DeserializationError::MissingData {
                        backtrace: ::backtrace::Backtrace::new_unresolved(),
                    })
                })
                .collect::<crate::DeserializationResult<Vec<_>>>()
                .map_err(|err| crate::DeserializationError::Context {
                    location: "rerun.archetypes.Points3D#points".into(),
                    source: Box::new(err),
                })?
        };
        let radii = if let Some(array) = arrays_by_name.get("radii") {
            Some(
                <crate::components::Radius>::try_from_arrow_opt(&**array)
                    .map_err(|err| crate::DeserializationError::Context {
                        location: "rerun.archetypes.Points3D#radii".into(),
                        source: Box::new(err),
                    })?
                    .into_iter()
                    .map(|v| {
                        v.ok_or_else(|| crate::DeserializationError::MissingData {
                            backtrace: ::backtrace::Backtrace::new_unresolved(),
                        })
                    })
                    .collect::<crate::DeserializationResult<Vec<_>>>()
                    .map_err(|err| crate::DeserializationError::Context {
                        location: "rerun.archetypes.Points3D#radii".into(),
                        source: Box::new(err),
                    })?,
            )
        } else {
            None
        };
        let colors = if let Some(array) = arrays_by_name.get("colors") {
            Some(
                <crate::components::Color>::try_from_arrow_opt(&**array)
                    .map_err(|err| crate::DeserializationError::Context {
                        location: "rerun.archetypes.Points3D#colors".into(),
                        source: Box::new(err),
                    })?
                    .into_iter()
                    .map(|v| {
                        v.ok_or_else(|| crate::DeserializationError::MissingData {
                            backtrace: ::backtrace::Backtrace::new_unresolved(),
                        })
                    })
                    .collect::<crate::DeserializationResult<Vec<_>>>()
                    .map_err(|err| crate::DeserializationError::Context {
                        location: "rerun.archetypes.Points3D#colors".into(),
                        source: Box::new(err),
                    })?,
            )
        } else {
            None
        };
        let labels = if let Some(array) = arrays_by_name.get("labels") {
            Some(
                <crate::components::Label>::try_from_arrow_opt(&**array)
                    .map_err(|err| crate::DeserializationError::Context {
                        location: "rerun.archetypes.Points3D#labels".into(),
                        source: Box::new(err),
                    })?
                    .into_iter()
                    .map(|v| {
                        v.ok_or_else(|| crate::DeserializationError::MissingData {
                            backtrace: ::backtrace::Backtrace::new_unresolved(),
                        })
                    })
                    .collect::<crate::DeserializationResult<Vec<_>>>()
                    .map_err(|err| crate::DeserializationError::Context {
                        location: "rerun.archetypes.Points3D#labels".into(),
                        source: Box::new(err),
                    })?,
            )
        } else {
            None
        };
        let draw_order = if let Some(array) = arrays_by_name.get("draw_order") {
            Some(
                <crate::components::DrawOrder>::try_from_arrow_opt(&**array)
                    .map_err(|err| crate::DeserializationError::Context {
                        location: "rerun.archetypes.Points3D#draw_order".into(),
                        source: Box::new(err),
                    })?
                    .into_iter()
                    .next()
                    .flatten()
                    .ok_or_else(|| crate::DeserializationError::MissingData {
                        backtrace: ::backtrace::Backtrace::new_unresolved(),
                    })
                    .map_err(|err| crate::DeserializationError::Context {
                        location: "rerun.archetypes.Points3D#draw_order".into(),
                        source: Box::new(err),
                    })?,
            )
        } else {
            None
        };
        let class_ids = if let Some(array) = arrays_by_name.get("class_ids") {
            Some(
                <crate::components::ClassId>::try_from_arrow_opt(&**array)
                    .map_err(|err| crate::DeserializationError::Context {
                        location: "rerun.archetypes.Points3D#class_ids".into(),
                        source: Box::new(err),
                    })?
                    .into_iter()
                    .map(|v| {
                        v.ok_or_else(|| crate::DeserializationError::MissingData {
                            backtrace: ::backtrace::Backtrace::new_unresolved(),
                        })
                    })
                    .collect::<crate::DeserializationResult<Vec<_>>>()
                    .map_err(|err| crate::DeserializationError::Context {
                        location: "rerun.archetypes.Points3D#class_ids".into(),
                        source: Box::new(err),
                    })?,
            )
        } else {
            None
        };
        let keypoint_ids = if let Some(array) = arrays_by_name.get("keypoint_ids") {
            Some(
                <crate::components::KeypointId>::try_from_arrow_opt(&**array)
                    .map_err(|err| crate::DeserializationError::Context {
                        location: "rerun.archetypes.Points3D#keypoint_ids".into(),
                        source: Box::new(err),
                    })?
                    .into_iter()
                    .map(|v| {
                        v.ok_or_else(|| crate::DeserializationError::MissingData {
                            backtrace: ::backtrace::Backtrace::new_unresolved(),
                        })
                    })
                    .collect::<crate::DeserializationResult<Vec<_>>>()
                    .map_err(|err| crate::DeserializationError::Context {
                        location: "rerun.archetypes.Points3D#keypoint_ids".into(),
                        source: Box::new(err),
                    })?,
            )
        } else {
            None
        };
        let instance_keys = if let Some(array) = arrays_by_name.get("instance_keys") {
            Some(
                <crate::components::InstanceKey>::try_from_arrow_opt(&**array)
                    .map_err(|err| crate::DeserializationError::Context {
                        location: "rerun.archetypes.Points3D#instance_keys".into(),
                        source: Box::new(err),
                    })?
                    .into_iter()
                    .map(|v| {
                        v.ok_or_else(|| crate::DeserializationError::MissingData {
                            backtrace: ::backtrace::Backtrace::new_unresolved(),
                        })
                    })
                    .collect::<crate::DeserializationResult<Vec<_>>>()
                    .map_err(|err| crate::DeserializationError::Context {
                        location: "rerun.archetypes.Points3D#instance_keys".into(),
                        source: Box::new(err),
                    })?,
            )
        } else {
            None
        };
        Ok(Self {
            points,
            radii,
            colors,
            labels,
            draw_order,
            class_ids,
            keypoint_ids,
            instance_keys,
        })
    }
}

impl Points3D {
    pub fn new(points: impl IntoIterator<Item = impl Into<crate::components::Point3D>>) -> Self {
        Self {
            points: points.into_iter().map(Into::into).collect(),
            radii: None,
            colors: None,
            labels: None,
            draw_order: None,
            class_ids: None,
            keypoint_ids: None,
            instance_keys: None,
        }
    }

    pub fn with_radii(
        mut self,
        radii: impl IntoIterator<Item = impl Into<crate::components::Radius>>,
    ) -> Self {
        self.radii = Some(radii.into_iter().map(Into::into).collect());
        self
    }

    pub fn with_colors(
        mut self,
        colors: impl IntoIterator<Item = impl Into<crate::components::Color>>,
    ) -> Self {
        self.colors = Some(colors.into_iter().map(Into::into).collect());
        self
    }

    pub fn with_labels(
        mut self,
        labels: impl IntoIterator<Item = impl Into<crate::components::Label>>,
    ) -> Self {
        self.labels = Some(labels.into_iter().map(Into::into).collect());
        self
    }

    pub fn with_draw_order(mut self, draw_order: impl Into<crate::components::DrawOrder>) -> Self {
        self.draw_order = Some(draw_order.into());
        self
    }

    pub fn with_class_ids(
        mut self,
        class_ids: impl IntoIterator<Item = impl Into<crate::components::ClassId>>,
    ) -> Self {
        self.class_ids = Some(class_ids.into_iter().map(Into::into).collect());
        self
    }

    pub fn with_keypoint_ids(
        mut self,
        keypoint_ids: impl IntoIterator<Item = impl Into<crate::components::KeypointId>>,
    ) -> Self {
        self.keypoint_ids = Some(keypoint_ids.into_iter().map(Into::into).collect());
        self
    }

    pub fn with_instance_keys(
        mut self,
        instance_keys: impl IntoIterator<Item = impl Into<crate::components::InstanceKey>>,
    ) -> Self {
        self.instance_keys = Some(instance_keys.into_iter().map(Into::into).collect());
        self
    }
}
