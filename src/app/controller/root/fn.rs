use super::*;

pub async fn handle(ctx: Context) {
    let dir: String = ctx.get_route_param(DIR_KEY).await.unwrap_or_default();
    let file: String = ctx.get_route_param(FILE_KEY).await.unwrap_or_default();
    if dir.is_empty() {
        ctx.set_response_status_code(200)
            .await
            .set_response_body(ROOT_HTML)
            .await
            .set_response_header(CONTENT_TYPE, TEXT_HTML)
            .await;
        return;
    }
    let file_path: String = if file.is_empty() {
        format!("./group-chat/dist/{dir}")
    } else {
        format!("./group-chat/dist/{dir}/{file}")
    };
    let extension_name: String = if file.is_empty() {
        FileExtension::get_extension_name(&file)
    } else {
        FileExtension::get_extension_name(&dir)
    };
    let content_type: &str = FileExtension::parse(&extension_name).get_content_type();
    let res: Vec<u8> = async_read_from_file(&file_path).await.unwrap();
    ctx.set_response_status_code(200)
        .await
        .set_response_header(CONTENT_TYPE, content_type)
        .await
        .set_response_body(res)
        .await;
}
