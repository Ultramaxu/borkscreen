use crate::gateways::FileSystemGateway;

pub struct ImageModuleFileSystemAdapter {}

impl ImageModuleFileSystemAdapter {
    pub fn new() -> Self {
        Self {}
    }
}

impl FileSystemGateway for ImageModuleFileSystemAdapter {
    fn save_image(&mut self, image_buffer: image::RgbImage, path: &String) -> anyhow::Result<()> {
        image_buffer
            .save(path)
            .map_err(|e| anyhow::anyhow!("Unable to save file: {:?}", e))
    }
}