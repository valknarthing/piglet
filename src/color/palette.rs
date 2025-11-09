"#ffff00".to_string(),
        ]).unwrap()
    }
    
    /// Create ocean palette
    pub fn ocean() -> Self {
        Self::from_strings(&[
            "#000080".to_string(),
            "#0000ff".to_string(),
            "#4169e1".to_string(),
            "#87ceeb".to_string(),
            "#add8e6".to_string(),
        ]).unwrap()
    }
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self::rainbow()
    }
}