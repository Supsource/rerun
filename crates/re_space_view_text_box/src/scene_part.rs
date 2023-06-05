use re_arrow_store::LatestAtQuery;
use re_components::Component;
use re_query::{query_entity_with_primary, QueryError};
use re_viewer_context::{
    ArchetypeDefinition, ScenePart, ScenePartCollection, SceneQuery, SpaceViewClassImpl,
    SpaceViewHighlights, ViewerContext,
};

use crate::TextBoxSpaceView;

// ---

#[derive(Debug, Clone)]
pub struct TextBoxEntry {
    pub body: String,
}

/// A text scene, with everything needed to render it.
#[derive(Default)]
pub struct SceneTextBox {
    pub text_entries: Vec<TextBoxEntry>,
}

impl ScenePartCollection<TextBoxSpaceView> for SceneTextBox {
    fn vec_mut(&mut self) -> Vec<&mut dyn ScenePart<TextBoxSpaceView>> {
        vec![self]
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ScenePart<TextBoxSpaceView> for SceneTextBox {
    fn archetype(&self) -> ArchetypeDefinition {
        vec1::vec1![re_components::TextBox::name()]
    }

    fn populate(
        &mut self,
        ctx: &mut ViewerContext<'_>,
        query: &SceneQuery<'_>,
        _space_view_state: &<TextBoxSpaceView as SpaceViewClassImpl>::SpaceViewState,
        _scene_context: &<TextBoxSpaceView as SpaceViewClassImpl>::SceneContext,
        _highlights: &SpaceViewHighlights,
    ) -> Vec<re_renderer::QueueableDrawData> {
        let store = &ctx.store_db.entity_db.data_store;

        for (ent_path, props) in query.iter_entities() {
            if !props.visible {
                continue;
            }

            let query = LatestAtQuery::new(query.timeline, query.latest_at);
            match query_entity_with_primary::<re_components::TextBox>(store, &query, ent_path, &[])
                .and_then(|ent_view| {
                    for text_entry in ent_view.iter_primary()?.flatten() {
                        let re_components::TextBox { body } = text_entry;
                        self.text_entries.push(TextBoxEntry { body });
                    }
                    Ok(())
                }) {
                Ok(_) | Err(QueryError::PrimaryNotFound) => {}
                Err(_) => {
                    re_log::warn_once!("text-box query failed for {ent_path:?}");
                }
            }
        }
        Vec::new()
    }
}