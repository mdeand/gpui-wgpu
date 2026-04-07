#[test]
#[ignore = "VisualContext trait shape differs between wgpui and gpui-ce macro output"]
fn test_derive_context() {
    use gpui::{App, Window};
    use gpui_ce_macros::{AppContext, VisualContext};

    #[derive(AppContext, VisualContext)]
    struct _MyCustomContext<'a, 'b> {
        #[app]
        app: &'a mut App,
        #[window]
        window: &'b mut Window,
    }
}