pub mod api;

#[macro_export]
macro_rules! serve_editor {
    ($app: expr) => {
        use actix_files::Files;
        $app = $app.service(
            scope("/editor")
            .service(editor::api::index)
            .service(editor::api::scripts)
            .service(editor::api::save)
            .service(editor::api::run)
            .service(
                Files::new("/vs", "./static/browser_editor/vs")
                    .show_files_listing()
                    .use_last_modified(true),
            ),
        );
        log::info!("editor is serving")
    };
}
