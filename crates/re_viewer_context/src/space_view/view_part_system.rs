use ahash::HashMap;

use re_types::ComponentNameSet;

use crate::{
    ApplicableEntities, IdentifiedViewSystem, SpaceViewSystemExecutionError, ViewContextCollection,
    ViewQuery, ViewSystemIdentifier, ViewerContext, VisualizableEntities,
    VisualizerAdditionalApplicabilityFilter,
};

/// Element of a scene derived from a single archetype query.
///
/// Is populated after scene contexts and has access to them.
pub trait ViewPartSystem: Send + Sync + 'static {
    // TODO(andreas): This should be able to list out the ContextSystems it needs.

    /// Returns the minimal set of components that the system _requires_ in order to be instantiated.
    ///
    /// This does not include indicator components.
    fn required_components(&self) -> ComponentNameSet;

    /// These are not required, but if _any_ of these are found, it is a strong indication that this
    /// system should be active (if also the `required_components` are found).
    #[inline]
    fn indicator_components(&self) -> ComponentNameSet {
        Default::default()
    }

    /// Filters a set of applicable entities (entities that have all required components),
    /// into to a set of visualizable entities.
    ///
    /// The context passed in here is generated by [`crate::SpaceViewClass::visualizable_filter_context`].
    #[inline]
    fn filter_visualizable_entities(
        &self,
        entities: ApplicableEntities,
        _context: &dyn std::any::Any,
    ) -> VisualizableEntities {
        VisualizableEntities(entities.0)
    }

    /// Additional filter for applicability.
    ///
    /// If none is specified, applicability is solely determined by required components.
    fn applicability_filter(&self) -> Option<Box<dyn VisualizerAdditionalApplicabilityFilter>> {
        None
    }

    /// Queries the data store and performs data conversions to make it ready for display.
    ///
    /// Mustn't query any data outside of the archetype.
    ///
    /// TODO(andreas): don't pass in `ViewerContext` if we want to restrict the queries here.
    /// If we want to make this restriction, then the trait-contract should be that something external
    /// to the `ViewPartSystemImpl` does the query and then passes an `ArchetypeQueryResult` into populate.
    fn execute(
        &mut self,
        ctx: &ViewerContext<'_>,
        query: &ViewQuery<'_>,
        view_ctx: &ViewContextCollection,
    ) -> Result<Vec<re_renderer::QueueableDrawData>, SpaceViewSystemExecutionError>;

    /// Optionally retrieves a data store reference from the scene element.
    ///
    /// This is useful for retrieving data that is common to several parts of a [`crate::SpaceViewClass`].
    /// For example, if most scene parts produce ui elements, a concrete [`crate::SpaceViewClass`]
    /// can pick those up in its [`crate::SpaceViewClass::ui`] method by iterating over all scene parts.
    fn data(&self) -> Option<&dyn std::any::Any> {
        None
    }

    fn as_any(&self) -> &dyn std::any::Any;
}

pub struct ViewPartCollection {
    pub systems: HashMap<ViewSystemIdentifier, Box<dyn ViewPartSystem>>,
}

impl ViewPartCollection {
    #[inline]
    pub fn get<T: ViewPartSystem + IdentifiedViewSystem + 'static>(
        &self,
    ) -> Result<&T, SpaceViewSystemExecutionError> {
        self.systems
            .get(&T::identifier())
            .and_then(|s| s.as_any().downcast_ref())
            .ok_or_else(|| {
                SpaceViewSystemExecutionError::PartSystemNotFound(T::identifier().as_str())
            })
    }

    #[inline]
    pub fn get_by_identifier(
        &self,
        name: ViewSystemIdentifier,
    ) -> Result<&dyn ViewPartSystem, SpaceViewSystemExecutionError> {
        self.systems
            .get(&name)
            .map(|s| s.as_ref())
            .ok_or_else(|| SpaceViewSystemExecutionError::PartSystemNotFound(name.as_str()))
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &dyn ViewPartSystem> {
        self.systems.values().map(|s| s.as_ref())
    }

    #[inline]
    pub fn iter_with_identifiers(
        &self,
    ) -> impl Iterator<Item = (ViewSystemIdentifier, &dyn ViewPartSystem)> {
        self.systems.iter().map(|s| (*s.0, s.1.as_ref()))
    }
}
