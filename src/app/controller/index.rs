use crate::*;

pub fn index(arc_lock_controller_data: ArcRwLockControllerData) {
    let send_res: ResponseResult =
        send_response(&arc_lock_controller_data, 200, "hello hyperlane => /index");
    let controller_data: ControllerData = get_read_controller_data(&arc_lock_controller_data);
    controller_data.get_log().info(
        format!("Response result => {:?}", send_res),
        log_debug_format_handler,
    );
}
