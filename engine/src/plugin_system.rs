use anyhow::Result;

/// Plugin execution stages
#[derive(Clone, Copy)]
pub enum PluginStage {
    PreEncrypt,
    PostDecrypt,
}

/// Trait all plugins must implement
pub trait Plugin: Send + Sync {
    fn execute(&self, data: &[u8]) -> Result<Vec<u8>>;
}

/// Plugin registry
pub struct PluginSystem {
    pre_encrypt: Vec<Box<dyn Plugin>>,
    post_decrypt: Vec<Box<dyn Plugin>>,
}

impl PluginSystem {
    pub fn new() -> Self {
        Self {
            pre_encrypt: Vec::new(),
            post_decrypt: Vec::new(),
        }
    }

    pub fn register(&mut self, stage: PluginStage, plugin: Box<dyn Plugin>) {
        match stage {
            PluginStage::PreEncrypt => self.pre_encrypt.push(plugin),
            PluginStage::PostDecrypt => self.post_decrypt.push(plugin),
        }
    }

    pub fn apply_pre_encrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        let mut buf = data.to_vec();
        for p in &self.pre_encrypt {
            buf = p.execute(&buf)?;
        }
        Ok(buf)
    }

    pub fn apply_post_decrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        let mut buf = data.to_vec();
        for p in &self.post_decrypt {
            buf = p.execute(&buf)?;
        }
        Ok(buf)
    }
}
