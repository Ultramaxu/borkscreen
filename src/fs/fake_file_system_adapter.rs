use image::RgbImage;

use crate::gateways::FileSystemGateway;

pub struct FakeFileSystemAdapter {
    pub vec: Vec<(RgbImage, String)>,
    result: Box<dyn Fn() -> anyhow::Result<()>>,
}

impl FakeFileSystemAdapter {
    pub fn new() -> Self {
        Self {
            vec: Vec::new(),
            result: Box::new(|| {Err(anyhow::anyhow!("Unable to save file"))}),
        }
    }
    pub fn with_result(mut self, result: Box<dyn Fn() -> anyhow::Result<()>>) -> Self {
        self.result = result;
        self
    }
}

impl FileSystemGateway for FakeFileSystemAdapter {
    fn save_image(&mut self, image_buffer: RgbImage, path: &String) -> anyhow::Result<()> {
        self.vec.push((image_buffer, path.clone()));
        (self.result)()
    }
}