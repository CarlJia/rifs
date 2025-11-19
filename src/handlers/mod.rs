pub mod auth_handler;
pub mod cache_handler;
pub mod health_handler;
pub mod image_handler;
pub mod static_files;
pub mod token_handler;

pub use auth_handler::{get_auth_config, verify_token};
pub use cache_handler::{
    auto_cleanup_cache, cache_management_dashboard, clean_cache, clear_all_cache, decay_heat_scores,
    get_cache_stats,
};
pub use health_handler::{get_system_stats, health_check_detailed};
pub use image_handler::{
    delete_image, get_image, get_image_info, get_stats, query_images_get, query_images_post,
    upload_image,
};
pub use static_files::{api_docs, gallery_page, login_page, serve_static, user_management_page};
pub use token_handler::{create_token, delete_token, get_token, list_tokens};
