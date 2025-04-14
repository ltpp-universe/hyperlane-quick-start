use super::*;

pub async fn handle(ctx: Context) {
    let dir: String = ctx.get_route_param(DIR_KEY).await.unwrap_or_default();
    let file: String = ctx.get_route_param(FILE_KEY).await.unwrap_or_default();
    let decode_dir: String = decode(CHARSETS, &dir).unwrap_or_default();
    let decode_file: String = decode(CHARSETS, &file).unwrap_or_default();
    if decode_dir.is_empty() || decode_file.is_empty() {
        return;
    }
    let path: String = format!("{}/{}/{}", UPLOAD_DIR, decode_dir, decode_file);
    let extension_name: String = FileExtension::get_extension_name(&decode_file);
    let content_type: &str = FileExtension::parse(&extension_name).get_content_type();
    let data: Vec<u8> = async_read_from_file(&path).await.unwrap_or_default();
    let content_type: String = content_type_charset(content_type, UTF8);
    ctx.set_response_header(CONTENT_TYPE, content_type_charset(TEXT_HTML, UTF8))
        .await
        .set_response_header(CONTENT_TYPE, content_type)
        .await
        .set_response_header(CACHE_CONTROL, "public, max-age=31536000, immutable")
        .await
        .set_response_header(EXPIRES, "Wed, 1 Apr 8888 00:00:00 GMT")
        .await
        .set_response_body(data)
        .await;
}
