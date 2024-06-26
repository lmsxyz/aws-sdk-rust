// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateScene`](crate::operation::update_scene::builders::UpdateSceneFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`workspace_id(impl Into<String>)`](crate::operation::update_scene::builders::UpdateSceneFluentBuilder::workspace_id) / [`set_workspace_id(Option<String>)`](crate::operation::update_scene::builders::UpdateSceneFluentBuilder::set_workspace_id):<br>required: **true**<br><p>The ID of the workspace that contains the scene.</p><br>
    ///   - [`scene_id(impl Into<String>)`](crate::operation::update_scene::builders::UpdateSceneFluentBuilder::scene_id) / [`set_scene_id(Option<String>)`](crate::operation::update_scene::builders::UpdateSceneFluentBuilder::set_scene_id):<br>required: **true**<br><p>The ID of the scene.</p><br>
    ///   - [`content_location(impl Into<String>)`](crate::operation::update_scene::builders::UpdateSceneFluentBuilder::content_location) / [`set_content_location(Option<String>)`](crate::operation::update_scene::builders::UpdateSceneFluentBuilder::set_content_location):<br>required: **false**<br><p>The relative path that specifies the location of the content definition file.</p><br>
    ///   - [`description(impl Into<String>)`](crate::operation::update_scene::builders::UpdateSceneFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_scene::builders::UpdateSceneFluentBuilder::set_description):<br>required: **false**<br><p>The description of this scene.</p><br>
    ///   - [`capabilities(impl Into<String>)`](crate::operation::update_scene::builders::UpdateSceneFluentBuilder::capabilities) / [`set_capabilities(Option<Vec::<String>>)`](crate::operation::update_scene::builders::UpdateSceneFluentBuilder::set_capabilities):<br>required: **false**<br><p>A list of capabilities that the scene uses to render.</p><br>
    ///   - [`scene_metadata(impl Into<String>, impl Into<String>)`](crate::operation::update_scene::builders::UpdateSceneFluentBuilder::scene_metadata) / [`set_scene_metadata(Option<HashMap::<String, String>>)`](crate::operation::update_scene::builders::UpdateSceneFluentBuilder::set_scene_metadata):<br>required: **false**<br><p>The scene metadata.</p><br>
    /// - On success, responds with [`UpdateSceneOutput`](crate::operation::update_scene::UpdateSceneOutput) with field(s):
    ///   - [`update_date_time(DateTime)`](crate::operation::update_scene::UpdateSceneOutput::update_date_time): <p>The date and time when the scene was last updated.</p>
    /// - On failure, responds with [`SdkError<UpdateSceneError>`](crate::operation::update_scene::UpdateSceneError)
    pub fn update_scene(&self) -> crate::operation::update_scene::builders::UpdateSceneFluentBuilder {
        crate::operation::update_scene::builders::UpdateSceneFluentBuilder::new(self.handle.clone())
    }
}
